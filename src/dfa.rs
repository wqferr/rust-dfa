use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub trait State: Hash + Eq + Copy {}
pub trait Event: Hash + Eq {}

#[derive(Debug)]
pub enum Err {
    IllegalTransition,
}
pub type Result<T> = std::result::Result<T, Err>;

pub struct Machine<S: State + 'static, E: Event> {
    state: S,
    transitions: HashMap<(S, E), S>,
}

impl<S: State, E: Event> Machine<S, E> {
    pub fn new<I>(initial_state: S, transition_list: I) -> Self
    where
        I: Iterator<Item = ((S, E), S)>,
    {
        Machine {
            state: initial_state,
            transitions: transition_list.collect(),
        }
    }

    pub fn feed(&mut self, event: E) -> Result<()> {
        if let Some(next_state) = self.transitions.get(&(self.state, event)) {
            self.state = *next_state;
            Ok(())
        } else {
            Err(Err::IllegalTransition)
        }
    }

    pub fn state(&self) -> S {
        self.state
    }
}
