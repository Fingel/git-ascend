use crate::git::collect_stats_since;
use clap::{Parser, Subcommand};
mod git;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, default_value = ".")]
    repo_path: String,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_commit = "515e6d07";
    let cli = Cli::parse();
    let repo_path = cli.repo_path;
    match cli.command {
        Some(Commands::Setup) => {
            println!("Setting up git-quest...");
            // Add setup logic here
        }
        None => {
            let stats = collect_stats_since(&repo_path, test_commit)?;
            println!("Found {} commits since the specified commit", stats.len());

            for stat in &stats {
                println!(
                    "{} | {} | +{} -{} | {} | {}",
                    &stat.sha[..8],
                    stat.author,
                    stat.lines_added,
                    stat.lines_deleted,
                    stat.message.lines().next().unwrap_or(""),
                    stat.timestamp
                );
            }

            let total_added: usize = stats.iter().map(|s| s.lines_added).sum();
            let total_deleted: usize = stats.iter().map(|s| s.lines_deleted).sum();

            println!("\nTotals: +{} -{} lines", total_added, total_deleted);
        }
    }

    Ok(())
}
