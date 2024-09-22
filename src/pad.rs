pub fn lpad(word: &str, config: &crate::Config) -> String {
    // pad the word to the length of the longest word
    let mut pad_str = String::new();
    // as long as the padding is more than the word length, pad
    if config.llen > word.chars().count() {
        pad_str = config
            .lpad
            .clone()
            .repeat(config.llen - word.chars().count());
    }
    pad_str.push_str(word);
    pad_str
}

pub fn lpad_word_list(words: &Vec<String>, config: &crate::Config) -> Vec<String> {
    let mut padded = Vec::new();
    for word in words {
        padded.push(lpad(word, config));
    }
    padded
}

pub fn rpad(word: &str, config: &crate::Config) -> String {
    // pad the word to the length of the longest word
    let mut pad_str = String::new();
    // as long as the padding is more than the word length, pad
    if config.rlen > word.chars().count() {
        pad_str = config
            .rpad
            .clone()
            .repeat(config.rlen - word.chars().count());
    }
    let mut rpadded = String::from(word);
    rpadded.push_str(&pad_str);
    rpadded
}

pub fn rpad_word_list(words: &Vec<String>, config: &crate::Config) -> Vec<String> {
    let mut padded = Vec::new();
    for word in words {
        padded.push(rpad(word, config));
    }
    padded
}

//------------------------------------------------------------------------------
// Unit Tests
//------------------------------------------------------------------------------
#[cfg(test)]
mod lpad {
    use super::*;
    mod word {
        use super::*;

        #[test]
        fn one() {
            let mut config = crate::Config::new();
            config.llen = 5;
            let word = String::from("one");
            let padded = lpad(&word, &config);
            assert_eq!(padded, "00one");
        }

        #[test]
        fn with_spaces() {
            let mut config = crate::Config::new();
            config.llen = 5;
            config.lpad = String::from(" ");
            let word = String::from("one");
            let padded = lpad(&word, &config);
            assert_eq!(padded, "  one");
        }

        #[test]
        fn longer_than_pad() {
            let mut config = crate::Config::new();
            config.llen = 5;
            let word = String::from("longer");
            let padded = lpad(&word, &config);
            assert_eq!(padded, "longer");
        }

        #[test]
        fn same_as_pad() {
            let mut config = crate::Config::new();
            config.llen = 5;
            let word = String::from("equal");
            let padded = lpad(&word, &config);
            assert_eq!(padded, "equal");
        }

        #[test]
        fn non_utf8() {
            let mut config = crate::Config::new();
            config.llen = 12;
            let word = String::from("ラウトは難しいです！");
            let padded = lpad(&word, &config);
            assert_eq!(padded, "00ラウトは難しいです！")
        }
    }

    mod word_list {
        #[test]
        fn five_words() {
            let mut words = Vec::new();
            words.push(String::from("one"));
            words.push(String::from("two"));
            words.push(String::from("three"));
            words.push(String::from("four"));
            words.push(String::from("five"));
            let mut config = crate::Config::new();
            config.llen = 5;
            let padded = crate::lpad_word_list(&words, &config);
            assert_eq!(padded[0], "00one");
            assert_eq!(padded[1], "00two");
            assert_eq!(padded[2], "three");
            assert_eq!(padded[3], "0four");
            assert_eq!(padded[4], "0five");
        }
        #[test]
        fn five_words_with_space() {
            let mut words = Vec::new();
            words.push(String::from("one"));
            words.push(String::from("two"));
            words.push(String::from("three"));
            words.push(String::from("four"));
            words.push(String::from("five"));
            let mut config = crate::Config::new();
            config.llen = 5;
            config.lpad = String::from(" ");
            let padded = crate::lpad_word_list(&words, &config);
            assert_eq!(padded[0], "  one");
            assert_eq!(padded[1], "  two");
            assert_eq!(padded[2], "three");
            assert_eq!(padded[3], " four");
            assert_eq!(padded[4], " five");
        }
    }
}

mod rpad {
    use super::*;

    mod word {
        use super::*;

        #[test]
        fn one() {
            let mut config = crate::Config::new();
            config.rlen = 5;
            let word = String::from("one");
            let padded = rpad(&word, &config);
            assert_eq!(padded, "one00");
        }

        #[test]
        fn with_spaces() {
            let mut config = crate::Config::new();
            config.rlen = 5;
            config.rpad = String::from(" ");
            let word = String::from("one");
            let padded = rpad(&word, &config);
            assert_eq!(padded, "one  ");
        }

        #[test]
        fn longer_than_pad() {
            let mut config = crate::Config::new();
            config.rlen = 5;
            let word = String::from("longer");
            let padded = rpad(&word, &config);
            assert_eq!(padded, "longer");
        }

        #[test]
        fn same_as_pad() {
            let mut config = crate::Config::new();
            config.rlen = 5;
            let word = String::from("equal");
            let padded = rpad(&word, &config);
            assert_eq!(padded, "equal");
        }

        #[test]
        fn non_utf8() {
            let mut config = crate::Config::new();
            config.rlen = 12;
            let word = String::from("ラウトは難しいです！");
            let padded = rpad(&word, &config);
            assert_eq!(padded, "ラウトは難しいです！00")
        }
    }

    mod word_list {
        #[test]
        fn five_words() {
            let mut words = Vec::new();
            words.push(String::from("one"));
            words.push(String::from("two"));
            words.push(String::from("three"));
            words.push(String::from("four"));
            words.push(String::from("five"));
            let mut config = crate::Config::new();
            config.rlen = 5;
            let padded = crate::rpad_word_list(&words, &config);
            assert_eq!(padded[0], "one00");
            assert_eq!(padded[1], "two00");
            assert_eq!(padded[2], "three");
            assert_eq!(padded[3], "four0");
            assert_eq!(padded[4], "five0");
        }
        #[test]
        fn five_words_with_space() {
            let mut words = Vec::new();
            words.push(String::from("one"));
            words.push(String::from("two"));
            words.push(String::from("three"));
            words.push(String::from("four"));
            words.push(String::from("five"));
            let mut config = crate::Config::new();
            config.rlen = 5;
            config.rpad = String::from(" ");
            let padded = crate::rpad_word_list(&words, &config);
            assert_eq!(padded[0], "one  ");
            assert_eq!(padded[1], "two  ");
            assert_eq!(padded[2], "three");
            assert_eq!(padded[3], "four ");
            assert_eq!(padded[4], "five ");
        }
    }
}
