mod http;
mod jwt;
mod text;

pub use http::*;
pub use jwt::*;
pub use text::*;

use clap::{command, Parser, Subcommand};

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

    #[command(subcommand)]
    Jwt(JwtSubCommand),

    #[command(subcommand)]
    Http(HttpSubCommand),
}
