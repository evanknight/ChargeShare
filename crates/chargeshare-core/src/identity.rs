use crate::error::LedgerError;

macro_rules! alias {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);
        impl $name {
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
