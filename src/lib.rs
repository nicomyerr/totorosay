use std::fs;

pub fn totorosay(text: String, big: bool) -> String {
    let text_bubble = text_bubble(&text);
    let totoro = get_totoro_ascii(big);
    // TODO: why does it add a trailing newline at the end
    return format!("{}{}", text_bubble, totoro);
}

pub fn text_bubble(text: &str) -> String {
    // TODO: handle longer input with linebreaks
    let mut top = String::from(" __");
    let mut bot = String::from(" --");
    let mut i = 0;
    while i < text.len() {
        top.push('_');
        bot.push('-');
        i += 1;
    }
    return format!("{}\n< {} >\n{}\n", top, text, bot);
}

pub fn get_totoro_ascii(big: bool) -> String {
    let path = "resources/".to_string();
    let file = size_to_file(big);
    return fs::read_to_string(path + &file).expect("Unable to read file");
}

// TODO: private function
pub fn size_to_file(big: bool) -> String {
    return match big {
        true => "totoro-big.txt".to_string(),
        false => "totoro.txt".to_string(),
    };
}
