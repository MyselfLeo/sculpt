use crate::context::Context;

pub enum InterpreterState {
    Idle,
    Context(Box<Context>)
}