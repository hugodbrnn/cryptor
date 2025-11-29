use clap::{Parser, Subcommand};
use crate::io;

use crate::crypto::{
    aes,
    chacha,
    xor,
    base64,
};

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

        // ======================================================================
        // ENCRYPT
        // ======================================================================
        Commands::Encrypt { algo, input, output, password } => {

            if !std::path::Path::new(&input).exists() {
                anyhow::bail!("Input file does not exist: {}", input);
            }

            let allowed = ["aes", "chacha", "xor"];
            if !allowed.contains(&algo.as_str()) {
                anyhow::bail!("Invalid algorithm '{}'. Allowed: aes, chacha, xor", algo);
            }

            if algo != "xor" && password.is_none() {
                anyhow::bail!("--password is required for AES and ChaCha20");
            }

            if algo == "xor" && password.is_some() {
                anyhow::bail!("--password must not be used with XOR");
            }

            let data = io::read_file(&input)?;

            let result = match algo.as_str() {
                "aes" => {
                    let pwd = password.as_ref().unwrap();
                    aes::encrypt(pwd, &data)?
                }
                "chacha" => {
                    let pwd = password.as_ref().unwrap();
                    chacha::encrypt(pwd, &data)?
                }
                "xor" => {
                    let key = b"simplexor";
                    xor::xor_encrypt(key, &data)
                }
                _ => unreachable!(),
            };

            io::write_file(&output, &result)?;
            println!("✔ Encryption complete: {} → {}", input, output);

            Ok(())
        }

        // ======================================================================
        // DECRYPT
        // ======================================================================
        Commands::Decrypt { algo, input, output, password } => {

            if !std::path::Path::new(&input).exists() {
                anyhow::bail!("Input file does not exist: {}", input);
            }

            let allowed = ["aes", "chacha", "xor"];
            if !allowed.contains(&algo.as_str()) {
                anyhow::bail!("Invalid algorithm '{}'. Allowed: aes, chacha, xor", algo);
            }

            if algo != "xor" && password.is_none() {
                anyhow::bail!("--password is required for AES and ChaCha20");
            }

            if algo == "xor" && password.is_some() {
                anyhow::bail!("--password must not be used with XOR");
            }

            let data = io::read_file(&input)?;

            let result = match algo.as_str() {
                "aes" => {
                    let pwd = password.as_ref().unwrap();
                    aes::decrypt(pwd, &data)?
                }
                "chacha" => {
                    let pwd = password.as_ref().unwrap();
                    chacha::decrypt(pwd, &data)?
                }
                "xor" => {
                    let key = b"simplexor";
                    xor::xor_decrypt(key, &data)
                }
                _ => unreachable!(),
            };

            io::write_file(&output, &result)?;
            println!("✔ Decryption complete: {} → {}", input, output);

            Ok(())
        }

        // ======================================================================
        // BASE64 ENCODE
        // ======================================================================
        Commands::Encode { algo, input, output } => {

            if algo != "base64" {
                anyhow::bail!("Invalid algorithm '{}'. Only 'base64' is allowed", algo);
            }

            let data = io::read_file(&input)?;
            let encoded = base64::encode(&data);

            io::write_file(&output, encoded.as_bytes())?;
            println!("✔ Base64 encoding complete: {} → {}", input, output);

            Ok(())
        }

        // ======================================================================
        // BASE64 DECODE
        // ======================================================================
        Commands::Decode { algo, input, output } => {

            if algo != "base64" {
                anyhow::bail!("Invalid algorithm '{}'. Only 'base64' is allowed", algo);
            }

            let data = io::read_file(&input)?;
            let as_string = String::from_utf8_lossy(&data);

            let decoded = base64::decode(&as_string)?;
            io::write_file(&output, &decoded)?;

            println!("✔ Base64 decoding complete: {} → {}", input, output);

            Ok(())
        }
    }
}
