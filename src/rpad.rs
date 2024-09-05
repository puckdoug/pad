pub fn rpad(word: String, config: &crate::Config) -> String {
    // pad the word to the length of the longest word
    let mut pad_str = String::new();
    // as long as the padding is more than the word length, pad
    if config.llen > word.len() {
        pad_str = std::iter::repeat(config.rpad.clone())
            .take(config.llen - word.len())
            .collect::<String>();
    }
    let mut rpadded = String::from(&word);
    rpadded.push_str(&pad_str);
    rpadded
}

mod word {
    use super::*;

    #[test]
    fn one() {
        let mut config = crate::Config::new();
        config.llen = 5;
        let word = String::from("one");
        let padded = rpad(word, &config);
        assert_eq!(padded, "one00");
    }

    #[test]
    fn with_spaces() {
        let mut config = crate::Config::new();
        config.llen = 5;
        config.rpad = String::from(" ");
        let word = String::from("one");
        let padded = rpad(word, &config);
        assert_eq!(padded, "one  ");
    }

    #[test]
    fn longer_than_pad() {
        let mut config = crate::Config::new();
        config.llen = 5;
        let word = String::from("longer");
        let padded = rpad(word, &config);
        assert_eq!(padded, "longer");
    }

    #[test]
    fn same_as_pad() {
        let mut config = crate::Config::new();
        config.llen = 5;
        let word = String::from("equal");
        let padded = rpad(word, &config);
        assert_eq!(padded, "equal");
    }
}
