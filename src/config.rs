use tui::style::{Color, Style};

const DARK_GREY: Color = Color::Rgb(90, 90, 90);

#[derive(Debug)]
pub struct CharConfig {
    pub top_left_corner: char,
    pub bottom_left_corner: char,
    pub top_right_corner: char,
    pub bottom_right_corner: char,
    pub empty: char,
}

impl Default for CharConfig {
    fn default() -> Self {
        Self {
            top_left_corner: '⌌',
            top_right_corner: '⌍',
            bottom_left_corner: '⌎',
            bottom_right_corner: '⌏',
            empty: '·',
        }
    }
}

#[derive(Debug)]
pub struct StyleConfig {
    pub normal_text: Style,
    pub comment: Style,
    pub command: Style,
    pub value: Style,
    pub selection: Style,
    pub cursor: Style,
}

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            normal_text: Style::default().fg(DARK_GREY),
            comment: Style::default().fg(Color::Black).bg(Color::DarkGray),
            command: Style::default().fg(Color::Black).bg(Color::Cyan),
            value: Style::default().fg(Color::White),
            selection: Style::default().fg(Color::Black).bg(Color::Magenta),
            cursor: Style::default().bg(Color::Yellow).fg(Color::Black),
        }
    }
}

#[derive(Debug, Default)]
pub struct Config {
    pub styles: StyleConfig,
    pub chars: CharConfig,
}
