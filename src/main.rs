use anyhow::Result;
use clap::{command, Args, Parser, Subcommand};
mod process;
use process::TextEncryptAndDecrypt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliSubCommand,
}

#[derive(Subcommand)]
enum CliSubCommand {
    #[command(subcommand)]
    Text(TextSubCommand),
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
            let text_encypt_and_decrypt = TextEncryptAndDecrypt::new();
            match text_subcommand {
                TextSubCommand::Encrypt(text_encrypt_params) => {
                    text_encypt_and_decrypt.process_text_encrypt(&text_encrypt_params.key)
                }
                TextSubCommand::Decrypt(text_decrypt_params) => {
                    text_encypt_and_decrypt.process_text_decrypt(&text_decrypt_params.key)
                }
            }
        }
    }
}
