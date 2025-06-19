use crate::git::{collect_stats_since, first_commit_hash, open_repository};
use crate::progress::progress_bar_with_label;
use crate::setup::{check_setup, setup};
use crate::state::{inc_xp, reset_xp};
use clap::{Parser, Subcommand};

mod git;
mod progress;
mod setup;
mod state;

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
    Reset,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let repo_path = cli.repo_path;
    match cli.command {
        Some(Commands::Setup) => {
            setup(&repo_path)?;
        }
        Some(Commands::Reset) => {
            reset_xp()?;
            println!("XP reset to 0");
        }
        None => {
            if !check_setup(&repo_path) {
                println!("Repository not setup. Run `git-quest setup` to setup the repository.");
                return Ok(());
            }
            let repo = open_repository(&repo_path)?;
            let from_commit = first_commit_hash(&repo)?; // if there is not history for this repo, otherwise fetch from store

            println!("Latest commit hash: {}", from_commit);
            let stats = collect_stats_since(&repo, &from_commit)?;
            println!("Found {} commits since the specified commit", stats.len());

            let total_added: usize = stats.iter().map(|s| s.lines_added).sum();
            let total_deleted: usize = stats.iter().map(|s| s.lines_deleted).sum();
            let total = total_added + total_deleted * 2;
            progress_bar_with_label(total, 10000, "Level 1");

            println!("\nTotals: +{} -{}(x2) lines", total_added, total_deleted);

            let xp = inc_xp(total).unwrap();
            println!("XP: {}", xp);
        }
    }

    Ok(())
}
