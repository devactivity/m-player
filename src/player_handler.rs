use crate::app::App;
use crate::listing::DirectoryContent;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rodio::Decoder;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

fn playing(app: &mut App) {
    if !app.sink.is_paused() && app.sink.empty() && !app.play_deque.is_empty() {
        if let Some(path) = app.play_deque.front() {
            app.playing = Some(path.split('/').last().unwrap_or("").to_string());

            if let Ok(file) = File::open(path) {
                let file = BufReader::new(file);

                if let Ok(source) = Decoder::new(file) {
                    app.sink.append(source);
                    app.pop_deque();
                } else {
                    eprintln!("error decode bang....: {path}");
                }
            } else {
                eprintln!("error open audio file bang....: {path}");
            }
        }
    }
}

fn add_sound(index: u32, app: &mut App) {
    if let Ok(mut paths) = fs::read_dir(&app.songs_list.path) {
        if let Some(path) = paths.nth(index as usize).and_then(Result::ok) {
            let path_str = path.path().display().to_string();

            if Path::new(&path_str).is_dir() {
                if let Ok(dir_content) = DirectoryContent::from_dir(&path_str) {
                    app.songs_list = dir_content;
                } else {
                    eprintln!("error to load directory content bang...: {path_str}");
                }
            } else {
                app.add_deque(path_str);
            }
        } else {
            eprintln!("error to read directory entry bang di index ...: {index}");
        }
    } else {
        eprintln!("error to read directory bang...: {}", app.songs_list.path);
    }
}

pub fn input_handler(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => app.quit(),
        KeyCode::Char(c) => match c {
            '0'..='9' => {
                if let Some(index) = c.to_digit(10) {
                    add_sound(index, app);
                }
            }
            ' ' => {
                if app.sink.is_paused() {
                    app.sink.play();
                } else {
                    app.sink.pause();
                }
            }
            '+' => app.sink.set_volume((app.sink.volume() + 0.05).min(2.)),
            '-' => app.sink.set_volume((app.sink.volume() - 0.05).min(2.)),
            'r' => app.pop_deque(),
            's' => {
                app.sink.clear();
                app.sink.play();
            }
            _ => {}
        },
        KeyCode::Up => app.songs_list.prev_item(),
        KeyCode::Down => app.songs_list.next_item(),
        KeyCode::Enter => {
            if let Ok(index) = u32::try_from(app.songs_list.index) {
                add_sound(index, app);
            }
        }
        _ => {}
    };

    playing(app);
}
