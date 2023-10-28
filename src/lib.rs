#![deny(unsafe_code)]
#![no_std]
#![doc = include_str!("../README.md")]

mod commands;
mod error;
mod sht4x;
mod types;

pub use crate::error::*;
pub use crate::sht4x::*;
pub use crate::types::*;

#[cfg(all(feature = "blocking", feature = "async"))]
compile_error!("Cannot enable both `blocking` and `async` features");
