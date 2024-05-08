use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum JwtSubCommand {
    Sign(JwtSignParams),
    Verify(JwtVerifyParams),
}

#[derive(Args, Debug)]
pub struct JwtSignParams {
    #[arg(long)]
    pub sub: String,

    #[arg(long)]
    pub aud: String,

    #[arg(long)]
    pub exp: String,
}

#[derive(Args, Debug)]
pub struct JwtVerifyParams {
    #[arg(long, short)]
    pub token: String,
}
