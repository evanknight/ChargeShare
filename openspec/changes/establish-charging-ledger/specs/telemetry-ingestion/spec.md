# Telemetry ingestion

## Purpose

Collect a single approved vehicle's telemetry as private, replayable evidence while making collection failures and unsupported signals visible.

## ADDED Requirements

### Requirement: Single-vehicle isolation
The system SHALL accept only the configured vehicle identity for ledger processing and SHALL reject unknown vehicle data without retaining its payload or exposing identifiers in ordinary logs.

#### Scenario: Unrecognized vehicle
- **WHEN** an event arrives for a vehicle outside the allowlist
- **THEN** the event is excluded from persistence and ledger processing and only a redacted rejection count is recorded

### Requirement: Durable evidence
The system SHALL preserve event time, receipt time, payload identity and invalid-value status for accepted telemetry, and SHALL support deterministic replay from durably stored evidence.

#### Scenario: Processor restart
- **WHEN** the processor restarts after an event was durably stored but not incorporated into the ledger
- **THEN** the event is replayed and contributes at most once

#### Scenario: Storage failure
- **WHEN** evidence cannot be durably persisted
- **THEN** collection health reports failure and affected sessions remain incomplete rather than appearing fully captured

### Requirement: Validation and deduplication
The system SHALL ignore invalid or malformed energy values in calculations while retaining their quality evidence, and SHALL handle duplicates and out-of-order arrivals without duplicate energy.

#### Scenario: Duplicate and late event
- **WHEN** an accepted payload is repeated or a valid older event arrives late
- **THEN** duplicate payloads have no additional energy effect and the ledger is deterministically recomputed using event time

#### Scenario: Invalid counter
- **WHEN** an AC energy field is marked invalid or is malformed
- **THEN** it does not become a counter baseline or endpoint and the affected record is flagged

### Requirement: Minimal data and private operation
The system SHALL omit location collection from the first version, keep credentials and raw telemetry out of source control and ordinary logs, and restrict ledger access to the authorized operator.

#### Scenario: Default telemetry configuration
- **WHEN** the initial telemetry configuration is reviewed for activation
- **THEN** it requests only approved charging evidence and no location signals or vehicle-control feature

### Requirement: Collection health
The system SHALL report authentication, certificate, receiver, storage and billing-related collection failures when detectable and SHALL distinguish a healthy transport from proven complete energy capture.

#### Scenario: Telemetry silence
- **WHEN** messages stop while a charging session has no confirmed terminal evidence
- **THEN** the session remains open or incomplete and is not automatically finalized from silence
