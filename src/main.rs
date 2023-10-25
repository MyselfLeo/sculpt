mod inductive;
mod parser;

fn main() {
    let formula_str = "(~A \\/ B) => (C => D) /\\ C";
    let tokens = parser::lex(formula_str.to_string()).unwrap();
    let postfix = parser::infix_to_postfix(&tokens).unwrap();
    let formula = parser::formula_from_tokens(&postfix).unwrap();
    println!("{:#?}", formula);
}
