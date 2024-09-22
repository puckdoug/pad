use crate::LR;

pub fn pad_word_list(words: &Vec<String>, config: &crate::Config, lr: LR) -> Vec<String> {
    let mut padded = Vec::new();
    for word in words {
        padded.push(pad(word, config, &lr));
    }
    padded
}

pub fn pad(word: &str, config: &crate::Config, lr: &LR) -> String {
    // pad the word to the length of the longest word
    let mut pad_str = String::new();
    let mut padded;

    if *lr == LR::Left {
        // as long as the padding is more than the word length, pad
        if config.llen > word.chars().count() {
            pad_str = config
                .lpad
                .clone()
                .repeat(config.llen - word.chars().count());
        }
        pad_str.push_str(word);
        padded = pad_str;
    } else {
        // as long as the padding is more than the word length, pad
        if config.rlen > word.chars().count() {
            pad_str = config
                .rpad
                .clone()
                .repeat(config.rlen - word.chars().count());
        }
        padded = String::from(word);
        padded.push_str(&pad_str);
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
            let padded = pad(&word, &config, &LR::Left);
            assert_eq!("00one", padded);
        }

        #[test]
        fn with_spaces() {
            let mut config = crate::Config::new();
            config.llen = 5;
            config.lpad = String::from(" ");
            let word = String::from("one");
            let padded = pad(&word, &config, &LR::Left);
            assert_eq!("  one", padded);
        }

        #[test]
        fn longer_than_pad() {
            let mut config = crate::Config::new();
            config.llen = 5;
            let word = String::from("longer");
            let padded = pad(&word, &config, &LR::Left);
            assert_eq!("longer", padded);
        }

        #[test]
        fn same_as_pad() {
            let mut config = crate::Config::new();
            config.llen = 5;
            let word = String::from("equal");
            let padded = pad(&word, &config, &LR::Left);
            assert_eq!("equal", padded);
        }

        #[test]
        fn non_utf8() {
            let mut config = crate::Config::new();
            config.llen = 12;
            let word = String::from("ラウトは難しいです！");
            let padded = pad(&word, &config, &LR::Left);
            assert_eq!("00ラウトは難しいです！", padded)
        }
    }

    mod word_list {
        use crate::LR;

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
            let padded = crate::pad_word_list(&words, &config, LR::Left);
            assert_eq!("00one", padded[0]);
            assert_eq!("00two", padded[1]);
            assert_eq!("three", padded[2]);
            assert_eq!("0four", padded[3]);
            assert_eq!("0five", padded[4]);
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
            let padded = crate::pad_word_list(&words, &config, LR::Left);
            assert_eq!("  one", padded[0]);
            assert_eq!("  two", padded[1]);
            assert_eq!("three", padded[2]);
            assert_eq!(" four", padded[3]);
            assert_eq!(" five", padded[4]);
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
            let padded = pad(&word, &config, &LR::Right);
            assert_eq!("one00", padded);
        }

        #[test]
        fn with_spaces() {
            let mut config = crate::Config::new();
            config.rlen = 5;
            config.rpad = String::from(" ");
            let word = String::from("one");
            let padded = pad(&word, &config, &LR::Right);
            assert_eq!("one  ", padded);
        }

        #[test]
        fn longer_than_pad() {
            let mut config = crate::Config::new();
            config.rlen = 5;
            let word = String::from("longer");
            let padded = pad(&word, &config, &LR::Right);
            assert_eq!("longer", padded);
        }

        #[test]
        fn same_as_pad() {
            let mut config = crate::Config::new();
            config.rlen = 5;
            let word = String::from("equal");
            let padded = pad(&word, &config, &LR::Right);
            assert_eq!("equal", padded);
        }

        #[test]
        fn non_utf8() {
            let mut config = crate::Config::new();
            config.rlen = 12;
            let word = String::from("ラウトは難しいです！");
            let padded = pad(&word, &config, &LR::Right);
            assert_eq!("ラウトは難しいです！00", padded)
        }
    }

    mod word_list {
        use crate::LR;

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
            let padded = crate::pad_word_list(&words, &config, LR::Right);
            assert_eq!("one00", padded[0]);
            assert_eq!("two00", padded[1]);
            assert_eq!("three", padded[2]);
            assert_eq!("four0", padded[3]);
            assert_eq!("five0", padded[4]);
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
            let padded = crate::pad_word_list(&words, &config, LR::Right);
            assert_eq!("one  ", padded[0]);
            assert_eq!("two  ", padded[1]);
            assert_eq!("three", padded[2]);
            assert_eq!("four ", padded[3]);
            assert_eq!("five ", padded[4]);
        }
    }
}
