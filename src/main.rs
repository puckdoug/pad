use std::env;
use std::io::IsTerminal;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // parse any arguments from the command line
    parse_command_line();

    // read stdin if there is data to be consumed
    read_stdin();

    // create sending and receiving ends of the channel
    let (sender, receiver) = channel();

    // start a thread reading input lines
    thread::spawn(move || {
        sender.send(read_input_lines()).unwrap();
    });

    // start a thread consuming and processing lines, which ouputs
    // the result to the console
    while let Ok(to_process) = receiver.recv() {
        let padded = process_lines(to_process);
        println!("{padded}");
    }
}

fn parse_command_line() {
    // parse the command line arguments
    //
    // position 1 or --length, --llength or --rlength
    // position 2 or --padding, --lpadding or --lpadding
    // --left or if command is named lpad
    // --right or if command is named rpad
    // --word to pad every single word, keeping line order
    // --line to pad every line (default)
    //
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

fn read_stdin() {
    // read stdin if there is data to be consumed
    let input = std::io::stdin();

    if input.is_terminal() {
        // No input available
        println!("No input available");
    } else {
        // Input available
        println!("Input available");
    }
    println!("read_stdin");
}

fn read_input_lines() -> String {
    // read lines from stdin
    // send lines to the processing thread
    let mut rec = String::new();
    rec.push_str("placeholder");
    rec
}

fn process_lines(line: String) -> String {
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
