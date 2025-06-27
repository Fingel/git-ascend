use crate::state::read_xp;
use anyhow::Result;
use bincode::{Decode, Encode};
use clap::ValueEnum;

pub const OUTPUT_SCALE: f64 = 10.0;
pub const PRECISION_SCALE: f64 = 50.0;
pub const PEDANTY_SCALE: f64 = 5.0;
pub const KNOWLEDGE_SCALE: f64 = 500.0;

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
    pub current_level_progress: u128,
    pub xp_needed_to_level: u128,
}

pub fn total_xp_gain(additions: u32, deletions: u32, commit_msg_len: u32) -> Result<u128> {
    let exp_state = read_xp()?;
    let precision = calculate_level_info(exp_state.precision, XpType::Precision);
    let output = calculate_level_info(exp_state.output, XpType::Output);
    let pedantry = calculate_level_info(exp_state.pedantry, XpType::Pedantry);
    let knowledge = calculate_level_info(exp_state.knowledge, XpType::Knowledge);

    let knowledge_mult = 1.0 + (knowledge.level as f64 / KNOWLEDGE_SCALE);
    let output_mult = additions as f64 * (1.0 + (output.level as f64 / OUTPUT_SCALE));
    let pedantry_mult = deletions as f64 * (1.0 + (pedantry.level as f64 / PEDANTY_SCALE));
    let precision_mult =
        (commit_msg_len / 10) as f64 * (1.0 + (precision.level as f64 / PRECISION_SCALE));
    let total = (1.0f64.max(output_mult) + 1.0f64.max(pedantry_mult) + 1.0f64.max(precision_mult))
        * knowledge_mult;

    Ok(total as u128)
}

/// Calculate XP required to reach a specific level from level 0
/// Uses logistic curve scaling: L / (1 + e^(-k(x - x_0)))
fn xp_required_for_level(level: u32, xp_type: XpType) -> u128 {
    // Logistic curve parameters: (L, k, x_0)
    let (max_xp, steepness, midpoint) = match xp_type {
        XpType::Total => (1500.0, 0.10, 100.0),
        XpType::Precision => (1500.0, 0.10, 100.0),
        XpType::Output => (1500.0, 0.10, 100.0),
        XpType::Pedantry => (1500.0, 0.10, 100.0),
        XpType::Knowledge => (1500.0, 0.10, 100.0),
    };

    if level == 0 {
        return 0;
    }

    let level_f = level as f64;

    if level_f <= midpoint {
        // For levels up to midpoint, use logistic curve: f(x) = L / (1 + e^(-k(x - x_0)))
        let exponent = -steepness * (level_f - midpoint);
        let xp = max_xp / (1.0 + exponent.exp());
        xp.round() as u128 + 2u128 * level as u128
    } else {
        // For levels beyond midpoint, use linear growth
        // Calculate XP at midpoint using logistic curve
        let exponent_at_midpoint = -steepness * 0.0; // This is 0
        let xp_at_midpoint = max_xp / (1.0 + exponent_at_midpoint.exp()); // This is max_xp / 2

        // Calculate slope as derivative of logistic curve at midpoint: L * k / 4
        let slope = max_xp * steepness / 4.0;

        // Linear growth from midpoint
        let xp = xp_at_midpoint + slope * (level_f - midpoint);
        xp.round() as u128
    }
}

/// Calculate total XP required to reach a specific level
/// This is the cumulative sum of XP required for each level
fn total_xp_for_level(level: u32, xp_type: XpType) -> u128 {
    if level == 0 {
        return 0;
    }

    (1..=level).map(|l| xp_required_for_level(l, xp_type)).sum()
}

/// Given current XP, calculate level and XP needed for next level
pub fn calculate_level_info(current_xp: u128, xp_type: XpType) -> LevelInfo {
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
pub fn get_level_progression(max_level: u32, xp_type: XpType) -> Vec<(u32, u128, u128)> {
    (1..=max_level)
        .map(|level| {
            let xp_for_this_level = xp_required_for_level(level, xp_type);
            let total_xp = total_xp_for_level(level, xp_type);
            (level, xp_for_this_level, total_xp)
        })
        .collect()
}
