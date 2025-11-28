use clap::{Parser, Subcommand};
use crate::io;
use crate::crypto;

#[derive(Parser)]
#[command(name = "cryptor", version)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encrypt {
        #[arg(long)]
        algo: String,
        #[arg(long)]
        input: String,
        #[arg(long)]
        output: String,
        #[arg(long)]
        password: Option<String>,
    },
    Decrypt {
        #[arg(long)]
        algo: String,
        #[arg(long)]
        input: String,
        #[arg(long)]
        output: String,
        #[arg(long)]
        password: Option<String>,
    },
    Encode {
        #[arg(long)]
        algo: String,
        #[arg(long)]
        input: String,
        #[arg(long)]
        output: String,
    },
    Decode {
        #[arg(long)]
        algo: String,
        #[arg(long)]
        input: String,
        #[arg(long)]
        output: String,
    },
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn run(args: Args) -> anyhow::Result<()> {
    match args.command {
        Commands::Encrypt { algo, input, output, password } => {
            let data = io::read_file(&input)?;
            match algo.as_str() {
                "aes" => unimplemented!("aes encrypt waiting for colleague"),
                "chacha" => unimplemented!("chacha encrypt waiting for colleague"),
                "xor" => unimplemented!("xor encrypt waiting for colleague"),
                _ => anyhow::bail!("unknown algo"),
            }
        }
        Commands::Decrypt { algo, input, output, password } => {
            let data = io::read_file(&input)?;
            match algo.as_str() {
                "aes" => unimplemented!("aes decrypt waiting for colleague"),
                "chacha" => unimplemented!("chacha decrypt waiting for colleague"),
                "xor" => unimplemented!("xor decrypt waiting for colleague"),
                _ => anyhow::bail!("unknown algo"),
            }
        }
        Commands::Encode { algo, input, output } => {
            let data = io::read_file(&input)?;
            match algo.as_str() {
                "base64" => unimplemented!("base64 encode waiting for colleague"),
                _ => anyhow::bail!("unknown algo"),
            }
        }
        Commands::Decode { algo, input, output } => {
            let data = io::read_file(&input)?;
            match algo.as_str() {
                "base64" => unimplemented!("base64 decode waiting for colleague"),
                _ => anyhow::bail!("unknown algo"),
            }
        }
    }
}