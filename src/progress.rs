use std::io::{self, Write};

pub fn format_progress_bar(
    current: u32,
    max: u32,
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
        (current * bar_width) / max
    };

    let filled_char = '█';
    let empty_char = '░';

    let filled_part: String = filled_char.to_string().repeat(filled_width as usize);
    let empty_part: String = empty_char
        .to_string()
        .repeat(bar_width.wrapping_sub(filled_width) as usize);

    format!(
        "{}{}{} {:>4.1}% ({}/{})",
        label_text, filled_part, empty_part, percentage, current, max
    )
}

pub fn animated_progress_bar(
    from: u32,
    to: u32,
    label: Option<&str>,
    cb: fn(total_xp: u32) -> (u32, u32, u32),
) {
    let frame_count = to - from;

    let base_frame_delay = 10; // ms per frame
    let natural_duration = frame_count * base_frame_delay;
    let max_duration = 1000; // 1 second max

    let frame_delay = if natural_duration <= max_duration {
        base_frame_delay
    } else {
        max_duration / frame_count
    };

    for i in from..=to {
        let (progress, required, level) = cb(i);
        let cur_bar = format_progress_bar(progress, required, None, label);
        print!("{}x {}", level, cur_bar);
        if i != to {
            print!("\r");
        }
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(frame_delay as u64));
    }
}

pub fn short_bar_outside_label(current: u32, max: u32, label: &str) -> String {
    let progress_bar = format_progress_bar(current, max, Some(25), None);
    format!("{:>3} {}", label, progress_bar)
}
