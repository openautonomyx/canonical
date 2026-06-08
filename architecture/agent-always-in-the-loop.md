# Agent Always in the Loop

## Core Law

Agent is always in the loop.

```text
Intent
→ Context
→ Agreement
→ Commitment
→ Action
→ Evidence
→ Conclusion
→ Learning
→ Next Loop
```

At every stage, an agent can help, but the agent remains bounded by identity, scope, policy, approval, and audit.

## Why

People need to talk.
Apps need to connect.
Work needs to conclude.
Value needs to be delivered.

The agent keeps the loop alive.

## Agent Responsibilities in the Loop

```text
Before Work
  understand intent
  resolve context
  identify target
  find required parties
  prepare shared context

During Work
  track tasks
  track milestones
  coordinate handoffs
  detect blockers
  recommend next actions
  call approved tools
  update status

Before Decision
  collect evidence
  compare options
  explain tradeoffs
  check policy
  request approval

After Action
  verify outcome
  collect proof
  generate report
  write audit event
  update memory

At Closure
  determine conclusion state
  summarize value delivered
  identify next loop
  recommend improvement
```

## Loop States

```text
Open
In Progress
Waiting Human
Waiting Agent
Waiting App
Waiting Evidence
Waiting Agreement
Blocked
Escalated
Completed
Cancelled
Archived
Improved
```

## Agent Types in the Loop

```text
Employee Twin Agent
  helps the employee move work forward

Team Agent
  keeps team commitments visible

Project Agent
  keeps targets, milestones, blockers, and status alive

Product Agent
  keeps product roadmap and release loops alive

Professional Agent
  performs domain-specific work with evidence

Performance Agent
  measures whether targets improve

Fabric Agent
  builds bridges between apps, people, data, and flows

Governance Agent
  checks policy, approvals, evidence, and audit readiness
```

## Critical Boundary

```text
Agent can keep the loop alive.
Agent cannot fake agreement.
Agent cannot force human acceptance.
Agent cannot bypass policy.
Agent cannot expand its own scope.
Agent cannot erase audit.
```

## Human Dependency

If human involvement is required and no response is given:

```text
Agent marks Waiting Human
→ sends reminder
→ escalates
→ proposes conclusion path
→ records audit
```

The transaction does not disappear.
It becomes blocked, escalated, expired, reassigned, cancelled, or concluded with reason.

## App Dependency

If an app or integration is required and does not respond:

```text
Agent marks Waiting App
→ retries if allowed
→ opens incident
→ escalates to owner
→ records evidence
```

## SurrealDB Schema

```sql
DEFINE TABLE work_loop SCHEMAFULL;
DEFINE FIELD tenant ON work_loop TYPE record<tenant>;
DEFINE FIELD workspace ON work_loop TYPE option<record<workspace>>;
DEFINE FIELD loop_type ON work_loop TYPE string;
DEFINE FIELD target ON work_loop TYPE option<string>;
DEFINE FIELD subject_type ON work_loop TYPE string;
DEFINE FIELD subject_id ON work_loop TYPE string;
DEFINE FIELD owner ON work_loop TYPE option<record<person> | record<team> | record<agent>>;
DEFINE FIELD agent ON work_loop TYPE option<record<agent>>;
DEFINE FIELD status ON work_loop TYPE string DEFAULT "open";
DEFINE FIELD current_state ON work_loop TYPE option<string>;
DEFINE FIELD blockers ON work_loop TYPE array<object> DEFAULT [];
DEFINE FIELD next_actions ON work_loop TYPE array<object> DEFAULT [];
DEFINE FIELD evidence_refs ON work_loop TYPE array<record<evidence>> DEFAULT [];
DEFINE FIELD audit_refs ON work_loop TYPE array<record<audit_event>> DEFAULT [];
DEFINE FIELD created_at ON work_loop TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON work_loop TYPE datetime DEFAULT time::now();

DEFINE TABLE loop_checkpoint SCHEMAFULL;
DEFINE FIELD tenant ON loop_checkpoint TYPE record<tenant>;
DEFINE FIELD work_loop ON loop_checkpoint TYPE record<work_loop>;
DEFINE FIELD checkpoint_type ON loop_checkpoint TYPE string;
DEFINE FIELD status ON loop_checkpoint TYPE string;
DEFINE FIELD summary ON loop_checkpoint TYPE option<string>;
DEFINE FIELD evidence_refs ON loop_checkpoint TYPE array<record<evidence>> DEFAULT [];
DEFINE FIELD next_actions ON loop_checkpoint TYPE array<object> DEFAULT [];
DEFINE FIELD created_by ON loop_checkpoint TYPE option<record<person> | record<agent>>;
DEFINE FIELD created_at ON loop_checkpoint TYPE datetime DEFAULT time::now();
```

## Final Law

```text
No open loop without owner.
No owner without agent support.
No action without evidence.
No blocker without escalation.
No conclusion without next step.
No value without proof.
```

## Product Statement

Canonical Work OS keeps agents always in the loop so every conversation, task, project, product, transaction, delivery, agreement, and workflow moves toward conclusion, value, evidence, and improvement.
