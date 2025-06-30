use std::io::{self, Write};

pub fn format_progress_bar(
    current: u128,
    max: u128,
    width: Option<u32>,
    label: Option<&str>,
) -> String {
    let bar_width = width.unwrap_or(50);
    let label_text = label.unwrap_or("");

    let percentage = if max == 0 {
        0.0
    } else {
        (current as f64 / max as f64) * 100.0
    };
    let filled_width = if max == 0 {
        0
    } else {
        (current * bar_width as u128) / max
    };

    let filled_char = '█';
    let empty_char = '░';

    let filled_part: String = filled_char.to_string().repeat(filled_width as usize);
    let empty_part: String = empty_char
        .to_string()
        .repeat(bar_width.wrapping_sub(filled_width.try_into().unwrap_or(0)) as usize);

    format!("{label_text}{filled_part}{empty_part} {percentage:>4.1}% ({current}/{max})")
}

pub fn animated_progress_bar(
    from: u128,
    to: u128,
    label: Option<&str>,
    cb: fn(total_xp: u128) -> (u128, u128, u32),
) {
    let total_range = to - from;

    const MAX_FRAMES: u128 = 50; // Limit frames for performance
    const FRAME_DELAY_MS: u64 = 10; // Consistent delay

    let frames_to_show = if total_range <= MAX_FRAMES {
        // Show every frame if range is small
        (from..=to).collect::<Vec<_>>()
    } else {
        // Sample 50 frames
        let mut frames = Vec::new();
        frames.push(from);

        // Add intermediate frames
        for i in 1..MAX_FRAMES {
            let value = from + (total_range * i) / MAX_FRAMES;
            if value != from && value != to {
                frames.push(value);
            }
        }

        frames.push(to);
        frames
    };

    for (i, &value) in frames_to_show.iter().enumerate() {
        let (progress, required, level) = cb(value);
        let cur_bar = format_progress_bar(progress, required, None, label);
        print!("{level}x {cur_bar}");

        if i != frames_to_show.len() - 1 {
            print!("\r");
        }

        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(FRAME_DELAY_MS));
    }
}

pub fn short_bar_outside_label(current: u128, max: u128, label: &str) -> String {
    let progress_bar = format_progress_bar(current, max, Some(25), None);
    format!("{label:>3} {progress_bar}")
}
