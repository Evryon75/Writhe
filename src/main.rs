mod lexer;

use crate::lexer::lex;
use std::{env, fs};
use std::fs::{File};
use std::io::{Read, Write};
use colour::{e_dark_grey, e_white, red_ln, white};
use writhe::{init, log, panic_confirm};
use writhe::PrintColors::*;

fn main() {

    std::panic::set_hook(Box::new(|_| {}));
    let args: Vec<String> = env::args().collect();
    //todo: add flags for:
    // show tokens
    // multiple files in the json like make a vector or something idk
    // config json
    println!("{:?}", args);

    let mut contents = String::new();
    for arg in &args[1..args.len()] {
        match arg.as_str() {
            "init" => init(),
            "run" => {},
            _ => {
                if arg.ends_with(".rs") {
                    if let Ok(_) = fs::create_dir("writhe") {
                        fs::remove_dir("writhe").unwrap();
                        init();
                    }
                    if let Ok(mut file) = File::open(&args[1]) {
                        file.read_to_string(&mut contents)
                        .unwrap();
                    } else {
                        panic_confirm("[ERROR] File not found, run `dir *.rs` to see all rust files in the directory")
                    }

                } else {
                    red_ln!("[MAIN WARNING] Unknown command! ({})", arg)
                }
            }
        }

    }

    log("[NOTE] writhe starting", Some(Green));
    log(format!("[NOTE] args: {:#?}", args).as_str(), None);
    println!("LEX FN PRINT:\n{:#?}", lex(contents));
    e_white!();
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
