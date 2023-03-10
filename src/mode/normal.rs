use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

#[derive(Debug)]
pub enum NormalModeCommand {
    Exit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    EnterInsertMode,
    EnterReplaceMode,
    EnterSelectMode,
}

pub struct NormalKeymap;

impl NormalKeymap {
    pub fn edit_state(ev: KeyEvent) -> Option<NormalModeCommand> {
        if let KeyEvent {
            code: KeyCode::Char(chr),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        } = ev
        {
            match chr {
                'i' => Some(NormalModeCommand::EnterInsertMode),
                'r' => Some(NormalModeCommand::EnterReplaceMode),
                'v' => Some(NormalModeCommand::EnterSelectMode),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn movement(ev: KeyEvent) -> Option<NormalModeCommand> {
        if let KeyEvent {
            code: KeyCode::Char(chr),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        } = ev
        {
            match chr {
                'h' => Some(NormalModeCommand::MoveLeft),
                'l' => Some(NormalModeCommand::MoveRight),
                'j' => Some(NormalModeCommand::MoveDown),
                'k' => Some(NormalModeCommand::MoveUp),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn exit(ev: KeyEvent) -> Option<NormalModeCommand> {
        if ev.kind != KeyEventKind::Press {
            return None;
        }

        let code = ev.code;
        let modi = ev.modifiers;

        match (code, modi) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL)
            | (KeyCode::Char('q'), KeyModifiers::NONE) => Some(NormalModeCommand::Exit),
            _ => None,
        }
    }

    pub fn parse_key(ev: KeyEvent) -> Option<NormalModeCommand> {
        NormalKeymap::edit_state(ev)
            .or_else(|| NormalKeymap::movement(ev))
            .or_else(|| NormalKeymap::exit(ev))
    }
}
