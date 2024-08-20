//! Crate prelude.

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

/// Generic wrapper tuple struct for newtype pattern
///
/// I don't fully understand this yet, but it seems to be a way to wrap a type in a new type.
pub struct W<T>(pub T);

/// Personal preferences
pub use std::format as f;
