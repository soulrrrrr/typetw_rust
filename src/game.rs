
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
    pub prompt_zh: String,
    pub input: Vec<Input>,
    pub cursor_pos: usize,
    pub finished: bool,
}

impl Game {
    pub fn new(
        prompt: String,
        prompt_zy: String,
        prompt_zh: String,
    ) -> Game {
        Game {
            prompt,
            prompt_zy,
            prompt_zh,
            input: Vec::new(),
            cursor_pos: 0,
            finished: false,
        }
    }
}

