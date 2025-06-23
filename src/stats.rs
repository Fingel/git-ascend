use crate::ascii::display_number_x;
use crate::git::git_username;
use crate::scaling::{XpType, calculate_level_info};
use crate::state::{read_current_stat, read_xp};
use anyhow::Result;

pub fn main_stats() -> Result<String> {
    let xp = read_xp()?;
    let level_info = calculate_level_info(xp.total, XpType::Total);
    let username = git_username()?;
    Ok(format!(
        "{}{}Total XP: {}",
        username,
        display_number_x(level_info.level),
        xp.total
    ))
}

pub fn xp_levels() -> Result<String> {
    let xp = read_xp()?;
    let mut result = String::new();
    let precision = calculate_level_info(xp.precision, XpType::Precision);
    let output = calculate_level_info(xp.output, XpType::Output);
    let pedantry = calculate_level_info(xp.pedantry, XpType::Pedantry);
    let knowledge = calculate_level_info(xp.knowledge, XpType::Knowledge);
    let current_stat = read_current_stat()?;
    println!("Current Stat: {:?}", current_stat);
    result.push_str(&format!(
        "Precision: {}, total: {}/{}\n",
        precision.level, precision.current_level_progress, precision.xp_needed_to_level
    ));
    result.push_str(&format!(
        "Output: {}, total: {}/{}\n",
        output.level, output.current_level_progress, output.xp_needed_to_level
    ));
    result.push_str(&format!(
        "Pedantry: {}, total: {}/{}\n",
        pedantry.level, pedantry.current_level_progress, pedantry.xp_needed_to_level
    ));
    result.push_str(&format!(
        "Knowledge: {}, total: {}/{}",
        knowledge.level, knowledge.current_level_progress, knowledge.xp_needed_to_level
    ));
    Ok(result)
}
