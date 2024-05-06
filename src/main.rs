use clap::{command, Args, Parser, Subcommand};
use anyhow::Result;
mod process;
use process::process_text_encrypt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliSubCommand,
}

#[derive(Subcommand)]
enum CliSubCommand {
    #[command(subcommand)]
    Text(TextSubCommand)
}

#[derive(Subcommand)]
enum TextSubCommand {
    Encrypt(TextEncryptParams),
    Decrypt(TextDecryptParams),
}

#[derive(Args)]
struct TextEncryptParams {
    #[arg(long, short)]
    key: String,
}

#[derive(Args)]
struct TextDecryptParams {
    #[arg(long, short)]
    key: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CliSubCommand::Text(text_subcommand) => {
            match text_subcommand {
                TextSubCommand::Encrypt(text_encrypt_params) => {
                    let key = text_encrypt_params.key;
                    process_text_encrypt(&key)
                }
                TextSubCommand::Decrypt(text_decrypt_params) => {
                    let key = text_decrypt_params.key;
                    println!("key: {}", key);
                    Ok(())
                }
            }
        }
    }
}
