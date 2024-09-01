use std::env;

fn main() {
    // Prints each argument on a separate line
    for argument in env::args_os() {
        println!("{argument:?}");
    }
}
