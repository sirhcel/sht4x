#![deny(unsafe_code)]
#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod commands;
mod error;
mod sht4x;
mod types;

#[cfg(feature = "embedded-hal-async")]
mod async_sht4x;
#[cfg(feature = "embedded-hal-async")]
pub use self::async_sht4x::Sht4xAsync;

pub use crate::error::*;
pub use crate::sht4x::*;
pub use crate::types::*;
