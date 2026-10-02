use std::fmt;

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

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for LedgerError {}
