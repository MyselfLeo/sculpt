mod proof;
mod repl;
mod tools;
mod context;
mod logic;


use lalrpop_util::lalrpop_mod;
use repl::{Repl, ReplError};

use std::env;


lalrpop_mod!(pub parser);



fn start_repl(starting_f: Option<String>) -> Result<(), ReplError> {
    let mut repl = match starting_f {
        Some(s) => Repl::from(s)?,
        None => Repl::new(),
    };
    repl.start().unwrap();
    Ok(())
}



fn get_input_formula() -> Option<String> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {return None}
    args.remove(0);

    let str: String = args
        .iter()
        .map(|f| vec![f.to_string(), " ".to_string()])
        .flatten()
        .collect();

    Some(str)
}


fn main() {
    let input = get_input_formula();
    
    match start_repl(input) {
        Ok(_) => (),
        Err(e) => eprintln!("ERROR: {e}"),
    }
}
