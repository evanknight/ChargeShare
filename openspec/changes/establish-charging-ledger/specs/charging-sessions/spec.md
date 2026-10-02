# Charging sessions

## Purpose

Reconstruct reviewable AC charging sessions without duplicate accounting, and distinguish shared-charger eligibility from uncertain or unrelated charging.

## ADDED Requirements

### Requirement: AC energy boundary
The system SHALL use valid ACChargingEnergyIn counter differences within a validated segment for AC session energy and SHALL identify the measurement boundary as vehicle-reported AC energy rather than certified utility-meter consumption.

#### Scenario: Normal AC segment
- **WHEN** valid samples in one segment increase from 2 kWh to 5 kWh
- **THEN** the segment contributes 3 kWh with its timestamps and measurement label

### Requirement: Conservative counter handling
The system SHALL retain missing baselines, counter resets, rollback and missing final readings as explicit quality conditions and SHALL prevent unresolved energy completeness errors from entering finalized reimbursement totals.

#### Scenario: Counter decreases
- **WHEN** a cumulative counter decreases during an apparent connection
- **THEN** no negative delta or speculative extra energy is billed and the transition is retained for review

#### Scenario: Missing final reading
- **WHEN** a connection ends without a reliable ending energy sample
- **THEN** the session is marked incomplete and cannot be automatically finalized

### Requirement: Connection and charging state
The system SHALL use state evidence to group charging intervals under a connection, preserve pauses and resumptions, and exclude DC charging from shared-home AC reimbursement.

#### Scenario: Pause and resume
- **WHEN** AC charging pauses and resumes without a confirmed disconnection
- **THEN** intervals remain traceable under one connection and no prior energy is counted twice

#### Scenario: DC charge
- **WHEN** the session is identified as DC charging
- **THEN** it is excluded from the shared-home AC reimbursement total

### Requirement: Manual shared-charger confirmation
The system SHALL default a session's charger classification to unconfirmed and SHALL require an explicit operator confirmation before including it in shared-charger reimbursement.

#### Scenario: Other AC location
- **WHEN** an AC session has not been marked as using the shared charger
- **THEN** it remains excluded even if all energy samples are otherwise valid

### Requirement: Reproducible revisions
The system SHALL preserve evidence and review decisions so late events or corrections create traceable ledger revisions without silently altering a previously exported statement.

#### Scenario: Late correction
- **WHEN** new evidence changes a previously exported session total
- **THEN** a new revision records the change and the earlier statement remains reproducible
