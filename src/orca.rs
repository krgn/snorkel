//! Module for all internal Orca things.

use std::cmp;

use jumprope::JumpRope;

pub enum Op {
    Add,
    Bang,
    Clock,
    Comment,
    East,
    Gen,
    Hold,
    If,
    Inc,
    Jmp,
    Konkat,
    Lerp,
    Less,
    Mul,
    NoOp,
    North,
    Push,
    Query,
    Rand,
    Read,
    South,
    Sub,
    Track,
    Uclid,
    Var,
    West,
    Write,
    Ymp,
}

impl From<Op> for char {
    fn from(value: Op) -> Self {
        match value {
            Op::Add => 'A',
            Op::Bang => '*',
            Op::Clock => 'C',
            Op::Comment => '#',
            Op::East => 'E',
            Op::Gen => 'G',
            Op::Hold => 'H',
            Op::If => 'F',
            Op::Inc => 'I',
            Op::Jmp => 'J',
            Op::Konkat => 'K',
            Op::Lerp => 'Z',
            Op::Less => 'L',
            Op::Mul => 'M',
            Op::NoOp => EMPTY_CELL,
            Op::North => 'N',
            Op::Push => 'P',
            Op::Query => 'Q',
            Op::Rand => 'R',
            Op::Read => 'O',
            Op::South => 'S',
            Op::Sub => 'B',
            Op::Track => 'T',
            Op::Uclid => 'U',
            Op::Var => 'V',
            Op::West => 'W',
            Op::Write => 'X',
            Op::Ymp => 'Y',
        }
    }
}

impl From<char> for Op {
    fn from(value: char) -> Self {
        match value {
            'A' => Op::Add,
            '*' => Op::Bang,
            'C' => Op::Clock,
            '#' => Op::Comment,
            'E' => Op::East,
            'G' => Op::Gen,
            'H' => Op::Hold,
            'F' => Op::If,
            'I' => Op::Inc,
            'J' => Op::Jmp,
            'K' => Op::Konkat,
            'Z' => Op::Lerp,
            'L' => Op::Less,
            'M' => Op::Mul,
            'N' => Op::North,
            'P' => Op::Push,
            'Q' => Op::Query,
            'R' => Op::Rand,
            'O' => Op::Read,
            'S' => Op::South,
            'B' => Op::Sub,
            'T' => Op::Track,
            'U' => Op::Uclid,
            'V' => Op::Var,
            'W' => Op::West,
            'X' => Op::Write,
            'Y' => Op::Ymp,
            _ => Op::NoOp,
        }
    }
}

/// Orca is the central data structure to track the state of
/// snorkel application.
pub struct Orca {
    pub cols: usize,
    pub data: JumpRope,
    pub rows: usize,
}

const TOP_LEFT_CORNER: char = '⌌';
const TOP_RIGHT_CORNER: char = '⌍';
const BOTTOM_LEFT_CORNER: char = '⌎';
const BOTTOM_RIGHT_CORNER: char = '⌏';
const EMPTY_CELL: char = '⸱';
const NL: char = '\n';

fn init_char(rs: usize, cs: usize, row: usize, col: usize) -> char {
    match (row, col) {
        (r, c) if c == 0 && r == 0 => TOP_LEFT_CORNER,
        (r, c) if c == 0 && r == rs - 1 => BOTTOM_LEFT_CORNER,
        (r, c) if c == cs - 1 && r == 0 => TOP_RIGHT_CORNER,
        (r, c) if c == cs - 1 && r == rs - 1 => BOTTOM_RIGHT_CORNER,
        _ => EMPTY_CELL,
    }
}

fn init_rope(rows: usize, cols: usize) -> JumpRope {
    let mut str = String::with_capacity(rows * (cols + 1));
    for row in 0..rows {
        for col in 0..cols {
            let char = init_char(rows, cols, row, col);
            str.push(char);
        }
        str.push(NL);
    }
    JumpRope::from(str)
}

fn is_whitespace(c: char) -> bool {
    match c {
        c if c == TOP_LEFT_CORNER => true,
        c if c == TOP_RIGHT_CORNER => true,
        c if c == BOTTOM_LEFT_CORNER => true,
        c if c == BOTTOM_RIGHT_CORNER => true,
        c if c == EMPTY_CELL => true,
        c if c == NL => true,
        _ => false,
    }
}

impl Orca {
    pub fn new(rows: usize, cols: usize) -> Orca {
        let data = init_rope(rows, cols);
        Orca { rows, cols, data }
    }

    // ░█▀▄░█▀▀░█▀▀░▀█▀░▀▀█░█▀▀
    // ░█▀▄░█▀▀░▀▀█░░█░░▄▀░░█▀▀
    // ░▀░▀░▀▀▀░▀▀▀░▀▀▀░▀▀▀░▀▀▀

    pub fn resize(&mut self, rows: usize, cols: usize) {
        // build new rope...
        let mut new_rope = init_rope(rows, cols);
        // ...and copy old contents into it
        'rows: for row in 0..self.rows {
            // the new rope may contain less rows, so break to prevent panic
            if row >= rows {
                break 'rows;
            }
            // each row has a specific offset
            let offset = row * (self.cols + 1);
            let line_end = offset + self.cols;
            let mut idx = 0;
            'cols: for char in self.data.slice_chars(offset..line_end) {
                // resizing into something smaller, break to prevent panic
                if idx >= cols {
                    break 'cols;
                }
                if !is_whitespace(char) {
                    let dest = row * (cols + 1) + idx;
                    new_rope.replace(dest..dest + 1, char.to_string().as_str());
                }
                idx += 1;
            }
        }
        self.cols = cols;
        self.rows = rows;
        self.data = new_rope;
    }

    pub fn render(&self) -> String {
        self.data.to_string()
    }

    // ░█▀▀░█▀█░█▀█░█░█
    // ░█░░░█░█░█▀▀░░█░
    // ░▀▀▀░▀▀▀░▀░░░░▀░

    pub fn copy_selection(
        &self,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
    ) -> Vec<String> {
        // the length of a line in the current setup
        let line_len = self.cols + 1;
        // the total length of the rope
        let total_chars = line_len * self.rows;

        // sanity check
        assert_eq!(self.data.len_chars(), total_chars);

        // sanitized rectangle:
        // we can get to the normalized start point by chosing
        // the lowest value for both, x and y.
        let start_x = cmp::min(start_x, end_x);
        let start_y = cmp::min(start_y, end_y);

        // same procedure, but also constrain the end point
        // by the rectangle size
        let end_x = cmp::min(cmp::max(start_x, end_x), self.cols - 1);
        let end_y = cmp::min(cmp::max(start_y, end_y), self.rows);

        // here we can safely already return nothing, as the grid
        // is simply not big enough for there to be any data to copy
        if start_x > self.cols || start_y > self.rows {
            return vec![];
        }

        let len_x = 1 + end_x - start_x;
        let len_y = 1 + end_y - start_y;

        // it makes no sense to proceed any further if the inputs
        // are zero, so we return early
        if len_y == 0 || len_x == 0 {
            return vec![];
        }

        let mut lines = Vec::with_capacity(len_y);

        'rows: for row in start_y..len_y + 1 {
            if row >= self.rows {
                break 'rows;
            }
            let offset = (row * line_len) + start_x;
            let end = offset + len_x;
            let mut str = String::with_capacity(len_x);
            for char in self.data.slice_chars(offset..end) {
                str.push(char);
            }
            println!(
                "row: {} offset: {} end: {} str: {:#?}",
                row, offset, end, str
            );
            lines.push(str);
        }
        return lines;
    }

    // ░█▀█░█▀█░█▀▀░▀█▀░█▀▀
    // ░█▀▀░█▀█░▀▀█░░█░░█▀▀
    // ░▀░░░▀░▀░▀▀▀░░▀░░▀▀▀

    pub fn paste_selection(&mut self, x: usize, y: usize, sel: &Vec<String>) {
        let mut cnt = 0;
        for slc in sel {
            self.paste_slice(x, y + cnt, slc);
            cnt += 1;
        }
    }

    pub fn paste_slice(&mut self, x: usize, y: usize, glyph: &str) {
        let len = glyph.len();
        if len == 0 || x >= self.cols || y >= self.rows {
            return;
        }
        // general offset in the rope
        let start = y * self.cols + x + y;
        // check if the inserted string is longer than the line
        let oversized = x + len > self.cols;
        // truncate the end idx if oversized
        let end = if oversized {
            start + (self.cols - x)
        } else {
            start + len
        };
        // truncate the glyph slice if oversized
        let slc = if oversized {
            &glyph[..self.cols - x]
        } else {
            glyph
        };
        self.data.replace(start..end, &slc);
    }
}

// ░▀█▀░█▀▀░█▀▀░▀█▀░█▀▀
// ░░█░░█▀▀░▀▀█░░█░░▀▀█
// ░░▀░░▀▀▀░▀▀▀░░▀░░▀▀▀

#[cfg(test)]
mod tests {
    use super::Orca;

    #[test]
    fn create_new_orca_renders_correctly() {
        let orca = Orca::new(4, 20);
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌍
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⌎⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered)
    }

    #[test]
    fn paste_slice_renders_correctly() {
        let mut orca = Orca::new(4, 20);
        orca.paste_slice(5, 2, "D");
        orca.paste_slice(13, 1, "X");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌍
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱X⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱D⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⌎⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered)
    }

    #[test]
    fn resize_bigger_renders_correctly() {
        let mut orca = Orca::new(4, 5);
        orca.paste_slice(3, 2, "D");
        orca.paste_slice(4, 3, "A");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⌍
⸱⸱⸱⸱⸱
⸱⸱⸱D⸱
⌎⸱⸱⸱A
"#;
        assert_eq!(expected.trim_start(), rendered);

        orca.resize(6, 20);
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌍
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱D⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱A⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⌎⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered)
    }

    #[test]
    fn paste_slice_should_ignore_oob() {
        let mut orca = Orca::new(5, 5);
        orca.paste_slice(6, 2, "X");
        orca.paste_slice(2, 6, "Y");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⌍
⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱
⌎⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered);
    }

    #[test]
    fn paste_slice_should_set_last() {
        let mut orca = Orca::new(5, 5);
        orca.paste_slice(4, 4, "X");
        orca.paste_slice(0, 4, "Y");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⌍
⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱
Y⸱⸱⸱X
"#;
        assert_eq!(expected.trim_start(), rendered);
    }

    #[test]
    fn paste_slice_should_ignore_empty_str() {
        let mut orca = Orca::new(5, 5);
        orca.paste_slice(0, 4, "");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⌍
⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱
⌎⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered);
    }

    #[test]
    fn paste_slice_should_set_all_chars() {
        let mut orca = Orca::new(5, 5);
        orca.paste_slice(1, 1, "foo");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⌍
⸱foo⸱
⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱
⌎⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered);
    }

    #[test]
    fn paste_slice_should_set_truncate_excess_chars() {
        let mut orca = Orca::new(5, 5);
        orca.paste_slice(1, 1, "foobar");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⌍
⸱foob
⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱
⌎⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered);
    }

    #[test]
    fn resize_smaller_renders_correctly() {
        let mut orca = Orca::new(6, 20);
        orca.paste_slice(2, 2, "X");
        orca.paste_slice(17, 4, "F");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌍
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱X⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱F⸱⸱
⌎⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered);

        orca.resize(5, 5);
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⌍
⸱⸱⸱⸱⸱
⸱⸱X⸱⸱
⸱⸱⸱⸱⸱
⌎⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered)
    }
}

// ░█▀▀░█▀█░█▀█░█░█
// ░█░░░█░█░█▀▀░░█░
// ░▀▀▀░▀▀▀░▀░░░░▀░

#[cfg(test)]
mod copy_selection {
    use crate::orca::Orca;

    #[test]
    fn copy_selection_of_multiple_lines() {
        let mut orca = Orca::new(6, 20);
        orca.paste_slice(2, 2, "X");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌍
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱X⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⌎⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered);

        let lines = orca.copy_selection(1, 1, 3, 3);
        assert_eq!(3, lines.len());
        assert_eq!("⸱⸱⸱", lines[0]);
        assert_eq!("⸱X⸱", lines[1]);
        assert_eq!("⸱⸱⸱", lines[2]);
    }

    #[test]
    fn copy_paste_selection_should_be_correct() {
        let mut orca = Orca::new(6, 20);
        orca.paste_slice(1, 1, "-----");
        orca.paste_slice(1, 2, "-111-");
        orca.paste_slice(1, 3, "-222-");
        orca.paste_slice(1, 4, "-333-");
        orca.paste_slice(1, 5, "-----");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌍
⸱-----⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱-111-⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱-222-⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱-333-⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⌎-----⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered);

        let lines = orca.copy_selection(1, 1, 5, 6);
        assert_eq!(5, lines.len());
        assert_eq!("-----", lines[0]);
        assert_eq!("-111-", lines[1]);
        assert_eq!("-222-", lines[2]);
        assert_eq!("-333-", lines[3]);
        assert_eq!("-----", lines[4]);

        orca.paste_selection(6, 1, &lines);
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌍
⸱----------⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱-111--111-⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱-222--222-⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱-333--333-⸱⸱⸱⸱⸱⸱⸱⸱⸱
⌎----------⸱⸱⸱⸱⸱⸱⸱⸱⌏
"#;
        assert_eq!(expected.trim_start(), rendered);
    }

    #[test]
    fn copy_selection_of_edge_lines() {
        let mut orca = Orca::new(6, 20);
        orca.paste_slice(17, 3, "1");
        orca.paste_slice(18, 3, "1");
        orca.paste_slice(19, 3, "1");
        orca.paste_slice(17, 4, "2");
        orca.paste_slice(18, 4, "2");
        orca.paste_slice(19, 4, "2");
        orca.paste_slice(17, 5, "3");
        orca.paste_slice(18, 5, "3");
        orca.paste_slice(19, 5, "3");
        let rendered = orca.render();
        let expected = r#"
⌌⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⌍
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱111
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱222
⌎⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱333
"#;
        assert_eq!(expected.trim_start(), rendered);

        let lines = orca.copy_selection(16, 2, 22, 6);
        assert_eq!(4, lines.len());
        assert_eq!("⸱⸱⸱⸱", lines[0]);
        assert_eq!("⸱111", lines[1]);
        assert_eq!("⸱222", lines[2]);
        assert_eq!("⸱333", lines[3]);
    }
}
