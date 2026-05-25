//! # Parser Configuration
//!
//! Provides the configuration structure required to initialize and execute
//! argument parsing operations within the FlexiArgs framework.

use crate::ArgHelp;

/// Defines the operational parameters for the command-line argument parser.
pub struct ParserOptions<'a> {
    /// The name of the active subcommand, used for grouping and error reporting.
    pub subcommand: &'a str,
    /// A reference to the slice of `ArgHelp` definitions for the current context.
    pub help_rules: &'a [ArgHelp<'a>],
    /// If set to true, disables the automatic processing of `--help` and `--version` flags.
    pub ignore_help: bool,
}

impl<'a> Default for ParserOptions<'a> {
    /// Creates a default `ParserOptions` instance with empty context and standard behavior.
    fn default() -> Self {
        Self {
            subcommand: "",
            help_rules: &[],
            ignore_help: false,
        }
    }
}