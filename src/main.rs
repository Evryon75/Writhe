#![feature(try_blocks)]

use colour::{cyan_ln, magenta_ln, red_ln};
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

fn main() {
    log("starting writhe");

    let args: Vec<String> = env::args().collect();
    log(format!("args: {:#?}", args).as_str());
    for i in &args {
        // Match flags here
    }

    if let Ok(_) = std::fs::create_dir("writhe") {
        log("writhe directory created");
    } else {
        log("writhe directory already exists");
    }

    File::create(format!("writhe/{}", args[1].replace("rs", "py")))
        .unwrap()
        .write_all(format!("{}\n", "**Resulting python code goes here**").as_bytes())
        .unwrap();

}

pub fn log(msg: &str) {
    //todo: put these on a log file in /writhe later i should probably put this fn in lib.rs
    magenta_ln!("{:?} >> {}\n", chrono::offset::Local::now(), msg);
}