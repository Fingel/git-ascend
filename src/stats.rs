use crate::ascii::display_number_x;
use crate::git::git_username;
use crate::progress::format_progress_bar;
use crate::scaling::{XpType, calculate_level_info};
use crate::state::{read_current_stat, read_xp};
use anyhow::Result;

pub fn main_stats() -> Result<()> {
    let xp = read_xp()?;
    let level_info = calculate_level_info(xp.total, XpType::Total);
    let username = git_username()?;
    let progress_bar = format_progress_bar(
        level_info.current_level_progress,
        level_info.xp_needed_to_level,
        Some(50),
        None,
    );
    println!(
        "Developer {}{}{}\nTotal XP: {}",
        username,
        display_number_x(level_info.level),
        progress_bar,
        xp.total
    );
    Ok(())
}

pub fn xp_levels() -> Result<()> {
    let xp = read_xp()?;
    let precision = calculate_level_info(xp.precision, XpType::Precision);
    let output = calculate_level_info(xp.output, XpType::Output);
    let pedantry = calculate_level_info(xp.pedantry, XpType::Pedantry);
    let knowledge = calculate_level_info(xp.knowledge, XpType::Knowledge);
    let current_stat = read_current_stat()?;
    println!("Current Stat: {:?}", current_stat);
    let precision_bar = format_progress_bar(
        precision.current_level_progress,
        precision.xp_needed_to_level,
        Some(25),
        Some(&precision.level.to_string()),
    );
    let output_bar = format_progress_bar(
        output.current_level_progress,
        output.xp_needed_to_level,
        Some(25),
        Some(&output.level.to_string()),
    );
    let pedantry_bar = format_progress_bar(
        pedantry.current_level_progress,
        pedantry.xp_needed_to_level,
        Some(25),
        Some(&pedantry.level.to_string()),
    );
    let knowledge_bar = format_progress_bar(
        knowledge.current_level_progress,
        knowledge.xp_needed_to_level,
        Some(25),
        Some(&knowledge.level.to_string()),
    );
    let output = format!(
        "{:<10} {} Increases xp per commit\n{:<10} {} Increases xp per LoC\n{:<10} {} Increases xp per LoC removed\n{:<10} {} Increases all xp gained",
        "Precision",
        precision_bar,
        "Output",
        output_bar,
        "Pedantry",
        pedantry_bar,
        "Knowledge",
        knowledge_bar
    );
    println!("{}", output);
    Ok(())
}
