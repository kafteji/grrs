use anyhow::{Context, Result};
use grrs::find_matches;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {}", args.path.display()))?;
    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
