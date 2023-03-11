use crate::state::{AppState, EditorState};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(state: &AppState) -> Paragraph {
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

    Paragraph::new(items).block(Block::default().borders(Borders::ALL))
}
