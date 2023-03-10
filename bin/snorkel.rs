use crossterm::{
    event::{self, Event},
    execute, terminal, Result,
};
use snorkel::{
    chars,
    mode::{NormalKeymap, NormalModeCommand},
    state::{self, EditorState},
};
use std::io;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

fn run_app<B: tui::backend::Backend>(terminal: &mut tui::Terminal<B>) -> io::Result<()> {
    let mut state = state::AppState::new(20, 80);
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

fn ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, state: &state::AppState) {
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
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    ));
                    strng = String::new();
                }
                Some(char) => strng.push(char),
                None if is_cursor => {
                    spn.push(Span::raw(strng));
                    spn.push(Span::styled(
                        chars::EMPTY_CELL.to_string(),
                        Style::default().bg(Color::Yellow).fg(Color::Black),
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
    use crate::{state::AppState, NormalModeCommand};

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
        assert_eq!(app.cursor.x, app.snrkl.rows);
        for _ in 0..22 {
            app.move_cursor(NormalModeCommand::MoveDown(1));
        }
        assert_eq!(app.cursor.y, app.snrkl.rows);
    }
}
