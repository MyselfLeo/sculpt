mod inductive;
mod parser;

fn main() {
    let formula = "~A \\/ (B => B) /\\ C";
    let tokens = parser::lex(formula.to_string()).unwrap();
    let postfix = parser::infix_to_postfix(&tokens).unwrap();
    let formula = parser::formula_from_tokens(&postfix).unwrap();
    println!("{formula}");
}
