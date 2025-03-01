#![deny(unsafe_code)]
#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod commands;
mod error;
mod responses;
mod sht4x;
mod types;

#[cfg(feature = "embedded-hal-async")]
mod sht4x_async;
#[cfg(feature = "embedded-hal-async")]
pub use self::sht4x_async::Sht4xAsync;

pub use crate::error::*;
pub use crate::sht4x::*;
pub use crate::types::*;
