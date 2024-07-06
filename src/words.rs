use eyre::Result;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::OnceLock,
};

fn re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new("^[a-z]{4,}$").unwrap())
}

pub(crate) fn list(path: Option<impl AsRef<Path>>) -> Result<Vec<String>> {
    match path {
        Some(path) => WordsFromFile { path }.words(),
        None => WordsFromFixture {}.words(),
    }
}

trait Words {
    fn words(&self) -> Result<Vec<String>>;
}

struct WordsFromFixture;

impl Words for WordsFromFixture {
    fn words(&self) -> Result<Vec<String>> {
        let bytes = include_bytes!("fixtures/words");
        Ok(String::from_utf8_lossy(bytes)
            .split('\n')
            .filter(|w| re().is_match(w))
            .map(std::borrow::ToOwned::to_owned)
            .collect())
    }
}

struct WordsFromFile<P: AsRef<Path>> {
    path: P,
}

impl<P: AsRef<Path>> Words for WordsFromFile<P> {
    fn words(&self) -> Result<Vec<String>> {
        let file = File::open(&self.path)?;
        Ok(BufReader::new(file)
            .lines()
            .map_while(std::result::Result::ok)
            .filter(|w| re().is_match(w))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_re() {
        let re = super::re();
        assert!(re.is_match("abcd"));
        assert!(!re.is_match("abc"));
        assert!(!re.is_match("abc!"));
        assert!(!re.is_match("1234"));
    }
}
