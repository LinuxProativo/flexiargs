//! # Parser Configuration
//!
//! Provides the configuration structure required to initialize and execute
//! argument parsing operations within the FlexiArgs framework.

use crate::{ArgHelp, NULL_PTR};

/// Defines the operational parameters for the command-line argument parser.
pub struct ParserOptions<'a> {
    /// The name of the active subcommand, used for grouping and error reporting.
    pub subcommand: &'a str,
    /// A reference to the slice of `ArgHelp` definitions for the current context.
    pub help_rules: &'a [ArgHelp<'a>],
    /// If set to true, disables the automatic processing of `--help` and `--version` flags.
    pub ignore_help: bool,
    /// If true, unmatched arguments trigger an error.
    pub strict: bool,
    /// Optional depth level for strictness.
    pub strict_level: Option<usize>,
    /// If provided, remaining arguments are collected here automatically.
    pub collect_args: Option<&'a mut Vec<String>>,
    /// If true, suppresses parsing errors, returning Ok(()) even if parsing fails.
    pub passthrough: bool,
    /// If true, ensures that at least one argument was provided (non-empty input).
    pub require_args: bool,
}

impl<'a> Default for ParserOptions<'a> {
    /// Creates a default `ParserOptions` instance with empty context and standard behavior.
    fn default() -> Self {
        Self {
            subcommand: NULL_PTR,
            help_rules: &[],
            ignore_help: false,
            strict: true,
            strict_level: None,
            collect_args: None,
            passthrough: false,
            require_args: false,
        }
    }
}
