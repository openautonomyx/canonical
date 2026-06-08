# Platform Protocol

Platform Protocol defines how a platform service creates, controls, moves, verifies, and exposes Boxes.

The Box remains the unit.

The protocol is the contract around the unit.

## Reference

Unleash is used as a platform-as-a-service reference for control-plane discipline:

```text
projects
environments
features
strategies
constraints
metrics
admin API
SDK boundary
```

UAF does not copy Unleash.

UAF borrows the separation:

```text
control plane
runtime SDK
environment boundary
policy/strategy
metrics feedback
```

## Core mapping

```text
Unleash project      -> Box namespace
Unleash environment  -> Box scope
Unleash feature      -> Box capability
Unleash strategy     -> Box direction rule
Unleash constraint   -> Box constraint
Unleash metrics      -> Box evidence
Unleash SDK          -> Box SDK
Unleash API          -> Platform Protocol API
```

## Protocol root

```text
platform.protocol
```

## Box protocol verbs

The protocol exposes structured actions around Box:

```text
instruct.box
create.box
read.box
append.box
verify.box
replay.box
report.box
pack.box
open.box
sign.box
ship.box
store.box
query.box
resolve.box
```

`instruct.box` is the root structured instruction.

Other verbs are protocol operations.

## Platform service boundary

The platform service owns:

```text
namespace
scope
registry
policy
strategy
constraint
identity binding
metrics/evidence collection
SDK keys later
API boundary
```

The platform service does not own:

```text
Box truth
Box schema authority
Box replay semantics
Box integrity contract
```

Those remain canonical-core responsibilities.

## Three-language contract

The Box carries the language contract inside the manifest:

```text
Rust -> canonical validator
Go   -> platform writer/operator
JS   -> application SDK/client
```

The protocol must keep all three aligned to the same Box schema.

## Platform-as-a-Service shape

```text
Console/API
    -> Platform Protocol
        -> Box Registry
            -> BoxPack
                -> Box
                    -> MicroBox
```

## Minimal API

```http
POST   /v1/boxes
GET    /v1/boxes/{id}
POST   /v1/boxes/{id}/instruct
POST   /v1/boxes/{id}/verify
POST   /v1/boxes/{id}/replay
POST   /v1/boxes/{id}/report
POST   /v1/boxes/{id}/pack
POST   /v1/boxes/{id}/sign
POST   /v1/boxes/{id}/ship
GET    /v1/boxes?namespace=&scope=
```

## Rule

Platform Protocol must never turn Box into a feature flag.

Feature-flag platforms are references for control-plane design only.

Box is a self-directed unit of canonical work.
