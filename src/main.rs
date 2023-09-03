mod game;
mod ui;

use crate::game::Game;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};
use std::{error::Error, io, time::Duration, fs};

#[derive(Debug)]
struct App {
    game: Game,
}

impl App {
    fn new(filename: &str) -> App {
        let prompt = fs::read_to_string(format!("resources/{}_en.txt", filename)).unwrap();
        let prompt_zy = fs::read_to_string(format!("resources/{}_zy.txt", filename)).unwrap();
        let prompt_zh = fs::read_to_string(format!("resources/{}_zh.txt", filename)).unwrap();
        App {
            game: Game::new(
                prompt,
                prompt_zy,
                prompt_zh,
            ),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("lzc");
    run(&mut terminal, &mut app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(app, f))?;

        loop {
            terminal.draw(|f| ui(app, f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Esc = key.code {
                        return Ok(());
                    }
                    else if let KeyCode::Char(c) = key.code {
                        app.game.input.push(game::Input {
                            char: c,
                            outcome: game::Outcome::Correct,
                        });

                        if app.game.prompt.chars().nth(app.game.cursor_pos).unwrap() != c {
                            app.game.input[app.game.cursor_pos].outcome = game::Outcome::Incorrect;
                        }

                        if app.game.cursor_pos < app.game.prompt.len() {
                            app.game.cursor_pos += 1;
                        }
                        
                        if app.game.cursor_pos == app.game.prompt.len()
                        {
                            app.game.finished = true;
                            break;
                        }
                    }
                }

            }
            
        }

        loop {
            terminal.draw(|f| ui(app, f))?;
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char(c) = key.code {
                        if c == 'q' {
                            return Ok(());
                        }
                        else if c == 'r' {
                            app.game.finished = false;
                            app.game.cursor_pos = 0;
                            app.game.input.clear();
                            break;
                        }
                    }
                }
            }

        }

    }

    Ok(())
}

fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    f.render_widget(&app.game, f.size());
}
