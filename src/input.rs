use crate::state::State;
use crossterm::event::{Event, KeyCode};

pub enum InputEvent {
    NoOp,
    Exit,
}

pub fn handle_input(state: &mut State) -> std::io::Result<InputEvent> {
    match crossterm::event::read()? {
        Event::Key(key) => Ok(handle_keypress(key, state)),
        Event::Mouse(_) => todo!(),
        _ => Ok(InputEvent::NoOp),
    }
}

fn handle_keypress(key: crossterm::event::KeyEvent, state: &mut State) -> InputEvent {
    match key.code {
        KeyCode::Esc => return InputEvent::Exit,
        KeyCode::Left => {
            if state.cursor > 0 {
                state.cursor -= 1;
            }
        }
        KeyCode::Right => {
            if state.cursor < state.file.chars().count() - 1 {
                state.cursor += 1;
            }
        }
        KeyCode::Down => {
            let (col, _) = state.get_cursor_position();

            let mut col_in_new_line = None;
            for c in state.file.chars().skip(state.cursor) {
                state.cursor += 1;
                if c == '\n' {
                    if col_in_new_line.is_some() {
                        state.cursor -= 1;
                        break;
                    }
                    col_in_new_line = Some(0);
                    continue;
                }

                if let Some(col_in_new_line) = col_in_new_line.as_mut() {
                    if *col_in_new_line == col {
                        state.cursor -= 1;
                        break;
                    }
                    *col_in_new_line += 1;
                }
            }
        }
        KeyCode::Up => {
            // TODO: up arrow input
            let (col, row) = state.get_cursor_position();

            if row == 0 {
                state.cursor = 0;
                return InputEvent::NoOp;
            }
        }
        KeyCode::Char(c) => {
            state.file.insert(state.cursor, c);
            state.cursor += 1;
        }
        KeyCode::Backspace => {
            if state.cursor > 0 {
                state.file.remove(state.cursor);
                state.cursor -= 1;
            }
        }
        _ => (),
    }
    InputEvent::NoOp
}
