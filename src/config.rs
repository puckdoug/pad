pub enum Approach {
    Line,
    Word,
}

pub struct Config {
    pub left: bool,
    pub lpad: String,
    pub llen: usize,
    pub right: bool,
    pub rpad: String,
    pub rlen: usize,
    pub approach: Approach,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            left: false,
            lpad: String::from("0"),
            llen: 0,
            right: false,
            rpad: String::from("0"),
            rlen: 0,
            approach: Approach::Line,
        }
    }
}
