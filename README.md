# ChargeShare

A private charging ledger for sharing a home charger, built in Rust.

## Start here

1. **Implemented:** offline, in-memory synthetic multi-vehicle Rust ledger
2. **Review:** [Spec 1](openspec/changes/offline-multi-vehicle-ledger/proposal.md) and [domain contract](docs/offline-ledger.md)
3. **Verified behavior:** separate observed AC energy and conservative shared-charger eligibility, with deterministic replay and explicit uncertainty

Spec 1 was approved for implementation after the planning review. It replays
fictional events only. Nothing is connected to Tesla. Owner associations and
vehicle scope checks are not authentication or approved cross-owner sharing.
Physical measurement accuracy remains unvalidated. No prices, bills, statements,
live collection, receiver integration or deployment are included.

The original broad single-vehicle OpenSpec change was deleted, not marked complete.
Only Spec 1 is active. Its planning artifacts retain the original review context.

## Develop

The Cargo workspace starts at `crates/chargeshare-core/`. Rust 1.99.0 is pinned in `rust-toolchain.toml`; Node.js 24 and OpenSpec 1.14.0 are development tools only.

```sh
npm ci --ignore-scripts
bash scripts/security/install-gitleaks.sh
bash scripts/security/install-hooks.sh
npm run spec:validate
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

These checks include every offline Spec 1 domain scenario. See [testing](docs/testing.md) for coverage and CI evidence, and [setup](docs/setup.md) if needed.

## Keep it safe

Only synthetic data belongs here. Never commit tokens, keys, real VINs, locations, bills or raw telemetry. Read [security](docs/security.md) before committing; scanners and hooks reduce risk but cannot guarantee that every secret is caught.

Tesla registration, authorization, key pairing, deployment and spending require separate approval. Vehicle controls and payments are outside the first version.

## References, when needed

- Earlier background research: [initial plan](docs/initial-plan.md), [architecture](docs/architecture.md), [measurement limits](docs/measurement.md)
- Contributor design guidance: [code and architecture design for coding agents](docs/code-design.md)
- [Spec 1 proposal and review questions](openspec/changes/offline-multi-vehicle-ledger/proposal.md), [publishing](docs/publishing.md)

Use the generated OpenSpec skills to plan changes before implementing them. Archive only after implementation and verification; `openspec/specs/` is intentionally empty for now.

No project license has been selected. Public visibility alone does not grant a general open-source license. OpenSpec-generated files retain their [upstream MIT notice](docs/third-party-notices.md).
