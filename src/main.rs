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

    // parse the arguments from the command line. Set the configuration based on inputs.
    parse_command_line(args, &mut config);
    if config.help {
        usage();
        std::process::exit(0);
    }

    // create sending and receiving ends of the channel for passing words to pad
    let (sender, receiver) = channel();

    // start a thread and pass words for padding
    thread::spawn(move || {
        sender.send(read_stdin(io::stdin())).unwrap();
    });

    // start a thread consuming and padding words, which ouputs
    // the result to the console
    while let Ok(to_process) = receiver.recv() {
        let padded = process_lines(to_process.clone(), &mut config);
        for word in padded {
            println!("{word}");
        }
    }
}
