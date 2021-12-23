// FIXME: Add support for defmt. Mutually exclusive with Debug?
#[derive(Clone, Copy, Debug)]
pub enum Command {
    SoftReset,
    SerialNumber,
}

impl Command {
    pub(crate) fn code(self) -> u8 {
        match self {
            Self::SerialNumber => 0x89,
            Self::SoftReset => 0x94,
        }
    }
}
