# Feature as Data

## Core Law

A feature is data.

```text
Feature = declared capability
Feature Definition = data
Feature Runtime = interpretation of data
Feature Rollout = governed data state
Feature Pricing = metered data rule
Feature Audit = proof data
```

## Why This Matters

Build time becomes negligible because the platform does not hardcode every feature.

```text
Old model:
  feature = code release

Canonical model:
  feature = governed data point interpreted by the platform runtime
```

## Feature Data Model

```yaml
feature:
  id: daily_status_updates
  name: Daily Status Updates
  feature_type: workspace_feature
  lifecycle:
    - workspace
    - project
    - team

  target_outcome:
    - reduce_status_reporting_time
    - improve_visibility
    - close_open_loops

  required_fabrics:
    - data_fabric
    - automation_fabric
    - agent_fabric
    - audit_fabric

  actions:
    - generate_daily_status
    - publish_status_update
    - summarize_blockers
    - recommend_next_actions

  agents:
    - employee_twin_agent
    - team_agent
    - project_agent

  policies:
    - workspace_visibility_policy
    - audit_required_policy

  meters:
    - status_generated
    - agent_runtime_minute
    - report_export

  rollout:
    default_state: disabled
    requires_it_approval: true
    requires_workspace_owner_approval: true

  audit:
    required: true
    evidence:
      - source_tasks
      - running_flows
      - blocker_list
      - published_status
```

## Feature Lifecycle

```text
Proposed
→ Defined
→ Generated
→ Validated
→ Reviewed
→ Approved
→ Published
→ Enabled
→ Measured
→ Improved
→ Deprecated
→ Archived
```

## Feature Categories

```text
Workspace Feature
Lifecycle Feature
Agent Feature
Flow Feature
Connector Feature
Policy Feature
Governance Feature
Audit Feature
Report Feature
DXP Feature
Developer Feature
Marketplace Feature
Pricing Feature
```

## SurrealDB Schema

```sql
DEFINE TABLE feature SCHEMAFULL;
DEFINE FIELD name ON feature TYPE string;
DEFINE FIELD slug ON feature TYPE string;
DEFINE FIELD feature_type ON feature TYPE string;
DEFINE FIELD lifecycle ON feature TYPE array<string> DEFAULT [];
DEFINE FIELD status ON feature TYPE string DEFAULT "proposed";
DEFINE FIELD target_outcome ON feature TYPE array<string> DEFAULT [];
DEFINE FIELD required_fabrics ON feature TYPE array<string> DEFAULT [];
DEFINE FIELD actions ON feature TYPE array<string> DEFAULT [];
DEFINE FIELD agents ON feature TYPE array<string> DEFAULT [];
DEFINE FIELD policies ON feature TYPE array<string> DEFAULT [];
DEFINE FIELD meters ON feature TYPE array<string> DEFAULT [];
DEFINE FIELD rollout ON feature TYPE object DEFAULT {};
DEFINE FIELD audit ON feature TYPE object DEFAULT {};
DEFINE FIELD manifest ON feature TYPE object DEFAULT {};
DEFINE FIELD created_at ON feature TYPE datetime DEFAULT time::now();
DEFINE INDEX feature_slug ON feature COLUMNS slug UNIQUE;

DEFINE TABLE feature_rollout SCHEMAFULL;
DEFINE FIELD tenant ON feature_rollout TYPE record<tenant>;
DEFINE FIELD workspace ON feature_rollout TYPE option<record<workspace>>;
DEFINE FIELD feature ON feature_rollout TYPE record<feature>;
DEFINE FIELD plan ON feature_rollout TYPE option<string>;
DEFINE FIELD enabled ON feature_rollout TYPE bool DEFAULT false;
DEFINE FIELD rollout_stage ON feature_rollout TYPE string DEFAULT "disabled";
DEFINE FIELD config ON feature_rollout TYPE object DEFAULT {};
DEFINE FIELD approvals ON feature_rollout TYPE object DEFAULT {};
DEFINE FIELD enabled_at ON feature_rollout TYPE option<datetime>;
DEFINE FIELD created_at ON feature_rollout TYPE datetime DEFAULT time::now();

DEFINE TABLE feature_usage_event SCHEMAFULL;
DEFINE FIELD tenant ON feature_usage_event TYPE record<tenant>;
DEFINE FIELD workspace ON feature_usage_event TYPE option<record<workspace>>;
DEFINE FIELD feature ON feature_usage_event TYPE record<feature>;
DEFINE FIELD actor ON feature_usage_event TYPE option<record<person> | record<agent> | record<service_account>>;
DEFINE FIELD event_type ON feature_usage_event TYPE string;
DEFINE FIELD meter ON feature_usage_event TYPE option<string>;
DEFINE FIELD quantity ON feature_usage_event TYPE number DEFAULT 1;
DEFINE FIELD metadata ON feature_usage_event TYPE object DEFAULT {};
DEFINE FIELD audit_ref ON feature_usage_event TYPE option<record<audit_event>>;
DEFINE FIELD created_at ON feature_usage_event TYPE datetime DEFAULT time::now();
```

## Pay-As-You-Go Pricing Connection

Feature usage emits metered events.

```text
Feature enabled
→ action performed
→ usage event written
→ meter increments
→ value measured
→ billing line produced
→ audit retained
```

## Product Law

```text
No feature without definition.
No definition without policy.
No policy without audit.
No rollout without approval.
No usage without meter.
No renewal without value.
```

## Final Statement

Canonical Work OS treats every platform capability as a governed data point. Features are declared, approved, enabled, metered, audited, and improved as data, allowing new capabilities to be configured in negligible time without rebuilding the core platform.
