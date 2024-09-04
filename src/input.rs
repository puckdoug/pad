// use std::ffi::OsStr;
use clap::Parser;
use std::io::{IsTerminal, Stdin};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // whether to pad left
    #[arg(short, long)]
    left: bool,

    // whether to pad right
    #[arg(short, long)]
    right: bool,

    // character to use to pad left
    #[arg(long, default_value = "0")]
    lpad: String,

    // character to use to pad right
    #[arg(long, default_value = "0")]
    rpad: String,

    // amount of left padding
    #[arg(long, default_value_t = 1)]
    llen: u32,

    // amount of right padding
    #[arg(long, default_value_t = 1)]
    rlen: u32,
}

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
        "lpad" => config.left = true,
        "rpad" => config.right = true,
        _ => config.left = true,
    }

    // skip command name (args[0]) and process everything else
    for arg in &args[1..] {
        match arg.as_str() {
            "--left" => config.left = true,
            "--right" => config.right = true,
            "--lpad" => config.lpad = String::from("foo"), // should be argument
            "--rpad" => config.rpad = String::from("bar"), // should be argument
            "--llen" => config.llen = 1,                   // should be argument
            "--rlen" => config.rlen = 1,                   // should be argument
            _ => (),                                       // print usage
        }
        println!("{arg}");
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

#[cfg(test)]
mod tests {

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
}
