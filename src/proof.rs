use std::collections::VecDeque;
use crate::inductive::{Formula, FormulaType, FormulaTyped};
use crate::repl::ReplCommand;
use crate::rule::{Rule, RuleType};
use crate::sequent::Sequent;


#[derive(Clone)]
pub struct Proof {
    pub goal: Formula,
    current_goal: Option<Box<Sequent>>,
    sub_goals: VecDeque<Box<Sequent>>,
    pub step: u16,
    pub previous_state: Option<Box<Proof>>
}


impl Proof {
    pub fn start(goal: Box<Formula>) -> Proof {
        let goal_seq = Sequent::new(vec![], goal.clone());

        Proof {
            goal: *goal.clone(),
            current_goal: Some(Box::new(goal_seq)),
            sub_goals: VecDeque::new(),
            step: 0,
            previous_state: None
        }
    }



    pub fn apply(&mut self, rule: Rule) -> Result<(), String> {
        let crrt_goal = match &self.current_goal {
            None => return Err("Proof finished".to_string()),
            Some(seq) => seq
        };

        let res = rule.apply(crrt_goal)?;

        self.previous_state = Some(Box::new(self.clone())); // Allow undo operation

        for new_seq in res.into_iter().rev() {
            self.sub_goals.insert(0, Box::new(new_seq))
        }

        self.current_goal = self.sub_goals.pop_front();
        self.step += 1;

        Ok(())
    }




    pub fn get_suggestions(&self) -> Option<Vec<Rule>> {
        match &self.current_goal {
            None => None,
            Some(e) => {
                let mut res = vec![];


                // Suggestion 1: if the consequent is in the antecedents, use axiom
                if e.antecedents.contains(&e.consequent) {res.push(Rule::Axiom);}


                // Suggestion 2: Intro when possible
                if [FormulaType::Implies, FormulaType::Forall].contains(&e.consequent.get_type()) {
                    res.push(Rule::Intro);
                }


                // Suggestion 3: Consume \/ if in the antecedents
                res.append(&mut
                    e.antecedents.iter()
                        .filter(|f| f.get_type() == FormulaType::Or)
                        .map(|f| Rule::FromOr(f.to_string()))
                        .collect::<Vec<_>>()
                );


                Some(res)
            }
        }
    }





    pub fn print(&self) {
        // sub goals + current goal (1)
        if self.is_finished() {
            println!("Goal: {} (finished)", self.goal);
            return;
        }

        println!("Goal: {}", self.goal);

        match self.sub_goals.len() + 1 {
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
}