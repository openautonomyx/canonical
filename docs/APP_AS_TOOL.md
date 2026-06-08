# App as Tool

Apps are tools around Box.

The Box remains the unit.

The platform does not become the app.

## Reference

Cortex is used as a reference for operating a distributed, horizontally scalable service with clear API, storage, query, and runtime boundaries.

Reference:

```text
https://github.com/cortexproject/cortex
```

UAF does not copy Cortex.

UAF borrows the separation:

```text
API boundary
runtime service boundary
storage boundary
query boundary
multi-tenant operation
observability-first operation
```

## Mapping

```text
Cortex API/query surface   -> App tool surface
Cortex storage separation  -> BoxPack / Box storage boundary
Cortex multi-tenancy       -> Box namespace + scope
Cortex ingestion path      -> instruct.box / append.box
Cortex query path          -> query.box / resolve.box
Cortex metrics discipline  -> Box evidence and platform telemetry
```

## Rule

An app is not the platform.

An app is a tool the platform can use.

```text
Box -> unit
Platform Protocol -> control plane
App -> tool
```

## App tool contract

An app can become a platform tool only when it declares:

```text
name
capability
input contract
output contract
state boundary
tenant boundary
evidence emitted
failure mode
observability surface
```

## Platform shape

```text
Platform Protocol
    -> App Tool Adapter
        -> App Service
            -> Evidence
                -> Box
```

## Canonical action path

```text
instruct.box
    -> call app tool
        -> collect evidence
            -> append MicroBox
                -> verify Box
```

## Non-negotiable

Apps are replaceable.

Box is not.

The protocol must make apps useful without making them foundational.
