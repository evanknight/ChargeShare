# ChargeShare

A private charging ledger for sharing a home charger, built in Rust.

## Start here

1. **Status:** this is a planning and build scaffold. Nothing is connected to Tesla yet
2. **Next milestone:** replay synthetic charging events in Rust and test the session-energy calculations entirely offline
3. **When ready:** implement that offline milestone from the [initial proposal](openspec/changes/establish-charging-ledger/proposal.md). Validate against a real Tesla session afterward; no need to read every document first

The first version will track one car, let you mark shared-charger sessions manually, and produce a reviewable electricity reimbursement estimate. Vehicle-reported AC energy is not guaranteed utility-meter accuracy; incomplete or estimated readings will be flagged.

## Develop

The empty Cargo workspace starts at `crates/chargeshare-core/`. Rust 1.99.0 is pinned in `rust-toolchain.toml`; Node.js 24 and OpenSpec 1.14.0 are development tools only.

```sh
npm ci --ignore-scripts
bash scripts/security/install-gitleaks.sh
bash scripts/security/install-hooks.sh
npm run spec:validate
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

These checks validate the scaffold. Charging logic and its domain tests are not implemented yet. See [setup](docs/setup.md) if needed.

## Keep it safe

Only synthetic data belongs here. Never commit tokens, keys, real VINs, locations, bills or raw telemetry. Read [security](docs/security.md) before committing; scanners and hooks reduce risk but cannot guarantee that every secret is caught.

Tesla registration, authorization, key pairing, deployment and spending require separate approval. Vehicle controls and payments are outside the first version.

## References, when needed

- [Initial plan](docs/initial-plan.md), [architecture](docs/architecture.md), [measurement limits](docs/measurement.md)
- [OpenSpec proposal and tasks](openspec/changes/establish-charging-ledger/proposal.md), [publishing](docs/publishing.md)

Use the generated OpenSpec skills to plan changes before implementing them. Archive only after implementation and verification; `openspec/specs/` is intentionally empty for now.

No project license has been selected. Public visibility alone does not grant a general open-source license. OpenSpec-generated files retain their [upstream MIT notice](docs/third-party-notices.md).
