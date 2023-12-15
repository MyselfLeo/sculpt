use crate::logic::Formula;
use crate::proof::Proof;

#[derive(Clone)]
pub struct Context {
    name: String,
    context: Vec<Box<Formula>>,
    current_proof: Option<Box<Proof>>,
    previous_state: Option<Box<Context>>
}

impl Context {
    pub fn new(name: String) -> Context {
        Context { name, context: vec![], current_proof: None, previous_state: None }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_context(&self) -> Vec<Box<Formula>> {
        self.context.clone()
    }

    /// Add a new assumption to the context. If it already exists,
    /// does not add anything.
    pub fn add_assumption(&mut self, assumption: Box<Formula>) {
        if !self.context.contains(&assumption) {
            self.context.push(assumption)
        }
    }


    /// Start a proof process in this context.
    /// Returns an error if a proof is already in the works
    pub fn start_proof(&mut self, goal: Box<Formula>) -> Result<(), String> {
        match self.current_proof {
            None => {
                self.current_proof = Some(Box::new(Proof::start_with_antecedents(goal, self.context.clone())));
                Ok(())
            }
            Some(_) => {
                Err("Cannot start a proof if the previous one is not finished".to_string())
            }
        }
    }


    /// If a proof is currently in the works and proven (no more goals), add its initial goal
    /// to the context and finish the proof.
    pub fn validate_proof(&mut self) -> Result<(), String> {
        match &self.current_proof {
            None => {
                Err("No current proof".to_string())
            }
            Some(p) => {
                if p.is_finished() {
                    self.context.push(Box::new(p.goal.clone()));
                    Ok(())
                } else {
                    Err("Some goals have not been proven yet".to_string())
                }
            }
        }
    }


    pub fn get_proof(&mut self) -> Option<&mut Proof> {
        match self.current_proof {
            None => None,
            Some(ref mut p) => Some(p.as_mut())
        }
    }

}