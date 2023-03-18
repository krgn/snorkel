use tui::style::{Color, Style};

const DARK_GREY: Color = Color::Rgb(90, 90, 90);

#[derive(Debug)]
pub struct CharConfig {
    pub top_left_corner: char,
    pub bottom_left_corner: char,
    pub top_right_corner: char,
    pub bottom_right_corner: char,
    pub grid_line_start: char,
    pub grid_line_end: char,
    pub grid_top_marker: char,
    pub grid_marker: char,
    pub grid_bottom_marker: char,
    pub empty: char,
}

impl Default for CharConfig {
    fn default() -> Self {
        Self {
            top_left_corner: '┌',
            top_right_corner: '┐',
            bottom_left_corner: '└',
            bottom_right_corner: '┘',
            grid_line_start: '├',
            grid_line_end: '┤',
            grid_top_marker: '┬',
            grid_marker: '+',
            grid_bottom_marker: '┴',
            empty: '·',
        }
    }
}

#[derive(Debug)]
pub struct StyleConfig {
    pub normal: Style,
    pub comment: Style,
    pub command: Style,
    pub value: Style,
    pub selection: Style,
    pub cursor: Style,
    pub result: Style,
}

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            normal: Style::default().fg(DARK_GREY),
            comment: Style::default().fg(Color::Black).bg(Color::DarkGray),
            command: Style::default().fg(Color::Black).bg(Color::Cyan),
            value: Style::default().fg(Color::White),
            selection: Style::default().fg(Color::Black).bg(Color::Magenta),
            cursor: Style::default().bg(Color::Yellow).fg(Color::Black),
            result: Style::default().bg(Color::White).fg(Color::Black),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub styles: StyleConfig,
    pub chars: CharConfig,
    pub grid_steps_x: u8,
    pub grid_steps_y: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            styles: StyleConfig::default(),
            chars: CharConfig::default(),
            grid_steps_x: 8,
            grid_steps_y: 8,
        }
    }
}
