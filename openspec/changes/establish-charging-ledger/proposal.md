# Proposal

## Why

Sharing a home charger needs a transparent record of one vehicle's AC energy and a reviewable way to allocate electricity costs. Establish a small proof of concept that makes missing measurements and cost uncertainty explicit before any reimbursement is agreed.

## What Changes

- Receive and retain an allowlisted vehicle's Fleet Telemetry events with replayable evidence
- Derive AC charging sessions from cumulative AC energy and state signals, retaining data-quality flags
- Require manual shared-charger confirmation; no location collection in the first version
- Price energy with versioned flat-rate or time-of-use tariffs and export private, reviewable session and monthly statements
- Validate the software offline, then seek approval for a supervised real-vehicle trial
- Exclude vehicle controls, charging commands, payments, public dashboards, automatic geofencing, multi-tenant billing and Tesla account enrollment from this change's initial implementation scope

## Capabilities

### New Capabilities

- `telemetry-ingestion`: Single-vehicle, AC-focused ingestion with durable storage, deduplication and collection-health visibility
- `charging-sessions`: Deterministic session reconstruction, counter semantics, manual shared-charger classification and review eligibility
- `reimbursement-ledger`: Versioned tariff allocation, uncertainty-aware statements and CSV export

### Modified Capabilities

None. This is a greenfield project with no implemented capabilities.

## Impact

Proposed implementation: Tesla's official receiver, a small Python processor, SQLite, and a private session view/CSV. An always-on host and app authorization are future dependencies; none is created by the planning scaffold. Real access, domain, budget and deployment require separate approval. This change is proposed and unimplemented. Tesla measurements are not certified utility-meter readings, and live accuracy remains unvalidated.
