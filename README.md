<p align="center">
  <img src="https://raw.githubusercontent.com/LinuxProativo/flexiargs/refs/heads/master/logo.png" width="300"/>
</p>

<h1 align="center">FlexiArgs - Flexible Arguments Parser</h1>
<h3 align="center">✨ Minimalist, Flexible, and Ergonomic CLI Parsing.</h3>

<p align="center">
    <img src="https://img.shields.io/badge/Platform-POSIX-FCC624?&logo=linux&style=flat-square"/>
    <img src="https://img.shields.io/github/actions/workflow/status/LinuxProativo/flexiargs/rust.yml?label=Test&style=flat-square&logo=github"/>
    <img src="https://img.shields.io/badge/RustC-1.85+-orange?style=flat-square&logo=rust"/>
    <img src="https://img.shields.io/github/languages/code-size/LinuxProativo/flexiargs?style=flat-square&logo=rust&label=Code%20Size"/>
</p>

## 🔍 Overview

`flexiargs` is a lightweight and dependency-friendly Rust crate designed for rule-based
command-line argument parsing without relying on macros, derive systems, or bloated
abstractions.

Instead of hiding behavior behind procedural magic, flexiargs provides a clean and
explicit API that allows developers to bind CLI arguments directly to variables while
retaining full control over parsing logic and application flow.

Built with simplicity and flexibility in mind, it gives you low-level control when
you need it, without sacrificing ergonomics. With flexiargs, you can easily manage:

- **⚙️ Parsing behavior and argument rules;**

- **✅ Validation and constraint handling;**

- **📦 Unmatched or forwarded arguments;**

- **🚀 Execution flow and command dispatching;**

- **🧩 Custom CLI architectures and dynamic behaviors;**

Unlike heavy CLI frameworks, flexiargs focuses on predictable behavior, small footprint,
and straightforward integration, making it ideal for projects where control and
portability matter. Perfect for:

- **📦 Package managers;**

- **🛠️ System utilities;**

- **📥 Installers and bootstrap tools;**

- **🧩 Embedded CLI environments;**

- **🐚 Custom shell-like applications;**

- **🧪 Internal developer tooling;**

- **⚡ Lightweight standalone binaries;**

- **🚀 Experimental runtime environments;**

## ✨ Features

`flexiargs` is designed to keep CLI parsing simple, explicit, and predictable, without relying on macros or heavy abstractions. Instead of hiding behavior behind complex frameworks, it exposes clear rule-based control over how arguments are interpreted and processed.

* 🧩 **Simple rule-based parser API**  
  Defines parsing rules in a declarative way, without DSLs or code generation.
  You have full control over how each argument is interpreted.

* 🚫 **No procedural macros**  
  No dependency on `derive` or procedural macros. This reduces compile time,
  avoids hidden behavior, and improves debugging clarity.

* 🔠 **Supports short and long flags**  
  Full support for both short flags (`-v`, `-h`) and long flags
  (`--verbose`, `--help`) for flexible CLI design.

* 📝 **Supports argument formats**
  * `--flag=value`  
    Inline assignment for compact CLI usage.
  * `--flag value`  
    Classic POSIX-style separation for readability in interactive usage.

* 🔄 **Typed parsing with `FromStr`**  
  Automatically converts string inputs into Rust types using the `FromStr` trait,
  ensuring type safety and reducing manual parsing code.

* ⚠️ **Automatic error formatting**  
  Parsing errors are automatically formatted in a clear and consistent way,
  improving end-user feedback.

* 📌 **Optional and required arguments**  
  Explicit support for required and optional parameters, removing the need for
  manual validation logic.

* 📚 **Multi-value collection**  
  Support for multiple values for the same flag (e.g. `--file a b c`), collected
  into
  typed containers.

* 📥 **Positional argument passthrough**  
  Positional arguments can be captured or forwarded directly to subprocesses or
  higher-level handlers.

* 🔒 **Strict validation modes**  
  Enables strict parsing mode to reject unknown or malformed arguments, ideal for
  robust tools and system utilities.

* 🧵 **Thread-safe shared settings (`RwLock`)**  
  Safe shared state across threads using `RwLock`, useful for concurrent CLI
  applications or embedded runtimes.

* ⚡ **Custom actions/callbacks**  
  Allows execution of custom callbacks during parsing for dynamic or
  context-aware behavior.

* 🪶 **Minimal and dependency-light design**  
  Keeps the core lightweight with minimal dependencies, focusing on portability
  and predictable behavior.


## 📦 Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
flexiargs = "2.0"
```

## 🌐 Public API

The crate intentionally exposes a very small API surface:

```rust
pub use messages::{invalid_arg, missing_arg};
pub use help::ArgHelp;
pub use options::ParserOptions;
pub use flexiargs::{Arg, NULL_PTR, parse_into_vars};
```

### 🚪 Main Entry Points

| Item              | Description                           |
|-------------------|---------------------------------------|
| `Arg`             | Defines parsing rules                 |
| `parse_into_vars` | Executes parsing with `ParserOptions` |
| `ParserOptions`   | Configuration for parsing behavior    |
| `invalid_arg`     | Standardized invalid argument error   |
| `missing_arg`     | Standardized missing parameter error  |

## 🚀 Quick Example

```rust
use flexiargs::{Arg, parse_into_vars, ParserOptions};
use std::collections::VecDeque;

let mut sync_mode = false;
let mut packages = Vec::new();
let mut cache_dir = String::new();
let mut config_file: Option<String> = None;
let mut use_overlay = true;

let opts = ParserOptions {
subcommand: "aports",
..Default::default()
};

let mut rules = [
    Arg::bool(Some("-S"), "--sync", &mut sync_mode),
    Arg::collect_list(None, "--pkgs", "packages", &mut packages),
    Arg::value(None, "--cache-dir", "path", &mut cache_dir),
    Arg::option(Some("-c"), "--config", "file", &mut config_file),
    Arg::set(None, "--disable-overlay", false, &mut use_overlay),
];

let args = VecDeque::from(vec![
     "-S".to_string(),
    "wget".to_string(),
    "curl".to_string(),
    "--cache-dir=/tmp".to_string(),
    "--disable-overlay".to_string()
]);

parse_into_vars(&mut rules, args, opts).ok();
drop(rules);

println!("Sync: {}", sync_mode);
println!("Packages: {:?}", packages);
println!("Cache dir: {}", cache_dir);
println!("Overlay enabled: {}", use_overlay);
```

## 🧠 Core Concepts

### 📏 Parsing Rules

Every CLI behavior is defined using an `Arg`.

Each rule describes:

- 🏷️ Accepted flags;
- ⚙️ Parsing behavior;
- 🎯 Target variable;
- ✅ Validation requirements;

Example:

```rust
Arg::bool(Some("-v"), "--verbose", &mut verbose)
```

## 🧱 Supported Rule Types

`flexiargs` is built around a small set of explicit rule types that define how command-line
input is interpreted, validated, and transformed. Instead of relying on implicit behavior
or hidden conventions, each rule type has a clear and predictable responsibility,
allowing you to compose CLI behavior in a controlled and deterministic way.

## 🚩 Boolean Flags

Sets a boolean to `true` when matched.

```rust
let mut verbose = false;

Arg::bool(Some("-v"), "--verbose", &mut verbose);
```

### ✅ Supports

- `-v`
- `--verbose`

## 🔢 Typed Values

Parses values using `FromStr`.

```rust
let mut port: u16 = 0;

Arg::value(
    Some("-p"),
    "--port",
    "port",
    &mut port
);
```

### ✅ Supports

- `--port 8080`
- `--port=8080`
- `-p 8080`

## ❓ Optional Values

Stores values in `Option<String>`.

```rust
let mut config: Option<String> = None;

Arg::option(
    Some("-c"),
    "--config",
    "file",
    &mut config
);
```

## 🔧 Fixed State Assignment

Assigns a predefined value when matched.

```rust
let mut overlay = true;

Arg::set(
    None,
    "--disable-overlay",
    false,
    &mut overlay
);
```

## 📚 Multi-Value Collection

Collects sequential positional values until another flag appears.

```rust
let mut packages = Vec::new();

Arg::collect_list(
    None,
    "--pkgs",
    "packages",
    &mut packages
);
```

### 🧪 Example

```bash
--pkgs wget curl git
```

### 📤 Result

```rust
["wget", "curl", "git"]
```

## ⚡ Custom Actions

Executes arbitrary logic.

```rust
Arg::action(
    Some("-V"),
    "--version",
    || {
        println!("myapp 1.0");
    }
);
```

### 💡 Useful for

- 📄 Version output
- 🛠️ Custom handlers
- 🚪 Early exits
- 🔄 Dynamic state manipulation

## ❗ Required Arguments

Arguments can be marked as essential:

```rust
Arg::value(
    None,
    "--root",
    "path",
    &mut root
).essential();
```

If no essential rule is matched:

```text
myapp: setup: no essential parameter specified
```

## 🧵 Thread-Safe Global State

flexiargs includes built-in support for shared application state using `RwLock`.

### 🚩 Shared Boolean Flags

```rust
use std::sync::RwLock;

static DEBUG: RwLock<bool> = RwLock::new(false);

Arg::rw_bool(
    Some("-d"),
    "--debug",
    &DEBUG
);
```

### 🔢 Shared Typed Values

```rust
use std::sync::RwLock;

static PORT: RwLock<u16> = RwLock::new(8080);

Arg::rw_value(
    None,
    "--port",
    "port",
    &PORT
);
```

### 🔧 Shared Fixed Assignment

```rust
Arg::rw_set(
    None,
    "--production",
    true,
    &MODE
);
```

## 🌍 Environment Variables

`flexiargs` supports documentation and parsing of environment variables,
allowing configuration via the environment.

```rust
Arg::env("ALPACK_ARCH", "Define the target architecture for rootfs"),
```

## 🔍 Parsing

Parsing in flexiargs is explicit, deterministic, and fully rule-driven. Each step of
the parsing process is defined by clear rules that transform raw command-line input
into structured, validated, and typed data.

### 📥 Basic Parsing

```rust
let opts = ParserOptions {
subcommand: "server",
..Default::default()
};

parse_into_vars(&mut rules, args, opts).ok()?;
```

## 📦 ParseResult

The parser returns a `ParseResult`. This structure provides a clean,
streamlined interface for controlling your CLI's execution flow:

- ✅ **Result Handling**: Encapsulates the success or failure of the parsing logic.
- 🛟 **Automatic Help/Version**: Handles `--help`, `--version`, and `--help-all`
  automatically, including output generation and exit state tracking.
- 🔒 **Execution Flow**: Provides a fluent API to bridge the gap between
  parsing and your main application logic.

### ✅ .help_or_err()

The primary method to control execution flow. It automatically handles help
flags, returns errors if parsing failed, and indicates if the application
should exit after showing help.

```rust
match parse_into_vars(&mut rules, args, opts).help_or_err() {
    Ok(true) => return Ok(()), // Help was shown, exit gracefully
    Ok(false) => { /* Continue execution */ }
    Err(e) => return Err(e),   // Parsing error occurred
}
```

### 🔓 .ok()

Extracts the raw parsing result, useful if you need to handle the
`Result<(), Box<dyn Error>>` manually without the built-in help/error logic.

```rust
parse_into_vars(&mut rules, args, opts).ok()?;
```

## 📍 Positional Arguments

The parser supports `--` to stop option parsing.

### 🧪 Example

```bash
myapp --verbose -- file1 file2
```

Everything after `--` becomes positional data.

## ⚠️ Error Messages

flexiargs provides standardized and human-readable errors automatically.

### ❌ Invalid arguments

```text
myapp: invalid argument '--unknown'
Use 'myapp --help' to see available options.
```

### ❌ Missing values

```text
myapp: setup: --port requires a <port>.
Usage: myapp setup --port <port>
```

## 🎯 Argument Matching

Rules in flexiargs define how input tokens are interpreted and matched against declared
CLI arguments. This matching system is flexible but explicit, allowing multiple naming
styles and aliasing strategies while keeping behavior predictable and rule-based.
Rules support:

- 🔤 Short flags;
- 🏷️ Long flags;
- 🔀 Aliases via `|`;
- 🧷 Inline assignment;

### 🧪 Example

```rust
Arg::bool(
    Some("-v"),
    "--verbose|--debug",
    &mut verbose
);
```

### ✅ Support

- `-v`
- `--verbose`
- `--debug`

Argument matching in flexiargs is designed to be both flexible and explicit.
Instead of enforcing a single naming convention, it allows multiple identifiers
and aliases per argument while keeping resolution deterministic and rule-based.

## ⚙️ ParserOptions

`ParserOptions` is the central configuration structure for `flexiargs`. It replaces
direct function parameters with a robust, extensible configuration object,
allowing you to fine-tune parsing behavior, validation rules, and context.

### 📝 Definition

```rust
pub struct ParserOptions<'a> {
    /// The name of the subcommand being parsed, used for error messages.
    pub subcommand: &'a str,
    /// If true, unknown arguments will trigger an error.
    pub strict: bool,
    /// Rejects unmatched arguments up to a specific depth (if Some).
    pub strict_level: Option<usize>,
    /// If true, ignores help/version flags and treats them as standard arguments.
    pub ignore_help: bool,
    /// If true, suppresses parsing errors, allowing unmatched args to pass through.
    pub passthrough: bool,
    /// Fails the parsing process if no arguments were supplied.
    pub require_args: bool,
    /// A mutable reference to collect unmatched positional arguments.
    pub collect_args: Option<&'a mut VecDeque<String>>,
    /// A list of help rules for automated documentation.
    pub help_rules: &'a [ArgHelp<'a>],
}
```

### 💡 Why use ParserOptions?

- **Centralized Control:** Configure all parsing aspects (strictness, collection, 
  requirements) in one place before invoking `parse_into_vars`.
- **Cleaner API:** Keeps the `parse_into_vars` signature stable even as you add new features.
- **Fluent Setup:** Easily derive from `Default` and override only the fields you need.

### 🚀 Usage Example

```rust
let mut remaining = VecDeque::new();

let opts = ParserOptions {
    subcommand: "profile",
    strict: true,
    require_args: true,
    collect_args: Some(&mut remaining),
    ..Default::default()
};

parse_into_vars(&mut rules, args, opts).help_or_err()?;
```

## 🤖 AutoHelp System

`flexiargs` provides an automated help generation system that maps your defined rules
and metadata into a structured, readable documentation output. Instead of maintaining
a separate manual, you define the application properties and command context directly
in your code.

### 📦 App Properties

Defines the core metadata for the application, such as name, version, and a brief description.

```rust
ArgHelp::properties(
    "ALPack", 
    "A flexible package manager utility", 
    "1.2.0"
);
```

### 🛠️ Subcommand Definition

Documents a subcommand. Use this to organize complex CLI tools into logical groups.

```rust
ArgHelp::subcommand(
    None,
    "profile",
    "Manage system profiles"
);
```

### 📄 Argument Documentation

Documents a specific argument. You can add metadata strings (like `<DIR>`) to improve clarity.

```rust
ArgHelp::arg(
    Some("-r"),
    "--rootfs",
    "Specify rootfs directory"
)
.meta("<DIR>");
```

### 🌍 Environment Variable Documentation

Documents environment variables that influence the application behavior, ensuring they
appear in the generated help output.


```rust
ArgHelp::env(
    "ALPACK_ARCH",
    "Define the target architecture for rootfs"
);
```

### 📍 Contextual Grouping

Limits the visibility of an argument to specific subcommands. This is essential
when generating full help outputs with `--help-all`.

```rust
ArgHelp::arg(
    Some("-c"),
    "--config",
    "Path to config file"
)
.context(&["profile", "setup"]);
```

## 🤝 Contributing

Contributions, improvements, and issue reports are welcome.

### 🚧 Possible Future Extensions

- 🐚 Shell completion generation

## 📜 MIT License

This repository has scripts created to be free software.  
Therefore, they can be distributed and/or modified within the terms of the ***MIT License***.

> ### See the [MIT License](LICENSE) file for details.

## 📬 Contact & Support

* 📧 **Email:** [m10ferrari1200@gmail.com](mailto:m10ferrari1200@gmail.com)
* 📧 **Email:** [contatolinuxdicaspro@gmail.com](mailto:contatolinuxdicaspro@gmail.com)
