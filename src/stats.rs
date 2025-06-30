use crate::ascii::display_number_x;
use crate::progress::{format_progress_bar, short_bar_outside_label};
use crate::scaling::{
    KNOWLEDGE_SCALE, OUTPUT_SCALE, PEDANTY_SCALE, PRECISION_SCALE, XpType, calculate_level_info,
};
use crate::state::{read_current_stat, read_xp};
use anyhow::Result;

pub fn main_stats() -> Result<()> {
    let xp = read_xp()?;
    let level_info = calculate_level_info(xp.total, XpType::Total);
    let progress_bar = format_progress_bar(
        level_info.current_level_progress,
        level_info.xp_needed_to_level,
        Some(50),
        None,
    );
    println!("{}{}\n", display_number_x(level_info.level), progress_bar,);
    Ok(())
}

pub fn xp_levels() -> Result<()> {
    let xp = read_xp()?;
    let precision = calculate_level_info(xp.precision, XpType::Precision);
    let output = calculate_level_info(xp.output, XpType::Output);
    let pedantry = calculate_level_info(xp.pedantry, XpType::Pedantry);
    let knowledge = calculate_level_info(xp.knowledge, XpType::Knowledge);
    let current_stat = read_current_stat()?;
    let mut result = String::new();

    let output_bar = short_bar_outside_label(
        output.current_level_progress,
        output.xp_needed_to_level,
        &output.level.to_string(),
    );
    result.push_str(&format!(
        "{:<10} {:<43} {:.2}x\n",
        "Output",
        output_bar,
        (1.0 + (output.level as f64 / OUTPUT_SCALE)),
    ));

    let pedantry_bar = short_bar_outside_label(
        pedantry.current_level_progress,
        pedantry.xp_needed_to_level,
        &pedantry.level.to_string(),
    );
    result.push_str(&format!(
        "{:<10} {:<43} {:.2}x\n",
        "Pedantry",
        pedantry_bar,
        (1.0 + (pedantry.level as f64 / PEDANTY_SCALE)),
    ));

    let precision_bar = short_bar_outside_label(
        precision.current_level_progress,
        precision.xp_needed_to_level,
        &precision.level.to_string(),
    );
    result.push_str(&format!(
        "{:<10} {:<43} {:.2}x\n",
        "Precision",
        precision_bar,
        (1.0 + (precision.level as f64 / PRECISION_SCALE)),
    ));

    let knowledge_bar = short_bar_outside_label(
        knowledge.current_level_progress,
        knowledge.xp_needed_to_level,
        &knowledge.level.to_string(),
    );
    result.push_str(&format!(
        "{:<10} {:<43} {:.2}x\n",
        "Knowledge",
        knowledge_bar,
        (1.0 + (knowledge.level as f64 / KNOWLEDGE_SCALE)),
    ));
    println!("{result}");
    println!("Active Stat: \x1b[1m{current_stat:?}\x1b[0m");
    println!(
        "Use \x1b[1mgit ascend switch\x1b[0m to level a different stat or view their descriptions."
    );
    Ok(())
}
