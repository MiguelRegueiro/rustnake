//! Input handling module for the Snake game.
//! Manages keyboard input and translates it to game commands.

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone)]
pub enum GameInput {
    Direction(crate::utils::Direction),
    Pause,
    Quit,
    MenuSelect(usize),
    MenuConfirm,
    ToggleMute,
}

pub fn setup_input_handler() -> mpsc::Receiver<GameInput> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                let input = match code {
                    KeyCode::Char('q') => GameInput::Quit,
                    KeyCode::Char('p') => GameInput::Pause,
                    KeyCode::Char('m') => GameInput::ToggleMute,
                    KeyCode::Char('w') | KeyCode::Up => {
                        GameInput::Direction(crate::utils::Direction::Up)
                    }
                    KeyCode::Char('s') | KeyCode::Down => {
                        GameInput::Direction(crate::utils::Direction::Down)
                    }
                    KeyCode::Char('a') | KeyCode::Left => {
                        GameInput::Direction(crate::utils::Direction::Left)
                    }
                    KeyCode::Char('d') | KeyCode::Right => {
                        GameInput::Direction(crate::utils::Direction::Right)
                    }
                    KeyCode::Char('1') => GameInput::MenuSelect(0),
                    KeyCode::Char('2') => GameInput::MenuSelect(1),
                    KeyCode::Char('3') => GameInput::MenuSelect(2),
                    KeyCode::Enter | KeyCode::Char('\n') => GameInput::MenuConfirm,
                    KeyCode::Char(' ') => GameInput::MenuConfirm, // Use space to confirm menu selections
                    _ => continue, // Ignore other keys
                };

                if tx.send(input.clone()).is_err() {
                    // Channel closed, exit the thread
                    break;
                }

                if let GameInput::Quit = input {
                    break;
                }
            }
        }
    });

    rx
}
