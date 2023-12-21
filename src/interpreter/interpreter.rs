use crate::error::Error;
use crate::interpreter::command::{RuleCommandType, RuleCommandTypeDefault};
use super::{InterpreterCommand, EngineCommand};
use crate::logic::Formula;
use crate::proof::Proof;

/// Effect that a command had on the interpreter status.
/// Returned by Interpreter::execute
#[derive(Clone, Debug)]
pub enum InterpreterEffect {
    NewFormula(Formula),
    StartedProof,
    AdvancedProof,
    Nothing
}



/// The interpreter accepts [InterpreterCommand] to build proofs.
#[derive(Clone, Debug)]
pub struct Interpreter {
    pub name: String,
    pub context: Vec<Box<Formula>>,
    pub current_proof: Option<Box<Proof>>,
    command_stack: Vec<InterpreterCommand>
}

impl Interpreter {
    pub fn new(name: String) -> Interpreter {
        Interpreter { name, context: vec![], current_proof: None, command_stack: vec![] }
    }



    /// Adds a new assumption to the context & return true.
    /// If it already exists, returns false & does not add anything.
    fn add_assumption(&mut self, assumption: Box<Formula>) -> bool {
        if !self.context.contains(&assumption) {
            self.context.push(assumption);
            true
        } else { false }
    }



    pub fn get_valid_commands(&self) -> Vec<InterpreterCommand> {
        match &self.current_proof {
            None => vec![
                InterpreterCommand::EngineCommand(EngineCommand::Admit("".to_string())),
                InterpreterCommand::EngineCommand(EngineCommand::Proof("".to_string()))
            ],
            Some(p) => {
                match p.get_applicable_rules() {
                    None => vec![InterpreterCommand::EngineCommand(EngineCommand::Qed)],
                    Some(ruletype) => {
                        ruletype
                            .iter()
                            .map(|rt| RuleCommandType::from_rule(rt))
                            .flatten()
                            .map(|t| t.get_default())
                            .map(|rc| InterpreterCommand::RuleCommand(rc))
                            .collect::<Vec<_>>()
                    }
                }
            }
        }
    }



    pub fn execute(&mut self, command: InterpreterCommand) -> Result<InterpreterEffect, Error> {
        //let current_proof_cpy = self.current_proof.clone();

        match (&command, &mut self.current_proof) {

            // Adding an assumption
            (InterpreterCommand::EngineCommand(EngineCommand::Admit(_)), Some(_)) => {
                Err(Error::CommandError("Cannot add an assumption during a proof".to_string()))
            }
            (InterpreterCommand::EngineCommand(EngineCommand::Admit(s)), None) if s.is_empty() => {
                return Err(Error::ArgumentsRequired("Expected a formula".to_string()))
            }
            (InterpreterCommand::EngineCommand(EngineCommand::Admit(s)), None) => {
                let formula = match Formula::from_str(&s) {
                    Ok(f) => f,
                    Err(e) => return Err(Error::InvalidArguments(e))
                };

                self.command_stack.push(command);

                match self.add_assumption(formula.clone()) {
                    true => Ok(InterpreterEffect::NewFormula(*formula.to_owned())),
                    false => Ok(InterpreterEffect::Nothing)
                }
            }


            // Start of a proof
            (InterpreterCommand::EngineCommand(EngineCommand::Proof(_)), Some(p)) => {
                Err(Error::CommandError(format!("Already proving {}", p.goal)))
            }
            (InterpreterCommand::EngineCommand(EngineCommand::Proof(s)), None) if s.is_empty() => {
                return Err(Error::ArgumentsRequired("Expected a formula".to_string()))
            }
            (InterpreterCommand::EngineCommand(EngineCommand::Proof(s)), None) => {
                let goal = match Formula::from_str(&s) {
                    Ok(f) => f,
                    Err(e) => return Err(Error::InvalidArguments(e))
                };

                self.current_proof = Some(Box::new(Proof::start_with_antecedents(goal, self.context.clone())));
                self.command_stack.push(command);
                Ok(InterpreterEffect::StartedProof)
            }


            // Rule application to a proof
            (InterpreterCommand::RuleCommand(rule), Some(ref mut p)) => {
                match p.apply(rule.clone().to_rule()) {
                    Ok(_) => {
                        self.command_stack.push(command);
                        Ok(InterpreterEffect::AdvancedProof)
                    },
                    Err(e) => Err(e)
                }
            }


            // Ending a proof
            (InterpreterCommand::EngineCommand(EngineCommand::Qed), None) => {
                Err(Error::CommandError("No proof to finish".to_string()))
            }
            (InterpreterCommand::EngineCommand(EngineCommand::Qed), p) => {
                if p.as_ref().unwrap().is_finished() {
                    self.context.push(Box::new(p.as_ref().unwrap().goal.clone()));
                    *p = None;
                    self.command_stack.push(command);
                    Ok(InterpreterEffect::NewFormula(p.as_ref().unwrap().goal.clone()))
                } else {
                    let txt = match p.as_ref().unwrap().remaining_goals_nb() {
                        1 => "One goal has not been proven yet".to_string(),
                        e => format!("{e} goals have not been proven yet")
                    };
                    Err(Error::CommandError(txt))
                }
            }

            // Shit happened
            (r, _) => {
                Err(Error::CommandError(format!("Unable to apply command {}", r)))
            }
        }
    }


    pub fn get_current_stack(&self) -> Vec<InterpreterCommand> {
        self.command_stack.clone()
    }
}