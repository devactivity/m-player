use std::{io, panic};

use color_eyre::Result;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{app::App, terminal_handler::EventsHandler, ui};

pub struct Tui {
    terminal: CrosstermTerminal,
    pub events: EventsHandler,
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal, events: EventsHandler) -> Self {
        Self { terminal, events }
    }

    pub fn initialize(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen)?;

        let panic_err = panic::take_hook();
        panic::set_hook(Box::new(move |p| {
            Self::reset().expect("failed to reset the terminal");
            panic_err(p);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;

        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen)?;

        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;

        Ok(())
    }
}
