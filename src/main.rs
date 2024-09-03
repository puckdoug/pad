use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = &args.clone()[0];

    // Prints each argument on a separate line
    println!("Program name: {program_name}");
    for argument in args {
        match argument.as_str() {
            "--help" => {
                println!("Usage: {program_name} [OPTION]... [FILE]...");
                println!("Concatenate FILE(s) to standard output.");
                println!("");
                println!("With no FILE, or when FILE is -, read standard input.");
                println!("");
                println!("  -h, --help     display this help and exit");
                println!("  -v, --verbose  verbose output");
                return;
            }
            "--verbose" => {
                println!("cat (rust) 0.1.0");
                return;
            }
            _ => {}
        }
        println!("{argument:?}");

        let input = grab_stdin();
        println!("STDIN: {input}");
    }
}

fn grab_stdin() -> String {
    let mut input = String::new();
    if !at_eof() {
        match io::stdin().read_to_string(&mut input) {
            Ok(_) => println!("stdin: {}", input.trim()),
            Err(e) => eprintln!("error: {}", e),
        }
    }
    input
}

fn at_eof() -> bool {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(0) => true,
        Err(_) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_eof() {
        // test run without any stdin should confirm end of file
        assert_eq!(at_eof(), true);
    }
}
