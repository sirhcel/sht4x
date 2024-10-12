use fixed::types::{I16F16, I18F14, U16F16};

/// I2C adresses used by STH4x sensors.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Address {
    Address0x44,
    Address0x45,
}

impl From<Address> for u8 {
    fn from(address: Address) -> Self {
        match address {
            Address::Address0x44 => 0x44,
            Address::Address0x45 => 0x45,
        }
    }
}

/// Heating power to apply when activating the internal heater.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HeatingPower {
    /// Operate the heater at 200 mW.
    Low,
    /// Operate the heater at 110 mW.
    Medium,
    /// Operate the heater at 20 mW.
    High,
}

/// Duration of heating when activating the internal heater.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HeatingDuration {
    /// Operate the heater for 100 ms.
    Short,
    /// Operate the heater for 1 s.
    Long,
}

/// A measurement from the sensor in SI units.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Measurement {
    /// The measurred temperature in degree Celsius (°C).
    temperature: I16F16,
    /// The measured relative humidity in percent (%).
    humidity: I16F16,
}

/// The precision to request for a measurement.
///
/// Higher-precision measurements take longer.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Precision {
    Low,
    Medium,
    High,
}

/// A measurement from the sensor in raw sensor data.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SensorData {
    /// The measured temperature as raw sensor value.
    pub temperature: u16,
    /// The measured realtive humidity as raw sensor value.
    pub humidity: u16,
}

#[cfg(feature = "defmt")]
impl defmt::Format for Measurement {
    fn format(&self, f: defmt::Formatter) {
        // Format as milli units for a tradeoff between readability and efficiency. The conversion
        // should compile to a shift and a multiplication.
        //
        // TODO: Are there any means to control the rendering on the host side?
        defmt::write!(
            f,
            "Measurement {{ {} m°C, {} m% }}",
            self.temperature_milli_celsius(),
            self.humidity_milli_percent(),
        );
    }
}

impl From<SensorData> for Measurement {
    /// Converts raw sensor data into SI units.
    fn from(raw: SensorData) -> Self {
        const MINUS_45: I16F16 = I16F16::const_from_int(-45);
        const MINUS_6: I16F16 = I16F16::const_from_int(-6);

        let temperature_quotient = U16F16::from_num(raw.temperature) / (u16::MAX as u32);
        let humidity_quotient = U16F16::from_num(raw.humidity) / (u16::MAX as u32);

        Self {
            temperature: MINUS_45 + 175 * temperature_quotient.to_num::<I16F16>(),
            humidity: MINUS_6 + 125 * humidity_quotient.to_num::<I16F16>(),
        }
    }
}

impl Measurement {
    /// Returns the measured temperature in degree Celsius (°C).
    pub fn temperature_celsius(&self) -> I16F16 {
        self.temperature
    }

    /// Returns the measured temperature in milli degree Celsius (m°C, a thousand of a degree
    /// Celsius).
    pub fn temperature_milli_celsius(&self) -> i32 {
        // Pre-scale to keep the multiplication to millis within the underlying
        // i32 type.
        let milli = self.temperature.to_num::<I18F14>() * 1000;
        milli.to_num::<i32>()
    }

    /// Returns the measured relative humidity in milli percent (m% RH, a thousand of a percent).
    pub fn humidity_milli_percent(&self) -> i32 {
        // Pre-scale to keep the multiplication to millis within the underlying
        // i32 type.
        let milli = self.humidity.to_num::<I18F14>() * 1000;
        milli.to_num::<i32>()
    }

    /// Returns the measured relative humidity in percent (%).
    pub fn humidity_percent(&self) -> I16F16 {
        self.humidity
    }
}
