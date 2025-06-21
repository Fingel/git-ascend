use crate::git::git_username;
use crate::scaling::calculate_level_info;
use crate::state::read_xp;
use anyhow::Result;

pub fn main_stats() -> Result<String> {
    let xp = read_xp()?;
    let level_info = calculate_level_info(xp as u32);
    let username = git_username()?;
    Ok(format!(
        "{}\nLevel: {}\nTotal xp: {}",
        username, level_info.level, xp
    ))
}
