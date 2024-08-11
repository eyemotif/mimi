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
    pub fn position_in_file(&self, char_index: usize) -> (usize, usize) {
        let mut col = 0;
        let mut line = 0;
        for c in self.file.chars().take(char_index) {
            if c == '\n' {
                col = 0;
                line += 1;
            } else {
                col += 1;
            }
        }

        (col, line)
    }
    // pub fn index_of_line(&self, line: usize) -> usize {
    //     let mut current_line = 0;
    //     for (i, c) in self.file.chars().enumerate() {
    //         if current_line == line {
    //             return i;
    //         }

    //         if c == '\n' {
    //             current_line += 1;
    //         }
    //     }

    //     self.file.chars().count()
    // }
}
