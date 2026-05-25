//! Argument Parser
//!
//! A lightweight, flexible argument parser library.
//! It supports subcommands, essential argument validation, and multiple argument collection.
//!
//! # Example
//!
//! ```rust,norun
//! use flexiargs::{Arg, parse_into_vars};
//! use std::collections::VecDeque;
//!
//! // Variables to store parsed values
//! let mut sync_mode = false;
//! let mut packages = Vec::new();
//! let mut cache_dir = String::new();
//! let mut config_file: Option<String> = None;
//! let mut use_overlay = true; // Default state
//!
//! let mut rules = [
//!     Arg::bool(Some("-S"), "--sync", &mut sync_mode),
//!     Arg::collect_list(None, "--pkgs", "packages", &mut packages),
//!     Arg::value(None, "--cache-dir", "path", &mut cache_dir),
//!     Arg::option(Some("-c"), "--config", "file", &mut config_file),
//!     Arg::set(None, "--disable-overlay", false, &mut use_overlay),
//! ];
//!
//! // Input arguments simulation
//! let args = VecDeque::from(vec![
//!     "-S".to_string(),
//!     "wget".to_string(),
//!     "curl".to_string(),
//!     "--cache-dir=/tmp".to_string(),
//!     "--disable-overlay".to_string()
//! ]);
//!
//! // Parse arguments into the defined variables
//! parse_into_vars("aports", &mut rules, args).ok();
//!
//! // Explicitly drop rules to release mutable borrows
//! drop(rules);
//!
//! // The variables are now available for use without borrow conflicts
//! println!("Sync: {}, Packages: {:?}", sync_mode, packages);
//! ```

mod flexiargs;
mod help;
mod into_result;
mod messages;
mod parse_result;
mod options;

/// Re-exports error reporting functions for invalid and missing arguments.
pub use messages::{invalid_arg, missing_arg};

/// Re-exports the help documentation structures and utilities.
pub use help::ArgHelp;

/// Re-exports the configuration structure used for argument parsing.
pub use options::ParserOptions;

/// Re-exports core parsing logic and argument rule definitions.
pub use flexiargs::{Arg, NULL_PTR, parse_into_vars};

#[cfg(test)]
mod tests;
