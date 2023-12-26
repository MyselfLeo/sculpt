use std::collections::HashMap;
use itertools::Itertools;
use crate::error::Error;
use crate::engine::command::{RuleCommandType, RuleCommandTypeDefault};
use super::{EngineCommand, ContextCommand};
use crate::logic::Formula;
use crate::proof::Proof;
use crate::syntax::lexer::Context;

/// Effect that a command had on the engine status.
/// Returned by Engine::execute
#[derive(Clone, Debug)]
pub enum EngineEffect {
    NewTheorem(Formula),
    EnteredProofMode,
    ExitedProofMode,
    Nothing
}



/// The engine accepts [EngineCommand] to build proofs.
#[derive(Clone, Debug)]
pub struct Engine {
    pub name: String,
    pub namespace: Context,
    pub context: HashMap<String, Box<Formula>>,
    pub current_proof: Option<(String, Box<Proof>)>,
    command_stack: Vec<EngineCommand>
}

impl Engine {
    pub fn new(name: String) -> Engine {
        Engine { name, namespace: Context::new(), context: HashMap::new(), current_proof: None, command_stack: vec![] }
    }



    /*/// Adds a new assumption to the context & return true.
    /// If it already exists, returns false & does not add anything.
    fn add_assumption(&mut self, name: String, assumption: Box<Formula>) -> bool {
        if !self.context.values().contains(&assumption) {
            self.context.push(assumption);
            true
        } else { false }
    }*/



    pub fn get_valid_commands(&self) -> Vec<EngineCommand> {
        match &self.current_proof {
            None => vec![
                EngineCommand::ContextCommand(ContextCommand::Theorem("".to_string(), "".to_string())),
            ],
            Some((_, p)) => {
                match p.get_applicable_rules() {
                    None => vec![
                        EngineCommand::ContextCommand(ContextCommand::Qed),
                        EngineCommand::ContextCommand(ContextCommand::Admit),
                    ],
                    Some(ruletype) => {
                        ruletype
                            .iter()
                            .map(|rt| RuleCommandType::from_rule(rt))
                            .flatten()
                            .map(|t| t.get_default())
                            .map(|rc| EngineCommand::RuleCommand(rc))
                            .collect::<Vec<_>>()
                    }
                }
            }
        }
    }



    pub fn execute(&mut self, command: EngineCommand) -> Result<EngineEffect, Error> {
        //let current_proof_cpy = self.current_proof.clone();

        match (&command, &mut self.current_proof) {

            /*// Adding an assumption
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
            }*/


            // Start of a proof
            (EngineCommand::ContextCommand(ContextCommand::Theorem(..)), Some((_, p))) => {
                Err(Error::CommandError(format!("Already proving {}", p.goal)))
            }
            (EngineCommand::ContextCommand(ContextCommand::Theorem(name, form)), None) => {
                if self.context.contains_key(name) {
                    return Err(Error::AlreadyExists(name.clone()))
                }

                let goal = match Formula::from_str(&form) {
                    Ok(f) => f,
                    Err(e) => return Err(Error::InvalidArguments(e))
                };

                let proof = Proof::start(goal);

                self.current_proof = Some((name.clone(), Box::new(proof)));
                self.command_stack.push(command);
                Ok(EngineEffect::EnteredProofMode)
            }


            (EngineCommand::ContextCommand(ContextCommand::Use(s)), Some((_, ref mut p))) => {
                if p.is_finished() {
                    return Err(Error::CommandError("Proof is finished".to_string()))
                }

                match self.context.get(s) {
                    None => return Err(Error::CommandError(format!("Unknown theorem {s}"))),
                    Some(thm) => {
                        p.add_antecedent(thm.clone())?;
                        Ok(EngineEffect::Nothing)
                    }
                }
            }

            // Rule application to a proof
            (EngineCommand::RuleCommand(rule), Some((_, ref mut p))) => {
                match p.apply(rule.clone().to_rule()) {
                    Ok(_) => {
                        self.command_stack.push(command);
                        Ok(EngineEffect::ExitedProofMode)
                    },
                    Err(e) => Err(e)
                }
            }


            // Ending a proof using Qed
            (EngineCommand::ContextCommand(ContextCommand::Qed), None) => {
                Err(Error::CommandError("Not in proof mode".to_string()))
            }
            (EngineCommand::ContextCommand(ContextCommand::Qed), proof) => {
                let proof_clone = proof.clone();

                match proof_clone {
                    None => unreachable!(),
                    Some((n, p)) => {
                        if p.is_finished() {
                            self.context.insert(n, Box::new(p.goal.clone()));
                            *proof = None;
                            self.command_stack.push(command);
                            Ok(EngineEffect::NewTheorem(p.goal))
                        }
                        else {
                            let txt = match p.remaining_goals_nb() {
                                1 => "One goal has not been proven yet".to_string(),
                                n => format!("{n} goals have not been proven yet")
                            };
                            Err(Error::CommandError(txt))
                        }
                    }
                }
            }


            // Ending a proof with admit
            (EngineCommand::ContextCommand(ContextCommand::Admit), None) => {
                Err(Error::CommandError("Not in proof mode".to_string()))
            }
            (EngineCommand::ContextCommand(ContextCommand::Admit), proof) => {
                let proof_clone = proof.clone();

                match proof_clone {
                    None => unreachable!(),
                    Some((n, p)) => {
                        self.context.insert(n, Box::new(p.goal.clone()));
                        *proof = None;
                        self.command_stack.push(command);
                        Ok(EngineEffect::NewTheorem(p.goal))
                    }
                }
            }


            // Shit happened
            (r, _) => {
                Err(Error::CommandError(format!("Unable to apply command {}", r)))
            }
        }
    }


    pub fn get_current_stack(&self) -> Vec<EngineCommand> {
        self.command_stack.clone()
    }
}