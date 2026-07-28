const SAMPLE: &str = "\
the quick brown fox
jumps over the lazy dog
The Quick Brown Fox
a line with no animals at all
fox fox fox
";

pub fn search_pattern<'a>(pattern: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in haystack.lines() {
        if contains_naive(pattern, line) {
            result.push(line);
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
        // todo!("compare the m bytes at position i against p; return true if equal")
    }

    false
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_haystack() {
       let expected = Vec::<&str>::new();
       assert_eq!(search_pattern("fox", " "), expected);
    }

    #[test]
    fn truly_empty_haystack() {
       let expected = Vec::<&str>::new();
       assert_eq!(search_pattern("fox", ""), expected);
    }
    
    #[test]
    fn empty_pattern() {
       let mut expected = Vec::<&str>::new();
       expected.push("the quick brown fox");
       expected.push("jumps over the lazy dog");
       expected.push("The Quick Brown Fox");
       expected.push("a line with no animals at all");
       expected.push("fox fox fox");

       assert_eq!(search_pattern("", SAMPLE), expected);
    }

    #[test]
    fn find_pattern() {
       let mut expected = Vec::<&str>::new();
       expected.push("the quick brown fox");
       expected.push("fox fox fox");
       assert_eq!(search_pattern("fox", SAMPLE), expected);
    }

    #[test]
    fn longer_pattern() {
       let expected = Vec::<&str>::new();
       assert_eq!(search_pattern("elephantine", "fox"), expected);
    }
}
