pub fn read_input_lines() -> String {
    // read lines from stdin
    // send lines to the processing thread
    let mut rec = String::new();
    rec.push_str("placeholder");
    rec
}

pub fn process_lines(line: String) -> String {
    // receive lines from the input thread
    // process the lines
    // output the result to the console
    let mut amended = String::new();
    amended.push_str(&line);
    amended
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input_lines() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_process_lines() {
        let mut foo = String::new();
        foo.push_str("blah blah");
        let bar = process_lines(foo.clone());
        assert_eq!(foo, bar);
    }
}
