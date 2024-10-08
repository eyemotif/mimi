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
        KeyCode::Esc => {
            if !state.history.edited_since_last_save() {
                return InputEvent::Exit;
            }
            state.enqueue_message("Not saved since last edit!".to_owned(), MessageType::Danger);
        }
        KeyCode::Left => {
            if state.cursor > 0 {
                state.cursor -= 1;
            }
        }
        KeyCode::Right => {
            if state.cursor < state.file.chars().count() {
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
            let byte_index = state
                .file
                .char_indices()
                .nth(state.cursor)
                .expect("char should exist")
                .0;
            state.file.insert(byte_index, c);
            state.history.edit_add(c, state.cursor);
            state.cursor += 1;
        }
        KeyCode::Backspace => {
            if state.cursor > 0 {
                state.cursor -= 1;
                let c = state.file.remove(
                    state
                        .file
                        .char_indices()
                        .nth(state.cursor)
                        .expect("char to remove should exist")
                        .0,
                );
                state.history.edit_del(c, state.cursor);
            }
        }
        KeyCode::Enter => return handle_keypress(KeyCode::Char('\n').into(), state),
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
                Ok(()) => {
                    state.history.save();
                    state.enqueue_message(
                        format!("Saved! (Wrote {} bytes)", state.file.len()),
                        MessageType::Status,
                    );
                }
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
        KeyCode::Char('z') => {
            let edit = state.history.undo(&mut state.file, &mut state.cursor);

            if let Some(edit) = edit {
                state.enqueue_message(format!("Undo {edit}"), MessageType::Status);
            }
        }
        KeyCode::Char('y') => {
            let edit = state.history.redo(&mut state.file, &mut state.cursor);

            if let Some(edit) = edit {
                state.enqueue_message(format!("Redo {edit}"), MessageType::Status);
            }
        }
        _ => (),
    }

    InputEvent::NoOp
}
