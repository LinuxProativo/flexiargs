//! Core argument parsing logic.
//!
//! This module defines the structures and functions necessary to map command-line
//! arguments to application variables using a rule-based system.

use crate::ParserOptions;
use crate::help::print_help;
use crate::into_result::IntoActionResult;
use crate::messages::{invalid_arg, missing_arg, parse_value};
use crate::parse_result::ParseResult;
use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;
use std::sync::RwLock;

/// Represents an empty or null value placeholder for argument configurations.
pub static NULL_PTR: &str = "";

/// Defines the action to be taken for a matched argument.
pub enum ArgAction<'a> {
    /// Simply sets a boolean flag to true.
    Bool(&'a mut bool),
    /// Support for thread-safe global settings.
    RwLockBool(&'a RwLock<bool>),
    /// Executes a closure to parse and store a value.
    /// This allows support for multiple types (String, PathBuf, Option) without trait conflicts.
    Value(
        Box<dyn FnMut(&str, &str, &str, &mut VecDeque<String>) -> Result<(), Box<dyn Error>> + 'a>,
    ),
}

/// Represents a command-line argument rule.
pub struct Arg<'a> {
    /// The short flag version or command name (e.g., Some("-v") or Some("add")).
    pub short: Option<&'a str>,
    /// The long flag version (e.g., "--version").
    pub long: &'a str,
    /// The name used to describe the value in error messages (e.g., "URL").
    pub error_name: &'a str,
    /// The action to perform when this argument is matched.
    pub action: ArgAction<'a>,
    /// Whether this argument is required for the command to proceed.
    pub essential: bool,
}

impl<'a> Arg<'a> {
    /// Creates a new boolean flag rule.
    ///
    /// # Arguments
    /// * `s` - Short flag.
    /// * `l` - Long flag.
    /// * `target` - Mutable reference to the boolean to be updated.
    pub fn bool(s: Option<&'a str>, l: &'a str, target: &'a mut bool) -> Self {
        Self {
            short: s,
            long: l,
            error_name: NULL_PTR,
            action: ArgAction::Bool(target),
            essential: false,
        }
    }

    /// Creates a new boolean flag rule for thread-safe global settings using an `RwLock`.
    ///
    /// When this flag is matched, the underlying boolean inside the `RwLock` will be set to `true`.
    /// This is particularly useful for global settings that need to be accessed across multiple threads.
    ///
    /// # Arguments
    /// * `s` - The short flag version (e.g., `Some("-v")`).
    /// * `l` - The long flag version (e.g., `"--verbose"`).
    /// * `target` - A reference to the `RwLock<bool>` that will be updated.
    pub fn rw_bool(s: Option<&'a str>, l: &'a str, target: &'a RwLock<bool>) -> Self {
        Self {
            short: s,
            long: l,
            error_name: NULL_PTR,
            action: ArgAction::RwLockBool(target),
            essential: false,
        }
    }

    /// Creates a rule that sets a specific value to the target.
    ///
    /// # Arguments
    /// * `s` - Short flag.
    /// * `l` - Long flag.
    /// * `value` - The value to assign when matched.
    /// * `target` - Mutable reference to the variable.
    pub fn set<T>(s: Option<&'a str>, l: &'a str, value: T, target: &'a mut T) -> Self
    where
        T: Clone + 'a,
    {
        let closure =
            move |_sub: &str, _val_name: &str, _arg: &str, _args: &mut VecDeque<String>| {
                *target = value.clone();
                Ok(())
            };
        Self {
            short: s,
            long: l,
            error_name: NULL_PTR,
            action: ArgAction::Value(Box::new(closure)),
            essential: false,
        }
    }

    /// Creates a new value rule for any type implementing FromStr (String, PathBuf, etc.).
    ///
    /// # Arguments
    /// * `s` - Short flag.
    /// * `l` - Long flag.
    /// * `err` - Name of the value for error reporting.
    /// * `target` - Mutable reference to the variable to be populated.
    pub fn value<T>(s: Option<&'a str>, l: &'a str, err: &'a str, target: &'a mut T) -> Self
    where
        T: FromStr + 'a,
        T::Err: Error + 'static,
    {
        let closure = move |sub: &str, val_name: &str, arg: &str, args: &mut VecDeque<String>| {
            let next = args.front().map(|s| s.as_str());
            let extracted = parse_value(sub, val_name, arg, next).map_err(|e| {
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)) as Box<dyn Error>
            })?;

            if !arg.contains('=') {
                args.pop_front();
            }

            *target = extracted
                .parse()
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            Ok(())
        };
        Self {
            short: s,
            long: l,
            error_name: err,
            action: ArgAction::Value(Box::new(closure)),
            essential: false,
        }
    }

    /// Creates a rule that sets a specific value to a thread-safe target (`RwLock`).
    ///
    /// When the argument is matched, the provided `value` is cloned and assigned
    /// to the data protected by the `RwLock`. This is ideal for switching
    /// global states or modes that are shared across the application.
    ///
    /// # Arguments
    /// * `s` - The short flag version (e.g., `Some("-m")`).
    /// * `l` - The long flag version (e.g., `"--mode"`).
    /// * `value` - The specific value of type `T` to be set upon matching.
    /// * `target` - A reference to the `RwLock<T>` that will be updated.
    pub fn rw_set<T>(s: Option<&'a str>, l: &'a str, value: T, target: &'a RwLock<T>) -> Self
    where
        T: Clone + 'a,
    {
        let closure =
            move |_sub: &str, _val_name: &str, _arg: &str, _args: &mut VecDeque<String>| {
                *target.write().unwrap() = value.clone();
                Ok(())
            };
        Self {
            short: s,
            long: l,
            error_name: NULL_PTR,
            action: ArgAction::Value(Box::new(closure)),
            essential: false,
        }
    }

    /// Creates a new value rule for thread-safe global settings (`RwLock`) that requires parsing.
    ///
    /// This rule extracts a value from the command line (either via `--flag=value` or `--flag value`),
    /// parses it using the `FromStr` trait, and updates the `RwLock` target.
    /// If the parsing fails or the value is missing, it returns a descriptive error.
    ///
    /// # Arguments
    /// * `s` - The short flag version (e.g., `Some("-p")`).
    /// * `l` - The long flag version (e.g., `"--port"`).
    /// * `err` - The human-readable name of the expected value used in error messages.
    /// * `target` - A reference to the `RwLock<T>` that will store the parsed value.
    ///
    /// # Errors
    /// Returns an error if the value is missing or if the string cannot be parsed into type `T`.
    pub fn rw_value<T>(s: Option<&'a str>, l: &'a str, err: &'a str, target: &'a RwLock<T>) -> Self
    where
        T: FromStr + 'a,
        T::Err: Error + 'static,
    {
        let closure = move |sub: &str, val_name: &str, arg: &str, args: &mut VecDeque<String>| {
            let next = args.front().map(|s| s.as_str());
            let extracted = parse_value(sub, val_name, arg, next).map_err(|e| {
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)) as Box<dyn Error>
            })?;

            if !arg.contains('=') {
                args.pop_front();
            }

            *target.write().unwrap() = extracted
                .parse()
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            Ok(())
        };
        Self {
            short: s,
            long: l,
            error_name: err,
            action: ArgAction::Value(Box::new(closure)),
            essential: false,
        }
    }

    /// Creates a new value rule for any type implementing FromStr (e.g., Option<String>, Option<PathBuf>).
    ///
    /// # Arguments
    /// * `s` - Short flag.
    /// * `l` - Long flag.
    /// * `err` - Name of the value for error reporting.
    /// * `target` - Mutable reference to the Option<T> to be populated.
    pub fn option<T>(
        s: Option<&'a str>,
        l: &'a str,
        err: &'a str,
        target: &'a mut Option<T>,
    ) -> Self
    where
        T: FromStr + 'a,
        T::Err: Error + 'static,
    {
        let closure = move |sub: &str, val_name: &str, arg: &str, args: &mut VecDeque<String>| {
            let next = args.front().map(|s| s.as_str());
            let extracted = parse_value(sub, val_name, arg, next).map_err(|e| {
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)) as Box<dyn Error>
            })?;

            if !arg.contains('=') {
                args.pop_front();
            }

            *target = Some(
                extracted
                    .parse()
                    .map_err(|e| Box::new(e) as Box<dyn Error>)?,
            );
            Ok(())
        };

        Self {
            short: s,
            long: l,
            error_name: err,
            action: ArgAction::Value(Box::new(closure)),
            essential: false,
        }
    }

    /// Marks the argument as essential/required.
    pub fn essential(mut self) -> Self {
        self.essential = true;
        self
    }

    /// Helper to collect multiple arguments into a list.
    ///
    /// # Arguments
    /// * `s` - Short flag.
    /// * `l` - Long flag.
    /// * `err` - Name of the items for error reporting.
    /// * `target` - Mutable reference to a Vec of Strings.
    pub fn collect_list(
        s: Option<&'a str>,
        l: &'a str,
        err: &'a str,
        target: &'a mut Vec<String>,
    ) -> Self {
        let closure = move |sub: &str, val_name: &str, arg: &str, args: &mut VecDeque<String>| {
            let next = args.front().map(|s| s.as_str());
            let first = parse_value(sub, val_name, arg, next).map_err(|e| {
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)) as Box<dyn Error>
            })?;

            if !arg.contains('=') {
                args.pop_front();
            }
            target.push(first);

            while let Some(next_arg) = args.front() {
                if next_arg.starts_with('-') {
                    break;
                }
                target.push(args.pop_front().unwrap());
            }
            Ok(())
        };
        Self {
            short: s,
            long: l,
            error_name: err,
            action: ArgAction::Value(Box::new(closure)),
            essential: false,
        }
    }

    /// A simplified custom action for flags that don't need to parse extra values.
    ///
    /// # Arguments
    /// * `s` - Short flag.
    /// * `l` - Long flag.
    /// * `callback` - A closure that executes when the flag is matched.
    pub fn action<F, R>(s: Option<&'a str>, l: &'a str, mut callback: F) -> Self
    where
        F: FnMut() -> R + 'a,
        R: IntoActionResult,
    {
        let closure = move |_sub: &str, _err: &str, _arg: &str, _args: &mut VecDeque<String>| {
            callback().into_result()
        };
        Self {
            short: s,
            long: l,
            error_name: NULL_PTR,
            action: ArgAction::Value(Box::new(closure)),
            essential: false,
        }
    }
}

/// Main parser that iterates through arguments and populates variables based on rules.
///
/// # Arguments
/// * `subcommand` - Name of the current subcommand context.
/// * `rules` - A slice of Arg rules to match against.
/// * `args` - The queue of command-line arguments.
///
/// # Returns
/// A `ParseResult` containing information about the success or failure of the operation.
pub fn parse_into_vars<'a>(
    rules: &mut [Arg<'a>],
    mut args: VecDeque<String>,
    opts: ParserOptions<'a>,
) -> ParseResult<'a> {
    let mut help_requested = false;
    let mut version_requested = false;
    let mut help_all_requested = false;

    if !opts.ignore_help {
        if let Some(arg) = args.front() {
            if arg == "--help" || arg == "-h" {
                help_requested = true;
            } else if arg == "--version" || arg == "-V" {
                version_requested = true;
            } else if arg == "--help-all" {
                help_all_requested = true;
            }
        }

        if help_requested || version_requested || help_all_requested {
            let props = opts.help_rules.iter().find_map(|r| r.properties.as_ref());

            if version_requested {
                println!("{}", props.map(|p| p.version).unwrap_or("0.1.0"));
            } else if help_all_requested {
                print_help(opts.subcommand, props, rules, opts.help_rules, true);
            } else {
                print_help(opts.subcommand, props, rules, opts.help_rules, false);
            }

            return ParseResult {
                sub: opts.subcommand,
                empty: true,
                res: Ok(()),
                remaining: VecDeque::new(),
                arg_indices: Vec::new(),
                essential_failed: false,
                help_requested: true,
            };
        }
    }

    let is_empty = args.is_empty();
    let mut essential_met = false;
    let mut positional_only = false;
    let mut remaining = VecDeque::new();
    let mut arg_indices = Vec::new();
    let mut current_idx = 0;
    let has_essentials = rules.iter().any(|r| r.essential);

    let result = (|| -> Result<(), Box<dyn Error>> {
        while let Some(arg) = args.pop_front() {
            current_idx += 1;

            if !positional_only && arg == "--" {
                positional_only = true;
                continue;
            }

            if positional_only {
                remaining.push_back(arg);
                arg_indices.push(current_idx);
                continue;
            }

            let rule = rules.iter_mut().find(|r| {
                r.long
                    .split('|')
                    .any(|l| arg == l || arg.starts_with(&format!("{}=", l)))
                    || r.short.map_or(false, |s| arg == s)
            });

            if let Some(r) = rule {
                if r.essential {
                    essential_met = true;
                }

                match &mut r.action {
                    ArgAction::Bool(val) => **val = true,
                    ArgAction::RwLockBool(lock) => *lock.write().unwrap() = true,
                    ArgAction::Value(callback) => {
                        callback(opts.subcommand, r.error_name, &arg, &mut args)?;
                    }
                }
            } else {
                remaining.push_back(arg.clone());
                arg_indices.push(current_idx);
                if arg.starts_with('-') && arg != "-" {
                    return Err(invalid_arg(opts.subcommand, &arg));
                }
            }
        }
        Ok(())
    })();

    let mut parse_result = ParseResult {
        sub: opts.subcommand,
        empty: is_empty,
        res: result,
        remaining,
        arg_indices,
        essential_failed: has_essentials && !essential_met && !is_empty,
        help_requested: false,
    };

    if parse_result.res.is_ok() && opts.strict {
        if let Some(level) = opts.strict_level {
            for (i, &pos) in parse_result.arg_indices.iter().enumerate() {
                if pos <= level {
                    if let Some(arg) = parse_result.remaining.get(i) {
                        parse_result.res = Err(invalid_arg(opts.subcommand, arg));
                        break;
                    }
                }
            }
        } else {
            if let Some(arg) = parse_result.remaining.front() {
                parse_result.res = Err(invalid_arg(opts.subcommand, arg));
            }
        }
    }

    if parse_result.res.is_ok() && parse_result.essential_failed {
        parse_result.res = Err(missing_arg(opts.subcommand, true));
    }

    parse_result
}
