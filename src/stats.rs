use crate::ascii::display_number_x;
use crate::git::git_username;
use crate::scaling::calculate_level_info;
use crate::state::read_xp;
use anyhow::Result;

pub fn main_stats() -> Result<String> {
    let xp = read_xp()?;
    let level_info = calculate_level_info(xp as u32);
    let username = git_username()?;
    Ok(format!(
        "{}{}Total XP: {}",
        username,
        display_number_x(level_info.level),
        xp
    ))
}
