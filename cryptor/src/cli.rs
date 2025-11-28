use clap::{Parser, Subcommand};
use crate::io;

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
            let _data = io::read_file(&input)?;
            println!(
                "[TEST] Encrypt | algo={} | input={} | output={} | password={:?}",
                algo, input, output, password
            );
            Ok(())
        }

        Commands::Decrypt { algo, input, output, password } => {
            let _data = io::read_file(&input)?;
            println!(
                "[TEST] Decrypt | algo={} | input={} | output={} | password={:?}",
                algo, input, output, password
            );
            Ok(())
        }

        Commands::Encode { algo, input, output } => {
            let _data = io::read_file(&input)?;
            println!(
                "[TEST] Encode | algo={} | input={} | output={}",
                algo, input, output
            );
            Ok(())
        }

        Commands::Decode { algo, input, output } => {
            let _data = io::read_file(&input)?;
            println!(
                "[TEST] Decode | algo={} | input={} | output={}",
                algo, input, output
            );
            Ok(())
        }
    }
}