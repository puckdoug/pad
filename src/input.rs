// use std::ffi::OsStr;
use std::io::{IsTerminal, Stdin};
use std::path::Path;

pub fn parse_command_line(args: Vec<String>, config: &mut crate::Config) {
    // parse the command line arguments
    //
    // position 1 or --length, --llength or --rlength
    // position 2 or --padding, --lpadding or --lpadding
    // --left or if command is named lpad
    // --right or if command is named rpad
    // --word to pad every single word, keeping line order
    // --line to pad every line (default)
    // all other args are tokens to be padded
    let path = Path::new(&args[0]);
    let bin_name = path.file_stem().unwrap().to_str().unwrap();

    match bin_name {
        "lpad" => config.left = true,
        "rpad" => config.right = true,
        _ => config.left = true,
    }

    // binary named lpad or --left means pad the left side
    // binary named rpad or --right means pad th right side
    // binary just named pad will only work with --left or --right
    println!("Executable: {bin_name}");

    println!("{:?}", args);
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

#[cfg(test)]
mod tests {

    #[test]
    fn left_true_when_named_lpad() {
        let args = vec![String::from("path/to/lpad")];
        let mut config = crate::Config::new();
        crate::parse_command_line(args, &mut config);
        assert_eq!(config.left, true);
    }

    #[test]
    fn right_true_when_named_rpad() {
        let args = vec![String::from("./rpad")];
        let mut config = crate::Config::new();
        crate::parse_command_line(args, &mut config);
        assert_eq!(config.right, true);
    }

    #[test]
    fn left_true_when_other_name() {
        let args = vec![String::from("./pad")];
        let mut config = crate::Config::new();
        crate::parse_command_line(args, &mut config);
        assert_eq!(config.left, true);
    }

    #[test]
    fn right_false_when_other_name() {
        let args = vec![String::from("./foo")];
        let mut config = crate::Config::new();
        crate::parse_command_line(args, &mut config);
        assert_eq!(config.right, false);
    }
}
