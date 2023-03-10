use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub struct SelectKeymap;

impl SelectKeymap {
    pub fn parse_key(ev: KeyEvent) -> Option<SelectModeCommand> {
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
}

#[derive(Debug)]
pub enum SelectModeCommand {
    Exit,
}
