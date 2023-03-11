use tui::{
    style::Color,
    widgets::{Block, Borders},
};

use crate::state::AppState;

pub fn render(_state: &AppState) -> tui_logger::TuiLoggerWidget {
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

    tui_w
}
