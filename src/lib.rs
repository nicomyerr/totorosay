const MAX_WIDTH: usize = 42;

const TOTORO: &[u8] = include_bytes!("../resources/totoro.txt");
const TOTORO_BIG: &[u8] = include_bytes!("../resources/totoro-big.txt");

pub fn totorosay(text: String, big: bool) -> String {
    let text_bubble = wrap_lines_into_speech_bubble(split_text_into_lines(&text));
    let totoro = get_totoro_ascii(big);
    return format!("{}{}", text_bubble, totoro);
}

fn wrap_lines_into_speech_bubble(lines: Vec<String>) -> String {
    let max_len = max_len(&lines);
    let top_border = draw_horizontal_border(max_len, '_');
    let content = wrap_content(lines, max_len);
    let bottom_border = draw_horizontal_border(max_len, '-');
    return format!("{}{}{}", top_border, content, bottom_border);
}

fn draw_horizontal_border(iterations: usize, border_char: char) -> String {
    return format!(" {}\n", border_char.to_string().repeat(iterations + 2));
}

fn wrap_content(lines: Vec<String>, max_len: usize) -> String {
    return match lines.len() {
        0 => String::new(),
        1 => draw_single_line(&lines[0], max_len),
        2 => draw_two_lines(&lines[0], &lines[1], max_len),
        _ => draw_multiple_lines(&lines, max_len),
    };
}

fn draw_single_line(line: &str, max_len: usize) -> String {
    return format_padded_line(line, max_len, "<", ">");
}

fn draw_two_lines(first_line: &str, second_line: &str, max_len: usize) -> String {
    let first_line = format_padded_line(first_line, max_len, "/", "\\");
    let second_line = format_padded_line(second_line, max_len, "\\", "/");
    return format!("{}{}", first_line, second_line);
}

fn draw_multiple_lines(lines: &[String], max_len: usize) -> String {
    let mut result = String::with_capacity(lines.len() * (max_len + 4));

    result.push_str(&format_padded_line(&lines[0], max_len, "/", "\\"));

    for line in &lines[1..lines.len() - 1] {
        result.push_str(&format_padded_line(line, max_len, "|", "|"));
    }

    result.push_str(&format_padded_line(
        &lines[lines.len() - 1],
        max_len,
        "\\",
        "/",
    ));

    return result;
}

fn format_padded_line(line: &str, max_len: usize, left_border: &str, right_border: &str) -> String {
    let padding = " ".repeat(max_len - line.len());
    return format!("{} {}{} {}\n", left_border, line, padding, right_border);
}

fn max_len(lines: &Vec<String>) -> usize {
    return lines.iter().map(|line| line.len()).max().unwrap_or(0);
}

fn split_text_into_lines(text: &str) -> Vec<String> {
    // TODO: refactor
    if text.len() <= MAX_WIDTH {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut remaining = text;

    while remaining.len() > MAX_WIDTH {
        match find_last_space_within(remaining, MAX_WIDTH) {
            Some(index) => {
                let (line, rest) = remaining.split_at(index);
                lines.push(line.to_string());
                remaining = &rest[1..];
            }
            None => {
                let (line, rest) = remaining.split_at(MAX_WIDTH);
                lines.push(line.to_string());
                remaining = rest;
            }
        }
    }

    if !remaining.is_empty() {
        lines.push(remaining.to_string());
    }

    return lines;
}

fn find_last_space_within(text: &str, max_width: usize) -> Option<usize> {
    return text[..max_width.min(text.len())].rfind(' ');
}

fn get_totoro_ascii(big: bool) -> String {
    let bytes = match big {
        true => TOTORO_BIG,
        false => TOTORO,
    };

    return std::str::from_utf8(bytes)
        .expect("Unable to read file")
        .to_string();
}
