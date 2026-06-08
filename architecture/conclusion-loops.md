# Conclusion Loops

## Core Law

Every conversation, action, task, milestone, target, project, product, and flow must conclude into value.

```text
Start
→ Act
→ Measure
→ Conclude
→ Learn
→ Improve
→ Loop
```

## Why Conclusion Loops Matter

Without conclusion, work leaks.
Without evidence, conclusion is opinion.
Without learning, conclusion does not compound.

```text
Open work must move to:
  completed
  blocked
  escalated
  deferred
  cancelled
  archived
  improved
```

## Universal Conclusion Loop

```text
Intent
→ Target
→ Timeline
→ Milestone
→ Task
→ Action
→ Output
→ Outcome
→ Evidence
→ Conclusion
→ Status Update
→ Next Loop
```

## Conclusion Types

```text
Completed
  target achieved, evidence attached

Partially Completed
  some milestones done, gap remains

Blocked
  cannot proceed without dependency, decision, data, approval, or party

Escalated
  moved to authority because SLA, risk, cost, or mandate requires attention

Deferred
  intentionally moved later with reason and owner

Cancelled
  stopped with business reason and audit trail

Failed
  attempted and failed, with remediation path

Archived
  closed historically with evidence retained

Improved
  loop produced a better flow, artifact, policy, or agent behavior
```

## Conclusion Object

```yaml
conclusion:
  id: conclusion_001
  tenant: acme
  workspace: product
  subject_type: target
  subject_id: launch_checkout_v2
  status: partially_completed

  summary: Release readiness reached 80%, security review still pending.

  evidence:
    - milestone_report
    - test_result
    - security_review_request

  blockers:
    - security_approval_pending

  next_actions:
    - escalate_security_review
    - generate_daily_status
    - revise_release_timeline

  owner: product_manager
  audit_required: true
```

## Target Conclusion Loop

```text
Target created
→ milestones defined
→ tasks created
→ actions executed
→ progress measured
→ conclusion generated
→ owner reviews
→ next target or next loop starts
```

## Project Conclusion Loop

```text
Project plan
→ milestone execution
→ status review
→ risk/issue review
→ delivery review
→ closure report
→ benefits review
→ archive evidence
→ improvement backlog
```

## Product Conclusion Loop

```text
Roadmap target
→ feature work
→ release readiness
→ launch
→ adoption measurement
→ feedback review
→ product conclusion
→ next roadmap loop
```

## Agent Conclusion Loop

```text
Agent receives target
→ acts through approved tools
→ collects evidence
→ generates status
→ identifies conclusion state
→ asks for human approval if needed
→ records audit
→ updates memory map
```

## Conversation Conclusion Loop

```text
User intent
→ useful output
→ artifact / action / decision
→ conclusion
→ next step
→ memory / repo update
```

A conversation is incomplete until one of these happens:

```text
artifact created
flow drafted
decision recorded
action triggered
next step defined
risk reduced
ambiguity reduced
```

## SurrealDB Schema Additions

```sql
DEFINE TABLE conclusion SCHEMAFULL;
DEFINE FIELD tenant ON conclusion TYPE record<tenant>;
DEFINE FIELD workspace ON conclusion TYPE option<record<workspace>>;
DEFINE FIELD subject_type ON conclusion TYPE string;
DEFINE FIELD subject_id ON conclusion TYPE string;
DEFINE FIELD status ON conclusion TYPE string;
DEFINE FIELD summary ON conclusion TYPE string;
DEFINE FIELD evidence_refs ON conclusion TYPE array<record<evidence>> DEFAULT [];
DEFINE FIELD blockers ON conclusion TYPE array<object> DEFAULT [];
DEFINE FIELD next_actions ON conclusion TYPE array<object> DEFAULT [];
DEFINE FIELD owner ON conclusion TYPE option<record<person> | record<agent> | record<team>>;
DEFINE FIELD audit_required ON conclusion TYPE bool DEFAULT true;
DEFINE FIELD created_at ON conclusion TYPE datetime DEFAULT time::now();

DEFINE TABLE loop_event SCHEMAFULL;
DEFINE FIELD tenant ON loop_event TYPE record<tenant>;
DEFINE FIELD workspace ON loop_event TYPE option<record<workspace>>;
DEFINE FIELD loop_type ON loop_event TYPE string;
DEFINE FIELD subject_type ON loop_event TYPE string;
DEFINE FIELD subject_id ON loop_event TYPE string;
DEFINE FIELD from_state ON loop_event TYPE option<string>;
DEFINE FIELD to_state ON loop_event TYPE string;
DEFINE FIELD actor ON loop_event TYPE option<record<person> | record<agent> | record<service_account>>;
DEFINE FIELD evidence_refs ON loop_event TYPE array<record<evidence>> DEFAULT [];
DEFINE FIELD created_at ON loop_event TYPE datetime DEFAULT time::now();
```

## UI Placement

```text
Workspace
├── Open Loops
├── Closing Today
├── Blocked Loops
├── Escalated Loops
├── Completed Loops
├── Conclusions
├── Next Actions
└── Evidence
```

## Conclusion Loop Score

```text
open loops
closed loops
blocked loops
escalated loops
overdue loops
average close time
conclusion quality
evidence completeness
next-action clarity
audit readiness
```

## Final Law

```text
No target without conclusion.
No milestone without status.
No task without owner.
No action without evidence.
No conclusion without next step.
No loop without learning.
```

## Product Statement

Canonical Work OS uses Conclusion Loops to ensure every conversation, target, milestone, task, project, product, flow, and agent action ends in a clear state, evidence, next action, and learning loop toward excellence.
