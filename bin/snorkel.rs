use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute, terminal, Result,
};
use snorkel::orca::Orca;
use std::io;
use tui::{style::Color, widgets::Paragraph};

fn run_app<B: tui::backend::Backend>(terminal: &mut tui::Terminal<B>) -> io::Result<()> {
    let orca = Orca::new(20, 20);
    loop {
        terminal.draw(|f| ui(f, &orca))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
            // control-c
            if let KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } = key
            {
                return Ok(());
            }
            log::info!("char: {:#?}", key);
        }
    }
}

fn ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, o: &snorkel::orca::Orca) {
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

    let p = Paragraph::new(o.render()).block(
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
