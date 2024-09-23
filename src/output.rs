use crate::LR;

/// Check the maximum width of all words provided in the list. This is needed
/// in the default case when no width is provided. The program then assumes it
/// should pad to the width of the widest word. The negative in this case is
/// that it requires the entire set of words to be ingested before padding can
/// begin. This limits the size of a file which can be padded to one which can
/// be held in memory. If a size is provided the padding and ouput is done on
/// a separate thread running concurrently which can limit memory usage.
pub fn max_word_length(words: &Vec<String>) -> usize {
    let mut max = 0;
    for word in words {
        if word.chars().count() > max {
            max = word.chars().count(); // len() is wrong if not utf-8
        }
    }
    max
}

/// Process the lines of words provided. At the moment, this expects a single word
/// per line. I expect this may get adapted to handle multiple words in a line, but
/// have not yet considered in sufficient detail how that would work. First word, all
/// words, every second word? More combinations and the real use case for building
/// this was padding zeros left on a list of numbers. Whther the complex case is even
/// needed isn't clear.
pub fn process_lines(mut lines: Vec<String>, config: &mut crate::Config) -> Vec<String> {
    if (config.left && config.llen == 0) || (config.right && config.rlen == 0) {
        let length = max_word_length(&lines);
        if config.left && config.llen == 0 {
            config.llen = length;
        }
        if config.right && config.rlen == 0 {
            config.rlen = length;
        }
    }

    // possible to optimize these two into a single pass?
    if config.left {
        lines = crate::pad_word_list(&lines, config, LR::Left);
    }

    if config.right {
        lines = crate::pad_word_list(&lines, config, LR::Right);
    }

    lines
}

//------------------------------------------------------------------------------
// Unit Tests
//------------------------------------------------------------------------------
#[cfg(test)]
mod processing {
    use super::*;

    mod base {
        use super::*;

        #[test]
        fn longest_word() {
            let mut lines = Vec::new();
            lines.push(String::from("one"));
            lines.push(String::from("two"));
            lines.push(String::from("three"));
            lines.push(String::from("four"));
            lines.push(String::from("five"));
            let max = max_word_length(&lines);
            assert_eq!(5, max);
        }

        #[test]
        fn longest_non_utf8() {
            let mut lines = Vec::new();
            lines.push(String::from("one"));
            lines.push(String::from("two"));
            lines.push(String::from("ラウトは難しいです！"));
            let max = max_word_length(&lines);
            assert_eq!(10, max);
        }

        #[test]
        fn process_default_left_only() {
            let mut config = crate::Config::new();
            config.left = true;
            let mut lines = Vec::new();
            lines.push(String::from("one"));
            lines.push(String::from("two"));
            lines.push(String::from("three"));
            lines.push(String::from("four"));
            lines.push(String::from("five"));
            let amended = process_lines(lines.clone(), &mut config);
            assert_eq!(amended, vec!["00one", "00two", "three", "0four", "0five"]);
        }

        #[test]
        fn process_default_right_only() {
            let mut config = crate::Config::new();
            config.right = true;
            let mut lines = Vec::new();
            lines.push(String::from("one"));
            lines.push(String::from("two"));
            lines.push(String::from("three"));
            lines.push(String::from("four"));
            lines.push(String::from("five"));
            let amended = process_lines(lines.clone(), &mut config);
            assert_eq!(amended, vec!["one00", "two00", "three", "four0", "five0"]);
        }

        #[test]
        fn process_both() {
            let mut config = crate::Config::new();
            config.left = true;
            config.llen = 5;
            config.right = true;
            config.rlen = 7;
            let mut lines = Vec::new();
            lines.push(String::from("one"));
            lines.push(String::from("two"));
            lines.push(String::from("three"));
            lines.push(String::from("four"));
            lines.push(String::from("five"));
            let amended = process_lines(lines.clone(), &mut config);
            assert_eq!(
                amended,
                vec!["00one00", "00two00", "three00", "0four00", "0five00"]
            );
        }

        #[test]
        fn process_both_non_utf8() {
            let mut config = crate::Config::new();
            config.left = true;
            config.llen = 6;
            config.right = true;
            config.rlen = 10;
            let mut lines = Vec::new();
            lines.push(String::from("one"));
            lines.push(String::from("two"));
            lines.push(String::from("ラウトは難しいです！"));
            let amended = process_lines(lines.clone(), &mut config);
            assert_eq!(
                amended,
                vec!["000one0000", "000two0000", "ラウトは難しいです！"]
            );
        }
    }
}
