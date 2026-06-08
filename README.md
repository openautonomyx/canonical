# Canonical AutonomyX Platform

Canonical AutonomyX is an edge-native platform substrate for recording identity, execution, evidence, and outcome as a replayable workspace.

## Platform Build Block

The first building block is the `platform-build-block`.

It is the smallest reusable unit that records:

```text
Identity
Execution
Evidence
Outcome
```

into a replayable workspace.

The current Rust implementation is in:

```text
crates/canonical-core
```

This crate currently implements the `platform-build-block` contract.

## First Executable Loop

```text
task fixture
  -> canonical workspace
  -> replay
  -> validate
  -> report
```

## Workspace

The workspace is an append-only JSONL file.

Default example path:

```text
.canonical/session.jsonl
```

## Run

```bash
cargo run -p edge-cli -- run \
  --task examples/tasks/basic-task.json \
  --workspace .canonical/session.jsonl
```

## Replay

```bash
cargo run -p edge-cli -- replay \
  --workspace .canonical/session.jsonl
```

## Validate

```bash
cargo run -p edge-cli -- validate \
  --workspace .canonical/session.jsonl
```

## Report

```bash
cargo run -p edge-cli -- report \
  --workspace .canonical/session.jsonl \
  --out reports/out/canonical-edge-report.md
```

## Crates

```text
crates/canonical-core
crates/edge-cli
```

## Boundary

The platform build block owns the substrate.

Adapters, federation, dashboards, cloud deployment, policy engines, and external systems remain outside the block.

## ActivityPub

ActivityPub alignment is documented in:

```text
docs/activitypub-alignment.md
```

ActivityPub is treated as a federation adapter, not as the platform build block.
