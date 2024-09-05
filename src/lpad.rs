pub fn lpad(word: String, config: &crate::Config) -> String {
    // pad the word to the length of the longest word
    let mut pad_str = String::new();
    // as long as the padding is more than the word length, pad
    if config.llen > word.len() {
        pad_str = std::iter::repeat(config.lpad.clone())
            .take(config.llen - word.len())
            .collect::<String>();
    }
    pad_str.push_str(&word);
    pad_str
}

mod word {
    use super::*;

    #[test]
    fn one() {
        let mut config = crate::Config::new();
        config.llen = 5;
        let word = String::from("one");
        let padded = lpad(word, &config);
        assert_eq!(padded, "00one");
    }

    #[test]
    fn with_spaces() {
        let mut config = crate::Config::new();
        config.llen = 5;
        config.lpad = String::from(" ");
        let word = String::from("one");
        let padded = lpad(word, &config);
        assert_eq!(padded, "  one");
    }

    #[test]
    fn longer_than_pad() {
        let mut config = crate::Config::new();
        config.llen = 5;
        let word = String::from("longer");
        let padded = lpad(word, &config);
        assert_eq!(padded, "longer");
    }

    #[test]
    fn same_as_pad() {
        let mut config = crate::Config::new();
        config.llen = 5;
        let word = String::from("equal");
        let padded = lpad(word, &config);
        assert_eq!(padded, "equal");
    }
}
