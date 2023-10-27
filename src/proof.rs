use std::collections::VecDeque;
use crate::inductive::Formula;
use crate::rule::Rule;
use crate::sequent::Sequent;

pub struct Proof {
    pub goal: Formula,
    current_goal: Option<Box<Sequent>>,
    sub_goals: VecDeque<Box<Sequent>>,
    step: u16
}


impl Proof {
    pub fn start(goal: Box<Formula>) -> Proof {
        let goal_seq = Sequent::new(vec![], goal.clone());

        Proof {
            goal: *goal.clone(),
            current_goal: Some(Box::new(goal_seq)),
            sub_goals: VecDeque::new(),
            step: 0
        }
    }


    pub fn get_subgoals(&self) -> &VecDeque<Box<Sequent>> {
        &self.sub_goals
    }


    pub fn apply(&mut self, rule: Rule) -> Result<(), String> {
        let crrt_goal = match &self.current_goal {
            None => return Err("Proof finished".to_string()),
            Some(seq) => seq
        };

        match rule.apply(crrt_goal) {
            Ok(res) => {
                for new_seq in res.into_iter().rev() {
                    self.sub_goals.insert(0, Box::new(new_seq))
                }

                self.current_goal = self.sub_goals.pop_front();
                self.step += 1;

                Ok(())
            }
            Err(_) => Err(format!("Unable to apply rule '{rule}'"))
        }
    }


    pub fn print(&self) {
        match &self.current_goal {
            None => println!("Goal: {} (finished)", self.goal),
            Some(cg) => {
                // sub goals + current goal (1)
                println!("Goal: {}", self.goal);

                match self.sub_goals.len() + 1 {
                    1 => println!("Step {}  (1 sub-goal remaining)", self.step),
                    x => println!("Step {}  ({} sub-goals remaining)", self.step, x)
                };

                println!("│");
                println!("{}", cg);
            }
        }
    }


    pub fn is_finished(&self) -> bool {
        self.current_goal.is_none()
    }
}