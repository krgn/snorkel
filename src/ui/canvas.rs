use super::DARK_GREY;
use crate::{chars, state::AppState, util::Coord};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(state: &AppState) -> Paragraph {
    let mut text = vec![];

    let bg_text_style = Style::default().fg(DARK_GREY);
    let comment_style = Style::default().fg(Color::Black).bg(Color::DarkGray);
    let command_style = Style::default().fg(Color::Black).bg(Color::Cyan);
    let val_style = Style::default().fg(Color::White);

    for y in 0..state.snrkl.rows {
        let mut spn = vec![];
        let mut strng = String::new();
        let mut in_comment = false;
        for x in 0..state.snrkl.cols {
            let is_cursor = state.cursor.x == x && state.cursor.y == y;
            match state.snrkl.get_cell(&Coord { x, y }) {
                Some(ref op) if is_cursor && !op.is_comment() => {
                    let chr: char = op.into();
                    let style = if in_comment {
                        comment_style
                    } else {
                        bg_text_style
                    };
                    spn.push(Span::styled(strng, style));
                    spn.push(Span::styled(
                        chr.to_string(),
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    ));
                    strng = String::new();
                }
                Some(ref op) if is_cursor && op.is_comment() && !in_comment => {
                    let chr: char = op.into();
                    spn.push(Span::styled(strng, bg_text_style));
                    spn.push(Span::styled(
                        chr.to_string(),
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    ));
                    in_comment = true;
                    strng = String::new();
                }
                Some(ref op) if is_cursor && op.is_comment() && in_comment => {
                    let chr: char = op.into();
                    spn.push(Span::styled(strng, comment_style));
                    spn.push(Span::styled(
                        chr.to_string(),
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    ));
                    in_comment = false;
                    strng = String::new();
                }
                Some(ref op) if op.is_primop() => {
                    let chr: char = op.into();
                    let prev_style = if in_comment {
                        comment_style
                    } else {
                        bg_text_style
                    };
                    let cur_style = if in_comment {
                        comment_style
                    } else {
                        command_style
                    };
                    spn.push(Span::styled(strng, prev_style));
                    spn.push(Span::styled(chr.to_string(), cur_style));
                    strng = String::new();
                }
                Some(ref op) if op.is_comment() && !in_comment => {
                    let chr: char = op.into();
                    spn.push(Span::styled(strng, bg_text_style));
                    spn.push(Span::styled(chr.to_string(), comment_style));
                    in_comment = true;
                    strng = String::new();
                }
                Some(ref op) if op.is_comment() && in_comment => {
                    let chr: char = op.into();
                    spn.push(Span::styled(strng, comment_style));
                    spn.push(Span::styled(chr.to_string(), comment_style));
                    in_comment = false;
                    strng = String::new();
                }
                Some(ref op) => {
                    let chr: char = op.into();
                    let prev_style = if in_comment {
                        comment_style
                    } else {
                        bg_text_style
                    };
                    let cur_style = if in_comment { comment_style } else { val_style };
                    spn.push(Span::styled(strng, prev_style));
                    spn.push(Span::styled(chr.to_string(), cur_style));
                    strng = String::new();
                }
                None if is_cursor => {
                    let style = if in_comment {
                        comment_style
                    } else {
                        bg_text_style
                    };
                    spn.push(Span::styled(strng, style));
                    spn.push(Span::styled(
                        chars::EMPTY_CELL.to_string(),
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    ));
                    strng = String::new();
                }
                None => strng.push(chars::EMPTY_CELL),
            }
        }
        if in_comment {
            spn.push(Span::styled(strng, comment_style));
        } else {
            spn.push(Span::styled(strng, bg_text_style));
        }
        text.push(Spans::from(spn))
    }

    Paragraph::new(text).block(Block::default().title("Snorkel").borders(Borders::ALL))
}
