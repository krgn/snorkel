use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

#[derive(Debug)]
pub enum NormalModeCommand {
    Exit,
    MoveUp(u8),
    MoveDown(u8),
    MoveLeft(u8),
    MoveRight(u8),
    EnterInsertMode,
    EnterReplaceMode,
    EnterSelectMode,
}

pub struct NormalKeymap;

const REGULAR_MOVE: u8 = 1;
const FAST_MOVE: u8 = 5;

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
        if ev.kind != KeyEventKind::Press {
            return None;
        }

        let code = ev.code;
        let modi = ev.modifiers;

        match (code, modi) {
            (KeyCode::Char('h'), KeyModifiers::NONE) => {
                Some(NormalModeCommand::MoveLeft(REGULAR_MOVE))
            }
            (KeyCode::Char('l'), KeyModifiers::NONE) => {
                Some(NormalModeCommand::MoveRight(REGULAR_MOVE))
            }
            (KeyCode::Char('j'), KeyModifiers::NONE) => {
                Some(NormalModeCommand::MoveDown(REGULAR_MOVE))
            }
            (KeyCode::Char('k'), KeyModifiers::NONE) => {
                Some(NormalModeCommand::MoveUp(REGULAR_MOVE))
            }
            (KeyCode::Char('H'), KeyModifiers::SHIFT) => {
                Some(NormalModeCommand::MoveLeft(FAST_MOVE))
            }
            (KeyCode::Char('L'), KeyModifiers::SHIFT) => {
                Some(NormalModeCommand::MoveRight(FAST_MOVE))
            }
            (KeyCode::Char('J'), KeyModifiers::SHIFT) => {
                Some(NormalModeCommand::MoveDown(FAST_MOVE))
            }
            (KeyCode::Char('K'), KeyModifiers::SHIFT) => Some(NormalModeCommand::MoveUp(FAST_MOVE)),
            _ => None,
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
