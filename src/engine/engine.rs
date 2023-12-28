use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::error::Error;
use crate::engine::command::{RuleCommandType, RuleCommandTypeDefault};
use super::{EngineCommand, ContextCommand};
use crate::logic::{Formula, Term};
use crate::logic::rule::Rule;
use crate::proof::Proof;


//struct TermSignature {
//    arity: usize
//}


#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum ValueType {
    Theorem,
    Relation,
    Term
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Theorem => write!(f, "theorem"),
            ValueType::Relation => write!(f, "relation"),
            ValueType::Term => write!(f, "term")
        }
    }
}



#[derive(Clone, Debug)]
pub struct Context {
    pub theorems: HashMap<String, Box<Formula>>,
    pub relations: HashMap<String, ()>,  // Those might change if we make them typed or with an arity
    pub terms: HashMap<String, ()>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            theorems: HashMap::new(),
            relations: HashMap::new(),
            terms: HashMap::new(),
        }
    }


    /// Return the type of the ident, if defined in the context.
    /// If not, return None.
    pub fn get_type(&self, ident: &str) -> Option<ValueType> {
        if self.theorems.contains_key(ident) {
            Some(ValueType::Theorem)
        }
        else if self.relations.contains_key(ident) {
            Some(ValueType::Relation)
        }
        else if self.terms.contains_key(ident) {
            Some(ValueType::Term)
        }
        else {None}
    }


    /// Push a new theorem to the context
    pub fn add_theorem(&mut self, ident: &str, f: Box<Formula>) -> Result<(), Error> {
        self.expect_not_defined(ident)?;

        match self.theorems.insert(ident.to_string(), f) {
            None => Ok(()),
            Some(_) => unreachable!()
        }
    }

    /// Push a new relation to the context
    pub fn add_relation(&mut self, ident: &str) -> Result<(), Error> {
        self.expect_not_defined(ident)?;

        match self.relations.insert(ident.to_string(), ()) {
            None => Ok(()),
            Some(_) => unreachable!()
        }
    }

    /// Push a new term to the context
    pub fn add_term(&mut self, ident: &str) -> Result<(), Error> {
        self.expect_not_defined(ident)?;

        match self.terms.insert(ident.to_string(), ()) {
            None => Ok(()),
            Some(_) => unreachable!()
        }
    }



    /// Return an Err if the ident is already defined in the context.
    pub fn expect_not_defined(&self, ident: &str) -> Result<(), Error> {
        match self.get_type(ident) {
            Some(ValueType::Theorem) => Err(Error::AlreadyExists(format!("'{ident}' is already a theorem"))),
            Some(ValueType::Relation) => Err(Error::AlreadyExists(format!("'{ident}' is already a relation"))),
            Some(ValueType::Term) => Err(Error::AlreadyExists(format!("'{ident}' is already a term"))),
            None => Ok(())
        }
    }
}





/// Effect that a command had on the engine status.
/// Returned by Engine::execute
#[derive(Clone, Debug)]
pub enum EngineEffect {
    NewTheorem(String, Formula),
    DefinedRelation(String),
    DefinedTerm(String),
    EnteredProofMode,
    ExitedProofMode
}



/// The engine accepts [EngineCommand] to build proofs.
#[derive(Clone, Debug)]
pub struct Engine {
    pub name: String,
    pub context: Context,

    pub current_proof: Option<(String, Box<Proof>)>,
    command_stack: Vec<EngineCommand>
}

impl Engine {
    pub fn new(name: String) -> Engine {
        Engine {
            name,
            context: Context::new(),
            current_proof: None,
            command_stack: vec![]
        }
    }




    /// Check that the relation & term names used in the formula matches the current context.
    /// If a relation/term/variable uses an identifier already defined and not of the same type, returns an Err.
    /// If a relation/term uses an identifier that is not yet defined & forgiving is set to true, define it silently.
    pub fn check_formula(&mut self, f: &Formula, forgiving: bool) -> Result<Vec<EngineEffect>, Error> {
        let mut effects = vec![];
        match f {
            Formula::Relation(n, terms) => {
                if self.context.terms.contains_key(n) {
                    Err(Error::InvalidFormula(f.clone(), format!("'{n}' used as a relation but defined as a term")))
                }
                else {
                    // Forgiving part: define if does not exists
                    if !self.context.relations.contains_key(n) && forgiving {
                        self.context.add_relation(n)?;
                        effects.push(EngineEffect::DefinedRelation(n.clone()));
                    }

                    for t in terms {
                        effects.append(&mut self.check_term(t.as_ref(), forgiving)?)
                    };

                    Ok(effects)
                }

            }

            Formula::Forall(n, f) | Formula::Exists(n, f) => {
                self.context.expect_not_defined(n)?;
                self.check_formula(f, forgiving)
            }

            Formula::And(l1, l2) | Formula::Or(l1, l2) => {
                effects.append(&mut self.check_formula(l1.as_ref(), forgiving)?);
                effects.append(&mut self.check_formula(l2.as_ref(), forgiving)?);
                Ok(effects)
            }

            Formula::Implies(l1, l2) => {
                effects.append(&mut self.check_formula(l1.as_ref(), forgiving)?);
                effects.append(&mut self.check_formula(l2.as_ref(), forgiving)?);
                Ok(effects)
            }

            Formula::Not(l) => {
                effects.append(&mut self.check_formula(l.as_ref(), forgiving)?);
                Ok(effects)
            }

            _ => Ok(vec![])
        }
    }


    /// Same as [check_formula] but for terms
    fn check_term(&mut self, t: &Term, forgiving: bool) -> Result<Vec<EngineEffect>, Error> {
        let mut effects = vec![];
        match t {
            Term::Variable(v) => {
                if self.context.relations.contains_key(v) {
                    Err(Error::InvalidTerm(t.clone(), format!("'{v}' used as a term but defined as a relation")))
                }
                else {
                    Ok(vec![])
                }
            }
            Term::Function(v, terms) => {
                if self.context.relations.contains_key(v) {
                    Err(Error::InvalidTerm(t.clone(), format!("'{v}' used as a term but defined as a relation")))
                }
                else {
                    // Forgiving part
                    if !self.context.terms.contains_key(v) && forgiving {
                        self.context.add_term(v)?;
                        effects.push(EngineEffect::DefinedTerm(v.to_string()));
                    }

                    for t in terms {
                        effects.append(&mut self.check_term(t.as_ref(), forgiving)?)
                    };

                    Ok(effects)
                }
            }
        }
    }



    pub fn get_valid_commands(&self) -> Vec<EngineCommand> {
        match &self.current_proof {
            None => vec![
                EngineCommand::ContextCommand(ContextCommand::Theorem("".to_string(), Box::default())),
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
                            .flat_map(RuleCommandType::from_rule)
                            .map(|t| t.get_default())
                            .map(EngineCommand::RuleCommand)
                            .collect::<Vec<_>>()
                    }
                }
            }
        }
    }



    pub fn execute(&mut self, command: EngineCommand) -> Result<Vec<EngineEffect>, Error> {
        let mut effects = vec![];
        let proof_cpy = self.current_proof.clone();

        match (&command, proof_cpy) {
            // Start of a proof
            (EngineCommand::ContextCommand(ContextCommand::Theorem(..)), Some((_, p))) => {
                return Err(Error::CommandError(format!("Already proving {}", p.goal)))
            }
            (EngineCommand::ContextCommand(ContextCommand::Theorem(name, goal)), None) => {
                self.context.expect_not_defined(name)?;

                // Check that the formula is valid
                self.check_formula(goal, true)?;

                let proof = Proof::start(goal.clone());

                self.current_proof = Some((name.clone(), Box::new(proof)));
                self.command_stack.push(command);
                effects.push(EngineEffect::EnteredProofMode);
            }


            (EngineCommand::ContextCommand(ContextCommand::Use(s)), Some((_, p))) => {
                if p.is_finished() {
                    return Err(Error::CommandError("Proof is finished".to_string()))
                }

                match self.context.theorems.get(s) {
                    None => return Err(Error::CommandError(format!("Unknown theorem {s}"))),
                    Some(thm) => {
                        let (_, curr_proof) = self.current_proof.as_mut().unwrap();
                        curr_proof.add_antecedent(thm.clone())?;
                    }
                }
            }

            // Rule application to a proof
            (EngineCommand::RuleCommand(rule), Some(_)) => {
                // Check that the formulas are valid
                match rule.to_rule() {
                    Rule::Trans(f)
                    | Rule::And(_, f)
                    | Rule::FromOr(f)
                    | Rule::Consider(f)
                    | Rule::ExFalso(f) => {
                        let mut e = self.check_formula(&f, true)?;
                        effects.append(&mut e)
                    },
                    _ => ()
                };

                let (_, curr_proof) = self.current_proof.as_mut().unwrap();
                match curr_proof.apply(rule.clone().to_rule()) {
                    Ok(_) => {
                        self.command_stack.push(command);
                    },
                    Err(e) => return Err(e)
                }
            }


            // Ending a proof using Qed
            (EngineCommand::ContextCommand(ContextCommand::Qed), None) => {
                return Err(Error::CommandError("Not in proof mode".to_string()))
            }
            (EngineCommand::ContextCommand(ContextCommand::Qed), proof) => {
                let proof_clone = proof.clone();

                match proof_clone {
                    None => unreachable!(),
                    Some((n, p)) => {
                        if p.is_finished() {
                            self.context.add_theorem(&n, Box::new(p.goal.clone()))?;
                            self.current_proof = None;
                            self.command_stack.push(command);
                            effects.push(EngineEffect::ExitedProofMode);
                            effects.push(EngineEffect::NewTheorem(n, p.goal));
                        }
                        else {
                            let txt = match p.remaining_goals_nb() {
                                1 => "One goal has not been proven yet".to_string(),
                                n => format!("{n} goals have not been proven yet")
                            };
                            return Err(Error::CommandError(txt))
                        }
                    }
                }
            }


            // Ending a proof with admit
            (EngineCommand::ContextCommand(ContextCommand::Admit), None) => {
                return Err(Error::CommandError("Not in proof mode".to_string()))
            }
            (EngineCommand::ContextCommand(ContextCommand::Admit), proof) => {
                let proof_clone = proof.clone();

                match proof_clone {
                    None => unreachable!(),
                    Some((n, p)) => {
                        self.context.add_theorem(&n, Box::new(p.goal.clone()))?;
                        self.current_proof = None;
                        self.command_stack.push(command);
                        effects.push(EngineEffect::ExitedProofMode);
                        effects.push(EngineEffect::NewTheorem(n, p.goal));
                    }
                }
            }


            // Shit happened
            (r, _) => {
                return Err(Error::CommandError(format!("Unable to apply command {}", r)))
            }
        };

        Ok(effects)
    }


    pub fn get_current_stack(&self) -> Vec<EngineCommand> {
        self.command_stack.clone()
    }
}