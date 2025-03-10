use sensirion_i2c::i2c;

/// Error conditions from accessing SHT4x sensors.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Error<E> {
    /// Failed I2C communication.
    I2c(E),
    /// Failed CRC verification of sensor data.
    Crc,
}

impl<E> From<i2c::Error<E>> for Error<E::Error>
where
    E: embedded_hal::i2c::ErrorType,
{
    fn from(err: i2c::Error<E>) -> Self {
        match err {
            i2c::Error::Crc => Error::Crc,
            i2c::Error::I2cRead(e) => Error::I2c(e),
            i2c::Error::I2cWrite(e) => Error::I2c(e),
        }
    }
}
