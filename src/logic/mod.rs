//! This module regroups first-order structures such as Terms and Formulas.

mod term;
mod formula;
pub mod rule;
mod sequent;

pub use formula::*;
pub use term::*;
pub use sequent::*;