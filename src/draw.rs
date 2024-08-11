use crate::state::State;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;

pub fn draw(frame: &mut ratatui::Frame, state: &State) {
    let [text_area] = Layout::vertical([Constraint::Min(0)]).areas(frame.size());

    let text = Paragraph::new(&*state.file);
    frame.render_widget(text, text_area);

    let (cursor_x, cursor_y) = calculate_cursor_pos(state, frame.size().width);
    frame.set_cursor(cursor_x, cursor_y);
}

fn calculate_cursor_pos(state: &State, frame_width: u16) -> (u16, u16) {
    // TODO: account for scroll
    let mut col = 0;
    let mut row = 0;

    for c in state.file.chars().take(state.cursor) {
        if c == '\n' || c == '\r' {
            row += 1;
            col = 0;
        } else {
            col += 1;
            if col >= frame_width {
                row += 1;
                col = 0;
            }
        }
    }

    (col, row)
}
