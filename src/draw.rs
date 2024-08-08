use crate::state::State;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;

pub fn draw(frame: &mut ratatui::Frame, state: &State) {
    let [text_area] = Layout::vertical([Constraint::Min(0)]).areas(frame.size());

    let text = Paragraph::new(&*state.file);
    frame.render_widget(text, text_area);
}
