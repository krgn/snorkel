use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::op::Op;

pub struct ReplaceKeymap;

impl ReplaceKeymap {
    pub fn op(ev: KeyEvent) -> Option<ReplaceModeCommand> {
        if ev.kind != KeyEventKind::Press {
            return None;
        }

        if let KeyCode::Char(c) = ev.code {
            Op::from(c).map(ReplaceModeCommand::Op)
        } else {
            None
        }
    }

    pub fn parse_key(ev: KeyEvent) -> Option<ReplaceModeCommand> {
        if ev.kind != KeyEventKind::Press {
            return None;
        }

        let code = ev.code;
        let modi = ev.modifiers;

        match (code, modi) {
            (KeyCode::Char('['), KeyModifiers::CONTROL) | (KeyCode::Esc, KeyModifiers::NONE) => {
                Some(ReplaceModeCommand::Exit)
            }
            (KeyCode::Char(_), _) => Self::op(ev),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ReplaceModeCommand {
    Op(Op),
    Exit,
}
