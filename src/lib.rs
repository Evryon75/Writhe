use crate::PrintColors::*;
use colour::{cyan_ln, green_ln, magenta_ln, red_ln, white_ln};
use std::fs::File;
use std::io::{Read, Write};

pub enum PrintColors {
    Red,
    Green,
    Cyan,
    Magenta,
    White,
}
pub fn init() {
    if let Ok(_) = std::fs::create_dir("writhe") {
        std::fs::create_dir("writhe/src").unwrap();
        std::fs::create_dir("writhe/logs").unwrap();
        std::fs::create_dir("writhe/lib").unwrap();
        File::create("writhe/logs/runtime_log.txt").unwrap();
        File::create("writhe/logs/attempt_log.py").unwrap();
        log("writhe directory created", Some(Green));
    }
    log("writhe starting", Some(Green));
}
pub fn log(msg: &str, pri: Option<PrintColors>) {
    let fm = format!(
        "[{}] >> {}\n",
        &chrono::offset::Local::now().to_string()[0..24],
        msg
    );
    let mut runtime_logs: File = File::options()
        .write(true)
        .read(true)
        .open("writhe/logs/runtime_log.txt")
        .unwrap();
    let mut contents: String = String::new();
    runtime_logs.read_to_string(&mut contents).unwrap();
    runtime_logs.write_all(fm.as_bytes()).unwrap();

    if let Some(color) = pri {
        match color {
            Red => red_ln!("{}", msg),
            Green => green_ln!("{}", msg),
            Cyan => cyan_ln!("{}", msg),
            Magenta => magenta_ln!("{}", msg),
            White => white_ln!("{}", msg),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Divide,
    Multiply,
    Subtract,
    Add,
    Comma,
    Trait,
    Impl,
    Struct,
    Undefined,
    Identifier(String), // Remember to limit this to 79 chars
    StringLiteral(String),
    BooleanLiteral(bool),
    NumericLiteral(usize),
    Fn,
    Let,
    Use,
    Mut,
    For,
    In,
    Match,
    As,
    Enum,
    Colon,
    Semicolon,
    RoundLeft,
    RoundRight,
    SquareLeft,
    SquareRight,
    CurlyLeft,
    CurlyRight,
    Equals,
    DoubleEquals,
    Greater,
    Lesser,
    And,
    Or,
}
