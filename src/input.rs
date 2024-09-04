use std::env;
use std::io::IsTerminal;

pub fn parse_command_line() {
    // parse the command line arguments
    //
    // position 1 or --length, --llength or --rlength
    // position 2 or --padding, --lpadding or --lpadding
    // --left or if command is named lpad
    // --right or if command is named rpad
    // --word to pad every single word, keeping line order
    // --line to pad every line (default)
    // all other args are tokens to be padded
    //
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

pub fn read_stdin() {
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
