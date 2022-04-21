use fixed::{
    const_fixed_from_int,
    types::{I16F16, I18F14, U16F16},
};

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

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug)]
pub enum Precision {
    Low,
    Medium,
    High,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug)]
pub struct SensorData {
    pub temperature: u16,
    pub humidity: u16,
}

#[cfg(feature = "defmt")]
impl defmt::Format for Measurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            // Let's start with just formatting it into the raw bits. The I16F16 representation
            // should be reable easily readable with some training and rendering the fields as
            // bitfields poses around the same challenges - at least for negative values.
            "Measurement {{ temperature_celsius: {:x}, humidity_percent: {:x} }}",
            self.temperature_celsius.to_bits(),
            self.humidity_percent.to_bits()
        );
    }
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
