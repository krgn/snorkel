use crate::{
    state::{AppState, EditorState},
    util::{Coord, Selection},
};
use tui::{
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

fn in_sel_area(sel: &Option<Selection>, loc: &Coord) -> bool {
    match sel {
        Some(ref sel) => sel.contains(loc),
        None => false,
    }
}

pub fn render(state: &AppState) -> Paragraph {
    let select_mode = state.edit_state == EditorState::Select;
    let selection = state
        .sel_start
        .as_ref()
        .map(|sel| Selection::from(&state.cursor, &sel));

    let styles = &state.config.styles;
    let chars = &state.config.chars;

    let max_x = state.snrkl.cols - 1;
    let max_y = state.snrkl.rows - 1;
    let grid_x = state.config.grid_steps_x as usize;
    let grid_y = state.config.grid_steps_y as usize;

    let mut text = vec![];
    // go through all rows
    for y in 0..state.snrkl.rows {
        let mut spn = vec![];
        let mut frag = String::new();
        let mut in_comment = false;
        let mut in_selection = false;

        // go through all cells in current row
        for x in 0..state.snrkl.cols {
            let point = Coord { x, y };
            let is_cursor = state.cursor == point;

            if let Some(ref op) = state.snrkl.get_cell(&point) {
                let is_comment = op.is_comment();
                let is_primop = op.is_primop();
                let is_value = op.is_value();

                let c: char = op.as_char(&chars);

                // ░█▀▀░█▀▀░█░░░█▀▀░█▀▀░▀█▀░▀█▀░█▀█░█▀█
                // ░▀▀█░█▀▀░█░░░█▀▀░█░░░░█░░░█░░█░█░█░█
                // ░▀▀▀░▀▀▀░▀▀▀░▀▀▀░▀▀▀░░▀░░▀▀▀░▀▀▀░▀░▀
                if select_mode && in_sel_area(&selection, &point) && !in_selection {
                    spn.push(Span::styled(frag, styles.normal));
                    spn.push(Span::styled(c.to_string(), styles.cursor));
                    in_selection = true;
                    frag = String::new();
                } else if select_mode && in_sel_area(&selection, &point) && in_selection {
                    frag.push(c);
                } else if select_mode && !in_sel_area(&selection, &point) && in_selection {
                    spn.push(Span::styled(frag, styles.selection));
                    in_selection = false;
                    frag = String::new();
                    frag.push(c);
                }
                // ░█▀▀░█░█░█▀▄░█▀▀░█▀█░█▀▄
                // ░█░░░█░█░█▀▄░▀▀█░█░█░█▀▄
                // ░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀▀▀░▀░▀
                else if is_cursor && is_comment && in_comment {
                    spn.push(Span::styled(frag, styles.comment));
                    spn.push(Span::styled(c.to_string(), styles.cursor));
                    in_comment = false;
                    frag = String::new();
                } else if is_cursor && is_comment && !in_comment {
                    spn.push(Span::styled(frag, styles.normal));
                    spn.push(Span::styled(c.to_string(), styles.cursor));
                    in_comment = true;
                    frag = String::new();
                } else if is_cursor {
                    let style = if in_comment {
                        styles.comment
                    } else {
                        styles.normal
                    };
                    spn.push(Span::styled(frag, style));
                    spn.push(Span::styled(c.to_string(), styles.cursor));
                    frag = String::new();
                }
                // ░█▀▀░█▀█░█▄█░█▄█░█▀▀░█▀█░▀█▀
                // ░█░░░█░█░█░█░█░█░█▀▀░█░█░░█░
                // ░▀▀▀░▀▀▀░▀░▀░▀░▀░▀▀▀░▀░▀░░▀░
                else if is_comment && !in_comment {
                    in_comment = true;
                    spn.push(Span::styled(frag, styles.normal));
                    spn.push(Span::styled(c.to_string(), styles.comment));
                    frag = String::new();
                } else if is_comment && in_comment {
                    in_comment = false;
                    frag.push(c);
                    spn.push(Span::styled(frag, styles.comment));
                    frag = String::new();
                }
                // ░█▀▀░█▀█░█▄█░█▄█░█▀█░█▀█░█▀▄░█▀▀
                // ░█░░░█░█░█░█░█░█░█▀█░█░█░█░█░▀▀█
                // ░▀▀▀░▀▀▀░▀░▀░▀░▀░▀░▀░▀░▀░▀▀░░▀▀▀
                else if is_primop && !in_comment {
                    spn.push(Span::styled(frag, styles.normal));
                    spn.push(Span::styled(c.to_string(), styles.command));
                    frag = String::new();
                } else if is_primop && in_comment {
                    frag.push(c)
                }
                //
                // // ░█▀▄░█▀█░█▀█░█▀▀
                // // ░█▀▄░█▀█░█░█░█░█
                // // ░▀▀░░▀░▀░▀░▀░▀▀▀
                // else if op.is_bang() && !in_comment {
                //     spn.push(Span::styled(frag, styles.normal));
                //     spn.push(Span::styled(c.to_string(), styles.result));
                //     frag = String::new();
                // } else if op.is_bang() && in_comment {
                //     frag.push(c)
                // }
                //
                // ░█▀▄░█▀▀░█▀▀░█░█░█░░░▀█▀
                // ░█▀▄░█▀▀░▀▀█░█░█░█░░░░█░
                // ░▀░▀░▀▀▀░▀▀▀░▀▀▀░▀▀▀░░▀░
                else if op.is_result() && !in_comment {
                    spn.push(Span::styled(frag, styles.normal));
                    spn.push(Span::styled(c.to_string(), styles.result));
                    frag = String::new();
                } else if op.is_result() && in_comment {
                    frag.push(c)
                }
                // ░█░█░█▀█░█░░░█░█░█▀▀░█▀▀
                // ░▀▄▀░█▀█░█░░░█░█░█▀▀░▀▀█
                // ░░▀░░▀░▀░▀▀▀░▀▀▀░▀▀▀░▀▀▀
                else if is_value && !in_comment {
                    spn.push(Span::styled(frag, styles.normal));
                    spn.push(Span::styled(c.to_string(), styles.value));
                    frag = String::new();
                } else if is_value && in_comment {
                    frag.push(c)
                } else {
                    frag.push(c)
                }
            } else {
                let c = if x == 0 && y == 0 {
                    chars.top_left_corner
                } else if x == max_x && y == 0 {
                    chars.top_right_corner
                } else if x == 0 && y == max_y {
                    chars.bottom_left_corner
                } else if x == max_x && y == max_y {
                    chars.bottom_right_corner
                } else if x != 0 && x != max_x && x % grid_x == 0 && y == 0 {
                    chars.grid_top_marker
                } else if x != 0 && x != max_x && x % grid_x == 0 && y == max_y {
                    chars.grid_bottom_marker
                } else if x == 0 && x != max_x && y % grid_y == 0 {
                    chars.grid_line_start
                } else if x == max_x && y % grid_y == 0 {
                    chars.grid_line_end
                } else if x != 0 && x != max_x && x % grid_x == 0 && y % grid_y == 0 {
                    chars.grid_marker
                } else {
                    chars.empty
                };

                if select_mode && in_sel_area(&selection, &point) && !in_selection {
                    spn.push(Span::styled(frag, styles.normal));
                    in_selection = true;
                    frag = String::new();
                    frag.push(c)
                } else if select_mode && in_sel_area(&selection, &point) && in_selection {
                    frag.push(c)
                } else if select_mode && !in_sel_area(&selection, &point) && in_selection {
                    frag.push(c);
                    spn.push(Span::styled(frag, styles.selection));
                    frag = String::new();
                    in_selection = false;
                } else if is_cursor {
                    let style = if in_comment {
                        styles.comment
                    } else {
                        styles.normal
                    };
                    spn.push(Span::styled(frag, style));
                    spn.push(Span::styled(c.to_string(), styles.cursor));
                    frag = String::new();
                } else {
                    frag.push(c)
                }
            }
        }

        if in_comment {
            spn.push(Span::styled(frag, styles.comment));
        } else {
            spn.push(Span::styled(frag, styles.normal));
        }
        text.push(Spans::from(spn))
    }

    Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE))
        .alignment(tui::layout::Alignment::Center)
}
