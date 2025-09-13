use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
    Quit,
    Help,
}

impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Choice::Rock => write!(f, "Rock"),
            Choice::Paper => write!(f, "Paper"),
            Choice::Scissors => write!(f, "Scissors"),
            Choice::Quit => write!(f, "Quit"),
            Choice::Help => write!(f, "Help"),
        }
    }
}

impl Choice {
    pub fn emoji(&self) -> &'static str {
        match &self {
            Choice::Rock => "🪨",
            Choice::Paper => "📄",
            Choice::Scissors => "✂️",
            Choice::Quit | Choice::Help => "",
        }
    }

    #[allow(dead_code)]
    pub fn to_char(&self) -> char {
        match &self {
            Choice::Rock => 'r',
            Choice::Paper => 'p',
            Choice::Scissors => 's',
            Choice::Quit => 'q',
            Choice::Help => 'h',
        }
    }
    pub fn counter_move(&self) -> Choice {
        match &self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
            Choice::Quit | Choice::Help => panic!("Should never happen"),
        }
    }
}
