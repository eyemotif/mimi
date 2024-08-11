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
