# Measurement boundary and validation

The central engineering question is whether the selected car's reported AC-session energy is complete and useful for an agreed reimbursement estimate. Passing a parser test cannot answer that physical measurement question.

## What the first version can claim

Tesla describes `ACChargingEnergyIn` as charger-measured AC-session energy. It is a stronger starting point than battery-side `charge_energy_added`, but this is not a utility-certified meter guarantee. Validate the actual vehicle/firmware, unit, reset behavior, reporting latency and captured final counter before relying on it.

Do not automatically add a generic charging-loss multiplier. Upstream wiring, charger standby, plugged-in auxiliary use and billing-meter boundaries may differ. An independent AC-energy reference is needed to quantify any systematic difference. If no reference is available, label measurement accuracy unvalidated and obtain agreement on that limitation before reimbursement.

## Required trial evidence

After separate setup approval, observe at least five supervised sessions over approximately one week, including a pause/resume and a time-of-use boundary if applicable. Record firmware, telemetry configuration, baseline/end readings, observed disconnects and unresolved gaps privately. If a permitted independent meter reference exists, agree a tolerance and document comparison results privately.

The system must retain or show flags for missing baseline, rollback/reset ambiguity, invalid signal, missing terminal reading, long telemetry gap, ambiguous AC/DC state, unconfirmed charger, estimated tariff allocation and unvalidated meter accuracy. Distinguish software-valid energy from externally validated physical accuracy.

A recoverable cumulative endpoint may restore total energy after an outage without restoring its timing. Do not apply a single start-time rate to an overnight session or make an exact time-of-use claim from an unknown energy distribution.

## Acceptance examples

- Invented flat rate: 10 kWh at 0.20 currency units/kWh produces 2.00 units before any separately agreed charges
- Invented time of use: 4 kWh at 0.20 plus 6 kWh at 0.40 produces 3.20 units
- The same raw events processed twice must produce the same energy and amount
- An unconfirmed charger, DC session or unresolved energy completeness error cannot enter the payable monthly total
- A short, explicitly bounded tariff interpolation appears as estimated; a long cross-rate outage blocks automatic finalization
- DST fall-back and spring-forward, month-end, tariff revisions and currency rounding have deterministic synthetic tests

Any eventual statement is a review aid, not automatic invoicing, a payment authorization or a legal determination of the correct reimbursement.
