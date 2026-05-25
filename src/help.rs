//! Help system for FlexiArgs.
//!
//! Provides structures to document commands and a formatter to print
//! the generated help information to the console.

use crate::messages::get_app_name;
use crate::{Arg, NULL_PTR};
use std::collections::BTreeMap;

/// Holds metadata about the application for help display.
pub struct AppProperties<'a> {
    /// The name of the application.
    pub name: &'a str,
    /// A short description of the application.
    pub desc: &'a str,
    /// The application version string.
    pub version: &'a str,
}

/// Defines documentation metadata for a command-line argument or subcommand.
pub struct ArgHelp<'a> {
    /// The short flag version (e.g., Some("-v")).
    pub short: Option<&'a str>,
    /// The long flag version or command name (e.g., "--version").
    pub long: &'a str,
    /// A brief description of the argument's purpose.
    pub desc: &'a str,
    /// Whether this item represents a subcommand.
    pub is_subcommand: bool,
    /// Optional application properties, usually defined for the main command.
    pub properties: Option<AppProperties<'a>>,
    /// Contexts (subcommands) in which this argument is available.
    pub context: &'a [&'a str],
}

impl<'a> ArgHelp<'a> {
    /// Creates a new entry containing application properties.
    ///
    /// # Arguments
    /// * `name` - The application name.
    /// * `desc` - The application description.
    /// * `ver` - The application version.
    pub const fn properties(name: &'a str, desc: &'a str, ver: &'a str) -> Self {
        Self {
            short: None,
            long: NULL_PTR,
            desc: NULL_PTR,
            is_subcommand: false,
            properties: Some(AppProperties {
                name,
                desc,
                version: ver,
            }),
            context: &[],
        }
    }

    /// Creates a new argument definition.
    ///
    /// # Arguments
    /// * `short` - Optional short flag (e.g., Some("-f")).
    /// * `long` - The long flag version (e.g., "--file").
    /// * `desc` - Description of the argument.
    pub const fn arg(short: Option<&'a str>, long: &'a str, desc: &'a str) -> Self {
        Self {
            short,
            long,
            desc,
            is_subcommand: false,
            properties: None,
            context: &[],
        }
    }

    /// Sets the contexts (subcommands) for this argument, used for grouping in --help-all.
    ///
    /// # Arguments
    /// * `context` - A slice of subcommand names where this argument is valid.
    pub const fn with_context(mut self, context: &'a [&'a str]) -> Self {
        self.context = context;
        self
    }

    /// Creates a new subcommand definition.
    ///
    /// # Arguments
    /// * `short` - Optional short flag.
    /// * `long` - The command name.
    /// * `desc` - Description of the command.
    pub const fn subcommand(short: Option<&'a str>, long: &'a str, desc: &'a str) -> Self {
        Self {
            short,
            long,
            desc,
            is_subcommand: true,
            properties: None,
            context: &[],
        }
    }
}

/// Prints the formatted help information to the console based on the provided rules.
///
/// # Arguments
/// * `sub` - The current active subcommand context.
/// * `props` - Optional application properties.
/// * `rules` - The list of active command arguments.
/// * `help_rules` - The full list of documented rules.
/// * `is_all` - If true, lists all documented arguments regardless of active rules.
pub(crate) fn print_help(
    sub: &str,
    props: Option<&AppProperties>,
    rules: &[Arg],
    help_rules: &[ArgHelp],
    is_all: bool,
) {
    let app_name = get_app_name();
    let mut grouped: BTreeMap<&str, Vec<&ArgHelp>> = BTreeMap::new();

    if let Some(p) = props {
        println!("{} - {}\n\n{}\n", app_name, p.name, p.desc);
    } else {
        println!("{}\n", app_name);
    }

    let act: Vec<&ArgHelp> = if is_all {
        help_rules.iter().collect()
    } else {
        help_rules
            .iter()
            .filter(|h| {
                rules.iter().any(|r| {
                    let long_match = r.long.split('|').any(|part| part == h.long);
                    let short_match = match (h.short, r.short) {
                        (Some(h_short), Some(r_short)) => h_short == r_short,
                        (None, _) => true,
                        (Some(_), None) => false,
                    };
                    long_match && short_match
                })
            })
            .collect()
    };

    let scmd: Vec<&ArgHelp> = act.iter().filter(|h| h.is_subcommand).cloned().collect();
    let sub_place = if !scmd.is_empty() {
        "[SUBCOMMAND] "
    } else {
        NULL_PTR
    };
    let place = if sub.is_empty() {
        sub_place
    } else {
        &format!("{} ", sub)
    };

    println!("Usage: {} {}[OPTIONS]", app_name, place);

    if !scmd.is_empty() {
        println!("\nSubcommands:");
        print_entries(&scmd);
    }

    for help in &act {
        if help.is_subcommand {
            continue;
        }

        if !is_all || help.context.is_empty() {
            grouped.entry("").or_default().push(help);
        } else {
            for ctx in help.context {
                grouped.entry(ctx).or_default().push(help);
            }
        }
    }

    for (ctx, items) in grouped {
        let subs: Vec<_> = items.iter().filter(|i| i.is_subcommand).cloned().collect();
        let opts: Vec<_> = items.iter().filter(|i| !i.is_subcommand).cloned().collect();

        if !subs.is_empty() {
            println!("\nSubcommands:");
            print_entries(&subs);
        }

        if !opts.is_empty() {
            let text = &format!(" for '{}'", ctx);
            println!("\nOptions{}:", if ctx.is_empty() { "" } else { text });
            print_entries(&opts);
        }
    }

    println!("\nGeneral Options:");
    println!("  {:<30}Show this help message", "-h, --help");
    println!("  {:<30}Show this all help message", "    --help-all");
    println!("  {:<30}Show version", "-V, --version");
}

/// Helper function to format and print a help entry consistently.
///
/// # Arguments
/// * `h` - The `ArgHelp` entry to display.
fn print_entries(items: &[&ArgHelp]) {
    for h in items {
        let display_long = match h.meta {
            Some(m) => format!("{} {}", h.long, m),
            None => h.long.to_string(),
        };

        let flag = match h.short {
            Some(s) => format!("{:>2}, {}", s, display_long),
            None => format!("{:>4}{}", NULL_PTR, display_long),
        };

        println!("  {:<30}{}", flag, h.desc);
    }
}
