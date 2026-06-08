# Useful Conversation Value Agent Law

## Core Law

Every conversation must be useful.
Every agent must add value.

```text
Conversation
→ understand intent
→ resolve context
→ identify target
→ perform useful action
→ produce artifact / decision / next step
→ record evidence
→ improve memory
```

## Conversation Value Standard

A conversation is useful only if it produces at least one of:

```text
clearer decision
usable artifact
running action
reduced ambiguity
next step
saved time
reduced risk
better evidence
better flow
better configuration
```

## Agent Value Standard

An agent adds value only if it:

```text
moves a target forward
reduces work for the user
improves quality
reduces cost
reduces risk
increases speed
creates reusable knowledge
creates evidence
prepares an approved artifact
improves a flow
```

## Conversation Object Model

```yaml
conversation_value_event:
  id: conv_value_001
  tenant: acme
  workspace: product
  actor: employee_or_agent
  target: launch_checkout_v2
  conversation_type: planning

  intent:
    detected: define_agent_model
    confidence: high

  value_delivered:
    - architecture_rule
    - data_model
    - github_artifact
    - next_step

  output:
    artifact_type: architecture_doc
    artifact_path: architecture/target-oriented-agents.md

  audit:
    write_event: true
```

## Agent Response Contract

Every professional agent response should try to include:

```text
1. What this means
2. Where it fits
3. What action it enables
4. What artifact or model is produced
5. What the next build step is
```

## No-Value Patterns to Avoid

```text
long generic explanation
unsupported claim
tool list without action
module list without outcome
workflow without business logic
agent without scope
recommendation without evidence
conversation without next step
```

## Value Loop

```text
User signal
→ agent interprets target
→ agent produces useful output
→ output becomes artifact / flow / decision / issue
→ user approves or corrects
→ agent updates model
→ next conversation starts stronger
```

## Final Law

```text
No conversation without target.
No target without movement.
No movement without output.
No output without value.
No value without evidence.
```

## Product Statement

Canonical Work OS treats conversation as an operating surface: every useful conversation should move a target, create or improve an artifact, trigger a governed action, or generate evidence that compounds into future value.
