# Reimbursement ledger

## Purpose

Convert eligible reviewed AC sessions into reproducible, uncertainty-aware reimbursement estimates using the shared charger's agreed tariff.

## ADDED Requirements

### Requirement: Versioned tariff rules
The system SHALL price eligible energy using a versioned tariff specifying currency, IANA timezone, effective dates and applicable rate windows, and SHALL retain the version used for each statement.

#### Scenario: Known flat rate
- **WHEN** a reviewed 10 kWh session uses an invented flat rate of 0.20 currency units per kWh
- **THEN** its unrounded variable energy cost is exactly 2.00 currency units

### Requirement: Time-boundary allocation
The system SHALL split pricing intervals at rate changes, midnight, month-end and tariff effective dates using unambiguous UTC instants and local timezone rules.

#### Scenario: Two-rate session
- **WHEN** 4 kWh is allocated at 0.20 and 6 kWh at 0.40 currency units per kWh
- **THEN** the variable energy cost is exactly 3.20 currency units

#### Scenario: Daylight-saving transition
- **WHEN** a session spans a skipped or repeated local hour
- **THEN** every interval maps to its actual UTC duration and each energy delta is priced once

### Requirement: Uncertain timing is visible
The system SHALL label short bounded tariff interpolation as estimated and SHALL hold long cross-rate telemetry gaps for review or show a cost range rather than claiming an exact allocation.

#### Scenario: Cross-rate outage
- **WHEN** endpoints recover total energy across a long outage spanning different prices
- **THEN** total energy can be retained but the amount is held or ranged until the uncertainty is resolved

### Requirement: Eligibility and accuracy review
The system SHALL include only confirmed shared-charger AC sessions with resolved energy-completeness flags in finalized monthly totals, and SHALL disclose measurement-accuracy status and any explicitly accepted tariff estimation.

#### Scenario: Incomplete evidence
- **WHEN** a session has an unresolved missing baseline or terminal reading
- **THEN** it is excluded from the finalized payable total and appears in a separate review list

#### Scenario: No independent meter comparison
- **WHEN** physically independent energy validation has not occurred
- **THEN** statements label measurement accuracy unvalidated and require the operator's recorded acknowledgment of the agreed limitation before finalization

### Requirement: Transparent private export
The system SHALL provide an authorized private session view and CSV with session alias, local times and timezone, AC energy, rates, currency, amount, tariff version, quality flags and statement revision, without secrets or real vehicle identifiers by default.

#### Scenario: Monthly export
- **WHEN** an authorized operator exports a month
- **THEN** eligible totals, excluded sessions and estimated allocations remain distinguishable and the output includes no tokens, precise locations or real VINs

### Requirement: Reproducible arithmetic
The system SHALL use decimal pricing and an explicit currency rounding policy, apply no unexplained loss multiplier, and preserve enough precision and source values to reproduce the statement.

#### Scenario: Repeated calculation
- **WHEN** the same evidence, tariff versions and review decisions are processed twice
- **THEN** the energy, cost and rounded monthly total are identical
