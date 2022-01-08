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

#[derive(Clone, Copy, Debug)]
pub struct Measurement {
    pub temperature_celsius: I16F16,
    pub humidity_percent: I16F16,
}

#[derive(Clone, Copy, Debug)]
pub struct SensorData {
    pub temperature: u16,
    pub humidity: u16,
}

pub struct Sht4x<I> {
    i2c: I,
    address: Address,
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

impl From<Precision> for Command {
    fn from(precision: Precision) -> Self {
        match precision {
            Precision::Low => Command::MeasureLowPrecision,
            Precision::Medium => Command::MeasureMediumPrecision,
            Precision::High => Command::MeasureHighPrecision,
        }
    }
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

    pub fn heat_and_measure(&mut self, power: HeatingPower, duration: HeatingDuration) -> Result<Measurement, Error<E>> {
        let raw = self.heat_and_measure_raw(power, duration)?;

        Ok(Measurement::from(raw))
    }

    pub fn heat_and_measure_raw(&mut self, power: HeatingPower, duration: HeatingDuration) -> Result<SensorData, Error<E>> {
        let command = Command::from((power, duration));

        self.write_command(command)?;
        let response = self.read_response()?;
        let raw = self.sensor_data_from_response(&response);

        Ok(raw)
    }

    pub fn measure(&mut self, precision: Precision) -> Result<Measurement, Error<E>> {
        let raw = self.measure_raw(precision)?;
        Ok(Measurement::from(raw))
    }

    pub fn measure_raw(&mut self, precision: Precision) -> Result<SensorData, Error<E>> {
        let command = Command::from(precision);

        self.write_command(command)?;
        let response = self.read_response()?;
        let raw = self.sensor_data_from_response(&response);

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

    fn read_response(&mut self) -> Result<[u8; RESPONSE_LEN], Error<E>> {
        let mut response = [0; RESPONSE_LEN];

        // FIXME: This sensor supports I2C fast mode plus (1 MHz) so by just
        // counting transactions, we could end up with an error of a magnitude.
        // Let's use a timer then.
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

    fn sensor_data_from_response(&self, response: &[u8; RESPONSE_LEN]) -> SensorData {
        SensorData {
            temperature: u16::from_be_bytes([response[0], response[1]]),
            humidity: u16::from_be_bytes([response[3], response[4]]),
        }
    }

    fn write_command(&mut self, command: Command) -> Result<(), Error<E>> {
        let code = command.code();

        i2c::write_command_u8(&mut self.i2c, self.address.into(), code).map_err(Error::I2c)
    }
}
