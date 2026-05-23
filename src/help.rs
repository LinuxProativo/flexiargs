//! Help system for FlexiArgs.
//!
//! Provides structures to document commands and a formatter to print
//! the generated help information to the console.

use crate::messages::get_app_name;
use crate::{Arg, NULL_PTR};
use std::collections::BTreeMap;

///
pub struct AppProperties<'a> {
    ///
    pub name: &'a str,
    ///
    pub desc: &'a str,
    ///
    pub version: &'a str,
}

/// todo completar descrição
pub struct ArgHelp<'a> {
    ///
    pub short: Option<&'a str>,
    ///
    pub long: &'a str,
    ///
    pub desc: &'a str,
    ///
    pub is_subcommand: bool,
    ///
    pub properties: Option<AppProperties<'a>>,
    pub context: &'a [&'a str],
}

/// todo descrição
impl<'a> ArgHelp<'a> {
    /// todo descrição argumento
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

    /// todo descrição argumento
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

    pub const fn with_context(mut self, context: &'a [&'a str]) -> Self {
        self.context = context;
        self
    }

    /// todo descrição argumento
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

/// Prints the help information based on the provided rules.
/// todo completar argumento
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

    let active_help: Vec<&ArgHelp> = if is_all {
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

    let subcommands: Vec<&&ArgHelp> = active_help.iter().filter(|h| h.is_subcommand).collect();
    let sub_placeholder = if !subcommands.is_empty() {
        "[SUBCOMMAND] "
    } else {
        ""
    };
    let place = if sub.is_empty() {
        sub_placeholder
    } else {
        &format!("{} ", sub)
    };

    println!("Usage: {} {}[OPTIONS]", app_name, place);

    if !subcommands.is_empty() {
        println!("\nSubcommands:");
        for help in subcommands {
            let flag = match help.short {
                Some(s) => format!("{:>2}, {}", s, help.long),
                None => format!("{:>4}{}", NULL_PTR, help.long),
            };
            println!("  {:<30}{}", flag, help.desc);
        }
    }

    for help in &active_help {
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
            for h in subs {
                let flag = match h.short {
                    Some(s) => format!("{:>2}, {}", s, h.long),
                    None => format!("{:>4}{}", NULL_PTR, h.long),
                };
                println!("  {:<30}{}", flag, h.desc);
            }
        }

        if !opts.is_empty() {
            let text = &format!(" for '{}'", ctx);
            println!("\nOptions{}:", if ctx.is_empty() { "" } else { text });
            for h in opts {
                let flag = match h.short {
                    Some(s) => format!("{:>2}, {}", s, h.long),
                    None => format!("{:>4}{}", NULL_PTR, h.long),
                };
                println!("  {:<30}{}", flag, h.desc);
            }
        }
    }

    println!("\nGeneral Options:");
    println!("  {:<30}Show this help message", "-h, --help");
    println!("  {:<30}Show this all help message", "    --help-all");
    println!("  {:<30}Show version", "-V, --version");
}
