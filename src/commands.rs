// FIXME: Add support for defmt. Mutually exclusive with Debug?
#[derive(Clone, Copy, Debug)]
pub enum Command {
    MeasureHighPrecision,
    MeasureMediumPrecision,
    MeasureLowPrecision,
    SoftReset,
    SerialNumber,
    MeasureHeated200Mw1S,
    MeasureHeated200Mw100Ms,
    MeasureHeated110Mw1S,
    MeasureHeated110Mw100Ms,
    MeasureHeated20Mw1S,
    MeasureHeated20Mw100Ms,
}

impl Command {
    pub(crate) fn code(self) -> u8 {
        match self {
            Self::MeasureHighPrecision => 0xfd,
            Self::MeasureMediumPrecision => 0xf6,
            Self::MeasureLowPrecision => 0xe0,
            Self::SerialNumber => 0x89,
            Self::SoftReset => 0x94,
            Self::MeasureHeated200Mw1S => 0x39,
            Self::MeasureHeated200Mw100Ms => 0x32,
            Self::MeasureHeated110Mw1S => 0x2f,
            Self::MeasureHeated110Mw100Ms => 0x24,
            Self::MeasureHeated20Mw1S => 0x1e,
            Self::MeasureHeated20Mw100Ms => 0x15,
        }
    }
}
