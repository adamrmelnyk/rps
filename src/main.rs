mod choice;
mod cpu;
mod display;
mod game;
mod markov;
mod outcome;

use game::Game;

fn main() {
    let mut g: Game = Game::new();
    g.play();
}
