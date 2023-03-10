pub const TOP_LEFT_CORNER: char = '⌌';
pub const TOP_RIGHT_CORNER: char = '⌍';
pub const BOTTOM_LEFT_CORNER: char = '⌎';
pub const BOTTOM_RIGHT_CORNER: char = '⌏';
pub const EMPTY_CELL: char = '⸱';
pub const NL: char = '\n';

pub fn init_char(rs: usize, cs: usize, row: usize, col: usize) -> char {
    match (row, col) {
        (r, c) if c == 0 && r == 0 => TOP_LEFT_CORNER,
        (r, c) if c == 0 && r == rs - 1 => BOTTOM_LEFT_CORNER,
        (r, c) if c == cs - 1 && r == 0 => TOP_RIGHT_CORNER,
        (r, c) if c == cs - 1 && r == rs - 1 => BOTTOM_RIGHT_CORNER,
        _ => EMPTY_CELL,
    }
}
