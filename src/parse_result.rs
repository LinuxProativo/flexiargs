//! # Parse Result Handling
//!
//! This module defines the `ParseResult` structure and its associated methods.
//! It provides a fluent API for enforcing validation rules (strictness) and
//! managing arguments that were not matched during the parsing phase.

use crate::missing_arg;
use std::collections::VecDeque;
use std::error::Error;
use std::io;

/// Holds the outcome of the parsing operation.
pub struct ParseResult<'a> {
    /// The subcommand name used for context in errors.
    pub sub: &'a str,
    /// Indicates if the input argument queue was empty.
    pub empty: bool,
    /// The result of the parsing loop.
    pub res: Result<(), Box<dyn Error>>,
    /// Arguments that were not matched by any rule or followed '--'.
    pub remaining: VecDeque<String>,
    /// The original input positions of the arguments stored in `remaining`.
    pub arg_indices: Vec<usize>,
    /// Internal flag indicating if a core parameter was missing.
    pub essential_failed: bool,
    /// Indicates if the help flag was requested by the user.
    pub help_requested: bool,
}

impl<'a> ParseResult<'a> {
    /// Suppresses any existing parsing errors, resetting the result to `Ok(())`.
    /// Useful for optional parsing passes where errors should be ignored.
    pub fn passthrough(mut self) -> Self {
        if let Err(_) = self.res {
            self.res = Ok(());
        }
        self
    }

    /// Ensures that at least one argument was provided.
    ///
    /// # Returns
    /// An error if the input was empty, otherwise returns the parsing result.
    pub fn require_args(self) -> Result<Self, Box<dyn Error>> {
        if self.help_requested {
            return Ok(self);
        }

        if self.empty {
            return Err(missing_arg(self.sub, false));
        }
        self.res
            .as_ref()
            .map_err(|e| Box::new(io::Error::new(io::ErrorKind::Other, e.to_string())))?;
        Ok(self)
    }

    /// todo
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
