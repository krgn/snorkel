use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::op::Op;

pub struct InsertKeymap;

impl InsertKeymap {
    pub fn op(ev: KeyEvent) -> Option<InsertModeCommand> {
        if ev.kind != KeyEventKind::Press {
            return None;
        }

        if let KeyCode::Char(c) = ev.code {
            Op::from(c).map(InsertModeCommand::Op)
        } else {
            None
        }
    }

    pub fn parse_key(ev: KeyEvent) -> Option<InsertModeCommand> {
        if ev.kind != KeyEventKind::Press {
            return None;
        }

        let code = ev.code;
        let modi = ev.modifiers;

        use InsertModeCommand::*;
        match (code, modi) {
            (KeyCode::Char('['), KeyModifiers::CONTROL) | (KeyCode::Esc, KeyModifiers::NONE) => {
                Some(Exit)
            }
            (KeyCode::Char(_), _) => Self::op(ev),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum InsertModeCommand {
    Op(Op),
    Exit,
}
