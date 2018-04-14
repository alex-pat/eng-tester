#[macro_use]
extern crate structopt;
extern crate eng_tester;

mod cli;
mod gui;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// Testing system based on org-mode table files.
#[derive(StructOpt, Debug)]
#[structopt(name = "eng_tester")]
struct Config {
    /// org-mode file with the table
    #[structopt(name = "file", parse(from_os_str))]
    pub file_path: PathBuf,

    /// gui-mode
    #[structopt(short = "g", long = "gui")]
    pub is_gui: bool,
}

fn main() {
    let config = Config::from_args();

    let mut file = File::open(config.file_path).expect("file opening error");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("file reading error");

    let db = eng_tester::Context::new(file_content).expect("db building error");
    if config.is_gui {
        gui::run(db);
    } else {
        cli::run(db);
    }
}
