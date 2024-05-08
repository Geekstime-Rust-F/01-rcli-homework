use anyhow::Result;
use clap::Parser;
use rcli::{Cli, CliSubCommand, TextEncryptAndDecrypt, TextSubCommand};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CliSubCommand::Text(text_subcommand) => {
            let text_encypt_and_decrypt = TextEncryptAndDecrypt::default();
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
