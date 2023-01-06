//! Module that provides useful constructs for safe global state.
mod mutex;
mod cell;

pub use mutex::*;
pub use cell::*;
