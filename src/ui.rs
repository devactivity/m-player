use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let mut path_str = String::new();
    let items = &app.songs_list.items;
    let index = app.songs_list.index;

    for (i, path) in items.iter().enumerate() {
        if let Some(p) = path.split('/').last() {
            if i == index {
                path_str.push_str(&format!("-> - {p}\n"));
            } else if i >= index.saturating_sub(3) {
                path_str.push_str(&format!("  - {p}\n"));
            }
        }
    }

    let mut deque = String::new();
    for p in &app.play_deque {
        if let Some(path) = p.split('/').last() {
            deque.push_str(&format!("  {path}\n"));
        }
    }

    let volume = app.sink.volume() * 100.;

    let now_playing = app.playing.as_deref().unwrap_or("None");

    let content = format!(
        "\n Volume: {}% \n  Now Playing:\n  {}\n  Queue:\n{}\n\n Audio Location: {}\n{}",
        volume.floor(),
        now_playing,
        deque,
        app.songs_list.path,
        path_str
    );

    let tui_widget = f.area();

    f.render_widget(
        Paragraph::new(content)
            .block(
                Block::default()
                    .title("Music Player")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::LightYellow))
            .alignment(Alignment::Left),
        tui_widget,
    );
}
