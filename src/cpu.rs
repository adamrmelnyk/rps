use core::panic;

use crate::choice::Choice;
use crate::markov::Markov;
use rand::prelude::IndexedRandom;

pub struct Cpu {
    memory: Vec<Choice>,
    markov: Markov,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strat {
    Random,
    BeatLast,
    Markov,
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu {
            markov: Markov::new(),
            memory: Vec::new(),
        };
    }

    pub fn memorize(&mut self, player_choice: Choice) {
        if player_choice != Choice::Quit {
            if self.memory.is_empty() {
                self.markov.record(self.random_choice(), player_choice);
            } else {
                let last_move: Choice = *self.memory.last().unwrap();
                self.markov.record(last_move, player_choice);
            }
            self.memory.push(player_choice);
        }
    }

    #[allow(dead_code)]
    pub fn choose_via_random_strat(&mut self) -> Choice {
        let strats: [Strat; 3] = [Strat::Random, Strat::BeatLast, Strat::Markov];
        let mut rng = rand::rng();
        let chosen_strat = *strats.choose(&mut rng).unwrap();
        match chosen_strat {
            Strat::Random => self.random_choice(),
            Strat::BeatLast => self.beat_last_choice(),
            Strat::Markov => self.markov_choice(),
        }
    }

    pub fn beat_last_choice(&mut self) -> Choice {
        return match self.memory.last() {
            Some(last_player_choice) => match last_player_choice {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
                _ => panic!("This should never happen, unknown choice memorized"),
            },
            None => self.random_choice(),
        };
    }

    pub fn random_choice(&self) -> Choice {
        let choices: [Choice; 3] = [Choice::Rock, Choice::Paper, Choice::Scissors];
        let mut rng = rand::rng();
        *choices.choose(&mut rng).unwrap()
    }

    pub fn markov_choice(&mut self) -> Choice {
        match self.memory.last() {
            Some(last_player_choice) => self
                .markov
                .predict(*last_player_choice)
                .unwrap_or_else(|| self.random_choice())
                .counter_move(),
            None => self.random_choice(),
        }
    }
}
