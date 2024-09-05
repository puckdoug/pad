// When no length is provided, pad to the width of the longest item in the
// list. For this, need to scan the entire list
pub fn max_word_length(words: Vec<String>) -> usize {
    let mut max = 0;
    for word in words {
        if word.chars().count() > max {
            max = word.chars().count(); // len() is wrong if not utf-8
        }
    }
    max
}

pub fn read_input_lines() -> String {
    // read lines from stdin
    // send lines to the processing thread
    let mut rec = String::new();
    rec.push_str("placeholder");
    rec
}

pub fn process_lines(lines: Vec<String>) -> Vec<String> {
    // receive lines from the input thread
    // process the lines
    // output the result to the console
    let mut amended = Vec::new();
    for line in lines {
        amended.push(crate::lpad(line, &crate::Config::new()));
    }
    amended
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
            let max = max_word_length(lines);
            assert_eq!(5, max);
        }

        #[test]
        fn longest_non_utf8() {
            let mut lines = Vec::new();
            lines.push(String::from("one"));
            lines.push(String::from("two"));
            lines.push(String::from("ラウトは難しいです！"));
            let max = max_word_length(lines);
            assert_eq!(10, max);
        }

        // #[test]
        // #[ignore = "not yet implemented"]
        // fn test_read_input_lines() {
        //     assert_eq!(1, 1);
        // }

        // #[test]
        // #[ignore = "not yet implemented"]
        // fn test_process_lines() {
        //     let mut foo = String::new();
        //     foo.push_str("blah blah");
        //     let bar = process_lines(foo.clone());
        //     assert_eq!(foo, bar);
        // }
    }

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

        //     config.llen = max_word_length(lines.clone());

        //     let mut padded = process_lines(lines.clone());

        //     for line in padded {
        //         assert_eq!(config.llen, line.chars().count());
        //     }
        // }
    }
}
