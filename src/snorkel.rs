use std::cmp;

use crate::{
    op::Op,
    state::UndoOp,
    util::{Coord, Selection},
};

pub struct Snorkel {
    pub rows: usize,
    pub cols: usize,
    pub frame: usize,
    data: Vec<Vec<Option<Op>>>,
}

impl Snorkel {
    // ░█▀█░█▀▀░█░█
    // ░█░█░█▀▀░█▄█
    // ░▀░▀░▀▀▀░▀░▀
    pub fn new(rows: usize, cols: usize) -> Snorkel {
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(vec![None; cols]);
        }
        let frame = 0;
        assert_eq!(data.len(), rows);
        Snorkel {
            rows,
            cols,
            data,
            frame,
        }
    }

    // ░▀█▀░▀█▀░█▀▀░█░█
    // ░░█░░░█░░█░░░█▀▄
    // ░░▀░░▀▀▀░▀▀▀░▀░▀
    pub fn tick(&mut self) {
        let mut coord = Coord { x: 0, y: 0 };
        for y in 0..self.rows {
            for x in 0..self.cols {
                coord.x = x;
                coord.y = y;
                match self.get_cell(&coord) {
                    Some(Op::Add) => {
                        if let Some(result) = self.op_add(&coord) {
                            coord.y += 1;
                            let _ignored = self.set_cell(&coord, result);
                        }
                    }
                    Some(Op::Sub) => {
                        if let Some(result) = self.op_sub(&coord) {
                            coord.y += 1;
                            let _ignored = self.set_cell(&coord, result);
                        }
                    }
                    Some(Op::If) => {
                        if let Some(result) = self.op_if(&coord) {
                            coord.y += 1;
                            let _ignored = self.set_cell(&coord, result);
                        }
                    }
                    Some(Op::Clock) => {
                        if let Some(result) = self.op_clock(&coord) {
                            coord.y += 1;
                            let _ignored = self.set_cell(&coord, result);
                        }
                    }
                    Some(Op::Delay) => {
                        let mut below = coord.clone();
                        below.y += 1;
                        let _ignored = match self.op_delay(&coord) {
                            Some(op) => self.set_cell(&below, op),
                            None => self.del_cell(&below),
                        };
                    }
                    _ => (),
                }
            }
        }
    }

    // ░█▀▀░█▀█░█▀█░█░█
    // ░█░░░█░█░█▀▀░░█░
    // ░▀▀▀░▀▀▀░▀░░░░▀░
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

    // ░█▀█░█▀█░█▀▀░▀█▀░█▀▀
    // ░█▀▀░█▀█░▀▀█░░█░░█▀▀
    // ░▀░░░▀░▀░▀▀▀░░▀░░▀▀▀
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

    // ░█▀█░█▀█░█▀▀
    // ░█░█░█▀▀░▀▀█
    // ░▀▀▀░▀░░░▀▀▀

    fn op_add(&self, loc: &Coord) -> Option<Op> {
        let left = self.left_of(loc, 1);
        let right = self.right_of(loc, 1);
        match (left, right) {
            (Some(lhs), Some(rhs)) => Op::add(lhs, rhs),
            (Some(Op::Val(c)), None) => Some(Op::Result(c)),
            (Some(Op::Result(c)), None) => Some(Op::Result(c)),
            (Some(Op::EmptyResult), None) => Some(Op::Result('0')),
            (None, Some(Op::Val(c))) => Some(Op::Result(c)),
            (None, Some(Op::Result(c))) => Some(Op::Result(c)),
            (None, Some(Op::EmptyResult)) => Some(Op::Result('0')),
            _ => Some(Op::Result('0')),
        }
    }

    fn op_sub(&self, loc: &Coord) -> Option<Op> {
        let left = self.left_of(loc, 1);
        let right = self.right_of(loc, 1);
        match (left, right) {
            (Some(lhs), Some(rhs)) => Op::sub(lhs, rhs),
            (Some(Op::Val(c)), None) => Some(Op::Result(c)),
            (Some(Op::Result(c)), None) => Some(Op::Result(c)),
            (Some(Op::EmptyResult), None) => Some(Op::Result('0')),
            (None, Some(Op::Val(c))) => Some(Op::Result(c)),
            (None, Some(Op::Result(c))) => Some(Op::Result(c)),
            (None, Some(Op::EmptyResult)) => Some(Op::Result('0')),
            _ => Some(Op::Result('0')),
        }
    }

    fn op_if(&self, loc: &Coord) -> Option<Op> {
        let left = self.left_of(loc, 1);
        let right = self.right_of(loc, 1);
        match (left, right) {
            (Some(lhs), Some(rhs)) if lhs == rhs => Some(Op::Bang),
            (Some(_), Some(_)) => Some(Op::EmptyResult),
            (Some(_), None) => Some(Op::EmptyResult),
            (None, Some(_)) => Some(Op::EmptyResult),
            (None, None) => Some(Op::Bang),
        }
    }

    fn op_clock(&self, loc: &Coord) -> Option<Op> {
        let rate = self
            .left_of(loc, 1)
            .and_then(|op| match op {
                Op::Result(c) | Op::Val(c) => Op::as_num(c),
                _ => Some(1),
            })
            .unwrap_or(1);
        let modulo = self
            .right_of(loc, 1)
            .and_then(|op| match op {
                Op::Result(c) | Op::Val(c) => Op::as_num(c),
                _ => Some(1),
            })
            .unwrap_or(8);
        let below = self.below_of(loc, 1);
        match below {
            Some(op) if self.frame % rate == 0 => {
                Op::binary_op(op, Op::Val('1'), |l, r| (l + r) % modulo)
            }
            Some(op) => Some(op),
            None => Some(Op::Result('0')),
        }
    }

    pub fn op_delay(&self, loc: &Coord) -> Option<Op> {
        let rate = self
            .left_of(loc, 1)
            .and_then(|op| match op {
                Op::Result(c) | Op::Val(c) => Op::as_num(c),
                _ => Some(1),
            })
            .unwrap_or(1);
        let modulo = self
            .right_of(loc, 1)
            .and_then(|op| match op {
                Op::Result(c) | Op::Val(c) => Op::as_num(c),
                _ => Some(1),
            })
            .unwrap_or(8);
        if self.frame % rate == 0 && self.frame % modulo == 0 {
            Some(Op::Bang)
        } else {
            Some(Op::EmptyResult)
        }
    }

    // ░█░█░▀█▀░▀█▀░█░░
    // ░█░█░░█░░░█░░█░░
    // ░▀▀▀░░▀░░▀▀▀░▀▀▀

    fn left_of(&self, loc: &Coord, offset: usize) -> Option<Op> {
        let left_x = loc.x.checked_sub(offset);
        if left_x.is_none() {
            return None;
        }
        let left_loc = Coord {
            x: left_x.unwrap(),
            y: loc.y,
        };
        self.get_cell(&left_loc)
    }

    fn right_of(&self, loc: &Coord, offset: usize) -> Option<Op> {
        let right_x = loc.x.checked_add(offset).and_then(|result| {
            if result >= self.cols {
                None
            } else {
                Some(result)
            }
        });
        if right_x.is_none() {
            return None;
        }
        let right_loc = Coord {
            x: right_x.unwrap(),
            y: loc.y,
        };
        self.get_cell(&right_loc)
    }

    fn below_of(&self, loc: &Coord, offset: usize) -> Option<Op> {
        let below_y = loc.y.checked_add(offset).and_then(|result| {
            if result >= self.rows {
                None
            } else {
                Some(result)
            }
        });
        if below_y.is_none() {
            return None;
        }
        let below_loc = Coord {
            x: loc.x,
            y: below_y.unwrap(),
        };
        self.get_cell(&below_loc)
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
                    out.push(op.as_char(&chars));
                } else {
                    out.push(chars.empty);
                }
            }
            out.push('\n');
        }
        out
    }
}

// ░▀█▀░▀█▀░█▀▀░█░█
// ░░█░░░█░░█░░░█▀▄
// ░░▀░░▀▀▀░▀▀▀░▀░▀ tests

#[cfg(test)]
mod tick_tests {
    use crate::{op::Op, util::Coord};

    use super::Snorkel;

    // Addition

    #[test]
    fn complete_add_should_do_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 0, y: 0 }, Op::Val('1'));
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::Add);
        snrkl.set_cell(&Coord { x: 2, y: 0 }, Op::Val('1'));
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Result('2')), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn incomplete_add_should_do_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::Add);
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Result('0')), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn add_with_rhs_only_should_do_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::Add);
        snrkl.set_cell(&Coord { x: 2, y: 0 }, Op::Val('1'));
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Result('1')), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn add_with_lhs_only_should_do_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 0, y: 0 }, Op::Val('1'));
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::Add);
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Result('1')), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    // Subtraction

    #[test]
    fn complete_sub_should_do_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 0, y: 0 }, Op::Val('2'));
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::Sub);
        snrkl.set_cell(&Coord { x: 2, y: 0 }, Op::Val('1'));
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Result('1')), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn incomplete_sub_should_do_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::Sub);
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Result('0')), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn sub_with_lhs_only_should_do_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 0, y: 0 }, Op::Val('2'));
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::Sub);
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Result('2')), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn sub_with_rhs_only_should_do_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::Sub);
        snrkl.set_cell(&Coord { x: 2, y: 0 }, Op::Val('2'));
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Result('2')), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    // If...
    #[test]
    fn complete_equal_if_expression_should_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 0, y: 0 }, Op::Val('1'));
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::If);
        snrkl.set_cell(&Coord { x: 2, y: 0 }, Op::Val('1'));
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Bang), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn complete_inequal_if_expression_should_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 0, y: 0 }, Op::Val('1'));
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::If);
        snrkl.set_cell(&Coord { x: 2, y: 0 }, Op::Val('2'));
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::EmptyResult), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn incomplete_equal_if_expression_should_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::If);
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(Some(Op::Bang), snrkl.get_cell(&Coord { x: 1, y: 1 }));
    }
}

// ░█▀▀░█▀█░█▀█░█░█░░░█░█▀█░█▀█░█▀▀░▀█▀░█▀▀
// ░█░░░█░█░█▀▀░░█░░▄▀░░█▀▀░█▀█░▀▀█░░█░░█▀▀
// ░▀▀▀░▀▀▀░▀░░░░▀░░▀░░░▀░░░▀░▀░▀▀▀░░▀░░▀▀▀ tests

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
