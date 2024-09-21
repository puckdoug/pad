pub const DEFAULT_PAD: &'static str = "0";

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
