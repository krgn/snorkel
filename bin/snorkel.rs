use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute, terminal, Result,
};
use snorkel::{chars, snrkl::Snrkl};
use std::{cmp, io};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

struct NormalKeymap;

impl NormalKeymap {
    fn edit_state(ev: KeyEvent) -> Option<NormalModeCommand> {
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

    fn movement(ev: KeyEvent) -> Option<NormalModeCommand> {
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

    fn exit(ev: KeyEvent) -> Option<NormalModeCommand> {
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

    fn parse_key(ev: KeyEvent) -> Option<NormalModeCommand> {
        NormalKeymap::edit_state(ev)
            .or_else(|| NormalKeymap::movement(ev))
            .or_else(|| NormalKeymap::exit(ev))
    }
}

#[derive(Default)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

struct InsertKeymap;

impl InsertKeymap {
    fn parse_key(ev: KeyEvent) -> Option<InsertModeCommand> {
        if ev.kind != KeyEventKind::Press {
            return None;
        }

        let code = ev.code;
        let modi = ev.modifiers;

        match (code, modi) {
            (KeyCode::Char('['), KeyModifiers::CONTROL) | (KeyCode::Esc, KeyModifiers::NONE) => {
                Some(InsertModeCommand::Exit)
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum InsertModeCommand {
    Exit,
}

struct SelectKeymap;

impl SelectKeymap {
    fn parse_key(ev: KeyEvent) -> Option<SelectModeCommand> {
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

struct ReplaceKeymap;

impl ReplaceKeymap {
    fn parse_key(ev: KeyEvent) -> Option<ReplaceModeCommand> {
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

#[derive(Default, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum EditorState {
    Insert,
    #[default]
    Normal,
    Replace,
    Select,
}

struct AppState {
    snrkl: Snrkl,
    cursor: Coord,
    edit_state: EditorState,
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

fn run_app<B: tui::backend::Backend>(terminal: &mut tui::Terminal<B>) -> io::Result<()> {
    let mut state = AppState::new(20, 80);
    loop {
        terminal.draw(|f| ui(f, &state))?;
        if let Event::Key(key) = event::read()? {
            if state.edit_state == EditorState::Normal {
                if let Some(NormalModeCommand::Exit) = NormalKeymap::exit(key) {
                    return Ok(());
                }
            }

            state.input(key);
            log::info!("char: {:#?}", key);
        }
    }
}

fn ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, state: &AppState) {
    let chunks = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            [
                tui::layout::Constraint::Max(4),
                tui::layout::Constraint::Percentage(80),
                tui::layout::Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let editor_state = match &state.edit_state {
        EditorState::Normal => Span::styled(
            "normal",
            Style::default().bg(Color::Black).fg(Color::White).clone(),
        ),
        EditorState::Insert => Span::styled(
            "insert",
            Style::default().bg(Color::LightGreen).fg(Color::Black),
        ),
        EditorState::Replace => Span::styled(
            "replace",
            Style::default().bg(Color::LightBlue).fg(Color::Black),
        ),
        EditorState::Select => Span::styled(
            "select",
            Style::default().bg(Color::Yellow).fg(Color::Black),
        ),
    };

    let items = Spans::from(vec![Span::raw("state: "), editor_state]);

    let p = Paragraph::new(items).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, chunks[0]);

    let mut text = vec![];

    for y in 0..state.snrkl.rows {
        let mut spn = vec![];
        let mut strng = String::new();
        for x in 0..state.snrkl.cols {
            let is_cursor = state.cursor.x == x && state.cursor.y == y;
            match state.snrkl.get(x, y) {
                Some(char) if is_cursor => {
                    spn.push(Span::raw(strng));
                    spn.push(Span::styled(
                        char.to_string(),
                        Style::default().bg(Color::Yellow),
                    ));
                    strng = String::new();
                }
                Some(char) => strng.push(char),
                None if is_cursor => {
                    spn.push(Span::raw(strng));
                    spn.push(Span::styled(
                        chars::EMPTY_CELL.to_string(),
                        Style::default().bg(Color::Yellow),
                    ));
                    strng = String::new();
                }
                None => strng.push(chars::EMPTY_CELL),
            }
        }
        spn.push(Span::raw(strng));
        text.push(Spans::from(spn))
    }

    let p = Paragraph::new(text).block(Block::default().title("Snorkel").borders(Borders::ALL));
    f.render_widget(p, chunks[1]);

    let tui_w = tui_logger::TuiLoggerWidget::default()
        .block(
            Block::default()
                .title("Logs")
                .border_style(
                    tui::style::Style::default()
                        .fg(Color::White)
                        .bg(Color::Black),
                )
                .borders(Borders::ALL),
        )
        .output_separator('|')
        .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
        .output_level(Some(tui_logger::TuiLoggerLevelOutput::Long))
        .output_target(false)
        .output_file(false)
        .output_line(false)
        .style(
            tui::style::Style::default()
                .fg(Color::White)
                .bg(Color::Black),
        );
    f.render_widget(tui_w, chunks[2]);
}

fn main() -> Result<()> {
    // ░█▀▀░▀█▀░█▀█░█▀▄░▀█▀░█░█░█▀█
    // ░▀▀█░░█░░█▀█░█▀▄░░█░░█░█░█▀▀
    // ░▀▀▀░░▀░░▀░▀░▀░▀░░▀░░▀▀▀░▀░░
    tui_logger::init_logger(log::LevelFilter::Info).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Info);

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;
    let backend = tui::backend::CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;

    // ░█▀█░█▀█░█▀█
    // ░█▀█░█▀▀░█▀▀
    // ░▀░▀░▀░░░▀░░

    let res = run_app(&mut terminal);

    // ░█▀▀░█░█░█░█░▀█▀░█▀▄░█▀█░█░█░█▀█
    // ░▀▀█░█▀█░█░█░░█░░█░█░█░█░█▄█░█░█
    // ░▀▀▀░▀░▀░▀▀▀░░▀░░▀▀░░▀▀▀░▀░▀░▀░▀

    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

#[cfg(test)]
mod move_cursor {
    use crate::{AppState, NormalModeCommand};

    #[test]
    fn move_cursor_around() {
        let mut app = AppState::new(20, 20);
        app.move_cursor(NormalModeCommand::MoveDown);
        app.move_cursor(NormalModeCommand::MoveRight);
        assert_eq!(app.cursor.x, 1);
        assert_eq!(app.cursor.y, 1);
    }

    #[test]
    fn should_handle_potential_overflow_correctly() {
        let mut app = AppState::new(20, 20);
        app.move_cursor(NormalModeCommand::MoveLeft);
        assert_eq!(app.cursor.x, 0);
        app.move_cursor(NormalModeCommand::MoveUp);
        assert_eq!(app.cursor.y, 0);
    }

    #[test]
    fn should_clamp_grid_size() {
        let mut app = AppState::new(20, 20);
        for _ in 0..22 {
            app.move_cursor(NormalModeCommand::MoveRight);
        }
        assert_eq!(app.cursor.x, app.snrkl.rows);
        for _ in 0..22 {
            app.move_cursor(NormalModeCommand::MoveDown);
        }
        assert_eq!(app.cursor.y, app.snrkl.rows);
    }
}
