mod proof;
mod tools;
mod logic;
mod engine;
mod repl;
mod error;
mod exec;
mod syntax;

use std::env;
use error::Error;
use repl::Repl;

#[cfg(feature = "exec")]
use exec::Executor;




fn start_repl() -> Result<(), Error> {
    let mut repl = Repl::new();
    repl.start().unwrap();
    Ok(())
}

#[cfg(feature = "exec")]
fn exec_file(filename: String) {
    let mut exec = Executor::from_file(filename).unwrap();

    match exec.exec_all() {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR: {}", e.0);
            println!("  from {:?} to {:?}", e.1.start, e.1.end)
        }
    }
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

#[cfg(feature = "exec")]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || &args[1] == "repl" {
        match start_repl() {
            Ok(_) => (),
            Err(e) => eprintln!("ERROR: {e}"),
        }
    }
    else if &args[1] == "exec" {
        if args.len() == 2 {
            println!("ERROR: Expected a file name");
            std::process::exit(1);
        }
        else {
            exec_file(args[2].clone())
        }
    }
}

#[cfg(not(feature = "exec"))]
fn main() {
    match start_repl() {
        Ok(_) => (),
        Err(e) => eprintln!("ERROR: {e}"),
    }
}