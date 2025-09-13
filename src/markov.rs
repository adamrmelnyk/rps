use crate::choice::Choice;
use std::collections::HashMap;

pub struct Markov {
    transitions: HashMap<Choice, HashMap<Choice, usize>>,
}

impl Markov {
    pub fn new() -> Markov {
        Markov {
            transitions: HashMap::new(),
        }
    }

    pub fn record(&mut self, from: Choice, to: Choice) {
        *self
            .transitions
            .entry(from)
            .or_default()
            .entry(to)
            .or_default() += 1;
    }

    pub fn predict(&self, choice: Choice) -> Option<Choice> {
        self.transitions
            .get(&choice)?
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&mv, _)| mv)
    }
}
