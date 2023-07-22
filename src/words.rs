use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub(crate) fn words(path: Option<impl AsRef<Path>>) -> Vec<String> {
    match path {
        Some(path) => words_from_file(path),
        None => words_from_fixture(),
    }
}

fn words_from_fixture() -> Vec<String> {
    let bytes = include_bytes!("fixtures/words");
    let string = String::from_utf8_lossy(bytes);
    string.split('\n').map(|l| l.to_owned()).collect()
}

fn words_from_file(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}
