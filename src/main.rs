mod inductive;
mod parser;

fn main() {
    let formula = "~A => (B /\\ C)";
    let tokens = parser::lex(formula.to_string()).unwrap();
    println!("{:?}", tokens);
    let postfix = parser::infix_to_postfix(&tokens).unwrap();
    println!("{:?}", postfix);
    let formula = parser::formula_from_tokens(&postfix).unwrap();
    println!("{:#?}", formula);
}
