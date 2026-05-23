//! Error message generation and argument parsing utilities.
//!
//! This module provides standardized functions for reporting command-line interface
//! errors and extracting values from arguments.

use std::error::Error;

/// Retrieves the current executable's name or returns an empty string if retrieval fails.
///
/// # Returns
/// A `String` containing the binary name.
pub(crate) fn get_app_name() -> String {
    std::env::current_exe()
        .unwrap_or_default()
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string()
}

/// Generates a formatted "invalid argument" error message.
///
/// # Arguments
/// * `sub` - The subcommand context (e.g., "setup").
/// * `other` - The invalid argument string received.
///
/// # Returns
/// A boxed dynamic error containing the formatted message.
pub fn invalid_arg(sub: &str, other: &str) -> Box<dyn Error> {
    let c = get_app_name();

    let context = if sub.is_empty() {
        c.clone()
    } else {
        format!("{c}: {sub}")
    };

    format!("{context}: invalid argument '{other}'\nUse '{c} --help' to see available options.")
        .into()
}

/// Unified error reporter for missing parameters.
///
/// # Arguments
/// * `sub` - The subcommand context.
/// * `essential` - If true, reports that a core parameter is missing.
///
/// # Returns
/// A boxed dynamic error containing the formatted message.
pub fn missing_arg(sub: &str, essential: bool) -> Box<dyn Error> {
    let c = get_app_name();

    let msg = if essential {
        "no essential parameter specified"
    } else {
        "no parameter specified"
    };

    format!("{c}: {sub}: {msg}\nUse '{c} --help' to see available options.").into()
}

/// Parses a value from a command-line argument.
///
/// Supports both `--key=value` and `--key value` formats.
///
/// # Arguments
/// * `sub` - The subcommand context (e.g., "setup").
/// * `val_name` - The descriptive name of the expected value (e.g., "url").
/// * `arg` - The current argument being processed.
/// * `next` - The subsequent argument, which may contain the value.
///
/// # Returns
/// A `Result` containing the extracted value as a `String` or a formatted error message.
pub fn parse_value(
    sub: &str,
    val_name: &str,
    arg: &str,
    next: Option<&str>,
) -> Result<String, String> {
    let extracted: Option<String> = if let Some(pos) = arg.find('=') {
        let val = &arg[pos + 1..];
        if val.is_empty() {
            None
        } else {
            Some(val.to_string())
        }
    } else {
        next.and_then(|n| {
            if n.is_empty() || n.starts_with('-') {
                None
            } else {
                Some(n.to_string())
            }
        })
    };

    match extracted {
        Some(value) => Ok(value),
        None => {
            let c = get_app_name();

            let key = arg.split('=').next().unwrap_or(arg);
            let sp = if arg.contains('=') { "=" } else { " " };

            Err(format!(
                "{}: {}: {} requires a <{}>.\nUsage: {} {} {}{}<{}>",
                c, sub, key, val_name, c, sub, key, sp, val_name
            ))
        }
    }
}
