use crossterm::{
    event::{self, Event},
    execute, terminal, Result,
};
use snorkel::{
    mode::{NormalKeymap, NormalModeCommand},
    state::{self, EditorState},
    ui,
};
use std::io;

fn ui_loop<B: tui::backend::Backend>(terminal: &mut tui::Terminal<B>) -> io::Result<()> {
    let mut state = state::AppState::new(20, 80);
    loop {
        terminal.draw(|f| ui::render(f, &state))?;
        if let Event::Key(key) = event::read()? {
            if state.edit_state == EditorState::Normal {
                if let Some(NormalModeCommand::Exit) = NormalKeymap::exit(key) {
                    return Ok(());
                }
            }
            state.input(key);
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
    let mut terminal = tui::Terminal::new(backend)?;

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
