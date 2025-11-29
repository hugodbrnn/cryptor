mod cli;
mod io;
mod crypto;

fn main() -> anyhow::Result<()> {
    let args = cli::parse_args();   // Clap robust
    cli::run(args)?;                 // exécution

    Ok(()) // Tout s'est bien passé
}
