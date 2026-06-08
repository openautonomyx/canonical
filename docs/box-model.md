# Box Model

A box is the canonical storage primitive of the platform.

It is one hardware-bounded storage unit for platform truth.

## Box Contents

A box consists of:

```text
packs
users
entities
event streams
```

## Packs

Packs are composable units of capability, context, policy, evidence, reports, tools, or domain knowledge.

Packs are installed into a box.

## Users

Users are identities that interact with the box.

A user may create, read, execute, approve, review, publish, or retire artifacts depending on policy.

## Entities

Entities are the things known, represented, acted on, or governed inside a box.

A box may contain multiple entities.

Entities may represent people, organizations, artifacts, tools, packs, workflows, policies, outcomes, or external references.

Each entity must be addressable and observable through event streams.

## Event Streams

Event streams record what happens inside the box.

Events are append-only.

Events are the source for evidence, replay, reports, and audit.

## Boundary

The box owns storage and truth.

Packs extend capability.

Users create and govern work.

Entities are represented and changed inside the box.

Event streams preserve what happened.

## Canonical Shape

```text
Box
  ├── Packs
  ├── Users
  ├── Entities
  └── Event Streams
```

## Rule

No pack, user, or entity bypasses the event stream.

No event stream bypasses the box.
