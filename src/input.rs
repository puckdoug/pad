use std::collections::VecDeque;
// use std::ffi::OsStr;
// use std::io;
use std::io::prelude::*;
use std::io::{IsTerminal, Stdin};
use std::path::Path;

fn usage() {
    println!("Usage: pad|lpad|rpad <width> <pad-char> [options] [tokens] [ < input ]");
    println!("Options:");
    println!("  -l, --left  <width> <pad-char>  Pad the left side");
    println!("  -r, --right <width> <pad-char>  Pad the right side");
    println!("  -h, --help                      Display this help message");
}

pub fn parse_command_line(args: Vec<String>, config: &mut crate::Config) {
    let path = Path::new(&args[0]);
    let bin_name = path.file_stem().unwrap().to_str().unwrap();
    match bin_name {
        "rpad" => config.right = true,
        _ => config.left = true,
    }

    if args.is_empty() {
        // Default configuration when no arguments are provided
        return;
    }

    let mut arglist = VecDeque::from(args);
    let _cmd = arglist.pop_front().unwrap(); // skip the command name

    while let Some(arg) = arglist.pop_front() {
        match arg.as_str() {
            "--left" | "-l" => {
                config.left = true;
                if let Some(width) = arglist.pop_front() {
                    if let Ok(width_num) = width.parse::<usize>() {
                        config.llen = width_num;
                        config.lpad = match arglist.pop_front() {
                            Some(pad_char) => pad_char,
                            None => String::from(" "),
                        };
                    }
                }
            }
            "--right" | "-r" => {
                config.right = true;
                if let Some(width) = arglist.pop_front() {
                    if let Ok(width_num) = width.parse::<usize>() {
                        config.rlen = width_num;
                        config.rpad = match arglist.pop_front() {
                            Some(pad_char) => pad_char,
                            None => String::from(" "),
                        };
                    }
                }
            }
            "--help" | "-h" => {
                usage();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                usage();
                std::process::exit(1);
            }
        }
    }
}

// todo - make this return word at a time instead of all at once
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
mod command_line {

    mod basic {

        #[test]
        fn left_true_when_named_lpad() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("path/to/lpad")], &mut config);
            assert!(config.left);
        }

        #[test]
        fn right_true_when_named_rpad() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./rpad")], &mut config);
            assert!(config.right);
        }

        #[test]
        fn left_true_when_other_name() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./pad")], &mut config);
            assert!(config.left);
        }

        #[test]
        fn right_false_when_other_name() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./foo")], &mut config);
            assert!(!config.right);
        }

        #[test]
        fn left_and_right_together_are_ok() {
            let mut config = crate::Config::new();
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
            );
            assert!(config.right);
            assert!(config.left);
        }

        #[test]
        fn r_is_ok_for_right() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./pad"), String::from("-r")], &mut config);
            assert!(config.right);
        }

        #[test]
        fn l_is_ok_for_left() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./pad"), String::from("-l")], &mut config);
            assert!(config.left);
        }
    }
}
