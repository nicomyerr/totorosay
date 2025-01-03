use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    text: String,
}

fn main() {
    let args = Args::parse();
    println!("Totoro says: {:?}", args.text);
}
