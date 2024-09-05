use crate::config::*;
use crate::input::*;
use crate::lpad::*;
use crate::output::*;
use crate::rpad::*;
use std::env;
use std::io;
// use std::result;
use std::sync::mpsc::channel;
use std::thread;

pub mod config;
pub mod input;
pub mod lpad;
pub mod output;
pub mod rpad;

fn main() {
    // gather the command-line arguments
    // let args: Vec<ArgsOs> = env::args_os().collect();
    let args: Vec<String> = env::args().collect(); // todo - don't collect, pass the iterator
    let mut config = Config::new();

    // parse the arguments from the command line
    parse_command_line(args, &mut config);

    // read stdin if there is data to be consumed
    read_stdin(io::stdin());

    // create sending and receiving ends of the channel
    let (sender, receiver) = channel();

    // start a thread reading input lines
    thread::spawn(move || {
        sender.send(read_input_lines()).unwrap();
    });

    // start a thread consuming and processing lines, which ouputs
    // the result to the console
    while let Ok(to_process) = receiver.recv() {
        let foo = vec![to_process];
        let _padded = process_lines(foo.clone(), &mut config);
    }
}
