use crate::state::State;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

pub fn draw(frame: &mut ratatui::Frame, state: &mut State) {
    let [text_area, info_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(frame.size());

    let text = Paragraph::new(&*state.file);
    frame.render_widget(text, text_area);

    let (message, message_type) = if let Some(message) = state.message_queue.pop_front() {
        message
    } else {
        let (col, line) = state.get_cursor_position();
        (
            format!("({col}, {line})"),
            crate::state::MessageType::Status,
        )
    };
    let info_text = Line::styled(
        message,
        match message_type {
            crate::state::MessageType::Status => Style::new(),
            crate::state::MessageType::Warn => Style::new().fg(Color::White).bg(Color::Yellow),
            crate::state::MessageType::Danger => Style::new().fg(Color::White).bg(Color::Red),
        },
    );

    let info = Paragraph::new(info_text);
    frame.render_widget(info, info_area);

    let (cursor_x, cursor_y) = calculate_cursor_pos(state, frame.size().width);
    frame.set_cursor(cursor_x, cursor_y);
}

fn calculate_cursor_pos(state: &State, frame_width: u16) -> (u16, u16) {
    // TODO: account for scroll
    let mut col = 0;
    let mut row = 0;

    for c in state.file.chars().take(state.cursor) {
        if c == '\n' {
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
