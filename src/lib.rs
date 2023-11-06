#![doc = include_str!("../README.md")]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![no_std]

mod assert;
mod cast;

pub use assert::Assert;
pub use cast::{Cast, CastFrom, CastInto};
