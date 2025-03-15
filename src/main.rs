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
    let args: Args = Args::parse();
    let text: String = args.text.join(" ");
    print!("{}", totorosay::totorosay(text, args.big));
}
