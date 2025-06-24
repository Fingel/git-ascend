use crate::state::read_xp;
use anyhow::Result;
use bincode::{Decode, Encode};
use clap::ValueEnum;

pub const OUTPUT_SCALE: f32 = 10.0;
pub const PRECISION_SCALE: f32 = 1.0;
pub const PEDANTY_SCALE: f32 = 5.0;
pub const KNOWLEDGE_SCALE: f32 = 50.0;

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode, ValueEnum)]
pub enum XpType {
    Total,
    Precision,
    Output,
    Pedantry,
    Knowledge,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LevelInfo {
    pub level: u32,
    pub current_level_progress: u32,
    pub xp_needed_to_level: u32,
}

pub fn total_xp_gain(additions: u32, deletions: u32, commits: u32) -> Result<f32> {
    let exp_state = read_xp()?;
    let precision = calculate_level_info(exp_state.precision, XpType::Precision);
    let output = calculate_level_info(exp_state.output, XpType::Output);
    let pedantry = calculate_level_info(exp_state.pedantry, XpType::Pedantry);
    let knowledge = calculate_level_info(exp_state.knowledge, XpType::Knowledge);

    let knowledge_mult = 1.0 + (knowledge.level as f32 / KNOWLEDGE_SCALE);

    let output_mult = additions as f32 * (1.0 + (output.level as f32 / OUTPUT_SCALE));
    let pedantry_mult = deletions as f32 * (1.0 + (pedantry.level as f32 / PEDANTY_SCALE));
    let precision_mult = commits as f32 * (1.0 + (precision.level as f32 / PRECISION_SCALE));

    let total = 1.0f32.max(output_mult * knowledge_mult)
        * 1.0f32.max(pedantry_mult * knowledge_mult)
        * (precision_mult * knowledge_mult);

    Ok(total)
}

/// Calculate XP required to reach a specific level from level 0
/// Uses logarithmic scaling: base_xp * log2(level + 1) * scaling_factor
fn xp_required_for_level(level: u32, xp_type: XpType) -> u32 {
    const BASE_XP: f64 = 50.0;
    let scaling_factor: f64 = match xp_type {
        XpType::Total => 1.5,
        XpType::Precision => 1.5,
        XpType::Output => 1.5,
        XpType::Pedantry => 1.5,
        XpType::Knowledge => 1.5,
    };

    if level == 0 {
        return 0;
    }

    // For level 1, we want exactly 50 XP
    if level == 1 {
        return 50;
    }

    // For higher levels, use logarithmic scaling
    let level_f = level as f64;
    let xp = BASE_XP * (level_f + 1.0).log2() * scaling_factor;
    xp.round() as u32
}

/// Calculate total XP required to reach a specific level
/// This is the cumulative sum of XP required for each level
fn total_xp_for_level(level: u32, xp_type: XpType) -> u32 {
    if level == 0 {
        return 0;
    }

    (1..=level).map(|l| xp_required_for_level(l, xp_type)).sum()
}

/// Given current XP, calculate level and XP needed for next level
pub fn calculate_level_info(current_xp: u32, xp_type: XpType) -> LevelInfo {
    if current_xp == 0 {
        return LevelInfo {
            level: 0,
            current_level_progress: 0,
            xp_needed_to_level: 50,
        };
    }

    // Find the current level by checking total XP thresholds
    let mut level = 1;
    while total_xp_for_level(level, xp_type) <= current_xp {
        level += 1;
    }
    level -= 1; // Back up to the level we actually achieved

    let total_xp_current_level = total_xp_for_level(level, xp_type);
    let total_xp_for_next_level = total_xp_for_level(level + 1, xp_type);
    let xp_needed_to_level = total_xp_for_next_level - total_xp_current_level;
    let current_level_progress = current_xp - total_xp_current_level;

    LevelInfo {
        level,
        current_level_progress,
        xp_needed_to_level,
    }
}

/// Get XP requirements for multiple levels (useful for displaying progression)
#[allow(dead_code)]
pub fn get_level_progression(max_level: u32, xp_type: XpType) -> Vec<(u32, u32, u32)> {
    (1..=max_level)
        .map(|level| {
            let xp_for_this_level = xp_required_for_level(level, xp_type);
            let total_xp = total_xp_for_level(level, xp_type);
            (level, xp_for_this_level, total_xp)
        })
        .collect()
}
