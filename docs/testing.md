# Testing plan

**Proposed tasks, not implemented tests.** Review Spec 1 before requesting its
implementation. Work on one spec at a time; the receiver integration needs its
own later spec. This PR changes no source code or GitHub Actions workflows.

## What runs today

[Repository checks](../.github/workflows/security.yml) runs strict OpenSpec
validation, security guard tests/history scans, and Rust formatting, clippy and
`cargo test --workspace --locked` on pushes and pull requests. The Rust crate is
empty: passing CI today does not establish charging or receiver behavior.

## Spec 1: offline domain tests

The [active tasks](../openspec/changes/offline-multi-vehicle-ledger/tasks.md)
require the implemented Rust tests to run automatically in GitHub Actions through
the normal workspace command, with no Tesla credentials or network dependency.

- Replay two fictional vehicles and assert separate observed totals of 10 kWh
  and 4 kWh; confirm charger eligibility independently
- Replay duplicates, shuffled/late events and identical cross-vehicle values;
  assert unchanged totals, stable sessions and no cross-vehicle mixing
- Cover missing/invalid counters, rollback, missing ending evidence, pause/resume,
  DC exclusion, unknown identities and cross-vehicle review rejection
- Map every behavioral spec scenario to a test; zero tests, ignored acceptance
  tests or a filtered subset are not milestone completion

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

Tesla's [test client](https://github.com/teslamotors/fleet-telemetry/blob/main/test/integration/server_test.go)
constructs protobuf payloads inside FlatBuffers messages and uses WSS with local
certificates. Its example payload contains name/location fields, not a complete
charging simulation. Adaptation must replace those fields with synthetic charging
evidence and omit location. The upstream [Makefile](https://github.com/teslamotors/fleet-telemetry/blob/main/Makefile)
provides certificate-generation and container integration entry points; inspect
those again at the pinned revision rather than assuming they test ChargeShare.

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
