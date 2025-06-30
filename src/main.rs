use crate::git::GitRepo;
use crate::progress::{animated_progress_bar, format_progress_bar};
use crate::scaling::{XpType, calculate_level_info, total_xp_gain};
use crate::setup::{check_setup, first_run, setup, welcome_message};
use crate::state::{inc_last_commit, inc_xp, read_xp, repo_state, reset_xp, set_current_stat};
use crate::stats::{main_stats, xp_levels};
use clap::{Parser, Subcommand};

mod ascii;
mod git;
mod progress;
mod scaling;
mod setup;
mod state;
mod stats;

/// Become the 1,000,000x developer you were destined to be
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, default_value = ".")]
    repo_path: String,
    /// Disable progress bar animations after commit
    #[arg(short, long, action)]
    disable_animations: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a git repository to your ascension
    Setup,
    /// View your experience levels and stat multipliers
    Stats,
    /// Change your currently leveling stat
    Switch { stat: Option<XpType> },
    /// Reset all experience levels
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
        Some(Commands::Stats) => {
            main_stats()?;
            xp_levels()?;
        }
        Some(Commands::Switch { stat }) => {
            let set_stat = match stat {
                Some(stat) => stat,
                None => query_stat(),
            };
            set_current_stat(set_stat)?;
            println!("Current stat set to {set_stat:?}");
        }
        None => {
            if first_run() {
                welcome_message();
                return Ok(());
            } else if !check_setup(&repo_path) {
                println!(
                    "This repository does not count towards your ascension. Run `git ascend setup` to add it."
                );
                return Ok(());
            }
            let repo = GitRepo::new(&repo_path)?;
            let repo_id = repo.id()?;
            let repo_state = repo_state(&repo_id)?;
            let pre_exp = read_xp()?;
            let stats = repo.commits_since(&repo_state.last_recorded_commit)?;
            let post_exp;
            if !stats.is_empty() {
                let total_added: u32 = stats.iter().map(|s| s.lines_added).sum();
                let total_deleted: u32 = stats.iter().map(|s| s.lines_deleted).sum();
                let commit_msg_len: u32 = stats.iter().map(|s| s.message.len() as u32).sum();
                let total = total_xp_gain(total_added, total_deleted, commit_msg_len)?;
                post_exp = inc_xp(total)?;
                inc_last_commit(&repo_id, &stats.first().unwrap().sha)?;
            } else {
                post_exp = read_xp()?;
            }
            if cli.disable_animations {
                let info = calculate_level_info(post_exp.total, XpType::Total);
                let cur_bar = format_progress_bar(
                    info.current_level_progress,
                    info.xp_needed_to_level,
                    None,
                    None,
                );
                print!("{}x {}", info.level, cur_bar);
            } else {
                animated_progress_bar(pre_exp.total, post_exp.total, None, |total_xp| {
                    let info = calculate_level_info(total_xp, XpType::Total);
                    (
                        info.current_level_progress,
                        info.xp_needed_to_level,
                        info.level,
                    )
                });
            }
            println!();
        }
    }

    Ok(())
}

fn query_stat() -> XpType {
    println!(
        "1. Precision increases XP gained based on commit message length.
2. Output increases XP gained per line of code added.
3. Pedantry increases XP gained per line of code deleted.
4. Knowledge increases all XP gained.
Enter choice 1-4: "
    );
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).unwrap();
    match input_str.trim().parse::<u8>().unwrap_or_default() {
        1 => XpType::Precision,
        2 => XpType::Output,
        3 => XpType::Pedantry,
        4 => XpType::Knowledge,
        _ => {
            println!("Invalid choice");
            query_stat()
        }
    }
}
