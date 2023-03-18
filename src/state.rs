use crate::config::Config;
use crate::mode::{
    InsertKeymap, InsertModeCommand, Movement, NormalKeymap, NormalModeCommand, ReplaceKeymap,
    ReplaceModeCommand, SelectKeymap, SelectModeCommand,
};
use crate::op::Op;
use crate::snorkel::Snorkel;
use crate::util::{Coord, Selection};
use crossterm::event::{KeyCode, KeyEvent};
use std::cmp;
use std::fmt::Display;
use std::time::Instant;

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

impl Display for EditorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EditorState::*;
        match self {
            Insert => f.write_str("insert"),
            Normal => f.write_str("normal"),
            Replace => f.write_str("replace"),
            Select => f.write_str("select"),
            QuitRequested => f.write_str("quit?"),
            QuitConfirmed => f.write_str("bye"),
        }
    }
}

#[derive(Debug)]
pub enum UndoOp {
    Step {
        ts: Instant,
        loc: Coord,
        op: Option<Op>,
    },
    Batch {
        ts: Instant,
        ops: Vec<(Coord, Option<Op>)>,
    },
}

impl UndoOp {
    pub fn step(loc: &Coord, op: Option<Op>) -> Self {
        UndoOp::Step {
            ts: Instant::now(),
            loc: loc.clone(),
            op,
        }
    }

    pub fn batch(ops: Vec<(Coord, Option<Op>)>) -> Self {
        UndoOp::Batch {
            ts: Instant::now(),
            ops,
        }
    }
}

pub struct AppState {
    pub cursor: Coord,
    pub edit_state: EditorState,
    pub undo_steps: Vec<UndoOp>,
    pub redo_steps: Vec<UndoOp>,
    pub clipboard: Option<Vec<Vec<Option<Op>>>>,
    pub sel_start: Option<Coord>,
    pub snrkl: Snorkel,
    pub config: Config,
    pub show_logs: bool,
}

impl AppState {
    pub fn new(rows: usize, cols: usize) -> AppState {
        AppState {
            clipboard: None,
            config: Config::default(),
            cursor: Coord::default(),
            edit_state: EditorState::default(),
            redo_steps: Vec::new(),
            sel_start: None,
            snrkl: Snorkel::new(rows, cols),
            undo_steps: Vec::new(),
            show_logs: false,
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
                        NextFrame => self.snrkl.frame += 1,
                        ResetFrame => self.snrkl.frame = 0,
                        ToggleLogs => self.show_logs = !self.show_logs,
                        Move(movement) => self.move_cursor(movement),
                        Delete => {
                            let old = self.snrkl.del_cell(&self.cursor);
                            self.undo_steps.push(UndoOp::step(&self.cursor, old));
                        }
                        Paste => {
                            if let Some(data) = &self.clipboard {
                                let undo = self.snrkl.paste_selection(&self.cursor, &data);
                                self.undo_steps.push(undo);
                            }
                        }
                        Undo => {
                            // TODO: we could make this into a method maybe to prevent
                            // code duplication?
                            use UndoOp::*;
                            match self.undo_steps.pop() {
                                Some(Step { ts: _, loc, op }) => {
                                    let old = match op {
                                        Some(op) => self.snrkl.set_cell(&loc, op),
                                        None => self.snrkl.del_cell(&loc),
                                    };
                                    self.redo_steps.push(UndoOp::step(&loc, old));
                                }
                                Some(Batch { ts: _, ops }) => {
                                    let mut old_ops = vec![];
                                    for (loc, op) in ops {
                                        let old = match op {
                                            Some(op) => self.snrkl.set_cell(&loc, op),
                                            None => self.snrkl.del_cell(&loc),
                                        };
                                        old_ops.push((loc, old));
                                    }
                                    self.redo_steps.push(UndoOp::batch(old_ops));
                                }
                                None => (),
                            }
                        }
                        Redo => {
                            // TODO: we could make this into a method maybe to prevent
                            // code duplication?
                            use UndoOp::*;
                            match self.redo_steps.pop() {
                                Some(Step { ts: _, loc, op }) => {
                                    let old = match op {
                                        Some(op) => self.snrkl.set_cell(&loc, op),
                                        None => self.snrkl.del_cell(&loc),
                                    };
                                    self.undo_steps.push(UndoOp::step(&loc, old));
                                }
                                Some(Batch { ts: _, ops }) => {
                                    let mut old_ops = vec![];
                                    for (loc, op) in ops {
                                        let old = match op {
                                            Some(op) => self.snrkl.set_cell(&loc, op),
                                            None => self.snrkl.del_cell(&loc),
                                        };
                                        old_ops.push((loc, old));
                                    }
                                    self.undo_steps.push(UndoOp::batch(old_ops));
                                }
                                None => (),
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
                            self.undo_steps.push(UndoOp::step(&self.cursor, old));
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
                            self.undo_steps.push(UndoOp::step(&self.cursor, old));
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
                        Copy => {
                            if let Some(sel_start) = &self.sel_start {
                                let sel = Selection::from(&self.cursor, &sel_start);
                                let data = self.snrkl.copy_selection(&sel);
                                self.clipboard = Some(data);
                            }
                        }
                        Paste => {
                            if let Some(data) = &self.clipboard {
                                let undo = self.snrkl.paste_selection(&self.cursor, &data);
                                self.undo_steps.push(undo);
                            }
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

    pub fn tick(&mut self) {
        self.snrkl.tick()
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
