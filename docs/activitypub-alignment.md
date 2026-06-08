# ActivityPub Alignment

ActivityPub is treated as a federation adapter, not as canonical core.

## Mapping

| ActivityPub concept | Canonical concept | Layer |
| --- | --- | --- |
| Actor | Identity | Canonical Core |
| Activity | Execution | Runtime / Adapter |
| Object | Evidence / Outcome | Canonical Core |
| Inbox | Contact Adapter | Adapter |
| Outbox | Contact Adapter | Adapter |
| Collection | Workspace View | Adapter / Report |
| Link | Reference | Adapter |

## Rule

Canonical does not become ActivityPub.

ActivityPub can carry canonical events across federated systems.

## Boundary

The canonical core owns:

- identity
- execution record
- evidence
- outcome
- replay

The ActivityPub adapter owns:

- actor representation
- inbox delivery
- outbox delivery
- collection projection
- federation addressing

## Canonical Event Shape

A canonical event may be projected into ActivityStreams JSON-LD when federation is needed.

The workspace remains the source of truth.

## Future Adapter

A future `activitypub-adapter` crate may provide:

- canonical entry to Activity conversion
- Activity to canonical entry ingestion
- inbox/outbox delivery hooks
- actor profile projection
- collection projection

This must not change the canonical core contract.
