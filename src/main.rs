use crate::config::*;
use crate::input::*;
use crate::output::*;
use crate::padding::*;
use std::env;
use std::io;
// use std::result;
use std::sync::mpsc::channel;
use std::thread;

pub mod config;
pub mod input;
pub mod output;
pub mod padding;

/// the default string to use to pad words if none is specified
pub const DEFAULT_PAD: &str = "0";

/// LR is used to flag whether the upcoming arguments apply to left or right
/// padding or none, in which case they should be treated as tokens to pad.
#[derive(PartialEq)]
pub enum LR {
    Left,
    Right,
    None,
}

fn main() {
    // gather the command-line arguments
    // let args: Vec<ArgsOs> = env::args_os().collect();
    let args: Vec<String> = env::args().collect(); // todo - don't collect, pass the iterator
    let mut config = Config::new();
    let mut words: Vec<String> = Vec::new();

    // parse the arguments from the command line. Set the configuration based on inputs.
    parse_command_line(args, &mut config, &mut words);
    if config.help {
        usage();
        std::process::exit(0);
    }

    // create sending and receiving ends of the channel for passing words to pad
    let (sender, receiver) = channel();

    // start a thread and pass words for padding
    thread::spawn(move || {
        sender.send(words).unwrap();
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
