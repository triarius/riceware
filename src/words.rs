use eyre::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub(crate) fn words(path: Option<impl AsRef<Path>>) -> Result<Vec<String>> {
    match path {
        Some(path) => words_from_file(path),
        None => words_from_fixture(),
    }
}

fn words_from_fixture() -> Result<Vec<String>> {
    let bytes = include_bytes!("fixtures/words");
    Ok(String::from_utf8_lossy(bytes)
        .split('\n')
        .map(|l| l.to_owned())
        .collect())
}

fn words_from_file(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect())
}
