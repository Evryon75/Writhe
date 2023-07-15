mod lexer;

use crate::lexer::lex;
use std::{env, fs};
use std::fs::{File};
use std::io::{Read, Write};
use colour::white_ln;
use writhe::{init, log, panic_confirm};
use writhe::PrintColors::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    //todo: add flags for:
    // show tokens
    // multiple files in the json like make a vector or something idk
    // config json

    let mut contents = String::new();
    white_ln!("{:#?}", args);
    let mut skip_first = false;
    for arg in &args {
        if skip_first {
            match arg.as_str() {
                "init" => init(),
                _ => {
                    white_ln!("{}", arg);
                    if arg.ends_with(".rs") {
                        if let Ok(_) = fs::create_dir("writhe") {
                            fs::remove_dir("writhe").unwrap();
                            init();
                        } // todo: file not found error here
                        File::open(&args[1])
                            .unwrap()
                            .read_to_string(&mut contents)
                            .unwrap();
                    } else {
                        panic_confirm("[MAIN WARNING] Unknown command!")
                    }
                }
            }
        }
        skip_first = true;
    }

    log("[NOTE] writhe starting", Some(Green));
    log(format!("[NOTE] args: {:#?}", args).as_str(), None);

    println!("LEX FN PRINT:\n{:#?}", lex(contents));

    //try this let (file, bytes) = match ...; line before the File::create line, and then it would be File::create(file).unwrap().write_all(bytes).unwrap()
    match Ok("hi") as Result<&str, &str> {
        Ok(res) => File::create(format!("writhe/src/{}", args[1].replace("rs", "py")))
            .unwrap()
            .write_all(format!("{}\n", res).as_bytes()),
        Err(res) => File::create("writhe/logs/attempt_log.py")
            .unwrap()
            .write_all(res.as_bytes()),
    }
    .unwrap();
}
