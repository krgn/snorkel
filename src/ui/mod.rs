use tui::{
    layout::{self, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear},
};

use crate::state::{self, EditorState};

mod bottombar;
mod canvas;
mod topbar;

fn full_layout(size: Rect) -> Vec<Rect> {
    layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints(
            [
                layout::Constraint::Max(4),
                layout::Constraint::Percentage(80),
                layout::Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(size)
}

fn minimal_layout(size: Rect) -> Vec<Rect> {
    layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints(
            [
                layout::Constraint::Max(4),
                layout::Constraint::Percentage(90),
            ]
            .as_ref(),
        )
        .split(size)
}

pub fn render<B: tui::backend::Backend>(f: &mut tui::Frame<B>, state: &state::AppState) {
    let chunks = if state.show_logs {
        full_layout(f.size())
    } else {
        minimal_layout(f.size())
    };

    let p = topbar::render(&state);
    f.render_widget(p, chunks[0]);

    let p = canvas::render(&state);
    f.render_widget(p, chunks[1]);

    if state.show_logs {
        let p = bottombar::render(&state);
        f.render_widget(p, chunks[2]);
    }

    if state.edit_state == EditorState::QuitRequested {
        let block = Block::default()
            .title("Quit Snorkel?")
            .borders(Borders::ALL);
        let area = centered_rect(60, 20, f.size());
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(block, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
