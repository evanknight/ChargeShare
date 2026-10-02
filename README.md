# ChargeShare

A small, auditable charging ledger for sharing a home charger.

**Status: planning scaffold, not a working application.** ChargeShare is a proposed single-vehicle Tesla Fleet Telemetry proof of concept. It will turn AC charging measurements into a private session list, CSV, and reviewable reimbursement estimate. No Tesla account connection, live vehicle data, deployment, payment, or vehicle control exists here yet.

## The first version

- One allowlisted vehicle; manually confirm sessions at the shared charger
- `ACChargingEnergyIn` as the starting energy signal, with quality flags and reproducible evidence
- Versioned flat-rate and time-of-use tariffs, including overnight and daylight-saving boundaries
- Private session review and CSV; only eligible, reviewed sessions enter monthly totals

Vehicle-reported AC energy is **not guaranteed utility-meter accuracy**. Charging losses, upstream consumption, missing samples and tariff timing require validation. ChargeShare will label estimates and unresolved data instead of presenting them as exact bills.

## Start here

- [Initial plan and official sources](docs/initial-plan.md)
- [Architecture and data model](docs/architecture.md)
- [Development setup](docs/setup.md)
- [Measurement and validation](docs/measurement.md)
- [Security and public-repository rules](docs/security.md)
- [Initial OpenSpec proposal](openspec/changes/establish-charging-ledger/proposal.md)

## Spec-driven development

OpenSpec **1.14.0** is pinned in `package.json` and `package-lock.json`. This repository was initialized with the official CLI and its Codex core skills. Start with the proposal, design, three capability specs, and unchecked implementation tasks in `openspec/changes/establish-charging-ledger/`.

```sh
npm ci --ignore-scripts
npm run spec:validate
npm run spec:status
```

In Codex, use `$openspec-propose` to plan future changes and `$openspec-apply-change` only after a plan is approved. Archive after implementation and verification, not merely after writing the plan. `openspec/specs/` is intentionally empty until an implemented change is archived.

Read [setup](docs/setup.md) and install the local security hooks before committing. The repository contains no secrets or live vehicle fixtures by design; automated scans reduce risk but cannot guarantee that every secret or personal detail will be caught.

## Boundaries

No charging commands, driving controls, payments, location collection, automatic charger matching, or multi-tenant service in the first version. App registration, OAuth, key pairing, hosting and spending require a separate approved step.

No project license has been selected yet. Public visibility alone does not grant a general open-source license. OpenSpec-generated skills retain their upstream MIT metadata; see [third-party notices](docs/third-party-notices.md).
