# Canonical Contracts

The platform defines the contract.

The Box conforms to the contract.

The agent must deliver.

## Root Contract

```text
PlatformContract
```

The PlatformContract is the authority for:

```text
identity
scope
instruction
box
microbox
boxpack
evidence
policy
skill
tool
app
runtime
agent
delivery
language binding
```

## Delivery Rule

An agent is not valid because it reasons.

An agent is valid only when it delivers.

Delivery means:

```text
accepted instruction
bounded execution
tangible output
evidence written
outcome reported
failure recorded when delivery fails
```

## Contract Rule

```text
Platform defines.
Box carries.
Agent delivers.
Runtime executes.
Evidence proves.
SDKs implement.
```

## Contract Ownership

```text
PlatformContract -> owns canonical meaning
Box              -> carries contract reference
MicroBox         -> carries bounded instruction execution
BoxPack          -> carries portable Box state
Agent            -> accepts responsibility to deliver outcome
Skill            -> exposes governed capability
Tool             -> implements capability
App              -> composes tools
Runtime          -> executes instructions
SDK              -> writes/reads contract-compatible data
Evidence         -> proves delivery or failure
```

## Agent Contract

An agent must declare:

```text
identity
scope
accepted instruction
required skills
tool permissions
expected output
evidence obligations
failure mode
delivery boundary
```

An agent must produce one of:

```text
delivered
failed_with_evidence
rejected_with_reason
```

An agent must not produce:

```text
silent failure
unbounded execution
unverified output
reasoning without delivery
```

## Box Contract

A Box must carry:

```text
header
manifest
payload
contract reference
evidence boundary
```

The Box does not define the platform contract.

The Box carries it.

## MicroBox Contract

A MicroBox must carry:

```text
seq
instruction
state
output
evidence
```

A MicroBox is valid only when sequence and evidence remain replayable.

## Evidence Contract

Evidence must prove one of:

```text
instruction accepted
execution started
tool called
skill applied
output produced
failure occurred
delivery completed
```

No evidence means no delivery claim.

## Platform Service Contract

The platform service must provide:

```text
create.box
instruct.box
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

## Language Contract

The same PlatformContract must be implemented across:

```text
Rust -> validator
Go   -> writer/operator
JS   -> app/client SDK
```

## Final Rule

A platform without delivery is only infrastructure.

An agent without delivery is only conversation.

A Box without evidence is only data.
