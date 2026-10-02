# Development setup

## Offline Spec 1 checkout

Prerequisites: Rust 1.99.0 through [rustup](https://doc.rust-lang.org/book/ch01-01-installation.html), a C linker, Node.js 24 or newer, npm, Git, Bash, curl, tar, SHA-256 tooling, and the verified Gitleaks version documented in [security](security.md). The Cargo workspace implements the offline synthetic ledger and its acceptance suite. There is no executable application or Tesla integration to launch. Rust is the application implementation language; Node.js is development tooling only.

```sh
npm ci --ignore-scripts
npm run spec:validate
npm run spec:status
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

The npm package has `private: true` to prevent accidental npm publication. That flag is unrelated to GitHub repository visibility. OpenSpec is pinned to 1.14.0 with registry integrity hashes in the lockfile. Set `OPENSPEC_TELEMETRY=0` or `DO_NOT_TRACK=1` for direct CLI use; the npm scripts already opt out.

Read and follow [security](security.md) before the first commit. Install the hooks through its documented installer; a fresh Git clone does not automatically activate hooks.

`rust-toolchain.toml` pins Rust 1.99.0 with rustfmt and Clippy; `Cargo.lock` is committed. This version was verified against the [official stable manifest](https://static.rust-lang.org/dist/channel-rust-stable.toml) dated 1 October 2026. The crate has no runtime dependencies. Its 17 offline tests validate the fictional domain contract; they do not validate live telemetry or physical accuracy.

## OpenSpec workflow

1. Review `openspec/changes/offline-multi-vehicle-ledger/proposal.md` first, then `design.md`, `specs/vehicle-ledger/spec.md` and `tasks.md`
2. Refine requirements and acceptance scenarios before implementing
3. After review, explicitly request implementation when ready; live setup and later capabilities need separate reviewed specs
4. Run tests, security scans and strict spec validation; attach actual verification evidence
5. Archive only after the change is implemented and verified so accepted capabilities move into `openspec/specs/`

The initial scaffold was generated using:

```sh
OPENSPEC_TELEMETRY=0 npx @fission-ai/openspec@1.14.0 init --tools codex --profile core --no-animation
```

This is provenance, not a command to rerun on each checkout. Generated Codex skills are committed under `.agents/skills/`.

For source-archive publication into an authorized checkout, follow [publishing](publishing.md).

## Live integration is a separate stage

Do not put Tesla configuration or credentials in the checkout now. A future approved preflight must verify:

- Supported vehicle firmware and actual Fleet Telemetry signal support
- Regional app registration, approved domain/host, virtual-key pairing and callback URLs
- Exact minimal scope required by `fleet_telemetry_config`; do not grant broader command permissions speculatively
- OAuth state validation, atomic refresh-token rotation and private server-side secret storage
- Correct official receiver mTLS/WebSocket handling, persistent storage and private dashboard access
- Electricity tariff, currency, timezone and agreed uncertainty policy
- Approved hosting/domain/API budget, payment setup and collection-health/billing-cap alerts
- Explicit stop/revoke and rollback procedures

Use only placeholders in `.env.example`. Copying it does not create working credentials or authorize live operation. Deployment manifests, app registration and key generation are deliberately deferred.
