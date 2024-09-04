use crate::config::*;
use crate::input::*;
use crate::output::*;
use clap::{arg, command};
use std::env;
use std::io;
// use std::result;
use std::sync::mpsc::channel;
use std::thread;

pub mod config;
pub mod input;
pub mod output;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     // whether to pad left
//     #[arg(short, long)]
//     left: bool,

//     // character to use to pad left
//     #[arg(long)]
//     lpad: String,

//     // amount of left padding
//     #[arg(long)]
//     llen: u32,

//     // whether to pad right
//     #[arg(short, long)]
//     right: bool,

//     // character to use to pad right
//     #[arg(long)]
//     rpad: String,

//     // amount of right padding
//     #[arg(long)]
//     rlen: u32,
// }

fn main() {
    // gather the command-line arguments
    // let args: Vec<String> = env::args().collect();
    let mut config = Config::new();

    // parse the arguments from the command line
    // parse_command_line(args, &mut config);
    let matches = command!()
        .arg(arg!(-l --left <string> <width> "pad from the left with count characters"))
        .arg(arg!(-r --right <string> <width> "pad from the right with count characters"))
        .get_matches();

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
        let padded = process_lines(to_process);
        println!("{padded}");
    }
}
