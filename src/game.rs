use ratatui::{widgets::*};




#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Outcome {
    Correct,
    Incorrect,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Input {
    pub char: char,
    pub outcome: Outcome,
}

#[derive(Debug)]
pub struct Game {
    pub prompt: String,
    pub prompt_zy: String,
    pub input: Vec<Input>,
    pub cursor_pos: usize,
}

impl Game {
    pub fn new(
        prompt: String,
        prompt_zy: String,
    ) -> Game {
        Game {
            prompt,
            prompt_zy,
            input: Vec::new(),
            cursor_pos: 0,
        }
    }
}

