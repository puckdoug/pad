pub mod config;
pub mod output;
pub use crate::config::*;

/// the default string to use to pad words if none is specified
pub const DEFAULT_PAD: &str = "0";

/// LR is used to flag whether the upcoming arguments apply to left or right
/// padding or none, in which case they should be treated as tokens to pad.
#[derive(Debug, PartialEq)]
pub enum LR {
    Left,
    Right,
    None,
}

/// Pad a complete list of words. This is typically needed in case the width
/// is not known so that the complete list must be scanned to identify the
/// maximum width word.
pub fn pad_word_list(words: &Vec<String>, config: &crate::pad::Config, lr: LR) -> Vec<String> {
    let mut padded = Vec::new();
    for word in words {
        padded.push(pad(word, config, &lr));
    }
    padded
}

/// The pad function that does the real work. This operates per-word. Once
/// The Config is set, each word passed in will be returned padded according
/// to that configuration. This expects as arguments:
/// - word: &str - the word to be padded
/// - config: &crate::pad::Config - the configuration
/// - lr: &LR - the direction to pad (Left or Right). Passing None will return
///   the word unchanged.
pub fn pad(word: &str, config: &crate::pad::Config, lr: &LR) -> String {
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
    } else if *lr == LR::Right {
        // as long as the padding is more than the word length, pad
        if config.rlen > word.chars().count() {
            pad_str = config
                .rpad
                .clone()
                .repeat(config.rlen - word.chars().count());
        }
        padded = String::from(word);
        padded.push_str(&pad_str);
    } else {
        padded = String::from(word);
    }
    padded
}

//------------------------------------------------------------------------------
// Unit Tests
//------------------------------------------------------------------------------
#[cfg(test)]
mod lpad {

    use crate::pad::pad;
    use crate::pad::LR;

    #[test]
    fn one() {
        let mut config = crate::pad::Config::new();
        config.llen = 5;
        let word = String::from("one");
        let padded = pad(&word, &config, &LR::Left);
        assert_eq!("00one", padded);
    }

    #[test]
    fn with_spaces() {
        let mut config = crate::pad::Config::new();
        config.llen = 5;
        config.lpad = String::from(" ");
        let word = String::from("one");
        let padded = pad(&word, &config, &LR::Left);
        assert_eq!("  one", padded);
    }

    #[test]
    fn longer_than_pad() {
        let mut config = crate::pad::Config::new();
        config.llen = 5;
        let word = String::from("longer");
        let padded = pad(&word, &config, &LR::Left);
        assert_eq!("longer", padded);
    }

    #[test]
    fn same_as_pad() {
        let mut config = crate::pad::Config::new();
        config.llen = 5;
        let word = String::from("equal");
        let padded = pad(&word, &config, &LR::Left);
        assert_eq!("equal", padded);
    }

    #[test]
    fn non_utf8() {
        let mut config = crate::pad::Config::new();
        config.llen = 12;
        let word = String::from("ラウトは難しいです！");
        let padded = pad(&word, &config, &LR::Left);
        assert_eq!("00ラウトは難しいです！", padded)
    }

    #[test]
    fn five_words() {
        let mut words = Vec::new();
        words.push(String::from("one"));
        words.push(String::from("two"));
        words.push(String::from("three"));
        words.push(String::from("four"));
        words.push(String::from("five"));
        let mut config = crate::pad::Config::new();
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
        let mut config = crate::pad::Config::new();
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
#[cfg(test)]
mod rpad {

    use crate::pad::pad;
    use crate::pad::LR;

    #[test]
    fn one() {
        let mut config = crate::pad::Config::new();
        config.rlen = 5;
        let word = String::from("one");
        let padded = pad(&word, &config, &LR::Right);
        assert_eq!("one00", padded);
    }

    #[test]
    fn with_spaces() {
        let mut config = crate::pad::Config::new();
        config.rlen = 5;
        config.rpad = String::from(" ");
        let word = String::from("one");
        let padded = pad(&word, &config, &LR::Right);
        assert_eq!("one  ", padded);
    }

    #[test]
    fn longer_than_pad() {
        let mut config = crate::pad::Config::new();
        config.rlen = 5;
        let word = String::from("longer");
        let padded = pad(&word, &config, &LR::Right);
        assert_eq!("longer", padded);
    }

    #[test]
    fn same_as_pad() {
        let mut config = crate::pad::Config::new();
        config.rlen = 5;
        let word = String::from("equal");
        let padded = pad(&word, &config, &LR::Right);
        assert_eq!("equal", padded);
    }

    #[test]
    fn non_utf8() {
        let mut config = crate::pad::Config::new();
        config.rlen = 12;
        let word = String::from("ラウトは難しいです！");
        let padded = pad(&word, &config, &LR::Right);
        assert_eq!("ラウトは難しいです！00", padded)
    }

    #[test]
    fn five_words() {
        let mut words = Vec::new();
        words.push(String::from("one"));
        words.push(String::from("two"));
        words.push(String::from("three"));
        words.push(String::from("four"));
        words.push(String::from("five"));
        let mut config = crate::pad::Config::new();
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
        let mut config = crate::pad::Config::new();
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

#[cfg(test)]
mod other {
    use crate::pad::config::*;
    use crate::pad::pad;
    use crate::pad::LR;

    #[test]
    fn no_pad_direction_returns_word() {
        let mut config = Config::new();
        config.rlen = 5;
        let word = String::from("one");
        let padded = pad(&word, &config, &LR::None);
        assert_eq!("one", padded);
    }
}
