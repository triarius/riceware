use eyre::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

lazy_static! {
    static ref RE: Regex = Regex::new("^[a-z]{4,}$").unwrap();
}

pub(crate) fn list(path: Option<impl AsRef<Path>>) -> Result<Vec<String>> {
    match path {
        Some(path) => words_from_file(path),
        None => words_from_fixture(),
    }
}

fn words_from_fixture() -> Result<Vec<String>> {
    let bytes = include_bytes!("fixtures/words");
    Ok(String::from_utf8_lossy(bytes)
        .split('\n')
        .filter(|w| RE.is_match(w))
        .map(|l| l.to_owned())
        .collect())
}

fn words_from_file(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file)
        .lines()
        .map_while(|l| l.ok())
        .filter(|w| RE.is_match(w))
        .collect())
}
