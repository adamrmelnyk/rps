use crate::choice::Choice;
use crate::outcome::Winner;
use chrono::Datelike;

fn christmas_print(user_wins: i32, cpu_wins: i32) {
    println!("🎅🎄🎁🎅🎄🎁🎅🎄🎁🎅🎄🎁🎅🎄");
    println!("🎁  Score: User {}, CPU {}  🎁", user_wins, cpu_wins);
    println!("🎅🎄🎁🎅🎄🎁🎅🎄🎁🎅🎄🎁🎅🎄\n");
}

fn halloween_print(user_wins: i32, cpu_wins: i32) {
    println!("🎃🦇🎃🦇🎃🦇🎃🦇🎃🦇🎃🦇🎃🦇");
    println!("🦇  Score: User {}, CPU {}  🎃", user_wins, cpu_wins);
    println!("🎃🦇🎃🦇🎃🦇🎃🦇🎃🦇🎃🦇🎃🦇\n");
}

fn thanksgiving_print(user_wins: i32, cpu_wins: i32) {
    println!("🦃🌽🦃🌽🦃🌽🦃🌽🦃🌽🦃🌽🦃🌽");
    println!("🌽 Score: User {}, CPU {}  🌽", user_wins, cpu_wins);
    println!("🦃🌽🦃🌽🦃🌽🦃🌽🦃🌽🦃🌽🦃🌽");
}

fn default_print(user_wins: i32, cpu_wins: i32) {
    println!("🎲🎲🎲🎲🎲🎲🎲🎲🎲🎲🎲🎲🎲");
    println!("🎲 Score: User {}, CPU {} 🎲", user_wins, cpu_wins);
    println!("🎲🎲🎲🎲🎲🎲🎲🎲🎲🎲🎲🎲🎲\n");
}

pub fn display_score(user_wins: i32, cpu_wins: i32, winner: Winner) {
    match winner {
        Winner::User => println!("🎉🎉🎉 You won! 🎉🎉🎉\n"),
        Winner::Cpu => println!("🤖🤖🤖 I won! 🤖🤖🤖\n"),
        Winner::Draw => println!("👔👔👔 Draw! 👔👔👔\n"),
    };
    let current_date = chrono::Utc::now();
    match current_date.month() {
        10 => halloween_print(user_wins, cpu_wins),
        11 => thanksgiving_print(user_wins, cpu_wins),
        12 => christmas_print(user_wins, cpu_wins),
        _ => default_print(user_wins, cpu_wins),
    }
}

pub fn display_winner(winner: Winner) {
    clear_screen_set_cursor_to_origin();
    match winner {
        Winner::User => println!("🎉🎉🎉 You Won!!!! 🎉🎉🎉"),
        Winner::Cpu => println!("🤖🤖🤖 The CPU won!!! Better luck next time. 🤖🤖🤖"),
        Winner::Draw => (),
    }
}

pub fn display_game_play(player_choice: Choice, cpu_choice: Choice) {
    println!(
        "Player: {}  CPU: {}",
        player_choice.emoji(),
        cpu_choice.emoji()
    );
}

fn clear_screen_set_cursor_to_origin() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn display_intro_text() {
    println!("Enter Rock, Paper, Scissors: ");
    println!("You may also press 'q' to quit, and 'h' to help");
    clear_screen_set_cursor_to_origin();
}

pub fn display_help() {
    display_intro_text();
    println!(
        "\nUSAGE: help\n\
    \n - 'r' | 'rock' | '🪨'
    \n - 'p' | 'paper' | '📄'
    \n - 's' | 'scissors' | '✂️'
    \n - 'q' | 'quit',
    \n - 'h' | 'help' | 'info' | 'man'
    \n"
    );
}
