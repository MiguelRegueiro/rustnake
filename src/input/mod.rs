//! Input handling module for the Snake game.
//! Manages keyboard input and translates it to game commands.

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
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
<<<<<<< HEAD
    FocusLost,
=======
    CycleLanguage,
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
    Resize(u16, u16),
}

pub fn setup_input_handler() -> mpsc::Receiver<GameInput> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            if let Ok(event) = event::read() {
                let maybe_input = match event {
                    Event::Resize(width, height) => Some(GameInput::Resize(width, height)),
                    Event::FocusLost => Some(GameInput::FocusLost),
                    Event::Key(KeyEvent { code, kind, .. }) => {
                        if kind != KeyEventKind::Press {
                            None
                        } else {
                            match code {
                                KeyCode::Char('q') | KeyCode::Char('Q') => Some(GameInput::Quit),
                                KeyCode::Char('p') | KeyCode::Char('P') => Some(GameInput::Pause),
                                KeyCode::Char('m') | KeyCode::Char('M') => {
                                    Some(GameInput::ToggleMute)
                                }
                                KeyCode::Char('l') | KeyCode::Char('L') => {
                                    Some(GameInput::CycleLanguage)
                                }
                                KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                                    Some(GameInput::Direction(crate::utils::Direction::Up))
                                }
                                KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                                    Some(GameInput::Direction(crate::utils::Direction::Down))
                                }
                                KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => {
                                    Some(GameInput::Direction(crate::utils::Direction::Left))
                                }
                                KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => {
                                    Some(GameInput::Direction(crate::utils::Direction::Right))
                                }
                                KeyCode::Char('1') => Some(GameInput::MenuSelect(0)),
                                KeyCode::Char('2') => Some(GameInput::MenuSelect(1)),
                                KeyCode::Char('3') => Some(GameInput::MenuSelect(2)),
                                KeyCode::Char('4') => Some(GameInput::MenuSelect(3)),
                                KeyCode::Char('5') => Some(GameInput::MenuSelect(4)),
<<<<<<< HEAD
                                KeyCode::Char('6') => Some(GameInput::MenuSelect(5)),
=======
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
                                KeyCode::Enter | KeyCode::Char('\n') => {
                                    Some(GameInput::MenuConfirm)
                                }
                                KeyCode::Char(' ') => Some(GameInput::MenuConfirm), // Use space to confirm menu selections
                                _ => None, // Ignore other keys
                            }
                        }
                    }
                    _ => None,
                };

                let Some(input) = maybe_input else {
                    continue;
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
