//! Offline, synthetic charging review. No authentication, live adapter or bills.
//! Energy is vehicle-reported AC evidence with unvalidated physical accuracy.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Exact nonnegative kWh with six decimal places (one milliwatt-hour per unit).
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Energy(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CounterProblem {
    Invalid,
    Negative,
    UnsupportedPrecision,
    Overflow,
}

impl Energy {
    pub const ZERO: Self = Self(0);

    /// Accept ASCII decimal digits, optionally followed by 1..=6 decimal digits.
    /// No signs, whitespace, exponent notation, NaN, infinity or silent rounding.
    pub fn parse_kwh(input: &str) -> Result<Self, CounterProblem> {
        if input.starts_with('-') {
            return Err(CounterProblem::Negative);
        }
        let (whole, fraction) = match input.split_once('.') {
            Some((whole, fraction)) => (whole, Some(fraction)),
            None => (input, None),
        };
        if whole.is_empty() || !whole.bytes().all(|b| b.is_ascii_digit()) {
            return Err(CounterProblem::Invalid);
        }
        let fraction = fraction.unwrap_or("");
        if (input.contains('.') && fraction.is_empty())
            || !fraction.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(CounterProblem::Invalid);
        }
        if fraction.len() > 6 {
            return Err(CounterProblem::UnsupportedPrecision);
        }
        let whole: u64 = whole.parse().map_err(|_| CounterProblem::Overflow)?;
        let fraction: u64 = if fraction.is_empty() {
            0
        } else {
            fraction.parse().map_err(|_| CounterProblem::Overflow)?
        };
        whole
            .checked_mul(1_000_000)
            .and_then(|v| v.checked_add(fraction * 10_u64.pow(6 - fraction_length(input))))
            .map(Self)
            .ok_or(CounterProblem::Overflow)
    }

    pub fn checked_add(self, other: Self) -> Result<Self, CounterProblem> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(CounterProblem::Overflow)
    }
}

fn fraction_length(input: &str) -> u32 {
    input.split_once('.').map_or(0, |(_, f)| f.len() as u32)
}

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:06} kWh", self.0 / 1_000_000, self.0 % 1_000_000)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LedgerError {
    InvalidAlias,
    DuplicateVehicle,
    MissingVehicle,
    UnknownVehicle,
    OutsideVehicleScope,
    UnknownSession,
    EnergyOverflow,
}

// Diagnostics deliberately never include aliases or rejected payloads.
impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for LedgerError {}

macro_rules! alias {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);
        impl $name {
            /// Synthetic ASCII aliases only, 1..=64 letters, digits, '_' or '-'.
            pub fn new(value: &str) -> Result<Self, LedgerError> {
                if value.is_empty()
                    || value.len() > 64
                    || !value
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
                {
                    return Err(LedgerError::InvalidAlias);
                }
                Ok(Self(value.to_owned()))
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}
alias!(OwnerId);
alias!(VehicleId);
alias!(ConnectionId);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SessionId {
    pub vehicle: VehicleId,
    pub connection: ConnectionId,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ChargeType {
    Ac,
    Dc,
    Ambiguous,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EventKind {
    Start,
    Sample,
    Pause,
    Resume,
    End,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CounterReading {
    Valid(Energy),
    Rejected(CounterProblem),
}
impl CounterReading {
    /// Retain only a safe reason for rejected synthetic counter input.
    pub fn parse_kwh(input: &str) -> Self {
        match Energy::parse_kwh(input) {
            Ok(value) => Self::Valid(value),
            Err(reason) => Self::Rejected(reason),
        }
    }
}

/// Positions are unique within a vehicle, not merely within a connection.
/// Times are synthetic integer ticks; there is no wall-clock or timeout logic.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Event {
    pub vehicle: Option<VehicleId>,
    pub connection: ConnectionId,
    pub position: u64,
    pub time: i64,
    pub kind: EventKind,
    pub charge_type: ChargeType,
    pub counter: Option<CounterReading>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum QualityFlag {
    MissingConnectionStart,
    MissingConnectionEnd,
    MissingBaseline,
    MissingTerminalSample,
    InvalidCounter(CounterProblem),
    CounterRollback,
    ConflictingEvidence,
    InvalidBoundaryOrder,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ChargerClassification {
    #[default]
    Unconfirmed,
    SharedCharger,
    OtherCharger,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ExclusionReason {
    UnconfirmedCharger,
    OtherCharger,
    DcEvidence,
    AmbiguousChargeType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
    pub id: SessionId,
    pub owner: OwnerId,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub observed_ac: Energy,
    pub quality_flags: BTreeSet<QualityFlag>,
    pub exclusion_reasons: BTreeSet<ExclusionReason>,
    pub classification: ChargerClassification,
}
impl Session {
    pub fn is_eligible(&self) -> bool {
        self.quality_flags.is_empty() && self.exclusion_reasons.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionReasons {
    pub id: SessionId,
    pub quality_flags: BTreeSet<QualityFlag>,
    pub exclusion_reasons: BTreeSet<ExclusionReason>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Summary {
    pub vehicle: VehicleId,
    pub owner: OwnerId,
    pub observed_ac: Energy,
    pub eligible_shared_charger: Energy,
    /// Each noneligible session retains both quality and classification reasons.
    pub held_or_excluded: Vec<SessionReasons>,
    pub evidence_label: &'static str,
}

/// In-memory evidence and reviews, partitioned before sorting and deduplication.
/// These scope checks are domain invariants, not authentication/authorization.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ledger {
    owners: BTreeMap<VehicleId, OwnerId>,
    events: BTreeMap<VehicleId, BTreeMap<u64, BTreeSet<Event>>>,
    reviews: BTreeMap<SessionId, ChargerClassification>,
}

impl Ledger {
    pub fn register(&mut self, vehicle: VehicleId, owner: OwnerId) -> Result<(), LedgerError> {
        if self.owners.contains_key(&vehicle) {
            return Err(LedgerError::DuplicateVehicle);
        }
        self.owners.insert(vehicle, owner);
        Ok(())
    }

    fn owner(&self, vehicle: &VehicleId) -> Result<&OwnerId, LedgerError> {
        self.owners.get(vehicle).ok_or(LedgerError::UnknownVehicle)
    }

    pub fn ingest(&mut self, event: Event) -> Result<(), LedgerError> {
        let vehicle = event.vehicle.as_ref().ok_or(LedgerError::MissingVehicle)?;
        self.owner(vehicle)?;
        self.events
            .entry(vehicle.clone())
            .or_default()
            .entry(event.position)
            .or_default()
            .insert(event);
        Ok(())
    }

    pub fn classify(
        &mut self,
        scope: &VehicleId,
        target: &SessionId,
        classification: ChargerClassification,
    ) -> Result<(), LedgerError> {
        self.owner(scope)?;
        if scope != &target.vehicle {
            return Err(LedgerError::OutsideVehicleScope);
        }
        if !self.sessions(scope)?.iter().any(|s| &s.id == target) {
            return Err(LedgerError::UnknownSession);
        }
        self.reviews.insert(target.clone(), classification);
        Ok(())
    }

    pub fn sessions(&self, scope: &VehicleId) -> Result<Vec<Session>, LedgerError> {
        let owner = self.owner(scope)?;
        let mut connections: BTreeMap<ConnectionId, Vec<(bool, &Event)>> = BTreeMap::new();
        if let Some(positions) = self.events.get(scope) {
            for values in positions.values() {
                for event in values {
                    connections
                        .entry(event.connection.clone())
                        .or_default()
                        .push((values.len() > 1, event));
                }
            }
        }
        let mut sessions = Vec::new();
        for (connection, mut events) in connections {
            events.sort_by_key(|(_, event)| (event.time, event.position));
            let id = SessionId {
                vehicle: scope.clone(),
                connection,
            };
            let classification = self.reviews.get(&id).copied().unwrap_or_default();
            sessions.push(reconstruct(id, owner.clone(), &events, classification)?);
        }
        Ok(sessions)
    }

    pub fn summary(&self, scope: &VehicleId) -> Result<Summary, LedgerError> {
        let mut summary = Summary {
            vehicle: scope.clone(),
            owner: self.owner(scope)?.clone(),
            observed_ac: Energy::ZERO,
            eligible_shared_charger: Energy::ZERO,
            held_or_excluded: Vec::new(),
            evidence_label: "Synthetic review: vehicle-reported AC energy; physical accuracy unvalidated",
        };
        for session in self.sessions(scope)? {
            summary.observed_ac = summary
                .observed_ac
                .checked_add(session.observed_ac)
                .map_err(|_| LedgerError::EnergyOverflow)?;
            if session.is_eligible() {
                summary.eligible_shared_charger = summary
                    .eligible_shared_charger
                    .checked_add(session.observed_ac)
                    .map_err(|_| LedgerError::EnergyOverflow)?;
            } else {
                summary.held_or_excluded.push(SessionReasons {
                    id: session.id,
                    quality_flags: session.quality_flags,
                    exclusion_reasons: session.exclusion_reasons,
                });
            }
        }
        Ok(summary)
    }
}

fn reconstruct(
    id: SessionId,
    owner: OwnerId,
    events: &[(bool, &Event)],
    classification: ChargerClassification,
) -> Result<Session, LedgerError> {
    let mut result = Session {
        id,
        owner,
        start_time: None,
        end_time: None,
        observed_ac: Energy::ZERO,
        quality_flags: BTreeSet::new(),
        exclusion_reasons: BTreeSet::new(),
        classification,
    };
    match classification {
        ChargerClassification::Unconfirmed => {
            result
                .exclusion_reasons
                .insert(ExclusionReason::UnconfirmedCharger);
        }
        ChargerClassification::OtherCharger => {
            result
                .exclusion_reasons
                .insert(ExclusionReason::OtherCharger);
        }
        ChargerClassification::SharedCharger => {}
    }
    let mut baseline = false;
    let mut terminal = false;
    let mut previous: Option<Energy> = None;
    for (index, (conflicting, event)) in events.iter().enumerate() {
        match event.charge_type {
            ChargeType::Dc => {
                result.exclusion_reasons.insert(ExclusionReason::DcEvidence);
            }
            ChargeType::Ambiguous => {
                result
                    .exclusion_reasons
                    .insert(ExclusionReason::AmbiguousChargeType);
            }
            ChargeType::Ac => {}
        }
        if *conflicting {
            result
                .quality_flags
                .insert(QualityFlag::ConflictingEvidence);
            if let Some(CounterReading::Rejected(problem)) = event.counter {
                result
                    .quality_flags
                    .insert(QualityFlag::InvalidCounter(problem));
            }
            // Every candidate's position/time is an uncertainty barrier. No
            // candidate contributes energy or establishes a connection boundary.
            previous = None;
            continue;
        }
        if event.kind == EventKind::Start {
            if result.start_time.is_some() || index != 0 {
                result
                    .quality_flags
                    .insert(QualityFlag::InvalidBoundaryOrder);
            }
            result.start_time.get_or_insert(event.time);
            baseline |= matches!(event.counter, Some(CounterReading::Valid(_)));
        }
        if event.kind == EventKind::End {
            if result.end_time.is_some() || index + 1 != events.len() {
                result
                    .quality_flags
                    .insert(QualityFlag::InvalidBoundaryOrder);
            }
            result.end_time.get_or_insert(event.time);
            terminal |= matches!(event.counter, Some(CounterReading::Valid(_)));
        }
        match event.counter {
            Some(CounterReading::Rejected(problem)) => {
                result
                    .quality_flags
                    .insert(QualityFlag::InvalidCounter(problem));
                previous = None;
            }
            Some(CounterReading::Valid(current)) => {
                if event.charge_type == ChargeType::Ac {
                    if let Some(prior) = previous {
                        if current < prior {
                            result.quality_flags.insert(QualityFlag::CounterRollback);
                        } else {
                            result.observed_ac = result
                                .observed_ac
                                .checked_add(Energy(current.0 - prior.0))
                                .map_err(|_| LedgerError::EnergyOverflow)?;
                        }
                    }
                    previous = Some(current);
                } else {
                    previous = None;
                }
            }
            None => {
                // Pause/resume markers without samples preserve the counter chain.
                // Non-AC markers break it so no uncertain interval becomes observed AC.
                if event.charge_type != ChargeType::Ac {
                    previous = None;
                }
            }
        }
    }
    if result.start_time.is_none() {
        result
            .quality_flags
            .insert(QualityFlag::MissingConnectionStart);
    }
    if result.end_time.is_none() {
        result
            .quality_flags
            .insert(QualityFlag::MissingConnectionEnd);
    }
    if !baseline {
        result.quality_flags.insert(QualityFlag::MissingBaseline);
    }
    if !terminal {
        result
            .quality_flags
            .insert(QualityFlag::MissingTerminalSample);
    }
    Ok(result)
}
