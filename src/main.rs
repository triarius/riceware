mod passphrase;
mod words;

use clap::Parser;
use eyre::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "4")]
    num_words: usize,

    #[arg(short, long, default_value = " ")]
    separator: String,

    #[arg(short, long)]
    dict_path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut words: Vec<String> = words::list(args.dict_path)?;
    let mut rng = rand::thread_rng();

    println!(
        "{}",
        passphrase::new(&mut rng, &mut words, args.num_words, &args.separator)
    );

    Ok(())
}
