use crate::DEFAULT_PAD;

/// The Config struct is used across main, input, and ouput to store the
/// options which define how words will be padded on output. Defaults are
/// set in the base implementation, which are then overridden by command-line
/// options at runtime.
#[derive(Debug)]
pub struct Config {
    pub help: bool,
    pub left: bool,
    pub lpad: String,
    pub llen: usize,
    pub right: bool,
    pub rpad: String,
    pub rlen: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            help: false,
            left: false,
            lpad: DEFAULT_PAD.to_string(),
            llen: 0,
            right: false,
            rpad: DEFAULT_PAD.to_string(),
            rlen: 0,
        }
    }
}
