pub type Terminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

#[derive(Debug)]
pub struct State {
    pub file: String,
    pub is_readonly: bool,
    pub terminal: Terminal,
}
