use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum TextSubCommand {
    Encrypt(TextEncryptParams),
    Decrypt(TextDecryptParams),
}

#[derive(Args)]
pub struct TextEncryptParams {
    #[arg(long, short)]
    pub key: String,
}

#[derive(Args)]
pub struct TextDecryptParams {
    #[arg(long, short)]
    pub key: String,
}
