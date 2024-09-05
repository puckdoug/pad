use std::collections::VecDeque;
// use std::ffi::OsStr;
use std::io::{IsTerminal, Stdin};
use std::path::Path;

pub fn parse_command_line(args: Vec<String>, config: &mut crate::Config) {
    // parse the command line arguments
    //
    // position 1 or --len, --llen or --rlen
    // position 2 or --pad, --lpad or --rpad
    // --left or if command is named lpad
    // --right or if command is named rpad
    // --word to pad every single word, keeping line order
    // --line to pad every line (default)
    // all other args are tokens to be padded
    let path = Path::new(&args[0]);
    let bin_name = path.file_stem().unwrap().to_str().unwrap();

    // binary named lpad or --left means pad the left side
    // binary named rpad or --right means pad th right side
    // binary just named pad will only work with --left or --right

    match bin_name {
        "rpad" => config.right = true,
        "lpad" => config.left = true, // drop if this is truly redunant
        _ => config.left = true,
    }

    // skip command name (args[0]) and process everything else
    let mut arglist = VecDeque::from(args);
    let _cmd = arglist.pop_front(); // skip the command name
    while let Some(arg) = arglist.pop_front() {
        match arg.as_str() {
            "--left" | "-l" => config.left = true,
            "--right" | "-r" => config.right = true,
            _ => (),
        }
    }
}

pub fn read_stdin(input: Stdin) {
    if input.is_terminal() {
        // No input available
        println!("No input available");
    } else {
        // Input available
        println!("Input available");
    }
    println!("read_stdin");
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
            assert_eq!(config.left, true);
        }

        #[test]
        fn right_true_when_named_rpad() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./rpad")], &mut config);
            assert_eq!(config.right, true);
        }

        #[test]
        fn left_true_when_other_name() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./pad")], &mut config);
            assert_eq!(config.left, true);
        }

        #[test]
        fn right_false_when_other_name() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./foo")], &mut config);
            assert_eq!(config.right, false);
        }

        #[test]
        fn left_and_right_together_are_ok() {
            let mut config = crate::Config::new();
            crate::parse_command_line(
                vec![
                    String::from("pad"),
                    String::from("--left"),
                    String::from("--right"),
                ],
                &mut config,
            );
            assert_eq!(config.right, true);
            assert_eq!(config.left, true);
        }

        #[test]
        fn r_is_ok_for_right() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./pad"), String::from("-r")], &mut config);
            assert_eq!(config.right, true);
        }

        #[test]
        fn l_is_ok_for_left() {
            let mut config = crate::Config::new();
            crate::parse_command_line(vec![String::from("./pad"), String::from("-l")], &mut config);
            assert_eq!(config.left, true);
        }
    }

    mod advanced {
        #[test]
        #[ignore = "not yet implemented"]
        fn with_no_arguments_lpad_to_width_of_largest_word() {
            assert_eq!(1, 1);
        }

        #[test]
        #[ignore = "not yet implemented"]
        fn rpad_with_no_args_pad_right_to_width_of_largest_word() {
            assert_eq!(1, 1);
        }
    }

    // #[test]
    // #[should_panic(expected = "values don't match")]
    // fn panic_example() {
    //     assert_eq!(1, 2, "values don't match");
    // }
}
