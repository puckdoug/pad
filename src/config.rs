pub enum Approach {
    Line,
    Word,
}

pub struct Config {
    pub left: bool,
    pub right: bool,
    pub lpad_chars: String,
    pub rpad_chars: String,
    pub approach: Approach,
}

impl Config {
    pub fn new() -> Config {
        Config {
            left: false,
            right: false,
            lpad_chars: String::from(""),
            rpad_chars: String::from(""),
            approach: Approach::Line,
        }
    }
}
