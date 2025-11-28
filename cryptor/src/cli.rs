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

            if algo == "aes" || algo == "chacha" {
                if password.is_none() {
                    anyhow::bail!("--password is required for AES and ChaCha20");
                }
            } else if algo == "xor" {
                if password.is_some() {
                    anyhow::bail!("--password must not be used with XOR");
                }
            }

            if std::path::Path::new(&output).is_dir() {
                anyhow::bail!("Output path is a directory: {}", output);
            }

            let data = io::read_file(&input)?;

            let result = match algo.as_str() {
                "aes" => {
                    let pwd = password.as_ref().unwrap();
                    // TON COLLÈGUE DEVRA implémenter crypto::aes::encrypt
                    todo!("AES encrypt: crypto::aes::encrypt(pwd, &data)");
                }
                "chacha" => {
                    let pwd = password.as_ref().unwrap();
                    todo!("ChaCha encrypt: crypto::chacha::encrypt(pwd, &data)");
                }
                "xor" => {
                    // XOR n’utilise PAS de password → clé générée ici plus tard !
                    todo!("XOR encrypt: crypto::xor::xor_encrypt(key, &data)");
                }
                _ => unreachable!(),
            };

            // Quand la crypto sera prête, ce sera :
            // io::write_file(&output, &result?)?;

            println!(
                "[TEST] Encrypt (future) | algo={} | input={} | output={} | password={:?} | size={} bytes",
                algo, input, output, password, data.len()
            );

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

            if algo == "aes" || algo == "chacha" {
                if password.is_none() {
                    anyhow::bail!("--password is required for AES and ChaCha20");
                }
            } else if algo == "xor" {
                if password.is_some() {
                    anyhow::bail!("--password must not be used with XOR");
                }
            }

            if std::path::Path::new(&output).is_dir() {
                anyhow::bail!("Output path is a directory: {}", output);
            }

            let data = io::read_file(&input)?;

            let result = match algo.as_str() {
                "aes" => {
                    let pwd = password.as_ref().unwrap();
                    todo!("AES decrypt: crypto::aes::decrypt(pwd, &data)");
                }
                "chacha" => {
                    let pwd = password.as_ref().unwrap();
                    todo!("ChaCha decrypt: crypto::chacha::decrypt(pwd, &data)");
                }
                "xor" => {
                    todo!("XOR decrypt: crypto::xor::xor_decrypt(key, &data)");
                }
                _ => unreachable!(),
            };

            println!(
                "[TEST] Decrypt (future) | algo={} | input={} | output={} | password={:?} | size={} bytes",
                algo, input, output, password, data.len()
            );

            Ok(())
        }

        // ----------------------------------------------------------------------
        // ENCODE
        // ----------------------------------------------------------------------
        Commands::Encode { algo, input, output } => {

            if !std::path::Path::new(&input).exists() {
                anyhow::bail!("Input file does not exist: {}", input);
            }

            if algo != "base64" {
                anyhow::bail!("Invalid algorithm '{}'. Only 'base64' is allowed", algo);
            }

            if std::path::Path::new(&output).is_dir() {
                anyhow::bail!("Output path is a directory: {}", output);
            }

            let data = io::read_file(&input)?;

            // Appel futur :
            // let encoded = crypto::base64::encode(&data);
            // io::write_file(&output, encoded.as_bytes())?;

            todo!("Base64 encode: crypto::base64::encode(&data)");

            println!(
                "[TEST] Encode (future) | algo={} | input={} | output={} | size={} bytes",
                algo, input, output, data.len()
            );

            Ok(())
        }

        // ----------------------------------------------------------------------
        // DECODE
        // ----------------------------------------------------------------------
        Commands::Decode { algo, input, output } => {

            if !std::path::Path::new(&input).exists() {
                anyhow::bail!("Input file does not exist: {}", input);
            }

            if algo != "base64" {
                anyhow::bail!("Invalid algorithm '{}'. Only 'base64' is allowed", algo);
            }

            if std::path::Path::new(&output).is_dir() {
                anyhow::bail!("Output path is a directory: {}", output);
            }

            let data = io::read_file(&input)?;

            let as_string = String::from_utf8_lossy(&data);

            // Appel futur :
            // let decoded = crypto::base64::decode(&as_string)?;
            // io::write_file(&output, &decoded)?;

            todo!("Base64 decode: crypto::base64::decode(&as_string)");

            println!(
                "[TEST] Decode (future) | algo={} | input={} | output={} | size={} bytes",
                algo, input, output, data.len()
            );

            Ok(())
        }
    }
}