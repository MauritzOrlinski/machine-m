mod ast;
mod interpreter;
mod optimiser;
mod parser;
use std::{collections::VecDeque, fs::read_to_string, path::Path};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Machine M simulator")]
struct Args {
    #[arg(value_name = "FILE")]
    file_path: String,
    #[arg(short = 'a', long, value_delimiter = ',', value_name = "NUMBER")]
    args: Vec<f32>,

    #[arg(short = 'd', long)]
    debug: bool,
}

fn main() {
    let Args {
        file_path,
        args,
        debug,
    } = Args::parse();
    let file = read_to_string(Path::new(&file_path)).unwrap();
    let program = parser::parse_program(&file).unwrap();
    let mut state = interpreter::State::new(VecDeque::from(args));
    if debug {
        state.execute_print_state(&program);
    } else {
        state.execute(&program);
    }
}
