use embedded_hal::i2c::I2c;

/// Error conditions from accessing SHT4x sensors.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
#[non_exhaustive]
pub enum Error<E> {
    /// Failed I2C communication.
    I2c(E),
    /// Failed CRC verification of sensor data.
    Crc,
}

impl<I: I2c> From<sensirion_i2c::i2c::Error<I>> for Error<I::Error> {
    fn from(err: sensirion_i2c::i2c::Error<I>) -> Self {
        match err {
            sensirion_i2c::i2c::Error::Crc => Error::Crc,
            sensirion_i2c::i2c::Error::I2cRead(e) => Error::I2c(e),
            sensirion_i2c::i2c::Error::I2cWrite(e) => Error::I2c(e),
        }
    }
}

#[cfg(feature = "async")]
impl<E> From<sensirion_i2c::crc8::Error> for Error<E> {
    fn from(value: sensirion_i2c::crc8::Error) -> Self {
        match value {
            sensirion_i2c::crc8::Error::CrcError => Error::Crc,
        }
    }
}
