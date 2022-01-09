use fixed::{
    types::{I16F16, I18F14, U16F16},
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

        let temperature_quotient = U16F16::from_num(raw.temperature) / (u16::MAX as u32);
        let humidity_quotient = U16F16::from_num(raw.humidity) / (u16::MAX as u32);

        Self {
            temperature_celsius: MINUS_45 + 175 * temperature_quotient.to_num::<I16F16>(),
            humidity_percent: MINUS_6 + 125 * humidity_quotient.to_num::<I16F16>(),
        }
    }
}

impl Measurement {
    pub fn temperature_milli_celsius(&self) -> i32 {
        // Pre-scale to keep the multiplication to millis within the underlying
        // i32 type.
        let milli = self.temperature_celsius.to_num::<I18F14>() * 1000;
        milli.to_num::<i32>()
    }

    pub fn humidity_milli_percent(&self) -> i32 {
        // Pre-scale to keep the multiplication to millis within the underlying
        // i32 type.
        let milli = self.humidity_percent.to_num::<I18F14>() * 1000;
        milli.to_num::<i32>()
    }
}
