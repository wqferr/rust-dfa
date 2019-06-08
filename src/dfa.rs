use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub enum MyErr {
    IllegalTransition,
}
pub type MyResult<T> = Result<T, MyErr>;

pub trait State: Hash + Eq {}
pub trait Event: Hash + Eq {}
impl State for i32 {}
impl Event for i32 {}

pub struct Machine<S: State + 'static, E: Event + 'static> {
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

    pub fn feed(&mut self, event: E) -> MyResult<()> {
        if let Some(next_state) = self.transitions.get(&(self.state, event)) {
            self.state = *next_state;
            Ok(())
        } else {
            Err(MyErr::IllegalTransition)
        }
    }

    pub fn state(&self) -> S {
        self.state
    }
}
