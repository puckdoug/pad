use crate::input::*;
use crate::output::*;
// use std::result;
use std::sync::mpsc::channel;
use std::thread;

pub mod input;
pub mod output;

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
