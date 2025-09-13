use crate::choice::Choice;

#[derive(Debug)]
pub enum Winner {
    User,
    Cpu,
    Draw,
}

pub fn determine_winner(player_choice: Choice, cpu_choice: Choice) -> Winner {
    match (player_choice, cpu_choice) {
        (Choice::Scissors, Choice::Paper) => Winner::User,
        (Choice::Rock, Choice::Scissors) => Winner::User,
        (Choice::Paper, Choice::Rock) => Winner::User,
        (user, cpu) if user == cpu => Winner::Draw,
        _ => Winner::Cpu,
    }
}
