# Testing plan

Spec 1 has been approved for implementation and now has an offline Rust suite.
Receiver integration still needs its own later reviewed spec.

## What runs today

[Repository checks](../.github/workflows/security.yml) runs strict OpenSpec
validation, security guard tests/history scans, Rust formatting, clippy and the
complete `cargo test --workspace --locked` on every push and pull request. The
pinned Rust 1.99.0 toolchain and Cargo.lock are used on clean hosted runners.
The wrapper `bash scripts/testing/offline-suite.sh` preserves Cargo's exit status,
fails on absent/ignored/filtered acceptance tests or fewer than 17 scenarios and produces a safe synthetic
summary plus test log. The allowlisted artifact has seven-day retention and is
uploaded on success or failure. No `continue-on-error`, credentials or network
calls are used in the domain suite. Dependency/tool downloads are setup only.

## Spec 1 scenario coverage

All scenarios are exercised in `crates/chargeshare-core/tests/offline_spec1.rs`:

- Two interleaved vehicles, independent 10/4 kWh counter boundaries and independent
  confirmation: `interleaved_ten_and_four_are_separate_and_independently_confirmed`
- Unknown/missing identities and ambiguous registration:
  `rejected_identity_and_duplicate_registration_leave_all_state_unchanged`
- Cross-vehicle review and independent reads:
  `scoped_reads_and_cross_vehicle_review_never_mutate_other_vehicle`
- Duplicates/late evidence:
  `shuffled_duplicates_and_late_evidence_preserve_ids_reviews_flags_and_totals`
- Matching cross-vehicle events:
  `identical_events_across_vehicles_are_not_deduplicated_together`
- Conflicting evidence and interior uncertainty barriers:
  `same_position_conflicts_hold_all_affected_connections_independent_of_arrival`
  and `interior_conflicts_break_counter_chain_and_retain_all_safe_evidence_reasons`
- Pause/resume and new connection:
  `pause_resume_stays_in_connection_and_new_connection_never_bridges_counters`
- DC/ambiguous evidence:
  `dc_ambiguous_and_mixed_type_evidence_is_explicitly_excluded`
- Bad/rollback counter and confirmation cannot override evidence:
  `rollback_counts_only_valid_positive_deltas_and_confirmation_cannot_clear_flags`
  and `invalid_negative_precision_and_overflow_counters_retain_safe_flags_no_invented_delta`
- Missing start/end/baseline/terminal and silence:
  `missing_baseline_terminal_and_boundaries_stay_visible`
  and `silence_never_fabricates_end_time_baseline_or_consumption`
- Exact parsing, accumulation overflow and deterministic tie/boundary ordering:
  the remaining arithmetic and boundary tests
- Run without Tesla access: all fixtures construct only fictional domain values;
  there are no network or credential dependencies in Cargo.toml

See [domain contract](offline-ledger.md) for precision, synthetic input rules,
ordering/conflict policy and physical/security limitations. Actual publication
checks and deliberate-failure evidence are recorded in [verification](spec1-verification.md).

## Future receiver integration: separate spec

Proposed path: fake vehicle → actual official Tesla Go receiver → selected
supported dispatcher/broker → Rust adapter → vehicle ledger. This tests transport
and decoding as well as accounting; a direct JSON injection into Rust is not an
end-to-end receiver test. Broker selection and adapter details are still open.

Carry this checklist into the later integration spec before building it:

- [ ] Select the dispatcher and pin the official receiver to a reviewed immutable commit or image digest; pin compatible protocol tools, dependencies and container images, and document the exact reproducible local command
- [ ] Adapt Tesla's test client into deterministic, multi-vehicle AC charging fixtures with distinct identities, event IDs, timestamps, counters and state transitions; verify the same 10 kWh / 4 kWh ledger expectations across the real receiver boundary
- [ ] Start the actual Go receiver, selected dispatcher and Rust consumer in isolated local containers on a fresh GitHub-hosted runner; generate ephemeral test-only CA/server/client certificates, wait for health checks and exercise authenticated WSS without any Tesla endpoint or account
- [ ] Test duplicate/out-of-order evidence, invalid/unknown vehicles and restart/reconnect behavior under the selected dispatcher's delivery semantics; assert ledger isolation, no double counting and explicit incomplete-data flags
- [ ] Add a GitHub Actions integration job on every pull request and push, with bounded startup/test timeouts and cleanup even on failure; assert that untrusted test certificates are rejected; prove unexpected handshake failures, unavailable dependencies and failed assertions fail the job rather than skip it
- [ ] Verify the full suite from a clean runner with no developer state or repository secrets; publish a safe result summary and allowlisted failure diagnostics, then record a passing final-commit run before calling the integration complete

Tesla's Fleet Telemetry test client (`test/integration/server_test.go` upstream)
constructs protobuf payloads inside FlatBuffers messages and uses WSS with local
certificates. Its example payload contains name/location fields, not a complete
charging simulation. Adaptation must replace those fields with synthetic charging
evidence and omit location. The upstream `Makefile` provides certificate-generation
and container integration entry points. A later approved integration spec must
record the reviewed revision and exact test setup; these upstream examples do not
test ChargeShare or authorize work beyond the current offline suite.

## CI isolation and failure evidence

Both automated suites must fail their CI job on failure; do not mask failures
with `continue-on-error`. Keep human-readable test counts/results in the run
summary and upload only allowlisted synthetic diagnostics when useful, including
on failure. Pin the artifact action and define a short retention period when
implementing it. A deliberate assertion failure followed by a restored passing
run demonstrates the gate. Requiring these checks in branch protection is a
separate repository setting; this document does not configure it.

Test certificate trust is isolated to the disposable test network. Never add the
test CA to production trust, disable TLS verification, use production keys,
configure OAuth, pair a real vehicle, expose a public listener or access Tesla
accounts from CI. Generate private test keys only in temporary runtime storage;
do not commit them or include keys, certificates, credential files or unrestricted
container dumps in uploaded artifacts. Clean up containers and temporary state
on success and failure. Dependency/image downloads during setup are distinct
from the isolated test traffic itself.

## Manual real-car validation gate

A future, separately approved supervised trial must confirm actual vehicle signal
support, counter/reset semantics, session boundaries, final samples and physical
measurement limits. Agree on private evidence handling and any independent meter
comparison first. Real charging, OAuth/key pairing, deployment and costs are not
automated CI tests and are not authorized by this plan. Passing simulated tests
proves software behavior for those fixtures, not utility-meter accuracy.
