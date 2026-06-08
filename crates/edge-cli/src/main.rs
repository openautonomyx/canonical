use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "canonical-edge")]
#[command(about = "Canonical AutonomyX edge runtime")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run {
        #[arg(long)]
        task: PathBuf,
        #[arg(long)]
        workspace: PathBuf,
    },
    Replay {
        #[arg(long)]
        workspace: PathBuf,
    },
    Validate {
        #[arg(long)]
        workspace: PathBuf,
    },
    Report {
        #[arg(long)]
        workspace: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { task, workspace } => {
            let summary = canonical_core::run(task, workspace)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Commands::Replay { workspace } => {
            let summary = canonical_core::replay(workspace)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Commands::Validate { workspace } => {
            canonical_core::validate(workspace)?;
            println!("valid");
        }
        Commands::Report { workspace, out } => {
            canonical_core::report(workspace, out)?;
            println!("report written");
        }
    }

    Ok(())
}
