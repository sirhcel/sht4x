use crate::{
    commands::Command,
    error::Error,
    types::{Address, HeatingDuration, HeatingPower, Measurement, Precision, SensorData},
};
use core::marker::PhantomData;
use embedded_hal::blocking::{
    delay::DelayMs,
    i2c::{Read, Write, WriteRead},
};
use sensirion_i2c::i2c;

// FIXME: Add defmt support for structs and enums. Mutually exclusive with
// Debug?

const RESPONSE_LEN: usize = 6;

pub struct Sht4x<I, D> {
    i2c: I,
    address: Address,
    // If we want to globally define the delay type for this struct, we have to consume the type
    // parameter.
    _delay: PhantomData<D>,
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

impl<I, D, E> Sht4x<I, D>
where
    I: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    D: DelayMs<u16>,
{
    pub fn new(i2c: I) -> Self {
        Self::new_with_address(i2c, Address::Address0x44)
    }

    pub fn new_with_address(i2c: I, address: Address) -> Self {
        Sht4x {
            i2c,
            address,
            _delay: PhantomData,
        }
    }

    pub fn heat_and_measure(
        &mut self,
        power: HeatingPower,
        duration: HeatingDuration,
        delay: &mut D,
    ) -> Result<Measurement, Error<E>> {
        let raw = self.heat_and_measure_raw(power, duration, delay)?;

        Ok(Measurement::from(raw))
    }

    pub fn heat_and_measure_raw(
        &mut self,
        power: HeatingPower,
        duration: HeatingDuration,
        delay: &mut D,
    ) -> Result<SensorData, Error<E>> {
        let command = Command::from((power, duration));

        self.write_command_and_delay_for_execution(command, delay)?;
        let response = self.read_response()?;
        let raw = self.sensor_data_from_response(&response);

        Ok(raw)
    }

    pub fn measure(
        &mut self,
        precision: Precision,
        delay: &mut D,
    ) -> Result<Measurement, Error<E>> {
        let raw = self.measure_raw(precision, delay)?;
        Ok(Measurement::from(raw))
    }

    pub fn measure_raw(
        &mut self,
        precision: Precision,
        delay: &mut D,
    ) -> Result<SensorData, Error<E>> {
        let command = Command::from(precision);

        self.write_command_and_delay_for_execution(command, delay)?;
        let response = self.read_response()?;
        let raw = self.sensor_data_from_response(&response);

        Ok(raw)
    }

    pub fn serial_number(&mut self, delay: &mut D) -> Result<u32, Error<E>> {
        self.write_command_and_delay_for_execution(Command::SerialNumber, delay)?;
        let response = self.read_response()?;

        Ok(u32::from_be_bytes([
            response[0],
            response[1],
            response[3],
            response[4],
        ]))
    }

    pub fn soft_reset(&mut self, delay: &mut D) -> Result<(), Error<E>> {
        self.write_command_and_delay_for_execution(Command::SoftReset, delay)
    }

    fn read_response(&mut self) -> Result<[u8; RESPONSE_LEN], Error<E>> {
        let mut response = [0; RESPONSE_LEN];

        i2c::read_words_with_crc(&mut self.i2c, self.address.into(), &mut response)?;

        Ok(response)
    }

    fn sensor_data_from_response(&self, response: &[u8; RESPONSE_LEN]) -> SensorData {
        SensorData {
            temperature: u16::from_be_bytes([response[0], response[1]]),
            humidity: u16::from_be_bytes([response[3], response[4]]),
        }
    }

    fn write_command_and_delay_for_execution(
        &mut self,
        command: Command,
        delay: &mut D,
    ) -> Result<(), Error<E>> {
        let code = command.code();

        i2c::write_command_u8(&mut self.i2c, self.address.into(), code).map_err(Error::I2c)?;
        if let Some(ms) = command.duration_ms() {
            delay.delay_ms(ms);
        }

        Ok(())
    }
}
