pub static ASCII_NUMBERS: [&str; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

pub fn display_number_x(number: u32) -> String {
    // number -> string -> vector of chars -> char to digit -> index lookup
    let mut digits = number
        .to_string()
        .chars()
        .map(|d| {
            let char = d.to_digit(10).unwrap() as usize;
            ASCII_NUMBERS[char]
        })
        .collect::<Vec<_>>();
    digits.push(X);
    display_side_by_side(&digits, 0)
}

fn display_side_by_side(ascii_arts: &[&str], spacing: usize) -> String {
    // Split each ASCII art into lines and collect them
    let all_lines: Vec<Vec<&str>> = ascii_arts.iter().map(|art| art.lines().collect()).collect();

    // Make sure the "height" (lines) of each character match
    let max_lines = all_lines.iter().map(|lines| lines.len()).max().unwrap_or(0);

    let mut result = String::new();

    // For each line position (moving down from top of character)
    for line_idx in 0..max_lines {
        // format first character's line position, then append the next character's, and so on
        for (art_idx, lines) in all_lines.iter().enumerate() {
            let line = lines.get(line_idx).unwrap_or(&"");

            let trimmed_line = line.trim_end();
            let padded_line = format!("{:<width$}", trimmed_line, width = 5);
            result.push_str(&padded_line);

            // Add spacing between ASCII arts (except for the last one)
            if art_idx < all_lines.len() - 1 {
                result.push_str(&" ".repeat(spacing));
            }
        }
        result.push('\n');
    }

    result
}

pub static ZERO: &str = r#"
 ▗▄▖
 █▀█
▐▌ ▐▌
▐▌█▐▌
▐▌ ▐▌
 █▄█
 ▝▀▘
"#;
pub static ONE: &str = r#"
 ▗▄
 ▛█
  █
  █
  █
▗▄█▄▖
▝▀▀▀▘
"#;

pub static TWO: &str = r#"
 ▄▄▖
▐▀▀█▖
   ▐▌
  ▗▛
 ▗▛
▗█▄▄▖
▝▀▀▀▘
"#;

pub static THREE: &str = r#"
 ▄▄▖
▐▀▀█▖
   ▟▌
 ▐██
   ▜▌
▐▄▄█▘
 ▀▀▘
"#;

pub static FOUR: &str = r#"
  ▗▄
  ▟█
 ▐▘█
▗▛ █
▐███▌
   █
   ▀
"#;

pub static FIVE: &str = r#"
▗▄▄▄
▐▛▀▀
▐▙▄▖
▐▀▀█▖
   ▐▌
▐▄▄█▘
 ▀▀▘
"#;

pub static SIX: &str = r#"
 ▗▄▖
 █▀▜
▐▌▄▖
▐█▀█▖
▐▌ ▐▌
▝█▄█▘
 ▝▀▘
"#;

pub static SEVEN: &str = r#"
▗▄▄▄▖
▝▀▀█▌
  ▗█
  ▐▌
  █
 ▐▌
 ▀
"#;

pub static EIGHT: &str = r#"
 ▗▄▖
▗█▀█▖
▐▙ ▟▌
 ███
▐▛ ▜▌
▝█▄█▘
 ▝▀▘
"#;

pub static NINE: &str = r#"
 ▗▄▖
▗█▀█▖
▐▌ ▐▌
▝█▄█▌
 ▝▀▐▌
 ▙▄█
 ▝▀▘
"#;

pub static X: &str = r#"


▝█ █▘
 ▐█▌
 ▗█▖
 ▟▀▙
▝▀ ▀▘
"#;
