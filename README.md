# OpenAutonomyX Canonical

OpenAutonomyX Canonical defines the smallest stable core of the ecosystem.

## Canonical Core

```text
Identity
    ↓
Contract
    ↓
ORB
    ↓
Outcome
```

ORB is the canonical operational model.

```text
Observe
    ↓
Reason
    ↓
Behave
```

## Principle

Every autonomous identity operates through a contract.
Every contract executes through ORB.
Every ORB cycle produces an outcome.

## What ORB Means

### Observe

Observe answers: what is true now?

It gathers context, state, signals, evidence, memory, and events.

### Reason

Reason answers: what should happen?

It evaluates objectives, constraints, alternatives, policy, risk, trust, and approvals.

### Behave

Behave answers: what will happen?

It performs actions, tool calls, transactions, publications, delegations, and state changes.

## What Does Not Belong Here

This repository does not define runtimes, databases, products, UI, deployment architecture, or framework-specific behavior.

Frameworks implement ORB. They do not define ORB.

## Repository Boundary

```text
openautonomyx/canonical
    = canonical contracts

openautonomyx/foundation
    = explanation, theory, mappings, reference models

AGenNext/*
    = implementation

unboxd.cloud
    = delivery
```

## Canonical Statement

OpenAutonomyX is the identity.
Domains are contracts.
ORB is the execution model.
Outcomes are the result.
