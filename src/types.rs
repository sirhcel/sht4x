use fixed::{
    const_fixed_from_int,
    types::{I16F16, I18F14, U16F16},
};

/// I2C adresses used by STH4x sensors.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
pub enum HeatingDuration {
    /// Operate the heater for 100 ms.
    Short,
    /// Operate the heater for 1 s.
    Long,
}

/// A measurement from the sensor in SI units.
#[derive(Clone, Copy, Debug)]
pub struct Measurement {
    /// The measurred temperature in degree Celsius (°C).
    pub temperature_celsius: I16F16,
    /// The measured relative humidity in percent (%).
    pub humidity_percent: I16F16,
}

/// The precision to request for a measurement.
///
/// Higher-precision measurements take longer.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug)]
pub enum Precision {
    Low,
    Medium,
    High,
}

/// A measurement from the sensor in raw sensor data.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug)]
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
            "Measurement {{ temperature m°C: {}, humidity m%: {} }}",
            self.temperature_milli_celsius(),
            self.humidity_milli_percent(),
        );
    }
}

impl From<SensorData> for Measurement {
    /// Converts raw sensor data into SI units.
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
    /// Returns the measured temperature in milli degree Celsius (m°C, a thousand of a degree
    /// Celsius).
    ///
    /// # Panic
    ///
    /// Panics if the conversion overflows. It is safe for safe for all [`Measurement`]s obtained
    /// from [`SensorData`] but might panic for other values.
    pub fn temperature_milli_celsius(&self) -> i32 {
        // Pre-scale to keep the multiplication to millis within the underlying
        // i32 type.
        let milli = self.temperature_celsius.to_num::<I18F14>() * 1000;
        milli.to_num::<i32>()
    }

    /// Returns the measured temperature in milli percent relative humidity (m% RH, a thousand of a
    /// percent).
    ///
    /// # Panic
    ///
    /// Panics if the conversion overflows. It is safe for safe for all [`Measurement`]s obtained
    /// from [`SensorData`] but might panic for other values.
    pub fn humidity_milli_percent(&self) -> i32 {
        // Pre-scale to keep the multiplication to millis within the underlying
        // i32 type.
        let milli = self.humidity_percent.to_num::<I18F14>() * 1000;
        milli.to_num::<i32>()
    }
}
