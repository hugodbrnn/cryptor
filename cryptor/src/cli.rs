use clap::{Parser, Subcommand};
use crate::io;
use crate::crypto::{aes, chacha, xor, base64};
use anyhow::Result;

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

pub fn run(args: Args) -> Result<()> {
    match args.command {

        // ----------------------------------------------------------------------
        // ENCRYPT
        // ----------------------------------------------------------------------
        Commands::Encrypt { algo, input, output, password } => {

            if !std::path::Path::new(&input).exists() {
                anyhow::bail!("Input file does not exist: {}", input);
            }

            let allowed = ["aes", "chacha", "xor"];
            if !allowed.contains(&algo.as_str()) {
                anyhow::bail!("Invalid algorithm '{}'. Allowed: aes, chacha, xor", algo);
            }

            // Password requirements
            if algo == "aes" || algo == "chacha" {
                if password.is_none() {
                    anyhow::bail!("--password is required for AES and ChaCha20");
                }
            }
            if algo == "xor" && password.is_some() {
                anyhow::bail!("--password must NOT be used with XOR");
            }

            let data = io::read_file(&input)?;

            match algo.as_str() {
                "aes" => {
                    let pwd = password.unwrap();
                    let encrypted = aes::encrypt(&pwd, &data)?;
                    io::write_file(&output, &encrypted)?;
                }
                "chacha" => {
                    let pwd = password.unwrap();
                    let encrypted = chacha::encrypt(&pwd, &data)?;
                    io::write_file(&output, &encrypted)?;
                }
                "xor" => {
                    let key = b"supersecretkey";
                    let encrypted = xor::xor_encrypt(key, &data);
                    io::write_file(&output, &encrypted)?;
                }
                _ => unreachable!(),
            }

            println!("[OK] Encrypted using {} → {}", algo, output);
            Ok(())
        }

        // ----------------------------------------------------------------------
        // DECRYPT
        // ----------------------------------------------------------------------
        Commands::Decrypt { algo, input, output, password } => {

            if !std::path::Path::new(&input).exists() {
                anyhow::bail!("Input file does not exist: {}", input);
            }

            let allowed = ["aes", "chacha", "xor"];
            if !allowed.contains(&algo.as_str()) {
                anyhow::bail!("Invalid algorithm '{}'. Allowed: aes, chacha, xor", algo);
            }

            // Password requirements
            if algo == "aes" || algo == "chacha" {
                if password.is_none() {
                    anyhow::bail!("--password is required for AES and ChaCha20");
                }
            }
            if algo == "xor" && password.is_some() {
                anyhow::bail!("--password must NOT be used with XOR");
            }

            let data = io::read_file(&input)?;

            match algo.as_str() {
                "aes" => {
                    let pwd = password.unwrap();
                    let decrypted = aes::decrypt(&pwd, &data)?;
                    io::write_file(&output, &decrypted)?;
                }
                "chacha" => {
                    let pwd = password.unwrap();
                    let decrypted = chacha::decrypt(&pwd, &data)?;
                    io::write_file(&output, &decrypted)?;
                }
                "xor" => {
                    let key = b"supersecretkey";
                    let decrypted = xor::xor_decrypt(key, &data);
                    io::write_file(&output, &decrypted)?;
                }
                _ => unreachable!(),
            }

            println!("[OK] Decrypted using {} → {}", algo, output);
            Ok(())
        }

        // ----------------------------------------------------------------------
        // BASE64 ENCODE
        // ----------------------------------------------------------------------
        Commands::Encode { algo, input, output } => {

            if algo != "base64" {
                anyhow::bail!("Invalid algorithm '{}'. Only base64 is supported", algo);
            }

            if !std::path::Path::new(&input).exists() {
                anyhow::bail!("Input file does not exist: {}", input);
            }

            let data = io::read_file(&input)?;
            let encoded = base64::encode(&data);
            io::write_file(&output, encoded.as_bytes())?;

            println!("[OK] Base64 encoded → {}", output);
            Ok(())
        }

        // ----------------------------------------------------------------------
        // BASE64 DECODE
        // ----------------------------------------------------------------------
        Commands::Decode { algo, input, output } => {

            if algo != "base64" {
                anyhow::bail!("Invalid algorithm '{}'. Only base64 is supported", algo);
            }

            if !std::path::Path::new(&input).exists() {
                anyhow::bail!("Input file does not exist: {}", input);
            }

            let data = io::read_file(&input)?;
            let text = String::from_utf8_lossy(&data);
            let decoded = base64::decode(&text)?;
            io::write_file(&output, &decoded)?;

            println!("[OK] Base64 decoded → {}", output);
            Ok(())
        }
    }
}
