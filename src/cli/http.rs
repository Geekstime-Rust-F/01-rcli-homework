use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum HttpSubCommand {
    Serve(HttpServeParams),
}

#[derive(Args)]
pub struct HttpServeParams {
    #[arg(long, short)]
    pub dir: String,
}
