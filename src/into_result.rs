//! # FlexiArgs
//!
//! A flexible, rule-based command dispatcher and argument parser.
//! This module provides traits and logic to map command-line arguments
//! to application actions with support for strict level validation.

use std::error::Error;

/// Trait for types that can be converted into a standard action result.
///
/// This enables `Arg::action` to accept closures that return either `()`
/// or `Result<(), Box<dyn Error>>`, providing flexibility in how
/// actions are implemented.
pub trait IntoActionResult {
    /// Converts the implementing type into a Result.
    fn into_result(self) -> Result<(), Box<dyn Error>>;
}

impl IntoActionResult for () {
    /// Returns a successful result for actions that don't explicitly return one.
    fn into_result(self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl IntoActionResult for Result<(), Box<dyn Error>> {
    /// Returns the result as-is.
    fn into_result(self) -> Result<(), Box<dyn Error>> {
        self
    }
}
