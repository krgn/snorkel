use crate::config::Config;
use crate::mode::{
    InsertKeymap, InsertModeCommand, Movement, NormalKeymap, NormalModeCommand, ReplaceKeymap,
    ReplaceModeCommand, SelectKeymap, SelectModeCommand,
};
use crate::op::Op;
use crate::snrkl::Snrkl;
use crate::util::Coord;
use crossterm::event::{KeyCode, KeyEvent};
use std::cmp;

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
    pub undo_steps: Vec<(Coord, Option<Op>)>,
    pub redo_steps: Vec<(Coord, Option<Op>)>,
    pub sel_start: Option<Coord>,
    pub snrkl: Snrkl,
    pub config: Config,
}

impl AppState {
    pub fn new(rows: usize, cols: usize) -> AppState {
        AppState {
            snrkl: Snrkl::new(rows, cols),
            cursor: Coord::default(),
            edit_state: EditorState::default(),
            undo_steps: Vec::new(),
            redo_steps: Vec::new(),
            sel_start: None,
            config: Config::default(),
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
                        Move(movement) => self.move_cursor(movement),
                        Delete => {
                            let old = self.snrkl.del_cell(&self.cursor);
                            self.undo_steps.push((self.cursor.clone(), old));
                        }
                        Undo => {
                            if let Some((loc, maybe_op)) = self.undo_steps.pop() {
                                let old = match maybe_op {
                                    Some(op) => self.snrkl.set_cell(&loc, op),
                                    None => self.snrkl.del_cell(&loc),
                                };
                                self.redo_steps.push((loc, old));
                            }
                        }
                        Redo => {
                            if let Some((loc, maybe_op)) = self.redo_steps.pop() {
                                let old = match maybe_op {
                                    Some(op) => self.snrkl.set_cell(&loc, op),
                                    None => self.snrkl.del_cell(&loc),
                                };
                                self.undo_steps.push((loc, old));
                            }
                        }
                        Exit => self.edit_state = EditorState::QuitRequested,
                    }
                }
            }
            EditorState::Insert => {
                use InsertModeCommand::*;
                if let Some(cmd) = InsertKeymap::parse_key(key) {
                    match cmd {
                        Exit => self.edit_state = EditorState::default(),
                        Op(op) => {
                            let old = self.snrkl.set_cell(&self.cursor, op);
                            self.undo_steps.push((self.cursor.clone(), old));
                        }
                    }
                }
            }
            EditorState::Replace => {
                use ReplaceModeCommand::*;
                if let Some(cmd) = ReplaceKeymap::parse_key(key) {
                    match cmd {
                        Exit => self.edit_state = EditorState::default(),
                        Op(op) => {
                            let old = self.snrkl.set_cell(&self.cursor, op);
                            self.undo_steps.push((self.cursor.clone(), old));
                            self.move_cursor(Movement::Right(1));
                        }
                    }
                }
            }
            EditorState::Select => {
                use SelectModeCommand::*;
                if let Some(cmd) = SelectKeymap::parse_key(key) {
                    match cmd {
                        Exit => {
                            self.sel_start = None;
                            self.edit_state = EditorState::default();
                        }
                        Move(movement) => {
                            if self.sel_start.is_none() {
                                self.sel_start = Some(self.cursor.clone());
                            }
                            self.move_cursor(movement);
                        }
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

    pub fn move_cursor(&mut self, mov: Movement) {
        let x = self.cursor.x;
        let y = self.cursor.y;

        use Movement::*;
        let (new_x, new_y) = match mov {
            Down(n) => (x, cmp::min(y + n as usize, self.snrkl.rows)),
            Up(n) => (x, y.checked_sub(n as usize).unwrap_or(0)),
            Left(n) => (x.checked_sub(n as usize).unwrap_or(0), y),
            Right(n) => (cmp::min(x + n as usize, self.snrkl.cols - 1), y),
        };

        self.cursor.x = new_x;
        self.cursor.y = new_y;
    }
}

#[cfg(test)]
mod move_cursor {
    use crate::{mode::Movement, state::AppState};

    #[test]
    fn move_cursor_around() {
        let mut app = AppState::new(20, 20);
        app.move_cursor(Movement::Down(1));
        app.move_cursor(Movement::Right(1));
        assert_eq!(app.cursor.x, 1);
        assert_eq!(app.cursor.y, 1);
    }

    #[test]
    fn should_handle_potential_overflow_correctly() {
        let mut app = AppState::new(20, 20);
        app.move_cursor(Movement::Left(1));
        assert_eq!(app.cursor.x, 0);
        app.move_cursor(Movement::Up(1));
        assert_eq!(app.cursor.y, 0);
    }

    #[test]
    fn should_clamp_grid_size() {
        let mut app = AppState::new(20, 20);
        for _ in 0..22 {
            app.move_cursor(Movement::Right(1));
        }
        assert_eq!(app.cursor.x, app.snrkl.cols - 1);
        for _ in 0..22 {
            app.move_cursor(Movement::Down(1));
        }
        assert_eq!(app.cursor.y, app.snrkl.rows);
    }
}
