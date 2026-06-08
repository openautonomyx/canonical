# Security as a Service (SECaaS) in the Fabric

## Core Law

Security is not a product.
Security is a continuously enforced service inside the fabric.

```text
Security = JIT + PAM + ITDR + Policy + Audit
Delivered as a Service
```

---

## 1. Security as a Service Model

```text
Identity → continuously verified
Access → just-in-time granted
Privilege → temporarily elevated
Actions → policy checked
Behavior → continuously monitored
Evidence → continuously recorded
```

Security is always active, never static.

---

## 2. Service Layers

### Identity Service
```text
DID / IAM / SSO / SCIM
verifies who the agent or user is
```

### Authorization Service (OpenFGA)
```text
relationship graph determines access
can(subject, action, object)
```

### Policy Service (OPA)
```text
defines rules for execution
blocks unsafe actions
```

### Privilege Service (PAM)
```text
controls elevated access
session-bound privileges only
```

### Threat Detection (ITDR)
```text
monitors behavior in real-time
detects anomalies and drift
```

### Audit Service
```text
immutable logs of all actions
evidence generation per event
```

---

## 3. Security Execution Flow

```text
Intent
→ Identity verification
→ OpenFGA authorization check
→ Policy evaluation (OPA)
→ JIT context assembly
→ PAM elevation (if needed)
→ Tool execution
→ ITDR monitoring
→ Evidence capture
→ Audit log write
→ Access revoke
```

---

## 4. Security Properties

```text
No persistent trust
No standing privilege
No blind execution
No unlogged action
No out-of-policy behavior
```

---

## 5. Agent Security Role

```text
Agent does NOT own security
Agent executes within security constraints
Agent requests access JIT
Agent is continuously monitored
Agent is revoked if anomaly detected
```

---

## 6. Fabric Security Principle

```text
Security is not a perimeter.
Security is a distributed enforcement graph.
```

Every node enforces security locally.
Every decision is globally auditable.

---

## 7. Threat Containment Model

```text
Anomaly detected
→ session throttled
→ privilege revoked
→ execution paused
→ escalation triggered
→ audit snapshot created
```

---

## 8. Delivery Guarantee

Security as a Service ensures:

```text
✔ every action is authorized
✔ every privilege is time-bound
✔ every execution is monitored
✔ every event is logged
✔ every anomaly is contained
```

---

## Final Law

```text
Security is not added to the system.
Security is the system.
```

---

## One-line Statement

> Security as a Service in the Canonical Fabric continuously enforces identity, authorization, policy, privilege, monitoring, and audit as a unified runtime layer ensuring every agent action is just-in-time, policy-bound, and fully observable.