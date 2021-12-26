// FIXME: Add support for defmt. Mutually exclusive with Debug?
#[derive(Clone, Copy, Debug)]
pub enum Command {
    MeasureHighPrecision,
    MeasureMediumPrecision,
    MeasureLowPrecision,
    SoftReset,
    SerialNumber,
}

impl Command {
    pub(crate) fn code(self) -> u8 {
        match self {
            Self::MeasureHighPrecision => 0xfd,
            Self::MeasureMediumPrecision => 0xf6,
            Self::MeasureLowPrecision => 0xe0,
            Self::SerialNumber => 0x89,
            Self::SoftReset => 0x94,
        }
    }
}
