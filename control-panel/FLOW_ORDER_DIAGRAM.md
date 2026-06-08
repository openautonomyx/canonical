# Canonical Work OS Flow Order Diagram

## Product Shell Order

```mermaid
flowchart LR
  Website[Website: sell] --> Docs[Docs: teach]
  Docs --> Auth[Auth: enter]
  Auth --> Console[Console: setup]
  Console --> Marketplace[Marketplace: buy flows]
  Marketplace --> Studio[Studio: build and customize]
  Studio --> Worklist[Worklist: do work]
  Worklist --> Operate[Operate: run and monitor]
  Operate --> Optimize[Optimize: improve]
  Optimize --> Admin[Admin: govern]
  Admin --> Audit[Audit: prove]
  Audit --> Billing[Billing: charge]
  Billing --> Settings[Settings: configure]
```

## Core Flow Lifecycle

```mermaid
flowchart LR
  Blueprint[Blueprint] --> Pack[Flow Pack]
  Pack --> Install[Install]
  Install --> Configure[Configure]
  Configure --> Simulate[Simulate]
  Simulate --> Approve[Approve]
  Approve --> Activate[Activate]
  Activate --> Running[Running Flow]
  Running --> Instance[Flow Instance]
  Instance --> Task[Task / Decision / Agent Step]
  Task --> Output[Output]
  Output --> Audit[Audit]
  Audit --> Optimize[Optimize]
```

## Flow Commerce Lifecycle

```mermaid
flowchart TD
  Buyer[Buyer chooses outcome] --> Marketplace[Browse Marketplace]
  Marketplace --> Detail[Open Flow Pack Detail]
  Detail --> Requirements[Review resources, roles, data, integrations]
  Requirements --> Compatibility{Tenant compatible?}
  Compatibility -- No --> Gap[Show setup gaps]
  Gap --> Requirements
  Compatibility -- Yes --> Install[Install Flow Pack]
  Install --> Configure[Configure tenant, roles, data, policy]
  Configure --> Test[Simulate / test]
  Test --> Approval{Approval required?}
  Approval -- Yes --> Approve[Human approval]
  Approval -- No --> Activate[Activate]
  Approve --> Activate
  Activate --> Run[Run Flow]
  Run --> Monitor[Monitor Running Flows]
  Monitor --> Complete[Complete / produce output]
  Complete --> Audit[Write audit proof]
  Audit --> Measure[Measure value]
  Measure --> Bill[Bill usage / outcome]
```

## Running Flows Visibility

```mermaid
flowchart TD
  Running[Running Flows] --> Dashboard[Dashboard card]
  Running --> Console[Console: tenant running flows]
  Running --> Worklist[Worklist: my running flows]
  Running --> Operate[Operate: all running flows]
  Running --> FlowDetail[Flow detail: instances]
  Running --> TenantDetail[Tenant detail: active work]
  Running --> InstalledPack[Installed pack: active runs]
  Running --> Audit[Audit: execution proof]
```

## Suite Responsibility Map

```mermaid
flowchart LR
  Console[Console\nsetup] --> Marketplace[Marketplace\nbuy]
  Marketplace --> Studio[Studio\nbuild]
  Studio --> Worklist[Worklist\ndo]
  Worklist --> Operate[Operate\nrun]
  Operate --> Optimize[Optimize\nimprove]
  Optimize --> Admin[Admin\ngovern]
  Admin --> Audit[Audit\nprove]
```

## Flow Product Rule

```text
People buy Flow Packs.
Admins install Flow Packs.
Builders configure Flow Packs.
Users execute Flow Runs.
Operators monitor Running Flows.
Executives optimize Flow Value.
Auditors prove Flow Execution.
Billing charges Flow Usage and Outcomes.
```
