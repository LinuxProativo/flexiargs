//! # Parse Result Handling
//!
//! This module defines the `ParseResult` structure and its associated methods.
//! It provides a fluent API for enforcing validation rules (strictness) and
//! managing arguments that were not matched during the parsing phase.

use std::collections::VecDeque;
use std::error::Error;
use crate::{invalid_arg, missing_arg};

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
}

impl<'a> ParseResult<'a> {
    /// Zero-tolerance mode: Any unmatched argument in `remaining` will trigger an error.
    pub fn strict(mut self) -> Self {
        if self.res.is_err() {
            return self;
        }

        if let Some(arg) = self.remaining.front() {
            self.res = Err(invalid_arg(self.sub, arg));
        }
        self
    }

    /// Invalidates unmatched arguments up to a specific input depth.
    ///
    /// # Arguments
    /// * `n` - The positional threshold. Any unmatched argument at or before this position
    ///         triggers an error. Arguments appearing after this position are allowed.
    pub fn strict_level(mut self, n: usize) -> Self {
        if self.res.is_err() {
            return self;
        }

        for (i, &pos) in self.arg_indices.iter().enumerate() {
            if pos <= n {
                if let Some(arg) = self.remaining.get(i) {
                    self.res = Err(invalid_arg(self.sub, arg));
                    break;
                }
            }
        }
        self
    }

    /// Shortcut for `strict_level(1)`. Ensures the first argument must match a rule.
    pub fn strict_first(self) -> Self {
        self.strict_level(1)
    }

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
        if self.empty {
            return Err(missing_arg(self.sub, false));
        }
        self.res.as_ref().map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        Ok(self)
    }

    /// Unwraps the inner result of the parsing operation.
    ///
    /// # Returns
    /// The result of the argument processing loop.
    pub fn ok(self) -> Result<(), Box<dyn Error>> {
        self.res
    }

    /// Collects remaining arguments into a provided vector.
    ///
    /// # Arguments
    /// * `target` - Mutable reference to a vector where remaining arguments will be stored.
    ///
    /// # Returns
    /// The original parsing result, allowing for further chaining if needed.
    pub fn collect_rest(self, target: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
        let result = self.res;
        if result.is_ok() {
            target.extend(self.remaining);
        }
        result
    }
}
