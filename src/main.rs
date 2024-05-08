use anyhow::Result;
use clap::Parser;
use rcli::{
    process_jwt_sign, process_jwt_verify, Cli, CliSubCommand, JwtSubCommand, TextEncryptAndDecrypt,
    TextSubCommand,
};

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
        CliSubCommand::Jwt(jwt_subcommand) => match jwt_subcommand {
            JwtSubCommand::Sign(jwt_sign_params) => process_jwt_sign(
                &jwt_sign_params.sub,
                &jwt_sign_params.aud,
                &jwt_sign_params.exp,
            ),
            JwtSubCommand::Verify(jwt_verify_params) => {
                process_jwt_verify(&jwt_verify_params.token)
            }
        },
    }
}
