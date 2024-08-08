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
        KeyCode::Esc => InputEvent::Exit,
        _ => InputEvent::NoOp,
    }
}
