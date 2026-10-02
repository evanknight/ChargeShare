# Tasks

Proposed implementation checklist only. All work is unimplemented and gated on
review followed by a separate explicit implementation request. Completing the
planning artifacts or merging this spec PR does not authorize implementation.
There are no live setup tasks in this milestone.

## 1. Scoped offline inputs

- [ ] 1.1 Add Rust domain types for synthetic owners, vehicles, connections and events; verify duplicate registration and missing/unknown identity tests leave state unchanged, and document accepted synthetic input semantics
- [ ] 1.2 Add checked exact energy representation; verify negative, invalid, unsupported-precision and overflow inputs fail explicitly, and document supported precision

## 2. Session replay

- [ ] 2.1 Implement vehicle-partitioned replay and deduplication with stable connection/session identities; verify interleaved, shuffled, duplicate, same-position conflict and identical-cross-vehicle fixtures, and document ordering and conflict policy
- [ ] 2.2 Implement conservative AC deltas and connection grouping; verify pause/resume, new connections, DC/ambiguous type, rollback, missing baseline/end and silence scenarios with explicit flags, and document synthetic versus real-telemetry limits

## 3. Energy accounting isolation

- [ ] 3.1 Add scoped session reads and manual charger classification; verify a cross-vehicle review fails without mutation and confirmation cannot clear evidence flags, and document that scope checks are not authentication
- [ ] 3.2 Add separate per-vehicle observed and eligible kWh summaries with held/excluded reasons; verify the 10 kWh / 4 kWh example and unconfirmed defaults, and document that no money owed or certified measurement is produced

## 4. Integrated acceptance

- [ ] 4.1 Run the complete synthetic replay suite without network or credentials; verify every Spec 1 scenario passes and repeat/shuffle runs preserve session identities, flags and totals
- [ ] 4.2 Run Rust formatting, clippy, workspace tests, strict OpenSpec validation and staged/history secret scans; inspect the exact diff and document actual results before requesting implementation review
