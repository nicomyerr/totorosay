use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Text Totoro should say
    text: Vec<String>,
    /// Use big Totoro
    #[arg(short, long)]
    big: bool,
}

fn main() {
    let args = Args::parse();
    let text = args.text.join(" ");
    let text_bubble = text_bubble(text);
    // TODO: refactoring
    // TODO: handle longer input with linebreaks
    let mut file = String::from("resources/totoro.txt");
    if args.big {
        file = file.replace("totoro", "totoro-big");
    }
    let totoro = fs::read_to_string(file).expect("Unable to read file");
    println!("{}{}", text_bubble, totoro);
}

fn text_bubble(text: String) -> String {
    let mut top = String::from(" __");
    let mut bot = String::from(" --");
    let mut i = 0;
    while i < text.len() {
        top.push('_');
        bot.push('-');
        i += 1;
    }
    return format!("{}\n< {} > \n{}\n", top, text, bot);
}
