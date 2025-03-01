use crate::{
    commands::Command,
    error::Error,
    types::{Address, HeatingDuration, HeatingPower, Measurement, Precision, SensorData},
};
use core::marker::PhantomData;
use embedded_hal::{delay::DelayNs, i2c::I2c};
use sensirion_i2c::i2c;

pub(crate) const RESPONSE_LEN: usize = 6;

/// Driver for STH4x sensors.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, Hash, PartialEq)]
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

impl<I, D> Sht4x<I, D>
where
    I: I2c,
    D: DelayNs,
{
    /// Creates a new driver instance using the given I2C bus. It configures the default I2C
    /// address 0x44 used by most family members.
    ///
    /// For operating multiple devices on the same bus,
    /// [`shared-bus`](https://github.com/Rahix/shared-bus) might come in handy.
    pub fn new(i2c: I) -> Self {
        Self::new_with_address(i2c, Address::Address0x44)
    }

    /// Crates a new driver instance using the given I2C bus and address. This constructor allows
    /// to instantiate the driver for the SHT40-BD1B which uses the non-default I2C address 0x45.
    ///
    /// For operating multiple devices on the same bus,
    /// [`shared-bus`](https://github.com/Rahix/shared-bus) might come in handy.
    pub fn new_with_address(i2c: I, address: Address) -> Self {
        Sht4x {
            i2c,
            address,
            _delay: PhantomData,
        }
    }

    /// Destroys the driver and returns the used I2C bus.
    pub fn destroy(self) -> I {
        self.i2c
    }

    /// Activates the heater and performs a measurement returning measurands in SI units.
    ///
    /// **Note:** The heater is designed to be used up to 10 % of the sensor's lifetime. Please
    /// check the
    /// [datasheet](https://sensirion.com/media/documents/33FD6951/624C4357/Datasheet_SHT4x.pdf),
    /// section 4.9 _Heater Operation_ for details.
    pub fn heat_and_measure(
        &mut self,
        power: HeatingPower,
        duration: HeatingDuration,
        delay: &mut D,
    ) -> Result<Measurement, Error<I::Error>> {
        let raw = self.heat_and_measure_raw(power, duration, delay)?;

        Ok(Measurement::from(raw))
    }

    /// Activates the heater and performs a measurement returning raw sensor data.
    ///
    /// **Note:** The heater is designed to be used up to 10 % of the sensor's lifetime. Please
    /// check the
    /// [datasheet](https://sensirion.com/media/documents/33FD6951/624C4357/Datasheet_SHT4x.pdf),
    /// section 4.9 _Heater Operation_ for details.
    pub fn heat_and_measure_raw(
        &mut self,
        power: HeatingPower,
        duration: HeatingDuration,
        delay: &mut D,
    ) -> Result<SensorData, Error<I::Error>> {
        let command = Command::from((power, duration));

        self.write_command_and_delay_for_execution(command, delay)?;
        let response = self.read_response()?;
        let raw = sensor_data_from_response(&response);

        Ok(raw)
    }

    /// Performs a measurement returning measurands in SI units.
    pub fn measure(
        &mut self,
        precision: Precision,
        delay: &mut D,
    ) -> Result<Measurement, Error<I::Error>> {
        let raw = self.measure_raw(precision, delay)?;
        Ok(Measurement::from(raw))
    }

    /// Performs a measurement returning raw sensor data.
    pub fn measure_raw(
        &mut self,
        precision: Precision,
        delay: &mut D,
    ) -> Result<SensorData, Error<I::Error>> {
        let command = Command::from(precision);

        self.write_command_and_delay_for_execution(command, delay)?;
        let response = self.read_response()?;
        let raw = sensor_data_from_response(&response);

        Ok(raw)
    }

    /// Reads the sensor's serial number.
    pub fn serial_number(&mut self, delay: &mut D) -> Result<u32, Error<I::Error>> {
        self.write_command_and_delay_for_execution(Command::SerialNumber, delay)?;
        let response = self.read_response()?;
        Ok(serial_number_from_response(&response))
    }

    /// Performs a soft reset of the sensor.
    pub fn soft_reset(&mut self, delay: &mut D) -> Result<(), Error<I::Error>> {
        self.write_command_and_delay_for_execution(Command::SoftReset, delay)
    }

    fn read_response(&mut self) -> Result<[u8; RESPONSE_LEN], Error<I::Error>> {
        let mut response = [0; RESPONSE_LEN];

        i2c::read_words_with_crc(&mut self.i2c, self.address.into(), &mut response)?;

        Ok(response)
    }

    fn write_command_and_delay_for_execution(
        &mut self,
        command: Command,
        delay: &mut D,
    ) -> Result<(), Error<I::Error>> {
        let code = command.code();

        i2c::write_command_u8(&mut self.i2c, self.address.into(), code).map_err(Error::I2c)?;
        delay.delay_ms(command.duration_ms());

        Ok(())
    }
}

pub(crate) fn sensor_data_from_response(response: &[u8; RESPONSE_LEN]) -> SensorData {
    SensorData {
        temperature: u16::from_be_bytes([response[0], response[1]]),
        humidity: u16::from_be_bytes([response[3], response[4]]),
    }
}

pub(crate) fn serial_number_from_response(response: &[u8; RESPONSE_LEN]) -> u32 {
    u32::from_be_bytes([response[0], response[1], response[3], response[4]])
}
