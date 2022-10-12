mod words;

use clap::Parser;
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "4")]
    num_words: usize,

    #[arg(short, long, default_value = " ")]
    seperator: String,

    #[arg(short, long)]
    dict_path: Option<String>,
}

lazy_static! {
    static ref RE: Regex = Regex::new("^[a-z]{4,}$").unwrap();
}

fn main() {
    let args = Args::parse();

    let mut words: Vec<String> = words::words(args.dict_path)
        .iter()
        .filter(|w| RE.is_match(w))
        .map(|x| x.to_owned())
        .collect();

    let mut rng = rand::thread_rng();

    if words.len() < args.num_words {
        eprintln!(
            "Your dictionary only has {} suitable words, but you asked for {} words.",
            words.len(),
            args.num_words
        );
        return;
    }

    (0..args.num_words).for_each(|i| {
        let j = rng.gen_range(0..words.len());
        words.swap(i, j)
    });

    let passphrase = (0..args.num_words)
        .map(|i| (&words[i]).to_owned())
        .collect::<Vec<String>>()
        .join(&args.seperator);

    println!("{}", passphrase)
}
