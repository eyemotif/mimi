use crate::state::{MessageType, State};
use crossterm::event::{Event, KeyCode};

pub enum InputEvent {
    NoOp,
    Exit,
}

pub fn handle_input(state: &mut State) -> std::io::Result<InputEvent> {
    match crossterm::event::read()? {
        Event::Key(key) => return Ok(handle_keypress(key, state)),
        Event::Mouse(_) => todo!(),
        _ => (),
    }

    Ok(InputEvent::NoOp)
}

fn handle_keypress(key: crossterm::event::KeyEvent, state: &mut State) -> InputEvent {
    if key
        .modifiers
        .contains(crossterm::event::KeyModifiers::CONTROL)
    {
        return handle_ctrl_keypress(key, state);
    }

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
            let (col, line) = state.position_in_file(state.cursor);

            state.cursor = state.index_in_file(col, line + 1);
        }
        KeyCode::Up => {
            let (col, line) = state.position_in_file(state.cursor);

            if line > 0 {
                state.cursor = state.index_in_file(col, line - 1);
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

fn handle_ctrl_keypress(key: crossterm::event::KeyEvent, state: &mut State) -> InputEvent {
    match key.code {
        KeyCode::Char('s') => {
            if state.is_readonly {
                state.enqueue_message("File is readonly!".to_owned(), MessageType::Danger);
                return InputEvent::NoOp;
            }
            match std::fs::write(&state.file_path, &state.file) {
                Ok(()) => state.enqueue_message(
                    format!("Saved! (Wrote {} bytes)", state.file.len()),
                    MessageType::Status,
                ),
                Err(err) => {
                    state.enqueue_message(
                        format!("Failed to save file: {err}"),
                        MessageType::Danger,
                    );
                }
            }
        }
        KeyCode::Char(' ') => {
            state.enqueue_message("open command palette!".to_owned(), MessageType::Info);
        }
        _ => (),
    }

    InputEvent::NoOp
}
