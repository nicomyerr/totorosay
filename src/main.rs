use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version)]
struct Args {
    text: String,
}

fn main() {
    let args = Args::parse();
    let text_bubble = text_bubble(args.text);
    let totoro = fs::read_to_string("resources/totoro.txt").expect("Unable to read file");
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
