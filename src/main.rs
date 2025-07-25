mod passphrase;
mod words;

use std::io::IsTerminal;

use clap::Parser;
use eyre::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number of words in the passphrase
    #[arg(short, long, default_value = "4")]
    num_words: usize,

    /// The string to separate words in the passphrase
    #[arg(short, long, default_value = " ")]
    separator: String,

    /// A path to a dictionary file. A builtin dictionary is used if not provided.
    #[arg(short, long)]
    dict_path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut words: Vec<String> = words::list(args.dict_path)?;
    let mut rng = rand::rng();

    let passphrase = passphrase::new(&mut rng, &mut words, args.num_words, &args.separator)?;

    print!("{passphrase}");
    if std::io::stdout().is_terminal() {
        println!();
    }

    Ok(())
}
