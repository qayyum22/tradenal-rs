//! CLI entry point for Tradenal developer tooling.

use clap::{Parser, Subcommand};
use trade_core::healthcheck;

/// Top-level CLI arguments.
#[derive(Debug, Parser)]
#[command(name = "trade-cli")]
#[command(about = "Developer and admin CLI for Tradenal")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Supported CLI subcommands.
#[derive(Debug, Subcommand)]
enum Command {
    /// Prints a simple health status message.
    Healthcheck,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Healthcheck => {
            println!("{}", healthcheck().as_str());
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::Cli;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }
}
