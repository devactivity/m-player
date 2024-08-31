mod app;
mod listing;
mod player_handler;
mod terminal_handler;
mod tui;
mod ui;

use app::App;
use color_eyre::Result;
use player_handler::input_handler;
use ratatui::{backend::CrosstermBackend, Terminal};
use terminal_handler::{Events, EventsHandler};
use tui::Tui;

use rodio::{OutputStream, Sink};

fn main() -> Result<()> {
    color_eyre::install()?;

    let (_stream, stream_handler) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handler).unwrap();

    let mut app = App::new(sink);

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventsHandler::new(100);
    let mut tui = Tui::new(terminal, events);

    tui.initialize()?;

    while !app.is_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Events::Key(key_event) => input_handler(&mut app, key_event),
            Events::Tick | Events::Resize(_, _) => {}
        };
    }

    tui.exit()?;

    Ok(())
}
