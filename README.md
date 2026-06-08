# Fabric Framework

Fabric Framework is a delivery-first framework for turning intent into evidence-backed work across platforms, agents, tools, apps, runtimes, and SDKs.

This repository publishes the canonical framework artifacts for OpenAutonomyX.

## Core frame

```text
Fabric Framework = theory
PlatformContract = contract authority
Agent = delivery obligation
Box = gap-filling carrier
Evidence = proof
```

## Root rule

```text
Platform defines the contract.
Agent must deliver.
Box carries the gap.
Evidence proves the outcome.
```

## Why Box exists

Box is not the theory.

Box fills the gap between:

```text
contract declared
```

and:

```text
delivery proven
```

A Box carries:

```text
header
manifest
payload
microboxes
evidence
artifacts
```

## Main primitives

```text
PlatformContract
StructuredInstruction
Box
MicroBox
BoxPack
Evidence
Skill
Tool
App
Runtime
Agent
```

## Current implementation

```text
crates/canonical-core  -> Rust validator/core
crates/edge-cli        -> executable edge loop
sdk/go/canonical       -> Go writer/operator SDK start
sdk/js                 -> JS client SDK start
```

## Executable loop

```bash
cargo run -p edge-cli -- run \
  --task examples/tasks/basic-task.json \
  --workspace .canonical/session.jsonl

cargo run -p edge-cli -- replay \
  --workspace .canonical/session.jsonl

cargo run -p edge-cli -- validate \
  --workspace .canonical/session.jsonl

cargo run -p edge-cli -- report \
  --workspace .canonical/session.jsonl \
  --out reports/out/canonical-edge-report.md
```

## Verification

CI gates are defined in:

```text
.github/workflows/ci.yml
```

They cover:

```text
Rust format/build/test/clippy
edge delivery loop
Rust dependency audit
JS build/audit
Go tests
secret scanning
```

A framework claim is valid only after evidence exists.

## Published framework docs

```text
CONTRACTS.md                  -> platform contract and agent delivery obligation
DRIFT_LOG.md                  -> preserved drift evidence

docs/FABRIC_FRAMEWORK.md      -> framework theory
docs/PLATFORM_PROTOCOL.md     -> platform-as-a-service protocol
docs/APP_AS_TOOL.md           -> app as tool boundary
docs/TOOL_AS_SKILL.md         -> tool as governed skill boundary
docs/GO_WRITER.md             -> Go writer/operator boundary
```

## Language binding

```text
Rust -> validator
Go   -> writer/operator
JS   -> application/client SDK
```

## Non-negotiables

```text
Do not erase drift.
Do not claim delivery without evidence.
Do not promote Box into theory.
Do not import platform/package words without approval.
```

## Current status

```text
framework published in repo
CI/CD created
delivery proof pending CI run result
```
