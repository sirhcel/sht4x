use embedded_hal::blocking::i2c::{Read, Write};
use sensirion_i2c::i2c;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
#[non_exhaustive]
pub enum Error<E> {
    I2c(E),
    Crc,
    Internal,
}

impl<E, W, R> From<i2c::Error<W, R>> for Error<E>
where
    W: Write<Error = E>,
    R: Read<Error = E>,
{
    fn from(err: i2c::Error<W, R>) -> Self {
        match err {
            i2c::Error::Crc => Error::Crc,
            i2c::Error::I2cRead(e) => Error::I2c(e),
            i2c::Error::I2cWrite(e) => Error::I2c(e),
        }
    }
}
