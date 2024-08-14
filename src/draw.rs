use crate::state::State;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

pub fn draw(frame: &mut ratatui::Frame, state: &mut State) {
    let [text_area, info_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(frame.size());

    update_scroll(state, text_area.height);

    let text = Paragraph::new(state.scrolled_file());
    frame.render_widget(text, text_area);

    let (message, message_type) = if let Some(message) = state.message_queue.pop_front() {
        message
    } else {
        let (col, line) = state.position_in_file(state.cursor);
        (
            format!(
                "({col}, {line}){}",
                if state.is_readonly { " readonly" } else { "" }
            ),
            crate::state::MessageType::Status,
        )
    };
    let info_text = Line::styled(
        message,
        match message_type {
            crate::state::MessageType::Status => Style::new(),
            crate::state::MessageType::Info => Style::new().fg(Color::White).bg(Color::Blue),
            crate::state::MessageType::Danger => Style::new().fg(Color::White).bg(Color::Red),
        },
    );
    let info = Paragraph::new(info_text);
    frame.render_widget(info, info_area);

    let (cursor_x, cursor_y) = calculate_cursor_pos(state, text_area.width);
    frame.set_cursor(cursor_x, cursor_y);
}

fn update_scroll(state: &mut State, area_height: u16) {
    let (_, file_line) = state.position_in_file(state.cursor);

    let first_visible_line = state.scroll_lines;
    let first_visible_index = state.index_of_line(first_visible_line);

    let last_visible_line = first_visible_line + usize::from(area_height) - 1;
    let last_visible_line = last_visible_line.min(state.file.split('\n').count() - 1);
    let last_visible_index = state.index_of_line(last_visible_line)
        + state
            .file
            .split('\n')
            .nth(last_visible_line)
            .expect("last line should exist")
            .chars()
            .count();

    if state.cursor < first_visible_index {
        state.scroll_lines -= first_visible_line - file_line;
    } else if state.cursor > last_visible_index {
        state.scroll_lines += file_line - last_visible_line;
    }
}

fn calculate_cursor_pos(state: &mut State, frame_width: u16) -> (u16, u16) {
    let mut col: u16 = 0;
    let mut row: usize = 0;

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

    (
        col,
        (row - state.scroll_lines).try_into().unwrap_or(u16::MAX),
    )
}
