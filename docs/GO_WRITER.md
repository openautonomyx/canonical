# Go Writer

Go writer is the orchestration-facing SDK surface for writing canonical boxes.

Rust protects the core.

Go writes, moves, watches, and integrates boxes.

## Position

```text
Rust canonical-core -> validates box truth
Go writer          -> creates and moves boxes
Edge runtime       -> runs boxes
```

## Responsibilities

Go writer owns:

```text
create box
append micro box
write workspace
read workspace
verify box
replay box
emit report
ship box
```

Go writer does not own:

```text
canonical truth rules
protocol versioning authority
box integrity semantics
```

## SDK shape

```go
type BoxWriter interface {
    CreateBox(input CreateBoxInput) (*Box, error)
    AppendBox(id string, input AppendBoxInput) (*MicroBox, error)
    ReadBox(id string) (*Box, error)
    VerifyBox(id string) error
    ReplayBox(id string) (*ReplaySummary, error)
    ReportBox(id string) (*Report, error)
    ShipBox(id string, target ShipTarget) error
}
```

## Instruction root

All writes use structured instruction:

```text
instruct.box
```

Payload carries:

```text
verb
scope
constraints
payload
```

## Rule

Go is the writer.

Rust is the validator.

Box is the unit.
