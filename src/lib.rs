use crate::PrintColors::*;
use std::fs::File;
use std::io::{Read, Write};
use colour::{cyan, green, magenta, red, white};
use serde_derive::{Deserialize, Serialize};

pub enum PrintColors {
    Red,
    Green,
    Cyan,
    Magenta,
    White,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    files: Vec<String>,
}
pub fn init() {
    if let Ok(_) = std::fs::create_dir("writhe") {
        let default_config = Config {
            files: vec![],
        };
        std::fs::create_dir("writhe/src").unwrap();
        std::fs::create_dir("writhe/logs").unwrap();
        std::fs::create_dir("writhe/lib").unwrap();
        File::create("writhe/logs/runtime_log.txt").unwrap();
        File::create("writhe/logs/attempt_log.py").unwrap();
        File::create("writhe/writhe_config.json")
            .unwrap()
            .write_all(format!("{:#?}", default_config).as_bytes())
            .unwrap();
        log("[NOTE] writhe directory created", Some(Green));
    }
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
            Red => red!("{}\n", msg),
            Green => green!("{}\n", msg),
            Cyan => cyan!("{}\n", msg),
            Magenta => magenta!("{}\n", msg),
            White => white!("{}\n", msg),
        }
    }
}

pub fn panic_confirm(msg: &str) {
    log(msg, Some(Red));
    white!();
    panic!()
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Iter(String),
    Operator(String),
    DoubleColon,
    Comma,
    Trait,
    Impl,
    Struct,
    Undefined,
    Identifier(String), // Remember to limit this to 79 chars
    StringLiteral(String),
    BooleanLiteral(bool),
    NumericLiteral(f64),
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
}
