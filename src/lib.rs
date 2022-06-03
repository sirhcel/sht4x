#![deny(unsafe_code)]
#![no_std]

//! A platform agnostic device driver for the Sensirion [SHT4x temperature and humidity sensor
//! family](https://sensirion.com/media/documents/33FD6951/624C4357/Datasheet_SHT4x.pdf). It is
//! based on [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits and works in
//! `no_std` environments.
//!
//! In theory, it supports all of the sensor family's devices but has only been tested with the
//! SHT40-AD1B so far.
//!
//!
//! # Features
//!
//! - Blocking operation
//! - Supports all commands specified in the
//!   [datasheet](https://sensirion.com/media/documents/33FD6951/624C4357/Datasheet_SHT4x.pdf)
//! - Explicitly borrows `DelayMs` for command execution so that it could be shared (among multiple
//!   sensors)
//! - Could be instatiated with the alternative I2C address for the SHT40-BD1B
//! - Uses fixed-point arithmetics for converting raw sensor data into measurements in SI units
//!     - Based on `I16F16` from the [`fixed`](https://gitlab.com/tspiteri/fixed) crate
//!     - Allows conversion to floating-point values, if needed
//!     - Convenience methods for fixed-point conversions to milli degree Celsius or milli percent
//!       relative humidity which are commonly used by drivers for other humidity and temperature
//!       sensors from Sensirion
//! - Optional support for [`defmt`](https://github.com/knurling-rs/defmt)

mod commands;
mod error;
mod sht4x;
mod types;

pub use crate::error::*;
pub use crate::sht4x::*;
pub use crate::types::*;
