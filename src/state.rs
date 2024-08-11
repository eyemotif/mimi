pub type Terminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

#[derive(Debug)]
pub struct State {
    pub file: String,
    pub is_readonly: bool,
    pub cursor: usize,
}

// impl State {
//     pub fn get_cursor_position(&self) -> (usize, usize) {
//         let mut line = 0;
//         let mut col = 0;
//         for c in self.file.chars().take(self.cursor + 1) {
//             if c == '\r' || c == '\n' {
//                 col = 0;
//                 line += 1;
//             } else {
//                 col += 1;
//             }
//         }

//         (line, col)
//     }
// }
