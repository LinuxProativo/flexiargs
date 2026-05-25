//! todo

use crate::ArgHelp;

/// todo comentar
pub struct ParserOptions<'a> {
    /// todo comentar
    pub subcommand: &'a str,
    /// todo comentar
    pub help_rules: &'a [ArgHelp<'a>],
    /// todo comentar
    pub ignore_help: bool,
}

impl<'a> Default for ParserOptions<'a> {
    /// todo comentar
    fn default() -> Self {
        Self {
            subcommand: "",
            help_rules: &[],
            ignore_help: false,
        }
    }
}
