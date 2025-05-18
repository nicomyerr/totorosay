use std::fs;

pub fn totorosay(text: String, big: bool) -> String {
    let text_bubble: String = wrap_text_bubble(format_text(&text));
    let totoro: String = get_totoro_ascii(big);
    // TODO: why does it add a trailing newline at the end
    return format!("{}{}", text_bubble, totoro);
}

fn wrap_text_bubble(lines: Vec<String>) -> String {
    // TODO: refactor
    let max_len = max_len(&lines);
    let mut result = String::new();

    result.push_str(" ");
    result.push_str(&"_".repeat(max_len as usize + 2));
    result.push('\n');

    match lines.len() {
        0 => {}
        1 => {
            let line = &lines[0];
            let padding = max_len - line.len() as i32;
            result.push_str(&format!("< {}{} >\n", line, " ".repeat(padding as usize)));
        }
        2 => {
            let line1 = &lines[0];
            let line2 = &lines[1];
            result.push_str(&format!(
                "/ {}{} \\\n",
                line1,
                " ".repeat(max_len as usize - line1.len())
            ));
            result.push_str(&format!(
                "\\ {}{} /\n",
                line2,
                " ".repeat(max_len as usize - line2.len())
            ));
        }
        _ => {
            result.push_str(&format!(
                "/ {}{} \\\n",
                lines[0],
                " ".repeat(max_len as usize - lines[0].len())
            ));
            for line in &lines[1..lines.len() - 1] {
                result.push_str(&format!(
                    "| {}{} |\n",
                    line,
                    " ".repeat(max_len as usize - line.len())
                ));
            }
            let last = &lines[lines.len() - 1];
            result.push_str(&format!(
                "\\ {}{} /\n",
                last,
                " ".repeat(max_len as usize - last.len())
            ));
        }
    }

    result.push_str(" ");
    result.push_str(&"-".repeat(max_len as usize + 2));
    result.push('\n');

    return result;
}

fn max_len(lines: &Vec<String>) -> i32 {
    let mut max_len: i32 = 0;
    for line in lines.iter() {
        if line.len() as i32 > max_len {
            max_len = line.len() as i32
        }
    }
    return max_len;
}

fn format_text(text: &str) -> Vec<String> {
    // TODO: refactor
    const MAX_WIDTH: usize = 42;

    if text.len() <= MAX_WIDTH {
        return vec![text.to_string()];
    }

    let mut lines: Vec<String> = Vec::new();
    let mut remaining = text;

    while remaining.len() > MAX_WIDTH {
        let index = index_to_space_within(remaining, MAX_WIDTH);
        if index == -1 {
            let (line, rest) = remaining.split_at(MAX_WIDTH);
            lines.push(line.to_string());
            remaining = rest;
        } else {
            let split_index = index as usize;
            let (line, rest) = remaining.split_at(split_index);
            lines.push(line.to_string());
            remaining = &rest[1..];
        }
    }

    if !remaining.is_empty() {
        lines.push(remaining.to_string());
    }

    return lines;
}

fn index_to_space_within(text: &str, max_width: usize) -> i32 {
    let search_limit = max_width.min(text.len());

    for (i, ch) in text[..search_limit].char_indices().rev() {
        if ch == ' ' {
            return i as i32;
        }
    }

    return -1;
}

fn get_totoro_ascii(big: bool) -> String {
    let path: String = "resources/".to_string();
    let file: String = size_to_file(big);
    return fs::read_to_string(path + &file).expect("Unable to read file");
}

fn size_to_file(big: bool) -> String {
    return match big {
        true => "totoro-big.txt".to_string(),
        false => "totoro.txt".to_string(),
    };
}
