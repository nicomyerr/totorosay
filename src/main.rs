use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(about = "cowsay but with totoroy")]
#[command(arg_required_else_help = true)]
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
    let text_bubble = totorosay::text_bubble(&text);

    let totoro = totorosay::get_totoro_ascii(args.big);

    println!("{}{}", text_bubble, totoro);
}
