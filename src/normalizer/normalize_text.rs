extern crate regex;
use regex::Regex;

pub fn normalize_text(text: &'static str) -> String {
    let mut text = text.to_lowercase();
    let re = Regex::new(r"[^a-z0-9 -]").unwrap();
    text = re.replace_all(&text, " ").to_string();
    let re = Regex::new(r"( +)").unwrap();
    re.replace_all(&text, " ").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_text_positive() {
        let input = "hello &()%& world";
        let expectation = "hello world";

        assert_eq!(
            normalize_text(&input),
            expectation
        )
    }
}