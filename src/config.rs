pub enum Approach {
    Line,
    Word,
}

pub struct Config {
    pub left: bool,
    pub right: bool,
    pub lpad: String,
    pub llen: u32,
    pub rpad: String,
    pub rlen: u32,
    pub approach: Approach,
}

impl Config {
    pub fn new() -> Config {
        Config {
            left: false,
            right: false,
            lpad: String::from(""),
            llen: 0,
            rpad: String::from(""),
            rlen: 0,
            approach: Approach::Line,
        }
    }
}
