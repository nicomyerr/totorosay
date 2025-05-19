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
    // TODO: refactor
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
    let path: String = "resources/".to_string();
    let file: String = size_to_file(big);
    return fs::read_to_string(path + &file).expect("Unable to read file");
}

fn size_to_file(big: bool) -> String {
    // TODO: Filenames -> const
    // TODO: bool -> enum (mapping function)
    return match big {
        true => "totoro-big.txt".to_string(),
        false => "totoro.txt".to_string(),
    };
}
