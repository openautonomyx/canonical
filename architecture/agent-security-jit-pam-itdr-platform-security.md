# Agent Security Model: JIT + PAM + ITDR + Secure Platform

## Core Law

The platform is secure when every agent operates under:

```text
JIT Access + PAM Control + ITDR Monitoring + Fabric Governance
```

No agent has permanent trust.
No agent has unrestricted access.
No action is outside policy.

---

## 1. JIT (Just-In-Time Access)

```text
Access is not persistent.
Access is granted per request.
Access expires automatically.
```

Agent requests:
- context
- tools
- data
- node access

All are time-bound.

```text
Agent → request → policy check → temporary grant → execution → revoke
```

---

## 2. PAM (Privileged Access Management)

```text
Privileged actions require elevation.
Elevation requires approval or policy match.
Elevation is scoped and temporary.
```

Examples:
- payment execution
- production deployment
- identity changes
- data export

```text
No standing privilege.
All privilege is session-bound.
```

---

## 3. ITDR (Identity Threat Detection & Response)

```text
All agent behavior is continuously monitored.
```

Signals:
- abnormal access patterns
- policy violations
- unusual tool usage
- cross-tenant attempts
- escalation anomalies

Actions:
- alert
- throttle
- revoke session
- isolate agent
- require re-auth

---

## 4. Platform Security Model

```text
Identity → OpenFGA graph
Policy → OPA rules
Execution → Agent runtime
Data → SurrealDB/Postgres
Tools → Registry
Audit → immutable log
```

Every action passes through all layers.

---

## 5. Agent Execution Security Flow

```text
Intent received
→ Identity verified
→ OpenFGA authorization check
→ Policy evaluation (OPA)
→ JIT context assembly
→ PAM elevation check (if needed)
→ Tool selection
→ Execution allowed
→ ITDR monitoring active
→ Evidence recorded
→ Audit log written
→ Access revoked
```

---

## 6. Threat Boundaries

```text
Agent cannot:
- persist privilege
- bypass OpenFGA graph
- access unauthorized tools
- operate outside session scope
- hide execution trace
```

---

## 7. Fabric Security Principle

```text
Security is not a layer.
Security is the fabric constraint system.
```

---

## Final Law

```text
The platform is secure only when:

JIT limits access
PAM controls privilege
ITDR detects anomalies
Fabric enforces policy
Audit proves every action
```

---

## One-line Statement

> The Canonical Work OS enforces security by combining JIT access, PAM-controlled privilege elevation, and ITDR-driven continuous monitoring within a policy-governed fabric where every agent action is time-bound, scoped, and fully auditable.