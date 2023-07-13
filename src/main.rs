mod lexer;

use crate::lexer::lex;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use writhe::{init, log};

fn main() {
    init();

    let args: Vec<String> = env::args().collect();
    for i in &args {
        // Match flags here
    }
    log(format!("args: {:#?}", args).as_str(), None);
    let mut contents = String::new();
    File::open(&args[1])
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    println!("LEX FN PRINT:{:#?}", lex(contents));

    //try this let (file, bytes) = match ...; line before the File::create line, and then it would be File::create(file).unwrap().write_all(bytes).unwrap()
    match Ok("hi") as Result<&str, &str> {
        Ok(res) => File::create(format!("writhe/{}", args[1].replace("rs", "py")))
            .unwrap()
            .write_all(format!("{}\n", res).as_bytes()),
        Err(res) => File::create("writhe/logs/attempt_log.py")
            .unwrap()
            .write_all(res.as_bytes()),
    }
    .unwrap();
}
