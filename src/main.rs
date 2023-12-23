mod proof;
mod tools;
mod logic;
mod interpreter;
mod repl;
mod error;
mod exec;
mod syntax;

use std::fs;
use error::Error;

use repl::Repl;
use crate::exec::Executor;
use crate::syntax::lexer::{Context, Lexer};


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

    let txt = fs::read_to_string(FILE).unwrap();

    let mut context = Context::new();
    context.relations.insert("A".to_string(), 0);
    context.relations.insert("J".to_string(), 0);
    context.relations.insert("Z".to_string(), 0);
    let res = Lexer::from(txt.as_ref(), Context::new());

    //println!("{}", ','.is_whitespace())

    for t in res {
        println!("{:?}", t.unwrap())
    }


    /*let mut exec= Executor::from_file(FILE.to_string()).unwrap();

    match exec.exec_all() {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR: {}", e.0);
            println!("  from {:?} to {:?}", e.1.start, e.1.end)
        }
    }*/


    /*match start_repl() {
        Ok(_) => (),
        Err(e) => eprintln!("ERROR: {e}"),
    }*/
}
