mod words;

use clap::Parser;
use eyre::Result;
use lazy_static::lazy_static;
use rand::{rngs::ThreadRng, Rng};
use regex::Regex;

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

lazy_static! {
    static ref RE: Regex = Regex::new("^[a-z]{4,}$").unwrap();
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut words: Vec<String> = words::words(args.dict_path)?
        .iter()
        .filter(|w| RE.is_match(w))
        .map(|x| x.to_owned())
        .collect();

    let mut rng = rand::thread_rng();

    println!(
        "{}",
        passphrase(&mut rng, &mut words, args.num_words, &args.separator)
    );

    Ok(())
}

fn passphrase(
    rng: &mut ThreadRng,
    words: &mut Vec<String>,
    num_words: usize,
    separator: &str,
) -> String {
    if words.len() < num_words {
        eprintln!(
            "Your dictionary only has {} suitable words, but you asked for {} words.",
            words.len(),
            num_words
        );
        return "".to_string();
    }

    (0..num_words).for_each(|i| {
        let j = rng.gen_range(0..words.len());
        words.swap(i, j)
    });

    (0..num_words)
        .map(|i| words[i].to_owned())
        .collect::<Vec<String>>()
        .join(separator)
}
