use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute, terminal, Result,
};
use snorkel::orca::Orca;
use std::{cmp, io};
use tui::{style::Color, widgets::Paragraph};

struct KeyBinding;

impl KeyBinding {
    fn is_ctrlc(ev: KeyEvent) -> bool {
        if let KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        } = ev
        {
            true
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub enum Cmd {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

struct AppState {
    orca: Orca,
    cursor: Coord,
}

impl AppState {
    pub fn new(rows: usize, cols: usize) -> AppState {
        let orca = Orca::new(rows, cols);
        let cursor = Coord::default();
        AppState { orca, cursor }
    }

    pub fn move_cursor(&mut self, cmd: Cmd) {
        let x = self.cursor.x;
        let y = self.cursor.y;
        let (new_x, new_y) = match cmd {
            Cmd::MoveDown => (x, cmp::min(y + 1, self.orca.rows)),
            Cmd::MoveUp => (x, y.checked_sub(1).unwrap_or(0)),
            Cmd::MoveLeft => (x.checked_sub(1).unwrap_or(0), y),
            Cmd::MoveRight => (cmp::min(x + 1, self.orca.cols), y),
        };
        self.cursor.x = new_x;
        self.cursor.y = new_y;
    }
}

fn run_app<B: tui::backend::Backend>(terminal: &mut tui::Terminal<B>) -> io::Result<()> {
    let state = AppState::new(20, 20);
    loop {
        terminal.draw(|f| ui(f, &state))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }

            if KeyBinding::is_ctrlc(key) {
                return Ok(());
            }

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

    let block = tui::widgets::Block::default()
        .title("Tabs")
        .borders(tui::widgets::Borders::ALL);
    f.render_widget(block, chunks[0]);

    let text = vec![];

    //     let beg = line[..5].to_owned();
    //     let end = line[5..].to_owned();
    //     vec![
    //         tui::text::Spans::from(tui::text::Span::styled(
    //             beg,
    //             tui::style::Style::default().fg(Color::Red),
    //         )),
    //         tui::text::Spans::from(tui::text::Span::styled(
    //             end,
    //             tui::style::Style::default().fg(Color::Green),
    //         )),
    //     ]
    // })
    // .collect::<Vec<tui::text::Spans>>();

    let p = Paragraph::new(text).block(
        tui::widgets::Block::default()
            .title("Snorkel")
            .borders(tui::widgets::Borders::ALL),
    );
    f.render_widget(p, chunks[1]);

    let tui_w = tui_logger::TuiLoggerWidget::default()
        .block(
            tui::widgets::Block::default()
                .title("Logs")
                .border_style(
                    tui::style::Style::default()
                        .fg(Color::White)
                        .bg(Color::Black),
                )
                .borders(tui::widgets::Borders::ALL),
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
    use crate::{AppState, Cmd};

    #[test]
    fn move_cursor_around() {
        let mut app = AppState::new(20, 20);
        app.move_cursor(Cmd::MoveDown);
        app.move_cursor(Cmd::MoveRight);
        assert_eq!(app.cursor.x, 1);
        assert_eq!(app.cursor.y, 1);
    }

    #[test]
    fn should_handle_potential_overflow_correctly() {
        let mut app = AppState::new(20, 20);
        app.move_cursor(Cmd::MoveLeft);
        assert_eq!(app.cursor.x, 0);
        app.move_cursor(Cmd::MoveUp);
        assert_eq!(app.cursor.y, 0);
    }

    #[test]
    fn should_clamp_grid_size() {
        let mut app = AppState::new(20, 20);
        for _ in 0..22 {
            app.move_cursor(Cmd::MoveRight);
        }
        assert_eq!(app.cursor.x, app.orca.cols);
        for _ in 0..22 {
            app.move_cursor(Cmd::MoveDown);
        }
        assert_eq!(app.cursor.y, app.orca.rows);
    }
}
