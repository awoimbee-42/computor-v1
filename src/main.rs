// TEMPORARY
#![allow(dead_code)]
#![feature(box_patterns)]
use clap::{App, Arg};

use lcomputor::Computor;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let arg_matches = App::new("computor-v1")
        .version("0.0.1")
        .author("Arthur W. <arthur.woimbee@gmail.com>")
        .about("Calculates stuff")
        .arg(
            Arg::with_name("debug")
                .short("H")
                .long("history")
                .takes_value(true)
                .help("load command history from file"),
        )
        .get_matches();

    env_logger::init();
    let mut rl = Editor::<()>::new();
    if let Some(path) = arg_matches.value_of("history") {
        match rl.load_history(path) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    let mut c = Computor::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let expr = c.compute_line(&line);
                println!("{}", expr);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    // rl.save_history("history.txt").unwrap();
}
