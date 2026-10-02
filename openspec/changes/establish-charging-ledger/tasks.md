# Tasks

All tasks below are proposed and unimplemented. This scaffold does not authorize live access or deployment.

## 1. Offline evidence and storage

- [ ] 1.1 Add the Python project, pinned dependency setup and synthetic fixture conventions; verify a clean offline development install and document its commands
- [ ] 1.2 Implement normalized event validation and an explicit single-vehicle allowlist; verify malformed fields and unknown vehicles cannot enter storage or logs
- [ ] 1.3 Implement durable evidence persistence, payload deduplication and replay offsets; verify duplicate, late and crash/restart fixture tests produce identical results
- [ ] 1.4 Document data records, private storage boundaries and backup/restore behavior; verify the documented restore procedure against synthetic data

## 2. AC session reconstruction

- [ ] 2.1 Implement cumulative AC counter segments; verify normal deltas, missing baselines, rollback, invalid readings and missing final values
- [ ] 2.2 Implement connection grouping, pauses and AC/DC eligibility; verify pause/resume and reconnection tests prevent double counting
- [ ] 2.3 Implement manual shared-charger confirmation and quality-review states; verify default-unconfirmed and unresolved sessions cannot become billable
- [ ] 2.4 Implement revisioned recomputation and document session semantics; verify a late correction preserves a previously exported revision

## 3. Tariffs and statements

- [ ] 3.1 Implement versioned flat-rate/time-of-use tariff input with currency, IANA timezone and effective dates; verify invalid or overlapping rules are rejected
- [ ] 3.2 Implement decimal energy pricing and an explicit rounding rule; verify invented flat-rate and two-rate examples exactly
- [ ] 3.3 Implement midnight, month-end, DST and effective-date interval splitting; verify skipped and repeated local-hour fixtures count each delta once
- [ ] 3.4 Implement bounded interpolation flags and long-gap review/ranges; verify unknown time-of-use allocation never appears exact and document the chosen threshold
- [ ] 3.5 Implement finalized-statement eligibility and revisions; verify unresolved evidence and unconfirmed sessions remain separate from totals and document accepted uncertainty

## 4. Private review and export

- [ ] 4.1 Add an authenticated private session list with quality flags and review actions; verify unauthorized requests reveal no ledger data
- [ ] 4.2 Add CSV export with aliases, timezone, units, currency and revision metadata; verify quoting, formula-injection handling and exclusion of credentials, real VINs and location
- [ ] 4.3 Add collection-health states and operator guidance; verify simulated storage, token, certificate and billing failures appear without sensitive logs
- [ ] 4.4 Document the review/export workflow and verify its examples against synthetic fixtures

## 5. Approved live preflight and one-session trial

- [ ] 5.1 Obtain explicit setup/deployment/spending approval and record private tariff, timezone, currency, host/domain and budget decisions; verify no live action begins without those decisions
- [ ] 5.2 Verify actual vehicle firmware, signal support and exact telemetry-configuration scopes from current official documentation; record findings privately and stop if required access exceeds approval
- [ ] 5.3 After approval, configure app authorization, virtual-key pairing and atomic token rotation with private secret storage; verify state validation and revocation without exposing secrets
- [ ] 5.4 After approval, deploy the official receiver with authenticated transport and durable storage; verify mTLS/WebSocket behavior, isolation, restart recovery and private dashboard access
- [ ] 5.5 Collect one supervised AC session; verify baseline/end units, reset semantics, arrival behavior and completeness, stopping expansion if evidence is ambiguous
- [ ] 5.6 Document the tested live setup and revoke/stop recovery procedure using public-safe placeholders; verify the procedure privately without publishing raw evidence

## 6. End-to-end measurement acceptance

- [ ] 6.1 Observe at least five reviewed sessions including pause/resume and a rate boundary when applicable; verify complete evidence or explicit unresolved flags for every session
- [ ] 6.2 Compare with a permitted independent AC-energy reference and agreed tolerance, or record unvalidated accuracy and the accepted limitation privately; verify every statement uses the resulting accuracy status
- [ ] 6.3 Run the full replay-to-statement workflow, strict OpenSpec validation and secret/history scans; verify deterministic totals and a reviewed public-safe diff
- [ ] 6.4 Update status docs with actual test results and remaining limitations; archive the change only after implemented behavior and evidence satisfy its acceptance requirements
