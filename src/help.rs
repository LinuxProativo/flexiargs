//! Help system for FlexiArgs.
//!
//! Provides structures to document commands and a formatter to print
//! the generated help information to the console.

pub struct ArgHelp<'a> {
    pub short: Option<&'a str>,
    pub long: &'a str,
    pub description: &'a str,
    pub is_subcommand: bool,
}

impl<'a> ArgHelp<'a> {
    pub fn arg(short: Option<&'a str>, long: &'a str, desc: &'a str) -> Self {
        Self { short, long, description: desc, is_subcommand: false }
    }

    pub fn subcommand(long: &'a str, desc: &'a str) -> Self {
        Self { short: None, long, description: desc, is_subcommand: true }
    }
}

/// Prints the help information based on the provided rules.
pub fn print_help(sub: &str, rules: &[ArgHelp]) {
    let app_name = std::env::current_exe()
        .unwrap_or_default()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    println!("Usage: {} {} [OPTIONS]", app_name, sub);
    println!("\nOptions:");

    for rule in rules {
        let flag = match rule.short {
            Some(s) => format!("{}, {}", s, rule.long),
            None => rule.long.to_string(),
        };
        println!("  {:<20} {}", flag, rule.description);
    }
}