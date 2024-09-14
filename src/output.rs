// When no length is provided, pad to the width of the longest item in the
// list. For this, need to scan the entire list to find the length.
pub fn max_word_length(words: &Vec<String>) -> usize {
    let mut max = 0;
    for word in words {
        if word.chars().count() > max {
            max = word.chars().count(); // len() is wrong if not utf-8
        }
    }
    max
}

pub fn process_lines(mut lines: Vec<String>, config: &mut crate::Config) -> Vec<String> {
    if config.left {
        if config.llen == 0 {
            config.llen = max_word_length(&lines);
        }
        lines = crate::lpad_word_list(&lines, config);
    }

    if config.right {
        if config.rlen == 0 {
            config.rlen = max_word_length(&lines);
        }
        lines = crate::rpad_word_list(&lines, config)
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
