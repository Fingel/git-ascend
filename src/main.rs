use crate::git::{collect_stats_since, first_commit_hash, open_repository};
use crate::progress::progress_bar_with_label;
use crate::scaling::calculate_level_info;
use crate::setup::{check_setup, setup};
use crate::state::{inc_last_commit, inc_xp, repo_state, reset_xp};
use clap::{Parser, Subcommand};

mod git;
mod progress;
mod scaling;
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
                println!("Repository not setup. Run `git-quest setup`.");
                return Ok(());
            }
            let repo = open_repository(&repo_path)?;
            let repo_id = first_commit_hash(&repo)?;
            let repo_state = repo_state(&repo_id)?;
            println!("{:?}", repo_state);
            let from_commit = if let Some(repo_state) = repo_state {
                repo_state.last_commit
            } else {
                first_commit_hash(&repo)?
            };

            let stats = collect_stats_since(&repo, &from_commit)?;
            if !stats.is_empty() {
                let total_added: usize = stats.iter().map(|s| s.lines_added).sum();
                let total_deleted: usize = stats.iter().map(|s| s.lines_deleted).sum();
                let total = total_added + total_deleted * 2;
                let xp = inc_xp(total).unwrap();
                let level_info = calculate_level_info(xp as u32);
                progress_bar_with_label(
                    level_info.current_level_progress,
                    level_info.xp_needed_to_level,
                    &format!("Level {}", level_info.level),
                );
                inc_last_commit(&repo_id, &stats.first().unwrap().sha)?;
            }
        }
    }

    Ok(())
}
