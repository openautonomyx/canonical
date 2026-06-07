//! `edge` — a std-only command line at the edge of the canonical core.
//!
//!   edge keygen           generate an identity (seed + id)
//!   edge id <seed>        derive the public id from a base64url seed
//!   edge demo             a guided allow/deny tour of the whole core
//!
//! Zero third-party dependencies: it is the same full-stack discipline as the
//! core. Argument parsing is a match, not a framework.

use std::collections::BTreeMap;
use std::process::ExitCode;

use canonical_core::{
    attenuate, b64url, b64url_decode, compose, now_ms, ComposeParams, Core, CorsGrant, Json,
    Outcome, Signer,
};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    let result = match cmd {
        "keygen" => keygen(),
        "id" => id(args.get(1)),
        "demo" => demo(),
        "help" | "-h" | "--help" => {
            usage();
            Ok(())
        }
        other => {
            eprintln!("unknown command: {other}\n");
            usage();
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn usage() {
    println!("edge — the canonical core at the edge\n");
    println!("usage:");
    println!("  edge keygen        generate an identity (seed + id)");
    println!("  edge id <seed>     derive the public id from a base64url seed");
    println!("  edge demo          a guided allow/deny tour of the core");
}

fn read_seed() -> std::io::Result<[u8; 32]> {
    use std::io::Read;
    let mut seed = [0u8; 32];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut seed)?;
    Ok(seed)
}

fn keygen() -> std::io::Result<()> {
    let seed = read_seed()?;
    let signer = Signer::from_seed(seed);
    println!("seed {}", b64url(&seed));
    println!("id   {}", signer.id);
    println!("\nkeep the seed secret; share the id freely (it grants nothing on its own).");
    Ok(())
}

fn id(seed_arg: Option<&String>) -> std::io::Result<()> {
    let seed_b64 = seed_arg.ok_or_else(|| err("usage: edge id <seed>"))?;
    let bytes = b64url_decode(seed_b64).ok_or_else(|| err("seed is not valid base64url"))?;
    let seed: [u8; 32] = bytes.try_into().map_err(|_| err("seed must be 32 bytes"))?;
    println!("{}", Signer::from_seed(seed).id);
    Ok(())
}

fn err(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, msg.to_string())
}

fn render(out: &Outcome) -> String {
    if out.decision.ok {
        let r = out.result.as_ref().map(render_json).unwrap_or_default();
        format!("ALLOW  {r}")
    } else {
        format!(
            "DENY   ({})",
            out.decision.reason.clone().unwrap_or_default()
        )
    }
}

fn render_json(j: &Json) -> String {
    canonical_core::canonical_string(j)
}

fn invoke(
    from: &Signer,
    to: &str,
    resource: &str,
    payload: Json,
    grant: Option<CorsGrant>,
) -> std::io::Result<canonical_core::Message> {
    compose(
        from,
        ComposeParams {
            to,
            action: "invoke",
            resource,
            payload,
            grant,
        },
        now_ms(),
    )
}

fn demo() -> std::io::Result<()> {
    println!("Canonical Autonomyx — edge demo");
    println!("===============================\n");

    let alice = Core::create()?;
    let mut bob = Core::create()?;
    bob.expose("tool/echo", |payload| {
        let mut m = BTreeMap::new();
        m.insert("echo".to_string(), payload);
        Json::Obj(m)
    });
    let echo = bob.resource("tool/echo");

    // 1. no contract -> denied. trust is not based on origin.
    let m = invoke(alice.signer(), bob.id(), &echo, Json::str("hi"), None)?;
    println!(
        "1. invoke, no contract            -> {}",
        render(&bob.handle(&m))
    );

    // 2. provider shares (CORS) -> allowed, through the edge.
    let contract = bob.share(alice.id(), "tool/echo", "invoke", vec![]);
    let m = invoke(
        alice.signer(),
        bob.id(),
        &echo,
        Json::str("hi"),
        Some(contract.clone()),
    )?;
    println!(
        "2. invoke with Bob's contract     -> {}",
        render(&bob.handle(&m))
    );

    // 3. attenuated delegation to Carol.
    let carol = Core::create()?;
    let sub = attenuate(
        alice.signer(),
        &contract,
        carol.id(),
        None,
        None,
        vec![],
        now_ms(),
    );
    let m = invoke(
        carol.signer(),
        bob.id(),
        &echo,
        Json::str("via carol"),
        Some(sub),
    )?;
    println!(
        "3. carol via attenuated contract  -> {}",
        render(&bob.handle(&m))
    );

    // 4. delegation cannot widen.
    let wide = bob.resource("tool/*");
    let widened = attenuate(
        alice.signer(),
        &contract,
        carol.id(),
        None,
        Some(&wide),
        vec![],
        now_ms(),
    );
    let secret = bob.resource("tool/secret");
    let m = invoke(
        carol.signer(),
        bob.id(),
        &secret,
        Json::str("x"),
        Some(widened),
    )?;
    println!(
        "4. carol tries to widen it        -> {}",
        render(&bob.handle(&m))
    );

    // 5. replay is rejected.
    let m = invoke(
        alice.signer(),
        bob.id(),
        &echo,
        Json::str("again"),
        Some(contract),
    )?;
    let _ = bob.handle(&m);
    println!(
        "5. replayed message               -> {}",
        render(&bob.handle(&m))
    );

    Ok(())
}
