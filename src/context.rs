use crate::inductive::Formula;

pub struct Context {
    name: String,
    context: Vec<Formula>
}