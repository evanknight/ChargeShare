# Design

## Context

See [proposal](proposal.md) for motivation and scope. The repository is a greenfield planning scaffold with no runtime application. Official Tesla documentation supports an AC-energy telemetry starting point but does not establish utility-meter accuracy or exact tariff timing during outages. The broader architecture and source references are in [docs/architecture.md](../../../docs/architecture.md) and [docs/initial-plan.md](../../../docs/initial-plan.md).

## Goals / Non-Goals

**Goals:** Keep one-vehicle operation comprehensible, preserve replayable evidence, and separate physical measurement uncertainty from deterministic software calculations. Make incomplete evidence impossible to mistake for a settled reimbursement total.

**Non-Goals:** A production accounting service, a utility meter, automated payments or an autonomous vehicle-control system. The initial repository change provides planning and security tooling only.

## Decisions

1. **Official receiver plus Python and SQLite.** Use Tesla's receiver for transport and authentication rather than recreating the vehicle protocol. Enforce the vehicle allowlist before any raw-payload sink and disable unfiltered receiver payload logs. A small processor and persistent SQLite are proportionate to one car. A distributed queue or multi-tenant service adds unproven complexity.
2. **AC counter deltas with immutable evidence.** Prefer ACChargingEnergyIn over battery-energy estimates. Keep counter segments and flags; raw evidence and processor offsets must be crash-recoverable. Replay is event-time ordered and idempotent; receipt-time remains diagnostic.
3. **Manual classification.** Default each session to unconfirmed and require operator confirmation. GPS or charger-account integration can be reconsidered later with explicit data-sharing approval; neither is necessary to demonstrate the ledger.
4. **Versioned tariffs and review decisions.** Use decimal arithmetic, UTC storage and IANA local-rate lookup. Record estimates as estimates. Long cross-rate gaps block automatic finalization; a generic charging-loss multiplier is not introduced.
5. **Private runtime, public source.** Synthetic fixtures live in Git; credentials, real payloads, identities and state live only in private runtime storage. Pre-commit and full-history scanning complement manual review but cannot guarantee absolute secret detection.
6. **Offline-first rollout.** Implement and validate synthetic cases before granting live access. Exact telemetry-configuration scopes, deployment topology and meter comparison are explicit live preflight gates rather than assumptions hidden inside an implementation.

## Risks / Trade-offs

- Missing ending samples or finite vehicle buffer → flag incomplete evidence; test crash/reconnect recovery and do not infer session end from silence
- Meter-boundary mismatch → compare against a permitted independent AC reference or disclose unvalidated accuracy before agreement
- Sparse samples across TOU boundaries → short bounded interpolation with flags; long intervals held or ranged
- Refresh-token rotation or certificate failure → atomic secret storage, collection-health warnings and a documented revoke/recovery procedure
- Local hooks can be absent or bypassed → fail closed when installed, document installation, scan full history in CI and review staged content
- Single-host availability → persistent storage, tested restore/replay and honest outage flags; no claim of lossless delivery until verified

## Migration Plan

There is no existing application or dataset to migrate. First build and test offline. After explicit live-setup approval, provision the minimal host and app, verify scopes and transport, collect one supervised session, and stop if counter semantics are ambiguous. Expand to five reviewed sessions and document accuracy evidence before relying on reimbursement output.

Rollback: stop telemetry configuration/collection using the approved revocation procedure, revoke access if appropriate, stop the private app and retain encrypted evidence for diagnosis under the agreed retention policy. Do not delete evidence or publish diagnostics during rollback.

## Open Questions

Runtime parameters to supply before live setup: actual tariff/currency/timezone, approved host/domain and budget, vehicle capability results, agreed interpolation threshold and physical measurement tolerance. These are deployment/acceptance inputs; the design already requires explicit uncertainty and blocks finalization where they are missing.
