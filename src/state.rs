use crate::mode::{
    InsertKeymap, InsertModeCommand, NormalKeymap, NormalModeCommand, ReplaceKeymap,
    ReplaceModeCommand, SelectKeymap, SelectModeCommand,
};
use crate::snrkl::Snrkl;
use crossterm::event::KeyEvent;
use std::cmp;

#[derive(Default)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum EditorState {
    Insert,
    #[default]
    Normal,
    Replace,
    Select,
}

pub struct AppState {
    pub cursor: Coord,
    pub edit_state: EditorState,
    pub snrkl: Snrkl,
}

impl AppState {
    pub fn new(rows: usize, cols: usize) -> AppState {
        AppState {
            snrkl: Snrkl::new(rows, cols),
            cursor: Coord::default(),
            edit_state: EditorState::default(),
        }
    }

    pub fn input(&mut self, key: KeyEvent) {
        match &self.edit_state {
            EditorState::Normal => {
                use NormalModeCommand::*;
                if let Some(cmd) = NormalKeymap::parse_key(key) {
                    match cmd {
                        EnterInsertMode => self.edit_state = EditorState::Insert,
                        EnterReplaceMode => self.edit_state = EditorState::Replace,
                        EnterSelectMode => self.edit_state = EditorState::Select,
                        MoveUp => self.move_cursor(cmd),
                        MoveDown => self.move_cursor(cmd),
                        MoveLeft => self.move_cursor(cmd),
                        MoveRight => self.move_cursor(cmd),
                        Exit => (),
                    }
                }
            }
            EditorState::Insert => {
                use InsertModeCommand::*;
                if let Some(cmd) = InsertKeymap::parse_key(key) {
                    match cmd {
                        Exit => self.edit_state = EditorState::default(),
                    }
                }
            }
            EditorState::Replace => {
                use ReplaceModeCommand::*;
                if let Some(cmd) = ReplaceKeymap::parse_key(key) {
                    match cmd {
                        Exit => self.edit_state = EditorState::default(),
                    }
                }
            }
            EditorState::Select => {
                use SelectModeCommand::*;
                if let Some(cmd) = SelectKeymap::parse_key(key) {
                    match cmd {
                        Exit => self.edit_state = EditorState::default(),
                    }
                }
            }
        }
    }

    pub fn move_cursor(&mut self, cmd: NormalModeCommand) {
        if self.edit_state != EditorState::Normal {
            return;
        }

        use NormalModeCommand::*;

        let x = self.cursor.x;
        let y = self.cursor.y;

        let (new_x, new_y) = match cmd {
            MoveDown => (x, cmp::min(y + 1, self.snrkl.rows)),
            MoveUp => (x, y.checked_sub(1).unwrap_or(0)),
            MoveLeft => (x.checked_sub(1).unwrap_or(0), y),
            MoveRight => (cmp::min(x + 1, self.snrkl.rows), y),
            _ => (x, y),
        };

        self.cursor.x = new_x;
        self.cursor.y = new_y;
    }
}
