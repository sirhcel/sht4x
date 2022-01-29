// FIXME: Add support for defmt. Mutually exclusive with Debug?
#[derive(Clone, Copy, Debug)]
pub enum Command {
    MeasureHighPrecision,
    MeasureMediumPrecision,
    MeasureLowPrecision,
    SoftReset,
    SerialNumber,
    MeasureHeated200mw1s,
    MeasureHeated200mw0p1s,
    MeasureHeated110mw1s,
    MeasureHeated110mw0p1s,
    MeasureHeated20mw1s,
    MeasureHeated20mw0p1s,
}

impl Command {
    pub(crate) fn code(&self) -> u8 {
        match self {
            Self::MeasureHighPrecision => 0xfd,
            Self::MeasureMediumPrecision => 0xf6,
            Self::MeasureLowPrecision => 0xe0,
            Self::SerialNumber => 0x89,
            Self::SoftReset => 0x94,
            Self::MeasureHeated200mw1s => 0x39,
            Self::MeasureHeated200mw0p1s => 0x32,
            Self::MeasureHeated110mw1s => 0x2f,
            Self::MeasureHeated110mw0p1s => 0x24,
            Self::MeasureHeated20mw1s => 0x1e,
            Self::MeasureHeated20mw0p1s => 0x15,
        }
    }

    pub(crate) fn duration_ms(&self) -> Option<u16> {
        // Values rounded up from the maximum durations given in the datasheet
        // table 4, 'System timing specifications'.
        match self {
            Self::MeasureHighPrecision => Some(9),
            Self::MeasureMediumPrecision => Some(5),
            Self::MeasureLowPrecision => Some(2),
            Self::SerialNumber => None,
            Self::SoftReset => Some(1),
            Self::MeasureHeated200mw1s => Some(1100),
            Self::MeasureHeated200mw0p1s => Some(110),
            Self::MeasureHeated110mw1s => Some(1100),
            Self::MeasureHeated110mw0p1s => Some(110),
            Self::MeasureHeated20mw1s => Some(1100),
            Self::MeasureHeated20mw0p1s => Some(110),
        }
    }
}
