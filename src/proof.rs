use std::collections::VecDeque;
use crate::error::Error;
use crate::logic::{Formula, Sequent};
use crate::logic::rule::{Rule, RuleType};


#[derive(Clone, Debug)]
pub struct Proof {
    pub goal: Formula,
    current_goal: Option<Box<Sequent>>,
    sub_goals: VecDeque<Box<Sequent>>,
    pub step: u16
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


    /*pub fn start_with_antecedents(goal: Box<Formula>, antecedents: Vec<Box<Formula>>) -> Proof {
        let goal_seq = Sequent::new(antecedents, goal.clone());

        Proof {
            goal: *goal.clone(),
            current_goal: Some(Box::new(goal_seq)),
            sub_goals: VecDeque::new(),
            step: 0
        }
    }*/


    pub fn add_antecedent(&mut self, ante: Box<Formula>) -> Result<(), Error> {
        match self.current_goal {
            None => Err(Error::CommandError("Proof is finished".to_string())),
            Some(ref mut cg) => {
                if !cg.antecedents.contains(&ante) {
                    cg.antecedents.push(*ante);
                }
                Ok(())
            }
        }
    }


    pub fn apply(&mut self, rule: Rule) -> Result<(), Error> {
        let crrt_goal = match &self.current_goal {
            None => return Err(Error::InvalidCommand("Proof finished".to_string())),
            Some(seq) => seq
        };

        let res = rule.apply(crrt_goal)?;

        //self.previous_state = Some(Box::new(self.clone())); // Allow undo operation

        for new_seq in res.into_iter().rev() {
            self.sub_goals.insert(0, Box::new(new_seq))
        }

        self.current_goal = self.sub_goals.pop_front();
        self.step += 1;

        Ok(())
    }



    pub fn get_applicable_rules(&self) -> Option<Vec<RuleType>> {
        Some(self.current_goal.clone()?.get_applicable_rules())
    }


    pub fn print(&self) {
        // sub goals + current goal (1)
        if self.is_finished() {
            println!("Goal: {} (finished)", self.goal);
            return;
        }

        println!("Goal: {}", self.goal);

        match self.remaining_goals_nb() {
            1 => println!("Step {}  (1 sub-goal remaining)", self.step),
            x => println!("Step {}  ({} sub-goals remaining)", self.step, x)
        };

        println!("│");
        
        match &self.current_goal {
            Some(cg) => println!("{}", cg),
            None => {
                println!("│──────────────────────────");
                println!("│ (no more goals)")
            },
        }
    }


    pub fn is_finished(&self) -> bool {
        self.current_goal.is_none()
    }

    pub fn remaining_goals_nb(&self) -> usize {
        self.sub_goals.len() + 1
    }
}