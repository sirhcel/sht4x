use crate::{
    commands::Command,
    error::Error,
};
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use fixed::{
    types::I16F16,
    const_fixed_from_int,
};
use sensirion_i2c::i2c;

// FIXME: Add defmt support for structs and enums. Mutually exclusive with
// Debug?

const RESPONSE_LEN: usize = 6;

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
pub enum Precision {
    Low,
    Medium,
    High,
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

impl From<(HeatingPower, HeatingDuration)> for Command {
    fn from((power, duration): (HeatingPower, HeatingDuration)) -> Self {
        match (power, duration) {
            (HeatingPower::Low, HeatingDuration::Short) => Command::MeasureHeated20mw0p1s,
            (HeatingPower::Low, HeatingDuration::Long) => Command::MeasureHeated20mw1s,
            (HeatingPower::Medium, HeatingDuration::Short) => Command::MeasureHeated110mw0p1s,
            (HeatingPower::Medium, HeatingDuration::Long) => Command::MeasureHeated110mw1s,
            (HeatingPower::High, HeatingDuration::Short) => Command::MeasureHeated200mw0p1s,
            (HeatingPower::High, HeatingDuration::Long) => Command::MeasureHeated200mw1s,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FixedSensorData {
    pub temperature_celsius: I16F16,
    pub humidity_percent: I16F16,
}

impl From<RawSensorData> for FixedSensorData {
    fn from(raw: RawSensorData) -> Self {
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

#[derive(Clone, Copy, Debug)]
pub struct RawSensorData {
    pub temperature: u16,
    pub humidity: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct FloatSensorData {
    pub temperature_celsius: f32,
    pub humidity_percent: f32,
}

impl From<RawSensorData> for FloatSensorData {
   fn from(raw: RawSensorData) -> Self {
       let fixed = FixedSensorData::from(raw);

       Self {
            temperature_celsius: fixed.temperature_celsius.to_num(),
            humidity_percent: fixed.humidity_percent.to_num(),
       }
   }
}

pub struct Sht4x<I> {
    i2c: I,
    address: Address,
}

impl<I, E> Sht4x<I>
where
    I: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I) -> Self {
        Self::new_with_address(i2c, Address::Address0x44)
    }

    pub fn new_with_address(i2c: I, address: Address) -> Self {
        Sht4x {
            i2c,
            address,
        }
    }

    pub fn heated_measurement(&mut self, power: HeatingPower, duration: HeatingDuration) -> Result<RawSensorData, Error<E>> {
        let command = Command::from((power, duration));

        self.write_command(command)?;
        let raw = self.read_raw_measurement()?;

        Ok(raw)
    }

    pub fn measurement(&mut self, precision: Precision) -> Result<RawSensorData, Error<E>> {
        let command = match precision {
            Precision::Low => Command::MeasureLowPrecision,
            Precision::Medium => Command::MeasureMediumPrecision,
            Precision::High => Command::MeasureHighPrecision,
        };

        self.write_command(command)?;
        let raw = self.read_raw_measurement()?;

        Ok(raw)
    }

    pub fn serial_number(&mut self) -> Result<u32, Error<E>> {
        self.write_command(Command::SerialNumber)?;
        let response = self.read_response()?;

        Ok(u32::from_be_bytes([response[0], response[1], response[3], response[4]]))
    }

    pub fn soft_reset(&mut self) -> Result<(), Error<E>> {
        self.write_command(Command::SoftReset)
    }

    fn read_raw_measurement(&mut self) -> Result<RawSensorData, Error<E>> {
        let response = self.read_response()?;
        let result = RawSensorData {
            temperature: u16::from_be_bytes([response[0], response[1]]),
            humidity: u16::from_be_bytes([response[3], response[4]]),
        };

        Ok(result)
    }

    fn read_response(&mut self) -> Result<[u8; RESPONSE_LEN], Error<E>> {
        let mut response = [0; RESPONSE_LEN];

        // FIXME: Choose a meaningful value. What about providing one from
        // Command?
        for _ in 0..10000 {
            let result = i2c::read_words_with_crc(&mut self.i2c, self.address.into(), &mut response);
            match result {
                // FIXME: Is there really no generic way for checking for NACK?
                Err(_) => {},
                Ok(_) => return Ok(response),
            }
        }

        Err(Error::NoResponse)
    }

    fn write_command(&mut self, command: Command) -> Result<(), Error<E>> {
        let code = command.code();

        i2c::write_command_u8(&mut self.i2c, self.address.into(), code).map_err(Error::I2c)
    }
}
