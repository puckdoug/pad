use std::env;
use std::io::IsTerminal;

fn main() {
    // parse any arguments from the command line
    parse_command_line();
    // read stdin if there is data to be consumed
    read_stdin();
    // start a thread reading input lines
    read_input_lines();
    // start a thread consuming and processing lines, which ouputs the result to the console
    process_lines();
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

fn read_input_lines() {
    // read lines from stdin
    // send lines to the processing thread
    println!("read_input_lines");
}

fn process_lines() {
    // receive lines from the input thread
    // process the lines
    // output the result to the console
    println!("process_lines");
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_read_input_lines() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_process_lines() {
        assert_eq!(1, 1);
    }
}
