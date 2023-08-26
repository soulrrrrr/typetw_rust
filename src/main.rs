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
use std::{error::Error, io};

#[derive(Debug)]
struct App {
    game: Game,
}

impl App {
    fn new() -> App {
        App {
            game: Game::new(
                "ji3g4go6c8 c8 c8 ".to_string(),
                "ㄨㄛˇㄕˋㄕㄟˊㄏㄚ ㄏㄚ ㄏㄚ ".to_string(),
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

    let mut app = App::new();
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

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }

    Ok(())
}

fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    f.render_widget(&app.game, f.size());
}
