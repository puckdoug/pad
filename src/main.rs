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
