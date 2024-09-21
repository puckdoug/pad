// use std::ffi::OsStr;
use std::io::prelude::*;
use std::io::{IsTerminal, Stdin};
use std::path::Path;

#[derive(PartialEq)]
enum LR {
    Left,
    Right,
    None,
}

pub fn usage() {
    println!("Usage: pad|lpad|rpad [ <width> [ <pad-str> ] ] [options] [tokens] [ < input ]");
    println!("Options:");
    println!("  -l, --left  [ <width> [ <pad-str> ] ]  Pad the left side");
    println!("  -r, --right [ <width> [ <pad-str> ] ]  Pad the right side");
    println!("  -h, --help                              Display this help message");
}

pub fn parse_command_line(args: Vec<String>, config: &mut crate::Config) {
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
        }
        println!("{arg}");
        match arg.as_str() {
            "--help" | "-h" => config.help = true,
            "--left" | "-l" => {
                config.left = true;
                lr1 = LR::Left;
            }
            "--right" | "-r" => {
                config.right = true;
                lr1 = LR::Right;
            }
            _ => {
                if lr1 == LR::Right {
                    if let Ok(width_num) = arg.parse::<usize>() {
                        config.rlen = width_num;
                        lr1 = LR::None;
                        lr2 = LR::Right;
                    }
                } else if lr1 == LR::Left {
                    if let Ok(width_num) = arg.parse::<usize>() {
                        config.llen = width_num;
                        lr1 = LR::None;
                        lr2 = LR::Left;
                    }
                } else if lr2 == LR::Right {
                    config.rpad = arg.clone();
                    lr2 = LR::None;
                } else if lr2 == LR::Left {
                    config.lpad = arg.clone();
                    lr2 = LR::None;
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

        #[test]
        fn no_option_but_args() {
            let mut config = crate::Config::new();
            crate::parse_command_line(
                vec![String::from("lpad"), String::from("5"), String::from("x")],
                &mut config,
            );
            assert!(config.left);
            assert_eq!(config.llen, 5);
            assert_eq!(config.lpad, "x");
        }
    }
}
