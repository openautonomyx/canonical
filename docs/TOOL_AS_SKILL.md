# Tool as Skill

Tools become skills when they can be invoked through structured instructions and return evidence into a Box.

The Box remains the unit.

The tool is not the platform.

The skill is the controlled capability boundary.

## References

Chaos Mesh reference:

```text
https://github.com/chaos-mesh/chaos-mesh
```

Vault reference:

```text
https://github.com/hashicorp/vault
```

These are references for capability boundaries only.

UAF does not copy either project.

## Core idea

```text
Tool -> wrapped by Skill -> invoked by instruct.box -> evidence appended to Box
```

## Chaos Mesh mapping

Chaos Mesh is a reference for controlled failure injection as a skill.

```text
Chaos experiment  -> Skill action
Experiment scope  -> Box scope
Blast radius      -> Box constraint
Experiment result -> Box evidence
Recovery signal   -> Box outcome
```

Canonical skill examples:

```text
inject.latency
inject.failure
verify.recovery
measure.resilience
```

## Vault mapping

Vault is a reference for secret and trust-boundary management as a skill.

```text
Secret engine      -> Skill provider
Policy             -> Box constraint
Token/lease        -> Box scoped access
Audit log          -> Box evidence
Secret rotation    -> Box lifecycle action
```

Canonical skill examples:

```text
read.secret
issue.credential
rotate.secret
verify.access
revoke.access
```

## Skill contract

A tool becomes a skill only when it declares:

```text
name
provider
capability
input schema
output schema
scope
constraints
permissions
evidence emitted
failure mode
replay behavior
```

## Invocation path

```text
instruct.box
    -> select skill
        -> call tool
            -> collect evidence
                -> append MicroBox
                    -> validate Box
```

## Box boundary

The skill may perform external work.

The Box records canonical truth.

```text
external tool state is not trusted until evidence is written into Box
```

## Rule

Tools are replaceable.

Skills are governed.

Box is canonical.
