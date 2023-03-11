use crate::mode::{
    InsertKeymap, InsertModeCommand, NormalKeymap, NormalModeCommand, ReplaceKeymap,
    ReplaceModeCommand, SelectKeymap, SelectModeCommand,
};
use crate::snrkl::Snrkl;
use crossterm::event::{KeyCode, KeyEvent};
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
    QuitRequested,
    QuitConfirmed,
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
                        MoveUp(_) => self.move_cursor(cmd),
                        MoveDown(_) => self.move_cursor(cmd),
                        MoveLeft(_) => self.move_cursor(cmd),
                        MoveRight(_) => self.move_cursor(cmd),
                        Delete => self.snrkl.del_cell(self.cursor.x, self.cursor.y),
                        Exit => self.edit_state = EditorState::QuitRequested,
                    }
                }
            }
            EditorState::Insert => {
                use InsertModeCommand::*;
                if let Some(cmd) = InsertKeymap::parse_key(key) {
                    match cmd {
                        Exit => self.edit_state = EditorState::default(),
                        Op(op) => self.snrkl.set_cell(self.cursor.x, self.cursor.y, op),
                    }
                }
            }
            EditorState::Replace => {
                use ReplaceModeCommand::*;
                if let Some(cmd) = ReplaceKeymap::parse_key(key) {
                    match cmd {
                        Exit => self.edit_state = EditorState::default(),
                        Op(op) => {
                            self.snrkl.set_cell(self.cursor.x, self.cursor.y, op);
                            self.move_cursor(NormalModeCommand::MoveRight(1));
                        }
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
            EditorState::QuitRequested => match key.code {
                KeyCode::Esc => self.edit_state = EditorState::Normal,
                KeyCode::Enter => self.edit_state = EditorState::QuitConfirmed,
                _ => (),
            },
            EditorState::QuitConfirmed => (),
        }
    }

    pub fn move_cursor(&mut self, cmd: NormalModeCommand) {
        use NormalModeCommand::*;

        let x = self.cursor.x;
        let y = self.cursor.y;

        let (new_x, new_y) = match cmd {
            MoveDown(n) => (x, cmp::min(y + n as usize, self.snrkl.rows)),
            MoveUp(n) => (x, y.checked_sub(n as usize).unwrap_or(0)),
            MoveLeft(n) => (x.checked_sub(n as usize).unwrap_or(0), y),
            MoveRight(n) => (cmp::min(x + n as usize, self.snrkl.cols - 1), y),
            _ => (x, y),
        };

        self.cursor.x = new_x;
        self.cursor.y = new_y;
    }
}

#[cfg(test)]
mod move_cursor {
    use crate::state::{AppState, NormalModeCommand};

    #[test]
    fn move_cursor_around() {
        let mut app = AppState::new(20, 20);
        app.move_cursor(NormalModeCommand::MoveDown(1));
        app.move_cursor(NormalModeCommand::MoveRight(1));
        assert_eq!(app.cursor.x, 1);
        assert_eq!(app.cursor.y, 1);
    }

    #[test]
    fn should_handle_potential_overflow_correctly() {
        let mut app = AppState::new(20, 20);
        app.move_cursor(NormalModeCommand::MoveLeft(1));
        assert_eq!(app.cursor.x, 0);
        app.move_cursor(NormalModeCommand::MoveUp(1));
        assert_eq!(app.cursor.y, 0);
    }

    #[test]
    fn should_clamp_grid_size() {
        let mut app = AppState::new(20, 20);
        for _ in 0..22 {
            app.move_cursor(NormalModeCommand::MoveRight(1));
        }
        assert_eq!(app.cursor.x, app.snrkl.cols - 1);
        for _ in 0..22 {
            app.move_cursor(NormalModeCommand::MoveDown(1));
        }
        assert_eq!(app.cursor.y, app.snrkl.rows);
    }
}
