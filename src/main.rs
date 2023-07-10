mod lexer;

use colour::{cyan_ln, green_ln, magenta_ln, red_ln, white_ln};
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    if let Ok(_) = std::fs::create_dir("writhe") {
        std::fs::create_dir("writhe/logs").unwrap();
    }
    let mut runtime_logs = File::create(format!("writhe/logs/runtime_log.txt")).unwrap();
    let mut log = |msg: &str, pri: Option<PrintColors>| {
        // Declared as a closure so that we can use a variable to reference the log file
        // instead of having to create it every time we log something, premature optimization? maybe
        // This is probably a terrible idea and i can see myself refactoring for HOURS but whatever
        let fm = format!(
            "[{}] >> {}\n",
            chrono::offset::Local::now().to_string(),
            msg
        );
        runtime_logs.write_all(fm.as_bytes()).unwrap();
        if let Some(color) = pri {
            match color {
                PrintColors::Red => red_ln!("{}", msg),
                PrintColors::Green => green_ln!("{}", msg),
                PrintColors::Cyan => cyan_ln!("{}", msg),
                PrintColors::Magenta => magenta_ln!("{}", msg),
                PrintColors::White => white_ln!("{}", msg),
            }
        }
    };

    let args: Vec<String> = env::args().collect();
    for i in &args {
        // Match flags here
    }

    log(format!("args: {:#?}", args).as_str(), None);
    log("starting writhe", Some(PrintColors::Green));

    let teething: Result<&str, &str> = Ok("hi");
    match teething {
        Ok(res) => File::create(format!("writhe/{}", args[1].replace("rs", "py")))
            .unwrap()
            .write_all(format!("{}\n", res).as_bytes())
            .unwrap(),
        Err(res) => File::create(format!("writhe/logs/attempt_log.txt"))
            .unwrap()
            .write_all(res.as_bytes())
            .unwrap(),
    }
}

enum PrintColors {
    Red,
    Green,
    Cyan,
    Magenta,
    White,
}
