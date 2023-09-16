use std::{char, time::SystemTime};

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
    pub started_at: Option<SystemTime>,
    pub wpm: f64,
    pub accuracy: f64,
}

impl Game {
    pub fn new(
        prompt: (String, String, String),
    ) -> Game {
        Game {
            prompt: prompt.0, // en
            prompt_zy: prompt.1, // zy
            prompt_zh: prompt.2, // zh
            input: Vec::new(),
            cursor_pos: 0,
            finished: false,
            started_at: None,
            wpm: 0.0,
            accuracy: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.started_at = Some(SystemTime::now());
    }

    pub fn calc_results(&mut self) {
        let elapsed_time = self.started_at.unwrap().elapsed().unwrap().as_secs_f64();
        self.wpm = self.prompt_zh.chars().count() as f64 * 60.0 / elapsed_time;     
        self.accuracy = self.input.iter().filter(|&n| (*n).outcome == Outcome::Correct).count() as f64 / self.prompt.len() as f64;
    }
}

