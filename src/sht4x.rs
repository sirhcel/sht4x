use crate::{
    commands::Command,
    error::Error,
};
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use sensirion_i2c::i2c;

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

    pub fn serial_number(&mut self) -> Result<u32, Error<E>> {
        let mut buf = [0; 6];

        self.write_command(Command::SerialNumber)?;
        // FIXME: What would be a meaningful number of attempts.
        for _ in 0..100 {
            let result = i2c::read_words_with_crc(&mut self.i2c, self.address.into(), &mut buf);
            match result {
                // FIXME: Is there really no generic way for checking for NACK?
                Err(_) => {},
                Ok(_) => return Ok(u32::from_be_bytes([buf[0], buf[1], buf[3], buf[4]])),
            }
        }

        Err(Error::NoResponse)
    }

    pub fn soft_reset(&mut self) -> Result<(), Error<E>> {
        self.write_command(Command::SoftReset)
    }

    fn write_command(&mut self, command: Command) -> Result<(), Error<E>> {
        let code = command.code();

        i2c::write_command_u8(&mut self.i2c, self.address.into(), code).map_err(Error::I2c)
    }
}
