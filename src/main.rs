use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version)]
struct Args {
    text: String,
}

fn main() {
    let args = Args::parse();
    let text_bubble = format!("________________\n< {} >\n----------------\n", args.text);
    let totoro = fs::read_to_string("resources/totoro.txt").expect("Unable to read file");
    println!("{}{}", text_bubble, totoro);
}
