# ChargeShare proof of concept plan

Public project planning document. Examples use invented values. Current scope is a single-vehicle measurement and reimbursement proof of concept; remote vehicle control and payments are out of scope.

Build a small, single-car charging ledger using Tesla Fleet Telemetry. It should record the selected Model Y’s AC charging energy and timing, let you confirm which sessions used the shared garage, and calculate a reviewable monthly reimbursement under the shared-charger electricity account’s tariff.

## Recommended first version

- Use ACChargingEnergyIn. Tesla describes this as charger-measured AC-session kWh. The ordinary charge_energy_added field maps to battery-side energy, so it is a weaker basis for this purpose. Tesla publishes no billing-grade accuracy guarantee for the AC signal. Validate it before treating the total as settled. [1]

- Track only the owner’s Model Y. Allowlist its VIN. Manually mark a session as “shared garage” before including it in reimbursement; other AC charging and Supercharging stay excluded. This avoids collecting location or relying on the charger owner’s Tesla account.

- Keep the output simple. A private session list and downloadable CSV should show local start/end times, AC kWh, tariff breakdown, amount, and any unresolved data. A monthly total includes only reviewed sessions.

Proposed components: official Tesla telemetry receiver → pre-persistence vehicle allowlist → durable raw-event file → small Python processor and SQLite → private session view and CSV. Use one always-on host with persistent storage. A static website alone cannot receive this stream. Avoid extra infrastructure until the first real session works.

Do not rely on charging_history for the ledger: Tesla’s public documentation does not promise complete private-home AC sessions, and charging_sessions is restricted to business fleet owners. [12]

## Build in four stages

- 1  Prove the calculation offline. Create synthetic event fixtures, the session model, tariff logic, and CSV export. Test them without Tesla access. Tesla provides no public staging environment. [2]

- 2  Prove one real charging session. After setup approval, authorize your account, pair the virtual key, and collect one supervised garage charge. Verify timestamps, units, counter resets, stopping behavior, and whether the last energy value is captured. Stop here if the signal is unavailable or ambiguous.

- 3  Add reliable accounting. Handle pauses, restarts, duplicate and late events, invalid readings, and rate changes. Add manual garage confirmation, review flags, and a private monthly summary. Preserve raw evidence so corrections can be recalculated.

- 4  Validate before reimbursement. Observe at least five sessions over about a week, including a pause/resume and a rate boundary if applicable. Compare a session against an independent AC-energy reference when one is available with permission. Otherwise label energy accuracy unvalidated and agree on this limitation with the charger owner.

Planning estimate: 3–5 focused build days after app access, hosting, and the tariff are ready, plus the observation week. This is an estimate, not a delivery commitment.

## Data and reimbursement rules

- Signals. Begin with ACChargingEnergyIn every 60 seconds, ACChargingPower every 60 seconds, and DetailedChargeState plus ChargingCableType at a 1-second minimum interval. These are proposed settings to test. Tesla emits changed values, so intervals are not a guaranteed heartbeat. Ignore invalid values and exclude DC charging. [1, 3]

- Raw events. Store the allowlisted VIN, vehicle event time in UTC, receipt time, field/value/units, invalid status, payload hash, and original payload. Deduplicate before calculating; retain event order separately from arrival order.

- Sessions. Store session ID, plug-in/out evidence, actual charging intervals, counter baseline/final value and resets, AC kWh, garage confirmation, tariff version, status, and review notes. Group pauses under one connection when supported by the data; represent counter resets explicitly rather than double-counting them.

- Tariffs. Store currency, the garage’s IANA timezone, effective dates, weekday/weekend/holiday rules, rate windows, and agreed per-kWh taxes or charges. Keep tariff versions so an old statement is reproducible. Fixed charges, demand charges, tiers, or solar/net-metering treatment require an agreed allocation rule.

For consecutive valid samples in the same counter segment, calculate the increase in cumulative AC kWh. Price that energy over the matching time interval. Split at each tariff boundary, midnight, month-end, and tariff effective date. Use UTC for storage and the garage timezone for rate lookup, including daylight-saving changes.

If a sample interval crosses a rate boundary, interpolate only over a short interval and label that allocation as estimated. If a long outage crosses different rates, hold the session for review or show a cost range. A later cumulative reading may recover total energy without revealing when it was used. Never apply the start-time rate to a whole overnight session.

Example test with invented rates: 4 kWh at $0.20 plus 6 kWh at $0.40 must total $3.20. Keep full numeric precision during calculation; round the final statement under an agreed currency rule. Do not add a generic “charging loss percentage” on top of charger-side energy.

## Handle missing data honestly

A disconnect is not proof that charging ended. Preserve an open or incomplete session until later evidence resolves it. Reprocess late events deterministically. Flag a missing baseline, counter rollback, invalid measurement, missing ending reading, or ambiguous AC/DC state. Do not infer a bill from battery percentage or fill a long gap with the last power reading.

Tesla documents a finite 5,000-message vehicle buffer, so extended outages can lose data. Persist raw events and processor offsets, test recovery after a crash, and flag any period where receipt or durability cannot be demonstrated. A logger-only first experiment does not establish guaranteed end-to-end delivery. [3, 4]

## Authentication and access

Use Tesla’s authorization-code flow, an exact registered callback, random state validation, and server-side token storage. Start with openid, offline_access, and vehicle_device_data. Confirm the telemetry-configuration endpoint’s exact scope requirement during setup before granting anything broader; the public page extraction does not expose that detail. Skip profile, location, energy-product, and charging-control permissions unless a later feature genuinely needs them. [5, 6]

Protect refresh tokens, client secrets, and signing keys outside source control and logs. Save rotated refresh tokens atomically; Tesla’s refresh tokens are single-use with a limited recovery window. Keep the command proxy private, expose no charging or driving controls in the app, restrict the dashboard to you, and provide a clear revoke/stop procedure. [6, 7]

## Setup required before a live test

Use a Tesla account with verified email and MFA. App enrollment may request legal/business details; Tesla explicitly supports hobbyist integrations, but approval is still a prerequisite. Register the app/domain in the vehicle’s region and host the public key at Tesla’s required .well-known path. You then authorize the app and add its virtual key through Tesla’s mobile app. [5, 7, 8]

Check firmware and Fleet Telemetry capability; the current signed configuration flow requires firmware 2024.26 or later, while DetailedChargeState starts at 2024.38. Use Tesla’s receiver with a public DNS name and valid certificate, preserving WebSockets and mTLS termination at the receiver. Configure through the Vehicle Command Proxy and verify the configuration is synced. The car initiates the internet connection; the garage needs usable Wi-Fi or cellular coverage. [1, 3, 4]

## Acceptance tests

- Isolation and eligibility. Only the chosen VIN is accepted; an unconfirmed garage session or DC charge cannot enter the monthly payable total. Other vehicles’ sessions never enter this vehicle feed.

- Correct accounting. Synthetic flat-rate and time-of-use examples match expected amounts exactly. Midnight, daylight-saving changes, month-end, pauses, counter resets, duplicates, out-of-order events, and reprocessing produce no double billing.

- Recovery and uncertainty. A network interruption and processor restart preserve committed records; incomplete energy or timing is visibly flagged. Token failure, revoked access, certificate expiry, and billing suspension produce a clear collection-health warning.

- Real-world evidence. Five reviewed sessions have plausible boundaries and a captured final AC counter. Establish any meter-comparison tolerance with the charger owner. Without an independent reference, passing software tests does not certify utility-meter accuracy or coverage of standby, upstream wiring, and plugged-in auxiliary consumption.

## Costs and operating limits

Tesla currently lists 150,000 streaming signals per US$1 and a US$10 monthly developer discount. As an illustration, two changing fields once per minute for 120 charging hours produce 14,400 signals, about US$0.096 before the discount, plus state changes and other usage. Hosting and domain fees are separate and need a budget. [9, 10]

Tesla requires a payment method. Set an approved billing cap and watch it: hitting the cap can disable access and remove telemetry configuration, which is not automatically restored. Use Fleet API for setup and occasional diagnosis only; Tesla advises against regular vehicle_data polling. Never wake the car just to collect routine reimbursement data. [10, 11]

## Decisions needed to start

Confirm the charger owner’s actual tariff, currency and garage timezone; the accepted measurement/uncertainty policy; a hosting/domain budget and destination. App registration, credentials, key pairing, spending and deployment are future steps requiring approval. No real credentials, tokens, VINs, locations, bills, or personal correspondence belong in this repository.

## Official sources checked on 2 October 2026

1. [Available vehicle data](https://developer.tesla.com/docs/fleet-api/fleet-telemetry/available-data)

2. [Fleet API FAQ and staging limitations](https://developer.tesla.com/docs/fleet-api/support/faq)

3. [Fleet Telemetry setup and system behavior](https://developer.tesla.com/docs/fleet-api/fleet-telemetry)

4. [Tesla reference receiver and security guidance](https://github.com/teslamotors/fleet-telemetry)

5. [Authentication scope definitions](https://developer.tesla.com/docs/fleet-api/authentication/overview)

6. [Authorization flow and refresh tokens](https://developer.tesla.com/docs/fleet-api/authentication/third-party-tokens)

7. [Virtual key developer guide](https://developer.tesla.com/docs/fleet-api/virtual-keys/developer-guide)

8. [Application onboarding](https://developer.tesla.com/docs/fleet-api/getting-started/what-is-fleet-api)

9. [Usage based pricing](https://developer.tesla.com/)

10. [Billing and limits](https://developer.tesla.com/docs/fleet-api/billing-and-limits)

11. [API best practices](https://developer.tesla.com/docs/fleet-api/getting-started/best-practices)

12. [Charging history limitations](https://developer.tesla.com/docs/fleet-api/endpoints/charging-endpoints)
