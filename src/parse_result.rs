//! # Parse Result Handling
//!
//! This module defines the `ParseResult` structure and its associated methods.
//! It provides a fluent API for enforcing validation rules (strictness) and
//! managing arguments that were not matched during the parsing phase.

use std::error::Error;
use std::io;

/// Holds the outcome of the parsing operation.
pub struct ParseResult {
    /// The result of the parsing loop.
    pub res: Result<(), Box<dyn Error>>,
    /// Indicates if the help flag was requested by the user.
    pub help_requested: bool,
}

impl ParseResult {
    /// Checks if help was requested and validates the parsing result.
    ///
    /// # Returns
    /// * `Ok(true)` - If help was requested by the user.
    /// * `Ok(false)` - If parsing was successful and no help was requested.
    /// * `Err(Box<dyn Error>)` - If a parsing error occurred.
    pub fn help_or_err(self) -> Result<bool, Box<dyn Error>> {
        if self.help_requested {
            return Ok(true);
        }

        self.res
            .as_ref()
            .map_err(|e| Box::new(io::Error::new(io::ErrorKind::Other, e.to_string())))?;

        Ok(false)
    }

    /// Unwraps the inner result of the parsing operation.
    ///
    /// # Returns
    /// The result of the argument processing loop.
    pub fn ok(self) -> Result<(), Box<dyn Error>> {
        self.res
    }
}
