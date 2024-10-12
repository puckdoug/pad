// use std::ffi::OsStr;
use crate::LR;
use std::io::prelude::*;
use std::io::{IsTerminal, Stdin};
use std::path::Path;

/// Usage prints the help message to the console. The program should exit after doing so.
pub fn usage() {
    println!("Usage: pad|lpad|rpad [ <width> [ <pad-str> ] ] [options] [tokens] [ < input ]");
    println!("Options:");
    println!("  -l, --left  [ <width> [ <pad-str> ] ]  Pad the left side");
    println!("  -r, --right [ <width> [ <pad-str> ] ]  Pad the right side");
    println!("  -h, --help                             Display this help message");
}

/// The heavy lifting of setting up the program for padding is done here. This parses all
/// options from the command-line and determines what padding to do, left, right, or both.
/// If specified, it also builds a Vec of words to pad.
pub fn parse_command_line(args: Vec<String>, config: &mut crate::Config, words: &mut Vec<String>) {
    let mut first = true;
    let mut lr1 = LR::None;
    let mut lr2 = LR::None;
    for arg in args.iter() {
        // only the first argument is possible to be the command-name
        if first {
            first = false;
            let path = Path::new(arg);
            let bin_name = path.file_stem().unwrap().to_str().unwrap();
            match bin_name {
                "rpad" => {
                    config.right = true;
                    lr1 = LR::Right;
                }
                "lpad" => {
                    config.left = true;
                    lr1 = LR::Left;
                }
                _ => (),
            }
            continue;
        }
        match arg.as_str() {
            "--help" | "-h" => {
                config.help = true;
                break;
            }
            "--left" | "-l" => {
                config.left = true;
                lr1 = LR::Left;
                continue;
            }
            "--right" | "-r" => {
                config.right = true;
                lr1 = LR::Right;
                continue;
            }
            _ => {
                if lr1 == LR::Right {
                    if let Ok(width_num) = arg.parse::<usize>() {
                        config.rlen = width_num;
                        lr2 = LR::Right;
                    } else {
                        words.push(arg.clone());
                    }
                    lr1 = LR::None;
                } else if lr1 == LR::Left {
                    if let Ok(width_num) = arg.parse::<usize>() {
                        config.llen = width_num;
                        lr2 = LR::Left;
                    } else {
                        words.push(arg.clone());
                    }
                    lr1 = LR::None;
                } else if lr2 == LR::Right {
                    config.rpad = arg.clone();
                    lr2 = LR::None;
                    continue;
                } else if lr2 == LR::Left {
                    config.lpad = arg.clone();
                    lr2 = LR::None;
                    continue;
                } else if lr1 == LR::None && lr2 == LR::None {
                    words.push(arg.clone());
                }
            }
        }
    }

    if !config.left && !config.right {
        // Default configuration when no arguments are provided
        config.left = true;
    }
}

// todo - make this return word at a time instead of all at once
/// read_stdin takes input on stdin and provides all words for padding.
pub fn read_stdin(input: Stdin) -> Vec<String> {
    let mut lines = Vec::new();

    if !input.is_terminal() {
        // Input available
        for line in input.lock().lines() {
            match line {
                Ok(line) => lines.push(line),
                Err(e) => println!("Error reading line: {}", e),
            }
        }
    }

    lines
}

//------------------------------------------------------------------------------
// Unit Tests
//------------------------------------------------------------------------------
#[cfg(test)]
#[test]
fn left_true_when_named_lpad() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(vec![String::from("path/to/lpad")], &mut config, &mut words);
    assert!(config.left);
}

#[test]
fn right_true_when_named_rpad() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(vec![String::from("./rpad")], &mut config, &mut words);
    assert!(config.right);
}

#[test]
fn left_true_when_other_name() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(vec![String::from("./pad")], &mut config, &mut words);
    assert!(config.left);
}

#[test]
fn left_true_right_false_when_other_name() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(vec![String::from("./foo")], &mut config, &mut words);
    assert!(!config.right);
    assert!(config.left);
}

#[test]
fn lpad_with_tokens_only_pads_default() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(
        vec![
            String::from("lpad"),
            String::from("f"),
            String::from("fo"),
            String::from("foo"),
        ],
        &mut config,
        &mut words,
    );
    assert!(config.left);
    assert_eq!(config.llen, 0);
    assert_eq!(config.lpad, "0");
    assert_eq!(words.len(), 3);
}

#[test]
fn left_and_right_together_are_ok() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(
        vec![
            String::from("pad"),
            String::from("--left"),
            String::from("5"),
            String::from("x"),
            String::from("--right"),
            String::from("10"),
            String::from("y"),
        ],
        &mut config,
        &mut words,
    );
    assert!(config.right);
    assert_eq!(config.rlen, 10);
    assert_eq!(config.rpad, "y");
    assert!(config.left);
    assert_eq!(config.llen, 5);
    assert_eq!(config.lpad, "x");
}

#[test]
fn left_and_right_together_are_ok_with_rpad_bi_with_rpad_bin() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(
        vec![
            String::from("rpad"),
            String::from("10"),
            String::from("y"),
            String::from("--left"),
            String::from("5"),
            String::from("x"),
        ],
        &mut config,
        &mut words,
    );
    assert!(config.right);
    assert_eq!(config.rlen, 10);
    assert_eq!(config.rpad, "y");
    assert!(config.left);
    assert_eq!(config.llen, 5);
    assert_eq!(config.lpad, "x");
}

#[test]
fn left_and_right_together_are_ok_with_lpad_bin() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(
        vec![
            String::from("lpad"),
            String::from("5"),
            String::from("x"),
            String::from("--right"),
            String::from("10"),
            String::from("y"),
        ],
        &mut config,
        &mut words,
    );
    assert!(config.right);
    assert_eq!(config.rlen, 10);
    assert_eq!(config.rpad, "y");
    assert!(config.left);
    assert_eq!(config.llen, 5);
    assert_eq!(config.lpad, "x");
}

#[test]
fn r_is_ok_for_right() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(
        vec![String::from("./pad"), String::from("-r")],
        &mut config,
        &mut words,
    );
    assert!(config.right);
}

#[test]
fn l_is_ok_for_left() {
    let mut config = crate::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(
        vec![String::from("./pad"), String::from("-l")],
        &mut config,
        &mut words,
    );
    assert!(config.left);
}

#[test]
fn no_option_but_args() {
    let mut config = crate::pad::Config::new();
    let mut words: Vec<String> = Vec::new();
    crate::parse_command_line(
        vec![String::from("lpad"), String::from("5"), String::from("x")],
        &mut config,
        &mut words,
    );
    assert!(config.left);
    assert_eq!(config.llen, 5);
    assert_eq!(config.lpad, "x");
}
