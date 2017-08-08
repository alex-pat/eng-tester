extern crate eng_tester;

mod ui;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let abort = |msg, err| -> ! {
        eprintln!("{}: {}", msg, err);
        std::process::exit(1);
    };
    let config = eng_tester::Config::new(env::args());
    let config = config.unwrap_or_else(|err| abort("Arguments parsing error", err));

    let mut file = File::open(config.file_path).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let db = eng_tester::Context::new(file_content);
    let db = db.unwrap_or_else(|err| abort("db building error", err));
    if config.is_gui {
        ui::run_gui(db);
    } else {
        ui::run_cli(db);
    }
}
