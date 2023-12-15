mod proof;
mod tools;
mod logic;
mod interpreter;
mod repl;


use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser);

use repl::{Repl, Error};




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

    match start_repl() {
        Ok(_) => (),
        Err(e) => eprintln!("ERROR: {e}"),
    }
}
