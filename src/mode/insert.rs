use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub struct InsertKeymap;

impl InsertKeymap {
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
            (KeyCode::Char(char), _) => Some(Val(char)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum InsertModeCommand {
    Val(char),
    Exit,
}
