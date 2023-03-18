use crate::state::{AppState, EditorState};
use tui::{
    layout::Constraint,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
};

pub fn render(state: &AppState) -> Table {
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
        EditorState::QuitRequested => Span::styled(
            "quitting",
            Style::default().bg(Color::Yellow).fg(Color::Black),
        ),
        EditorState::QuitConfirmed => {
            Span::styled("quit", Style::default().bg(Color::Yellow).fg(Color::Black))
        }
    };

    let grid = format!("{}x{}", state.snrkl.cols, state.snrkl.rows);
    let pos = format!("{},{}", state.cursor.x, state.cursor.y);
    let frame = format!("{}f", state.snrkl.frame);

    let rows = vec![
        Row::new(vec![
            Cell::from(grid),
            Cell::from(frame),
            Cell::from(""),
            Cell::from(""),
            Cell::from(""),
        ]),
        Row::new(vec![
            Cell::from(pos),
            Cell::from(editor_state),
            Cell::from(""),
            Cell::from(""),
            Cell::from(""),
        ]),
    ];

    Table::new(rows)
        .block(Block::default().borders(Borders::NONE))
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Length(20),
            Constraint::Min(10),
        ])
}
