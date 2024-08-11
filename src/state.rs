use std::collections::VecDeque;

pub type Terminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

#[derive(Debug)]
pub struct State {
    pub file: String,
    pub file_path: std::path::PathBuf,
    pub is_readonly: bool,
    pub cursor: usize,
    pub message_queue: VecDeque<(String, MessageType)>,
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Status,
    Info,
    Danger,
}

impl State {
    pub fn enqueue_message(&mut self, message: String, message_type: MessageType) {
        self.message_queue.push_back((message, message_type));
    }
    pub fn get_cursor_position(&self) -> (usize, usize) {
        let mut col = 0;
        let mut line = 0;
        for c in self.file.chars().take(self.cursor) {
            if c == '\n' {
                col = 0;
                line += 1;
            } else {
                col += 1;
            }
        }

        (col, line)
    }
}
