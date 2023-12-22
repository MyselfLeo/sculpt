mod proof;
mod tools;
mod logic;
mod interpreter;
mod repl;
mod error;
mod exec;


use std::fs;
use error::Error;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser);

use repl::Repl;
use crate::exec::Executor;
use crate::interpreter::Interpreter;


fn start_repl() -> Result<(), Error> {
    let mut repl = Repl::new();
    repl.start().unwrap();
    Ok(())
}



/*fn get_input_formula() -> Option<String> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {return None}
    args.remove(0);

    let str: String = args
        .iter()
        .map(|f| vec![f.to_string(), " ".to_string()])
        .flatten()
        .collect();

    Some(str)
}*/


fn main() {

    const FILE: &str = "examples/test.sculpt";

    let mut exec= Executor::from_file(FILE.to_string()).unwrap();

    match exec.exec_all() {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR: {}", e.0);
            println!("  from {:?} to {:?}", e.1.start, e.1.end)
        }
    }


    /*match start_repl() {
        Ok(_) => (),
        Err(e) => eprintln!("ERROR: {e}"),
    }*/
}
