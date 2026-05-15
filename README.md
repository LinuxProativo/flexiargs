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
flexiargs = "1.0"
```

## 🌐 Public API

The crate intentionally exposes a very small API surface:

```rust
pub use messages::{invalid_arg, missing_arg};
pub use flexiargs::{Arg, parse_into_vars};
```

### 🚪 Main Entry Points

| Item | Description |
|---|---|
| `Arg` | Defines parsing rules |
| `parse_into_vars` | Executes parsing |
| `invalid_arg` | Standardized invalid argument error |
| `missing_arg` | Standardized missing parameter error |

## 🚀 Quick Example

```rust
use flexiargs::{Arg, parse_into_vars};
use std::collections::VecDeque;

let mut sync_mode = false;
let mut packages = Vec::new();
let mut cache_dir = String::new();
let mut config_file: Option<String> = None;
let mut use_overlay = true;

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

parse_into_vars("aports", &mut rules, args).ok();
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
## 🔍 Parsing

Parsing in flexiargs is explicit, deterministic, and fully rule-driven. Each step of
the parsing process is defined by clear rules that transform raw command-line input
into structured, validated, and typed data. This makes behavior predictable, easier
to debug, and consistent across different CLI designs.

### 📥 Basic Parsing

```rust
parse_into_vars(
    "server",
    &mut rules,
    args
).ok()?;
```

## 📦 ParseResult

The parser returns a `ParseResult`.

This provides:

- ✅ Parsing success/failure;
- 📦 Unmatched arguments;
- 📍 Positional handling;
- 🔒 Strict validation helpers;

### ✅ `.ok()`

Extracts the parsing result.

```rust
parse_into_vars("app", &mut rules, args)
    .ok()?;
```

### 🔒 `.strict()`

Rejects any unmatched arguments.

```rust
parse_into_vars("app", &mut rules, args)
    .strict()
    .ok()?;
```

### 🥇 `.strict_first()`

Ensures the first argument matches a rule.

```rust
.strict_first()
```

### 📏 `.strict_level(n)`

Rejects unmatched arguments up to a given depth.

```rust
.strict_level(2)
```

### 🪶 `.passthrough()`

Suppresses parsing errors.

Useful for optional parsing stages.

```rust
.passthrough()
```

### 📌 `.require_args()`

Fails if no arguments were supplied.

```rust
.require_args()?
```

### 📦 `.collect_rest()`

Collects unmatched positional arguments.

```rust
let mut remaining = Vec::new();

parse_into_vars("app", &mut rules, args)
    .collect_rest(&mut remaining)?;
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

## 🤝 Contributing

Contributions, improvements, and issue reports are welcome.

### 🚧 Possible Future Extensions

- 🤖 Auto-help generation
- 🌎 Environment variable integration
- 🐚 Shell completion generation

## 📜 MIT License

This repository has scripts created to be free software.  
Therefore, they can be distributed and/or modified within the terms of the ***MIT License***.

> ### See the [MIT License](LICENSE) file for details.

## 📬 Contact & Support

* 📧 **Email:** [m10ferrari1200@gmail.com](mailto:m10ferrari1200@gmail.com)
* 📧 **Email:** [contatolinuxdicaspro@gmail.com](mailto:contatolinuxdicaspro@gmail.com)
