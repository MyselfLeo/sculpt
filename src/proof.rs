use std::env::set_current_dir;
use crate::inductive::Formula;
use crate::rule::Rule;
use crate::sequent::Sequent;

pub struct Proof {
    pub goal: Formula,
    current_goal: Box<Sequent>,
    sub_goals: Vec<Box<Sequent>>,

    current_sub_goal: u16
}


impl Proof {
    pub fn start(goal: Box<Formula>) -> Proof {
        Proof {
            goal: *goal.clone(),
            current_goal: Box::new(Sequent::new(vec![], goal)),
            sub_goals: vec![],
            current_sub_goal: 0
        }
    }


    pub fn get_sub_goal_idx(&self) -> u16 {
        self.current_sub_goal
    }
    pub fn get_subgoals(&self) -> &Vec<Box<Sequent>> {
        &self.sub_goals
    }


    pub fn apply(&mut self, rule: Box<dyn Rule>) -> Result<(), String> {
        match self.current_goal.apply_rule(rule) {
            Ok(res) => {

            }
            Err(_) => Err(format!("Unable to apply rule '{rule}'"))
        }
    }
}