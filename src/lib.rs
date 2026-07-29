use std::path::{PathBuf};
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// pattern to search for
    pub pattern: String,
    /// file to search in
    pub path: PathBuf
}

pub fn search_pattern<'a>(pattern: &str, haystack: &'a str) -> Vec<(usize, &'a str)> {
    let mut result = Vec::new();
    for (i, line) in haystack.lines().enumerate() {
        if contains_naive(pattern, line) {
            let i = i + 1; 
            result.push((i, line));
        }
    }

    result
}

fn contains_naive(pattern: &str, haystack: &str) -> bool {
    let p = pattern.as_bytes();
    let h = haystack.as_bytes();

    if p.len() > h.len() {
        return false;
    }

    for i in 0..=(h.len() - p.len()) {
        if &h[i..i + p.len()] == p {
            return true;
        }
    }

    false
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
the quick brown fox
jumps over the lazy dog
The Quick Brown Fox
a line with no animals at all
fox fox fox
";
    #[test]
    fn empty_haystack() {
        let expected = Vec::<(usize, &str)>::new();
        assert_eq!(search_pattern("fox", " "), expected);
    }

    #[test]
    fn truly_empty_haystack() {
        let expected = Vec::<(usize, &str)>::new();
        assert_eq!(search_pattern("fox", ""), expected);
    }

    #[test]
    fn empty_pattern() {
        let expected = vec![
            (1, "the quick brown fox"),
            (2, "jumps over the lazy dog"),
            (3, "The Quick Brown Fox"),
            (4, "a line with no animals at all"),
            (5, "fox fox fox"),
        ];
        assert_eq!(search_pattern("", SAMPLE), expected);
    }

    #[test]
    fn find_pattern() {
        let expected = vec![(1, "the quick brown fox"), (5, "fox fox fox")];
        assert_eq!(search_pattern("fox", SAMPLE), expected);
    }

    #[test]
    fn longer_pattern() {
        let expected = Vec::<(usize, &str)>::new();
        assert_eq!(search_pattern("elephantine", "fox"), expected);
    }
}
