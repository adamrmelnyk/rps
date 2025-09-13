use crate::choice;
use crate::cpu::Cpu;
use crate::display::{
    display_game_play, display_help, display_intro_text, display_score, display_winner,
};
use crate::outcome::{determine_winner, Winner};
use choice::Choice;
use std::io;

pub struct Game {
    user_wins: i32,
    cpu_wins: i32,
    cpu_player: Cpu,
    max_wins: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            user_wins: 0,
            cpu_wins: 0,
            cpu_player: Cpu::new(),
            max_wins: 10,
        }
    }

    pub fn play(&mut self) {
        println!("Let's play Rock, Paper, Scissors!");
        println!("The first player to win 🔟 rounds, wins!");
        loop {
            let has_winner = self.game_round();
            match has_winner {
                Some(winner) => {
                    match winner {
                        Winner::User => self.user_wins += 1,
                        Winner::Cpu => self.cpu_wins += 1,
                        Winner::Draw => (),
                    }
                    display_score(self.user_wins, self.cpu_wins, winner);
                }
                None => (), // Do nothing
            }
            match self.check_winner() {
                Some(winner) => {
                    display_winner(winner);
                    Self::quit_game();
                }
                None => (),
            }
        }
    }

    fn user_won(&self) -> bool {
        return self.user_wins == self.max_wins;
    }

    fn cpu_won(&self) -> bool {
        return self.cpu_wins == self.max_wins;
    }

    fn check_winner(&self) -> Option<Winner> {
        if self.user_won() {
            Some(Winner::User)
        } else if self.cpu_won() {
            Some(Winner::Cpu)
        } else {
            None
        }
    }

    fn quit_game() {
        println!("Thanks for playing!");
        std::process::exit(0);
    }

    fn get_user_input() -> Result<Choice, &'static str> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        match input.trim().to_lowercase().as_str() {
            "r" | "rock" | "🪨" => Ok(Choice::Rock),
            "p" | "paper" | "📄" => Ok(Choice::Paper),
            "s" | "scissors" | "✂️" => Ok(Choice::Scissors),
            "q" | "quit" => Ok(Choice::Quit),
            "h" | "help" | "info" | "man" => Ok(Choice::Help),
            _ => Err("Invalid choice"),
        }
    }

    fn game_round(&mut self) -> Option<Winner> {
        display_intro_text();

        let player_input = Self::get_user_input();
        match player_input {
            Ok(player_choice) => match player_choice {
                Choice::Quit => {
                    Self::quit_game();
                    None
                }
                Choice::Help => {
                    display_help();
                    None
                }
                Choice::Rock | Choice::Paper | Choice::Scissors => {
                    let cpu_choice = self.cpu_player.markov_choice();
                    self.cpu_player.memorize(player_choice);

                    display_game_play(player_choice, cpu_choice);
                    Some(determine_winner(player_choice, cpu_choice))
                }
            },
            Err(_) => {
                println!("🙅 Are you trying to cheat? Please enter a valid option 🙅 \n\n");
                None
            }
        }
    }
}
