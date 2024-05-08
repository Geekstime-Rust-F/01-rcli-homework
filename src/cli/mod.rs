mod text;

use clap::{command, Parser, Subcommand};
pub use text::TextSubCommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliSubCommand,
}

#[derive(Subcommand)]
pub enum CliSubCommand {
    #[command(subcommand)]
    Text(TextSubCommand),
}
