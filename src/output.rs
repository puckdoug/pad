pub fn read_input_lines() -> String {
    // read lines from stdin
    // send lines to the processing thread
    let mut rec = String::new();
    rec.push_str("placeholder");
    rec
}

// When no length is provided, pad to the width of the longest item in the
// list. For this, need to scan the entire list
pub fn find_max_length(lines: Vec<String>) -> usize {
    let mut max = 0;
    for line in lines {
        if line.len() > max {
            max = line.len();
        }
    }
    max
}

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

pub fn process_lines(lines: Vec<String>) -> Vec<String> {
    // receive lines from the input thread
    // process the lines
    // output the result to the console
    let mut amended = Vec::new();
    for line in lines {
        amended.push(lpad(line, &crate::Config::new()));
    }
    amended
}

#[cfg(test)]
mod processing {

    use super::*;

    mod default {

        // use super::*;

        // #[test]
        // #[ignore = "not yet implemented"]
        // fn left_pad_no_options() {
        //     let mut config = crate::config::Config::new();
        //     config.left = true;

        //     let mut lines = Vec::new();
        //     lines.push(String::from("one"));
        //     lines.push(String::from("two"));
        //     lines.push(String::from("three"));
        //     lines.push(String::from("four"));
        //     lines.push(String::from("five"));

        //     config.llen = find_max_length(lines.clone());

        //     let mut padded = process_lines(lines.clone());

        //     for line in padded {
        //         assert_eq!(config.llen, line.len());
        //     }
        // }
    }

    mod base {
        use super::*;

        mod lpad {
            use super::*;

            #[test]
            fn lpad_one_word() {
                let mut config = crate::Config::new();
                config.llen = 5;
                let word = String::from("one");
                let padded = lpad(word, &config);
                assert_eq!(padded, "00one");
            }

            #[test]
            fn lpad_with_spaces() {
                let mut config = crate::Config::new();
                config.llen = 5;
                config.lpad = String::from(" ");
                let word = String::from("one");
                let padded = lpad(word, &config);
                assert_eq!(padded, "  one");
            }

            #[test]
            fn lpad_word_longer_than_pad() {
                let mut config = crate::Config::new();
                config.llen = 5;
                let word = String::from("longer");
                let padded = lpad(word, &config);
                assert_eq!(padded, "longer");
            }

            #[test]
            fn lpad_word_same_as_pad() {
                let mut config = crate::Config::new();
                config.llen = 5;
                let word = String::from("equal");
                let padded = lpad(word, &config);
                assert_eq!(padded, "equal");
            }
        }

        mod rpad {
            use super::*;

            #[test]
            fn rpad_one_word() {
                let mut config = crate::Config::new();
                config.llen = 5;
                let word = String::from("one");
                let padded = rpad(word, &config);
                assert_eq!(padded, "one00");
            }

            #[test]
            fn rpad_with_spaces() {
                let mut config = crate::Config::new();
                config.llen = 5;
                config.rpad = String::from(" ");
                let word = String::from("one");
                let padded = rpad(word, &config);
                assert_eq!(padded, "one  ");
            }

            #[test]
            fn rpad_word_longer_than_pad() {
                let mut config = crate::Config::new();
                config.llen = 5;
                let word = String::from("longer");
                let padded = rpad(word, &config);
                assert_eq!(padded, "longer");
            }

            #[test]
            fn rpad_word_same_as_pad() {
                let mut config = crate::Config::new();
                config.llen = 5;
                let word = String::from("equal");
                let padded = rpad(word, &config);
                assert_eq!(padded, "equal");
            }
        }

        #[test]
        #[ignore = "not yet implemented"]
        fn test_read_input_lines() {
            assert_eq!(1, 1);
        }

        #[test]
        fn test_find_max_length() {
            let mut lines = Vec::new();
            lines.push(String::from("one"));
            lines.push(String::from("two"));
            lines.push(String::from("three"));
            lines.push(String::from("four"));
            lines.push(String::from("five"));
            let max = find_max_length(lines);
            assert_eq!(5, max);
        }

        // #[test]
        // #[ignore = "not yet implemented"]
        // fn test_process_lines() {
        //     let mut foo = String::new();
        //     foo.push_str("blah blah");
        //     let bar = process_lines(foo.clone());
        //     assert_eq!(foo, bar);
        // }
    }
}
