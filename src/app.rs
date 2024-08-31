use crate::listing::DirectoryContent;
use rodio::Sink;
use std::collections::VecDeque;

pub struct App {
    pub is_quit: bool,
    pub sink: Sink,
    pub songs_list: DirectoryContent,
    pub play_deque: VecDeque<String>,
    pub playing: Option<String>,
}

impl App {
    pub fn new(s: Sink) -> Self {
        let songs_list = match DirectoryContent::from_dir("/home/bill/Music/audio") {
            Ok(list) => list,
            Err(e) => {
                eprintln!("Failed to load directory content: {e}");
                DirectoryContent::default()
            }
        };

        App {
            is_quit: false,
            sink: s,
            songs_list,
            play_deque: VecDeque::new(),
            playing: None,
        }
    }

    pub fn quit(&mut self) {
        self.is_quit = true;
    }

    pub fn pop_deque(&mut self) {
        self.play_deque.pop_front();
    }

    pub fn add_deque(&mut self, s: String) {
        self.play_deque.push_back(s);
    }
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("is_quit", &self.is_quit)
            .field("songs_list", &self.songs_list)
            .field("play_deque", &self.play_deque)
            .field("playing", &self.playing)
            .field("sink", &"Sink (details omit)")
            .finish()
    }
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "App status: {self:?}")
    }
}
