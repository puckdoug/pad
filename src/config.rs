pub enum Approach {
    Line,
    Word,
}

pub struct Config {
    pub left: bool,
    pub lpad: String,
    pub llen: u32,
    pub right: bool,
    pub rpad: String,
    pub rlen: u32,
    pub approach: Approach,
}

impl Config {
    pub fn new() -> Config {
        Config {
            left: false,
            lpad: String::from("0"),
            llen: 2,
            right: false,
            rpad: String::from("0"),
            rlen: 2,
            approach: Approach::Line,
        }
    }
}
