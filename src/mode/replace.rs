use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub struct ReplaceKeymap;

impl ReplaceKeymap {
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
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ReplaceModeCommand {
    Exit,
}
