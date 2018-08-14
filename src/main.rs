#[macro_use]
extern crate structopt;
extern crate eng_tester;
extern crate failure;

mod cli;

#[cfg(feature = "default")]
mod gui;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;
use eng_tester::Context;

/// Testing system based on org-mode table files.
#[derive(StructOpt, Debug)]
#[structopt(name = "eng_tester")]
struct Config {
    /// org-mode file with the table
    #[structopt(name = "file", parse(from_os_str))]
    pub file_path: PathBuf,

    /// gui-mode
    #[cfg(feature = "default")]
    #[structopt(short = "g", long = "gui")]
    pub is_gui: bool,
}

fn run() -> Result<(), failure::Error> {
    let config = Config::from_args();

    let mut file = File::open(&config.file_path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let db = Context::new(&file_content)?;
    do_run(config, db);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("eng_tester: {}", e);
    }
}

#[cfg(feature = "default")]
#[inline]
fn do_run(conf: Config, db: Context) {
    if conf.is_gui {
        gui::run(db);
    } else {
        cli::run(db);
    }
}

#[cfg(not(feature = "default"))]
#[inline]
fn do_run(_: Config, db: Context) {
    cli::run(db);
}
