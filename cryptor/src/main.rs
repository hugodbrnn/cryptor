mod cli;
mod io;
mod crypto;

fn main() -> anyhow::Result<()> {
    let args = cli::parse_args();
    cli::run(args)?;

    Ok(())
}
