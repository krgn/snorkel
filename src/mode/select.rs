use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use super::Movement;

pub struct SelectKeymap;

const REGULAR_MOVE: u8 = 1;
const FAST_MOVE: u8 = 5;

impl SelectKeymap {
    pub fn exit(ev: KeyEvent) -> Option<SelectModeCommand> {
        if ev.kind != KeyEventKind::Press {
            return None;
        }

        let code = ev.code;
        let modi = ev.modifiers;

        match (code, modi) {
            (KeyCode::Char('['), KeyModifiers::CONTROL) | (KeyCode::Esc, KeyModifiers::NONE) => {
                Some(SelectModeCommand::Exit)
            }
            _ => None,
        }
    }

    pub fn movement(ev: KeyEvent) -> Option<SelectModeCommand> {
        if ev.kind != KeyEventKind::Press {
            return None;
        }

        let code = ev.code;
        let modi = ev.modifiers;

        match (code, modi) {
            (KeyCode::Char('h'), KeyModifiers::NONE) => {
                Some(SelectModeCommand::Move(Movement::Left(REGULAR_MOVE)))
            }
            (KeyCode::Char('l'), KeyModifiers::NONE) => {
                Some(SelectModeCommand::Move(Movement::Right(REGULAR_MOVE)))
            }
            (KeyCode::Char('j'), KeyModifiers::NONE) => {
                Some(SelectModeCommand::Move(Movement::Down(REGULAR_MOVE)))
            }
            (KeyCode::Char('k'), KeyModifiers::NONE) => {
                Some(SelectModeCommand::Move(Movement::Up(REGULAR_MOVE)))
            }
            (KeyCode::Char('H'), KeyModifiers::SHIFT) => {
                Some(SelectModeCommand::Move(Movement::Left(FAST_MOVE)))
            }
            (KeyCode::Char('L'), KeyModifiers::SHIFT) => {
                Some(SelectModeCommand::Move(Movement::Right(FAST_MOVE)))
            }
            (KeyCode::Char('J'), KeyModifiers::SHIFT) => {
                Some(SelectModeCommand::Move(Movement::Down(FAST_MOVE)))
            }
            (KeyCode::Char('K'), KeyModifiers::SHIFT) => {
                Some(SelectModeCommand::Move(Movement::Up(FAST_MOVE)))
            }
            _ => None,
        }
    }

    pub fn parse_key(ev: KeyEvent) -> Option<SelectModeCommand> {
        Self::exit(ev).or_else(|| Self::movement(ev))
    }
}

#[derive(Debug)]
pub enum SelectModeCommand {
    Move(Movement),
    Exit,
}
