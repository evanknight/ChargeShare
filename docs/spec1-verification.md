# Spec 1 implementation verification

Implementation approved and applied on 2026-10-02. Only synthetic offline Rust
behavior is delivered; physical accuracy, user authorization, persistence, actual
Go receiver integration and live-car validation remain outside this milestone.
The historical review/planning context is retained in the active OpenSpec change.

## Local checks

- Rust 1.99.0: formatting and clippy with warnings denied passed
- `cargo test --workspace --locked`: 17 passed, zero failed/ignored/filtered
- `bash scripts/testing/offline-suite.sh`: same complete command and passing summary
- Checked exact six-decimal arithmetic and session/aggregate overflow
- Every behavioral Spec 1 scenario mapped in [testing](testing.md)
- Independent review identified an interior conflicting sample could bridge an
  uncertain AC interval; fixed by conflict barriers retaining all safe evidence
  reasons, with forward/reverse regression coverage

- Strict OpenSpec validation: 1 change passed, zero failed
- Security guard tests: safe scans passed; 17 sensitive paths and a synthetic token blocked
- Staged and all-history Gitleaks 8.30.1 scans: passed, zero leaks
- Exact staged paths/diff reviewed: only public-safe Rust source, fictional fixture
  values, CI script/workflow and project/spec status documentation

These checks are repeated before each publication. Hosted deliberate-failure and
restored passing-run evidence will be recorded below when established.

## Hosted gate evidence

- Initial correct implementation `f4e30001a197dbf4398be259989808ff9c379f74`:
  [push run](https://github.com/jestrada/ChargeShare/actions/runs/37060010078)
  and [PR run](https://github.com/jestrada/ChargeShare/actions/runs/37060017575)
  passed all three jobs from clean hosted runners
- Deliberate wrong-total probe `4fe4a2d9a7815a59a45c2fa6a1f69fdc94bed3c6`
  changed only the expected vehicle-a total from 10 to 11 kWh.
  The normal local test command and CI wrapper both exited 101.
  [Hosted failure run](https://github.com/jestrada/ChargeShare/actions/runs/37060083478)
  failed its Rust job at `Test workspace`: actual 10 kWh, expected 11 kWh;
  16 passed, 1 failed, 0 ignored/filtered. OpenSpec and security still passed.
  The always-running artifact step succeeded, retaining
  `offline-spec1-test-results` (artifact 11250295410, seven-day retention).
  No errors were masked and no environment/credential/real-data dump was uploaded.
- Correct 10 kWh assertion restored; local formatting, clippy and all 17 tests
  passed again. The restored hosted run is checked before completing task 4.4.

Independent read-only review approved the corrected core and CI wrapper; eight
isolated guard smoke cases verified propagation of exit 101 and rejection of
missing, zero, reduced, ignored or filtered acceptance suites. There are no
outstanding review blockers. Review does not replace the hosted final-SHA check.
