//! Module for all internal Orca things.

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
    cols: usize,
    data: JumpRope,
    rows: usize,
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
