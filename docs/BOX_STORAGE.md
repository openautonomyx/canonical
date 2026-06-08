# Box Storage

A box is one unit of block storage.

It is the smallest portable storage unit that can hold canonical work.

## Unit

```text
1 box = 1 block storage unit
```

## Box contains

```text
identity
execution
evidence
outcome
metadata
manifest
```

## Box must support

```text
write
append
read
verify
replay
report
ship
retire
```

## Storage rule

A box stores work without losing proof.

```text
no evidence loss
no identity loss
no execution drift
no outcome ambiguity
```

## Platform mapping

```text
canonical-core -> validates box truth
box            -> stores canonical work
canonical-edge -> runs and moves boxes
```

## Builder rule

The builder does not build loose files.

The builder builds boxes.
