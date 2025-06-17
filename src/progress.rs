use std::io::{self, Write};

pub fn print_progress_bar(current: usize, max: usize, width: Option<usize>, label: Option<&str>) {
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

    let filled_part: String = filled_char.to_string().repeat(filled_width);
    let empty_part: String = empty_char.to_string().repeat(bar_width - filled_width);

    print!(
        "{} [{}{}] {:.1}% ({}/{})",
        label_text, filled_part, empty_part, percentage, current, max
    );

    io::stdout().flush().unwrap();
}

#[allow(dead_code)]
pub fn progress_bar(current: usize, max: usize) {
    print_progress_bar(current, max, None, None);
}

pub fn progress_bar_with_label(current: usize, max: usize, label: &str) {
    print_progress_bar(current, max, None, Some(label));
}
