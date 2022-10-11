use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub(crate) fn words(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}
