use crossterm::{
    event::{self, Event},
    execute, terminal, Result,
};
use snorkel::{
    state::{self, EditorState},
    ui,
};
use std::io;
use tui::{backend::Backend, Terminal};

fn ui_loop<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut state = state::AppState::new(20, 80);
    loop {
        if state.edit_state == EditorState::QuitConfirmed {
            return Ok(());
        }

        terminal.draw(|frame| ui::render(frame, &state))?;

        if let Event::Key(key) = event::read()? {
            state.input(key);
            state.tick();
        }
    }
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
    let mut terminal = Terminal::new(backend)?;

    // ░█▀█░█▀█░█▀█
    // ░█▀█░█▀▀░█▀▀
    // ░▀░▀░▀░░░▀░░

    let res = ui_loop(&mut terminal);

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
