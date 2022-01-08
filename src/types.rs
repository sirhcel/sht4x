use fixed::{
    types::I16F16,
    const_fixed_from_int,
};

#[derive(Clone, Copy, Debug)]
pub enum Address {
    Address0x44,
    Address0x45,
}

impl Into<u8> for Address {
    fn into(self) -> u8 {
        match self {
            Self::Address0x44 => 0x44,
            Self::Address0x45 => 0x45,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum HeatingPower {
    /// Operate the heater at 200 mW.
    Low,
    /// Operate the heater at 110 mW.
    Medium,
    /// Operate the heater at 20 mW.
    High,
}

#[derive(Clone, Copy, Debug)]
pub enum HeatingDuration {
    /// Operate the heater for 100 ms.
    Short,
    /// Operate the heater for 1 s.
    Long,
}

#[derive(Clone, Copy, Debug)]
pub struct Measurement {
    pub temperature_celsius: I16F16,
    pub humidity_percent: I16F16,
}

#[derive(Clone, Copy, Debug)]
pub enum Precision {
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, Debug)]
pub struct SensorData {
    pub temperature: u16,
    pub humidity: u16,
}

impl From<SensorData> for Measurement {
    fn from(raw: SensorData) -> Self {
        const_fixed_from_int! {
            const MINUS_45: I16F16 = -45;
            const MINUS_6: I16F16 = -6;
        }

        let temperature_quotient = ((raw.temperature as u32) << 16) / (u16::MAX as u32);
        let humidity_quotient = ((raw.humidity as u32) << 16) / (u16::MAX as u32);

        Self {
            temperature_celsius: MINUS_45 + 175 * I16F16::from_bits(temperature_quotient as i32),
            humidity_percent: MINUS_6 + 125 * I16F16::from_bits(humidity_quotient as i32),
        }
    }
}
