use std::cmp;

use crate::{
    op::Op,
    state::UndoOp,
    util::{Coord, Selection},
};

pub struct Snorkel {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Vec<Option<Op>>>,
}

impl Snorkel {
    pub fn new(rows: usize, cols: usize) -> Snorkel {
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(vec![None; cols]);
        }
        assert_eq!(data.len(), rows);
        Snorkel { rows, cols, data }
    }

    pub fn copy_selection(&self, sel: &Selection) -> Vec<Vec<Option<Op>>> {
        let mut out = vec![];
        for y in sel.start_y..sel.end_y + 1 {
            let mut row = vec![];
            for x in sel.start_x..sel.end_x + 1 {
                let point = Coord { x, y };
                row.push(self.get_cell(&point));
            }
            out.push(row)
        }
        out
    }

    pub fn paste_selection(&mut self, loc: &Coord, selection: &Vec<Vec<Option<Op>>>) -> UndoOp {
        let mut undo_ops = vec![];
        'outer: for y in 0..selection.len() {
            let row = &selection[y]; // get the source row from the selection
            let offset_y = cmp::min(y + loc.y, self.rows);
            if offset_y >= self.rows {
                break 'outer;
            }
            'inner: for x in 0..row.len() {
                let offset_x = cmp::min(x + loc.x, self.cols);
                if offset_x >= self.cols {
                    break 'inner;
                }
                let target = Coord {
                    x: offset_x,
                    y: offset_y,
                };
                match row[x] {
                    Some(op) => {
                        let old = self.set_cell(&target, op);
                        undo_ops.push((target, old));
                    }
                    None => {
                        let old = self.del_cell(&target);
                        undo_ops.push((target, old));
                    }
                }
            }
        }
        UndoOp::batch(undo_ops)
    }

    pub fn get_cell(&self, loc: &Coord) -> Option<Op> {
        self.data[loc.y][loc.x]
    }

    pub fn set_cell(&mut self, loc: &Coord, op: Op) -> Option<Op> {
        if loc.y >= self.rows || loc.x >= self.cols {
            return None;
        }
        let old = self.get_cell(loc);
        self.data[loc.y][loc.x] = Some(op);
        old
    }

    pub fn del_cell(&mut self, loc: &Coord) -> Option<Op> {
        if loc.y >= self.rows || loc.x >= self.cols {
            return None;
        }
        let old = self.get_cell(loc);
        self.data[loc.y][loc.x] = None;
        old
    }

    pub fn resize(&mut self, x: usize, y: usize) {
        // first resize x (the cols) if necessary, then append
        // new rows if necessary.
        if x > self.cols {
            for row in &mut self.data {
                let diff = x.checked_sub(row.len()).unwrap_or(0);
                if diff > 0 {
                    row.extend(vec![None; diff]);
                }
            }
        }
        self.cols = x;

        // now add new rows if requested
        if y > self.rows {
            let diff = y.checked_sub(self.data.len()).unwrap_or(0);
            for _ in 0..diff {
                self.data.push(vec![None; self.cols])
            }
        }
        self.rows = y;
    }

    // Only used in tests.
    #[cfg(test)]
    pub fn render(&self) -> String {
        use crate::config::CharConfig;
        let chars = CharConfig::default();
        let mut out = String::with_capacity(self.rows * self.cols + self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(op) = &self.data[row][col] {
                    out.push(op.into());
                } else {
                    out.push(chars.empty);
                }
            }
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod copy_paste_tests {
    use crate::{
        op::Op,
        snorkel::Snorkel,
        util::{Coord, Selection},
    };

    #[test]
    fn should_copy_selection() {
        let mut snrkl = Snorkel::new(4, 20);
        snrkl.set_cell(&Coord { x: 1, y: 1 }, Op::Add);
        snrkl.set_cell(&Coord { x: 2, y: 2 }, Op::Clock);

        let rendered = snrkl.render();
        let expected = r#"
····················
·A··················
··C·················
····················
"#;
        assert_eq!(expected.trim_start(), rendered);

        let sel = Selection {
            start_x: 1,
            start_y: 1,
            end_x: 2,
            end_y: 2,
        };
        let copy_buffer = snrkl.copy_selection(&sel);

        assert_eq!(2, copy_buffer.len());
        assert_eq!(Some(Op::Add), copy_buffer[0][0]);
        assert_eq!(None, copy_buffer[0][1]);
        assert_eq!(None, copy_buffer[1][0]);
        assert_eq!(Some(Op::Clock), copy_buffer[1][1]);
    }

    #[test]
    fn should_paste_selection() {
        // should result in something like:
        // -----
        // |1A1|
        // |.2.|
        // -----
        let selection = vec![
            vec![Some(Op::Val('1')), Some(Op::Add), Some(Op::Val('1'))],
            vec![None, Some(Op::Val('2')), None],
        ];
        let target = Coord { x: 1, y: 2 };

        let mut snrkl = Snorkel::new(4, 20);
        snrkl.paste_selection(&target, &selection);
        let rendered = snrkl.render();
        let expected = r#"
····················
····················
·1A1················
··2·················
"#;
        assert_eq!(expected.trim_start(), rendered)
    }

    #[test]
    fn should_paste_not_crash_at_edge() {
        let selection = vec![
            vec![Some(Op::Val('1')), Some(Op::Add), Some(Op::Val('1'))],
            vec![None, Some(Op::Val('2')), None],
        ];
        let target = Coord { x: 1, y: 2 };

        let mut snrkl = Snorkel::new(3, 3);
        snrkl.paste_selection(&target, &selection);
        let rendered = snrkl.render();
        let expected = r#"
···
···
·1A
"#;
        assert_eq!(expected.trim_start(), rendered)
    }
}

#[cfg(test)]
mod tests {
    use crate::{op::Op, util::Coord};

    use super::Snorkel;

    #[test]
    fn create_new_snrkl_renders_correctly() {
        let snrkl = Snorkel::new(4, 20);
        let rendered = snrkl.render();
        let expected = r#"
····················
····················
····················
····················
"#;
        assert_eq!(expected.trim_start(), rendered)
    }

    #[test]
    fn process_op_should_work_correctly() {
        let mut snrkl = Snorkel::new(4, 20);
        let rendered = snrkl.render();
        let expected = r#"
····················
····················
····················
····················
"#;
        assert_eq!(expected.trim_start(), rendered);

        snrkl.set_cell(&Coord { x: 1, y: 1 }, Op::Add);
        snrkl.set_cell(&Coord { x: 19, y: 3 }, Op::Clock);

        let rendered = snrkl.render();
        let expected = r#"
····················
·A··················
····················
···················C
"#;
        assert_eq!(expected.trim_start(), rendered);
    }

    #[test]
    fn resize_should_work() {
        let mut snrkl = Snorkel::new(4, 4);
        snrkl.set_cell(&Coord { x: 1, y: 1 }, Op::Add);
        snrkl.set_cell(&Coord { x: 2, y: 2 }, Op::Clock);
        let rendered = snrkl.render();
        let expected = r#"
····
·A··
··C·
····
"#;
        assert_eq!(expected.trim_start(), rendered);

        snrkl.resize(10, 10);
        let rendered = snrkl.render();
        let expected = r#"
··········
·A········
··C·······
··········
··········
··········
··········
··········
··········
··········
"#;

        assert_eq!(expected.trim_start(), rendered);

        snrkl.set_cell(&Coord { x: 9, y: 9 }, Op::Uclid);
        let rendered = snrkl.render();
        let expected = r#"
··········
·A········
··C·······
··········
··········
··········
··········
··········
··········
·········U
"#;

        assert_eq!(expected.trim_start(), rendered);

        snrkl.resize(4, 4);
        let rendered = snrkl.render();
        let expected = r#"
····
·A··
··C·
····
"#;

        assert_eq!(expected.trim_start(), rendered);

        snrkl.resize(10, 10);
        let rendered = snrkl.render();
        let expected = r#"
··········
·A········
··C·······
··········
··········
··········
··········
··········
··········
·········U
"#;

        assert_eq!(expected.trim_start(), rendered);
    }
}
