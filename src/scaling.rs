/// XP scaling system with logarithmic progression
/// Level 1 requires 50 XP, subsequent levels scale logarithmically

#[derive(Debug, Clone, PartialEq)]
pub struct LevelInfo {
    pub level: u32,
    pub current_level_progress: u32,
    pub xp_needed_to_level: u32,
}

/// Calculate XP required to reach a specific level from level 0
/// Uses logarithmic scaling: base_xp * log2(level + 1) * scaling_factor
fn xp_required_for_level(level: u32) -> u32 {
    const BASE_XP: f64 = 50.0;
    const SCALING_FACTOR: f64 = 1.5;

    if level == 0 {
        return 0;
    }

    // For level 1, we want exactly 50 XP
    if level == 1 {
        return 50;
    }

    // For higher levels, use logarithmic scaling
    let level_f = level as f64;
    let xp = BASE_XP * (level_f + 1.0).log2() * SCALING_FACTOR;
    xp.round() as u32
}

/// Calculate total XP required to reach a specific level
/// This is the cumulative sum of XP required for each level
fn total_xp_for_level(level: u32) -> u32 {
    if level == 0 {
        return 0;
    }

    (1..=level).map(xp_required_for_level).sum()
}

/// Given current XP, calculate level and XP needed for next level
pub fn calculate_level_info(current_xp: u32) -> LevelInfo {
    if current_xp == 0 {
        return LevelInfo {
            level: 0,
            current_level_progress: 0,
            xp_needed_to_level: 50,
        };
    }

    // Find the current level by checking total XP thresholds
    let mut level = 1;
    while total_xp_for_level(level) <= current_xp {
        level += 1;
    }
    level -= 1; // Back up to the level we actually achieved

    let total_xp_current_level = total_xp_for_level(level);
    let total_xp_for_next_level = total_xp_for_level(level + 1);
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
pub fn get_level_progression(max_level: u32) -> Vec<(u32, u32, u32)> {
    (1..=max_level)
        .map(|level| {
            let xp_for_this_level = xp_required_for_level(level);
            let total_xp = total_xp_for_level(level);
            (level, xp_for_this_level, total_xp)
        })
        .collect()
}
