use anyhow::Result;
use clap::Parser;
use podcast_agg::sqlite_reader;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "refresh-subs", about = "Export Apple Podcasts subscriptions to data/subscriptions.json")]
struct Args {
    /// Path to MTLibrary.sqlite (defaults to the Apple Podcasts location for the current user).
    #[arg(long)]
    library: Option<PathBuf>,

    /// Output file (default: data/subscriptions.json under the project root).
    #[arg(long)]
    out: Option<PathBuf>,

    /// Project root (default: current working directory).
    #[arg(long)]
    repo: Option<PathBuf>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let repo = args.repo.unwrap_or_else(|| std::env::current_dir().expect("cwd"));
    let library = args.library.unwrap_or_else(sqlite_reader::default_library_path);
    let out = args.out.unwrap_or_else(|| repo.join("data/subscriptions.json"));

    let count = sqlite_reader::refresh_to_file(&library, &out)?;
    println!("wrote {} subscriptions to {}", count, out.display());
    Ok(())
}
