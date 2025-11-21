//! Fusabi - Functional Scripting for Rust
//!
//! Command-line interface for executing Mini-F# scripts through the Fusabi pipeline.
//!
//! # Usage
//!
//! ```bash
//! # Run a script file (JIT execution)
//! fus run examples/hello.fsx
//!
//! # Compile to bytecode
//! fus grind examples/arithmetic.fsx
//!
//! # Run with disassembly output
//! fus run --disasm examples/arithmetic.fsx
//!
//! # Evaluate an expression directly
//! fus run -e "let x = 42 in x + 1"
//!
//! # Package manager (coming soon)
//! fus root install some-package
//!
//! # Show help
//! fus --help
//! ```

use colored::*;
use fusabi_demo::{run_file, run_file_with_disasm, run_source, run_source_with_disasm};
use std::env;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Print ASCII banner with brand colors
fn print_banner() {
    println!(
        r#"
     {}
    {} {}  Fusabi v{}
     {}   Small. Potent. Functional.
        "#,
        "/ \\".truecolor(153, 204, 51),
        "(".truecolor(153, 204, 51),
        "F".truecolor(153, 204, 51).bold(),
        ")".truecolor(153, 204, 51),
        VERSION,
        "\\_/".truecolor(153, 204, 51)
    );
}

/// Print help text with colorization
fn print_help() {
    print_banner();
    println!(
        r#"
{}
    fus <COMMAND> [OPTIONS] [FILE]
    fus run -e <EXPRESSION>

{}
    {}                 JIT execution of .fsx script (default)
    {}               Compile script to .fzb bytecode
    {}                Package manager (coming soon)

{}
    {}          Show this help message
    {}       Show version information
    {}   Evaluate an expression directly (run mode only)
    {}        Show bytecode disassembly before execution

{}
    {}                Path to .fsx script file

{}
    {} JIT execute a script
    fus run examples/arithmetic.fsx

    {} Compile to bytecode
    fus grind examples/arithmetic.fsx

    {} Evaluate an expression
    fus run -e "let x = 10 in x + 5"

    {} Show bytecode disassembly
    fus run --disasm examples/conditionals.fsx

    {} Package manager (placeholder)
    fus root install some-package

For more information, see: {}
"#,
        "USAGE:".bright_green().bold(),
        "COMMANDS:".bright_green().bold(),
        "run".truecolor(153, 204, 51),
        "grind".truecolor(153, 204, 51),
        "root".truecolor(153, 204, 51),
        "OPTIONS:".bright_green().bold(),
        "-h, --help".truecolor(153, 153, 153),
        "-v, --version".truecolor(153, 153, 153),
        "-e, --eval <EXPR>".truecolor(153, 153, 153),
        "-d, --disasm".truecolor(153, 153, 153),
        "ARGUMENTS:".bright_green().bold(),
        "FILE".truecolor(153, 153, 153),
        "EXAMPLES:".bright_green().bold(),
        "#".truecolor(153, 153, 153),
        "#".truecolor(153, 153, 153),
        "#".truecolor(153, 153, 153),
        "#".truecolor(153, 153, 153),
        "#".truecolor(153, 153, 153),
        "https://github.com/fusabi-lang/fusabi".truecolor(153, 204, 51)
    );
}

struct Config {
    mode: Mode,
    disasm: bool,
}

enum Mode {
    RunFile(String),
    Eval(String),
    Grind(String),
    Root(Vec<String>),
    Help,
    Version,
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();

    let mut mode = None;
    let mut disasm = false;
    let mut i = 1;

    // Check for global flags first
    if i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                mode = Some(Mode::Help);
            }
            "-v" | "--version" => {
                mode = Some(Mode::Version);
            }
            _ => {}
        }
    }

    // If we haven't matched a global flag, parse command
    if mode.is_none() && i < args.len() {
        let command = args[i].as_str();
        match command {
            "run" => {
                i += 1;
                // Parse run options
                while i < args.len() {
                    match args[i].as_str() {
                        "-d" | "--disasm" => {
                            disasm = true;
                            i += 1;
                        }
                        "-e" | "--eval" => {
                            if i + 1 >= args.len() {
                                return Err("--eval requires an expression argument".to_string());
                            }
                            mode = Some(Mode::Eval(args[i + 1].clone()));
                            i += 2;
                        }
                        arg if arg.starts_with('-') => {
                            return Err(format!("Unknown option: {}", arg));
                        }
                        file => {
                            mode = Some(Mode::RunFile(file.to_string()));
                            break;
                        }
                    }
                }
                // Default to hello.fus if no file specified
                if mode.is_none() {
                    mode = Some(Mode::RunFile("examples/hello.fus".to_string()));
                }
            }
            "grind" => {
                i += 1;
                if i >= args.len() {
                    return Err("grind command requires a script file".to_string());
                }
                mode = Some(Mode::Grind(args[i].clone()));
            }
            "root" => {
                i += 1;
                let subcommands: Vec<String> = args[i..].to_vec();
                mode = Some(Mode::Root(subcommands));
            }
            arg if arg.starts_with('-') => {
                return Err(format!("Unknown option: {}", arg));
            }
            // If no command specified, treat first arg as file for run mode
            file => {
                mode = Some(Mode::RunFile(file.to_string()));
            }
        }
    }

    let mode = mode.unwrap_or_else(|| Mode::RunFile("examples/hello.fus".to_string()));

    Ok(Config { mode, disasm })
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    match config.mode {
        Mode::Help => {
            print_help();
            Ok(())
        }
        Mode::Version => {
            print_banner();
            Ok(())
        }
        Mode::Eval(expr) => {
            let result = if config.disasm {
                run_source_with_disasm(&expr, "<eval>")?
            } else {
                run_source(&expr)?
            };
            println!("{} {}", "✅".bright_green(), result);
            Ok(())
        }
        Mode::RunFile(path) => {
            let result = if config.disasm {
                run_file_with_disasm(&path)?
            } else {
                run_file(&path)?
            };
            println!("{} {}", "✅".bright_green(), result);
            Ok(())
        }
        Mode::Grind(path) => {
            println!(
                "{} Compiling {} to bytecode...",
                "🔥".bright_yellow(),
                path.truecolor(153, 204, 51).bold()
            );
            println!(
                "{} Feature coming soon: .fzb bytecode compilation",
                "⚠️ ".yellow()
            );
            println!(
                "{} For now, use {} for JIT execution",
                "💡".truecolor(153, 153, 153),
                format!("fus run {}", path).truecolor(153, 204, 51)
            );
            Ok(())
        }
        Mode::Root(subcommands) => {
            println!(
                "{} Fusabi Package Manager - Coming Soon",
                "🟢".bright_green()
            );
            if !subcommands.is_empty() {
                println!(
                    "{} Requested: {}",
                    "💡".truecolor(153, 153, 153),
                    format!("fus root {}", subcommands.join(" ")).truecolor(153, 204, 51)
                );
            }
            println!("\n{}:", "Planned features".bright_green().bold());
            println!(
                "  {} fus root install <package>  # Install package",
                "•".truecolor(153, 204, 51)
            );
            println!(
                "  {} fus root search <query>     # Search packages",
                "•".truecolor(153, 204, 51)
            );
            println!(
                "  {} fus root update             # Update packages",
                "•".truecolor(153, 204, 51)
            );
            println!(
                "  {} fus root init               # Initialize project",
                "•".truecolor(153, 204, 51)
            );
            Ok(())
        }
    }
}

fn main() {
    let config = match parse_args() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{} {}", "❌".red().bold(), err.truecolor(183, 65, 14));
            eprintln!(
                "{} Try {} for more information.",
                "💡".truecolor(153, 153, 153),
                "fus --help".truecolor(153, 204, 51)
            );
            process::exit(1);
        }
    };

    if let Err(err) = run(config) {
        eprintln!("{} {}", "❌".red().bold(), err.truecolor(183, 65, 14));
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_version_constant() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_banner_prints() {
        // This just ensures the banner function doesn't panic
        print_banner();
    }

    #[test]
    fn test_help_prints() {
        // This just ensures the help function doesn't panic
        print_help();
    }
}
