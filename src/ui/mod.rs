use tui::style::Color;

use crate::state;

mod bottombar;
mod canvas;
mod topbar;

const DARK_GREY: Color = Color::Rgb(90, 90, 90);

pub fn render<B: tui::backend::Backend>(f: &mut tui::Frame<B>, state: &state::AppState) {
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

    let p = topbar::render(&state);
    f.render_widget(p, chunks[0]);

    let p = canvas::render(&state);
    f.render_widget(p, chunks[1]);

    let p = bottombar::render(&state);
    f.render_widget(p, chunks[2]);
}
