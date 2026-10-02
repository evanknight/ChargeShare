# ChargeShare

A private charging ledger for sharing a home charger, built in Rust.

## Start here

1. **Status:** planning-only scaffold; nothing is connected to Tesla
2. **Review now:** [Spec 1: offline multi-vehicle ledger](openspec/changes/offline-multi-vehicle-ledger/proposal.md)
3. **Next step:** review the spec, then explicitly request implementation. One spec at a time

Spec 1 proposes replaying synthetic events in Rust while keeping each vehicle's
sessions, AC energy and eligibility separate. It does not implement user access,
prices, statements or live collection. Owner visibility is a review assumption,
not approved sharing. Measurement accuracy remains unvalidated.

The original broad single-vehicle OpenSpec change is deleted, not marked complete.
Only Spec 1 is active. No implementation is authorized by this planning PR.

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

- Earlier background research: [initial plan](docs/initial-plan.md), [architecture](docs/architecture.md), [measurement limits](docs/measurement.md)
- [Spec 1 proposal and review questions](openspec/changes/offline-multi-vehicle-ledger/proposal.md), [publishing](docs/publishing.md)

Use the generated OpenSpec skills to plan changes before implementing them. Archive only after implementation and verification; `openspec/specs/` is intentionally empty for now.

No project license has been selected. Public visibility alone does not grant a general open-source license. OpenSpec-generated files retain their [upstream MIT notice](docs/third-party-notices.md).
