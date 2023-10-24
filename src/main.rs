mod inductive;
mod parser;

fn main() {
    let formula = "~A => (B /\\ C)";
    println!("{:?}", parser::lex(formula.to_string()));
}
