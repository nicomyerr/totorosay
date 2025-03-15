use std::fs;

pub fn totorosay(text: String, big: bool) -> String {
    let text_bubble: String = text_bubble(&text);
    let totoro: String = get_totoro_ascii(big);
    // TODO: why does it add a trailing newline at the end
    return format!("{}{}", text_bubble, totoro);
}

fn text_bubble(text: &str) -> String {
    // TODO: handle longer input with linebreaks
    let mut top: String = String::from(" __");
    let mut bot: String = String::from(" --");
    let mut i: usize = 0;
    while i < text.len() {
        top.push('_');
        bot.push('-');
        i += 1;
    }
    return format!("{}\n< {} >\n{}\n", top, text, bot);
}

fn get_totoro_ascii(big: bool) -> String {
    let path: String = "resources/".to_string();
    let file: String = size_to_file(big);
    return fs::read_to_string(path + &file).expect("Unable to read file");
}

// TODO: private function
fn size_to_file(big: bool) -> String {
    return match big {
        true => "totoro-big.txt".to_string(),
        false => "totoro.txt".to_string(),
    };
}
