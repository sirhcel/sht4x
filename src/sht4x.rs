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
    Milliwatts20,
    Milliwatts110,
    Milliwatts200,
}

#[derive(Clone, Copy, Debug)]
pub enum HeatingDuration {
    Milliseconds100,
    Milliseconds1000,
}

#[derive(Clone, Copy, Debug)]
pub struct FixedSensorData {
    pub temperature_celsius: I16F16,
    pub humidity_percent: I16F16,
}

impl From<(HeatingPower, HeatingDuration)> for Command {
    fn from((power, duration): (HeatingPower, HeatingDuration)) -> Self {
        match (power, duration) {
            (HeatingPower::Milliwatts20, HeatingDuration::Milliseconds100) => Command::MeasureHeated20Mw100Ms,
            (HeatingPower::Milliwatts20, HeatingDuration::Milliseconds1000) => Command::MeasureHeated20Mw1S,
            (HeatingPower::Milliwatts110, HeatingDuration::Milliseconds100) => Command::MeasureHeated110Mw100Ms,
            (HeatingPower::Milliwatts110, HeatingDuration::Milliseconds1000) => Command::MeasureHeated110Mw1S,
            (HeatingPower::Milliwatts200, HeatingDuration::Milliseconds100) => Command::MeasureHeated200Mw100Ms,
            (HeatingPower::Milliwatts200, HeatingDuration::Milliseconds1000) => Command::MeasureHeated200Mw1S,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RawSensorData {
    pub temperature: u16,
    pub humidity: u16,
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
pub struct SensorData {
    pub temperature_celsius: f32,
    pub humidity_percent: f32,
}

impl From<RawSensorData> for SensorData {
   fn from(raw: RawSensorData) -> Self {
       Self {
            // TODO: What about using the variant from Sensirion's examples
            // which uses mor integer arithmetic? Or simply converting the
            // FixedSensorData to floats?
            temperature_celsius: -45.0 + 175.0 * (raw.temperature as f32) / (u16::MAX as f32),
            humidity_percent: -6.0 + 125.0 * (raw.humidity as f32) / (u16::MAX as f32),
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

    pub fn heated_measurement(&mut self, power: HeatingPower, duration: HeatingDuration) -> Result<SensorData, Error<E>> {
        let command = Command::from((power, duration));

        self.write_command(command)?;
        let raw = self.read_raw_measurement()?;

        Ok(SensorData::from(raw))
    }

    pub fn heated_measurement_fixed(&mut self, power: HeatingPower, duration: HeatingDuration) -> Result<FixedSensorData, Error<E>> {
        let command = Command::from((power, duration));

        self.write_command(command)?;
        let raw = self.read_raw_measurement()?;

        Ok(FixedSensorData::from(raw))
    }

    pub fn measurement(&mut self, precision: Precision) -> Result<SensorData, Error<E>> {
        let raw = self.sensor_output(precision)?;
        Ok(SensorData::from(raw))
    }

    pub fn measurement_fixed(&mut self, precision: Precision) -> Result<FixedSensorData, Error<E>> {
        let raw = self.sensor_output(precision)?;
        Ok(FixedSensorData::from(raw))
    }

    pub fn sensor_output(&mut self, precision: Precision) -> Result<RawSensorData, Error<E>> {
        let command = match precision {
            Precision::Low => Command::MeasureLowPrecision,
            Precision::Medium => Command::MeasureMediumPrecision,
            Precision::High => Command::MeasureHighPrecision,
        };

        self.write_command(command)?;
        let result = self.read_raw_measurement()?;

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
