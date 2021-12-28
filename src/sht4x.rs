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

const RESPONSE_LEN: usize = 6;

// FIXME: Add support for defmt. Mutually exclusive with Debug?
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

// FIXME: Add support for defmt. Mutually exclusive with Debug?
#[derive(Clone, Copy, Debug)]
pub enum Precision {
    Low,
    Medium,
    High,
}

// FIXME: Add support for defmt. Mutually exclusive with Debug?
#[derive(Clone, Copy, Debug)]
pub struct FixedSensorData {
    pub temperature_celsius: I16F16,
    pub humidity_percent: I16F16,
}

// FIXME: Add support for defmt. Mutually exclusive with Debug?
#[derive(Clone, Copy, Debug)]
pub struct RawSensorData {
    pub temperature: u16,
    pub humidity: u16,
}

// FIXME: Add support for defmt. Mutually exclusive with Debug?
#[derive(Clone, Copy, Debug)]
pub struct SensorData {
    pub temperature_celsius: f32,
    pub humidity_percent: f32,
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

    pub fn measurement(&mut self, precision: Precision) -> Result<SensorData, Error<E>> {
        let raw = self.sensor_output(precision)?;

        let result = SensorData {
            temperature_celsius: (((21_875 * raw.temperature as i32) >> 13) - 45_000) as f32 / 1_000.0,
            humidity_percent: (((15_625 * raw.humidity as i32) >> 13) - 6_000) as f32 / 1_000.0,
        };

        Ok(result)
    }

    pub fn measurement_datasheet(&mut self, precision: Precision) -> Result<SensorData, Error<E>> {
        let raw = self.sensor_output(precision)?;

        let result = SensorData {
            temperature_celsius: -45.0 + 175.0 * (raw.temperature as f32) / (u16::MAX as f32),
            humidity_percent: -6.0 + 125.0 * (raw.humidity as f32) / (u16::MAX as f32),
        };

        Ok(result)
    }

    pub fn measurement_fixed(&mut self, precision: Precision) -> Result<FixedSensorData, Error<E>> {
        const_fixed_from_int! {
            const MINUS_45: I16F16 = -45;
            const MINUS_6: I16F16 = -6;
        }

        let raw = self.sensor_output(precision)?;

        let temperature_quotient = ((raw.temperature as u32) << 16) / (u16::MAX as u32);
        let humidity_quotient = ((raw.humidity as u32) << 16) / (u16::MAX as u32);

        let result = FixedSensorData {
            temperature_celsius: MINUS_45 + 175 * I16F16::from_bits(temperature_quotient as i32),
            humidity_percent: MINUS_6 + 125 * I16F16::from_bits(humidity_quotient as i32),
        };

        Ok(result)
    }

    pub fn sensor_output(&mut self, precision: Precision) -> Result<RawSensorData, Error<E>> {
        let command = match precision {
            Precision::Low => Command::MeasureLowPrecision,
            Precision::Medium => Command::MeasureMediumPrecision,
            Precision::High => Command::MeasureHighPrecision,
        };

        self.write_command(command)?;
        let response = self.read_response()?;
        let result = RawSensorData {
            temperature: u16::from_be_bytes([response[0], response[1]]),
            humidity: u16::from_be_bytes([response[3], response[4]]),
        };

        Ok(result)
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

        for _ in 0..100 {
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
