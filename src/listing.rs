use std::{fs, io};

#[derive(Debug)]
pub struct DirectoryContent {
    pub index: usize,
    pub items: Vec<String>,
    pub path: String,
}

impl Default for DirectoryContent {
    fn default() -> Self {
        DirectoryContent::new()
    }
}

impl DirectoryContent {
    pub fn new() -> Self {
        DirectoryContent {
            index: 0,
            items: Vec::new(),
            path: String::from("."),
        }
    }

    pub fn from_dir(directory: &str) -> Result<Self, io::Error> {
        let mut items = Vec::new();

        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path().display().to_string();

            items.push(path);
        }

        Ok(DirectoryContent {
            index: 0,
            items,
            path: directory.to_string(),
        })
    }

    pub fn add_item(&mut self, s: String) {
        self.items.push(s);
    }

    pub fn next_item(&mut self) {
        self.index = (self.index + 1) % self.items.len();
    }

    pub fn prev_item(&mut self) {
        self.index = if self.index == 0 {
            self.items.len() - 1
        } else {
            self.index - 1
        }
    }

    pub fn get_item(&self) -> Option<&str> {
        self.items.get(self.index).map(String::as_str)
    }
}
