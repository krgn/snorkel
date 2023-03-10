use crate::op::Op;

pub struct Snrkl {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Vec<Option<Op>>>,
}

impl Snrkl {
    pub fn new(rows: usize, cols: usize) -> Snrkl {
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(vec![None; cols]);
        }
        assert_eq!(data.len(), rows);
        Snrkl { rows, cols, data }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Op> {
        self.data[y][x]
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

    #[cfg(test)]
    pub fn render(&self) -> String {
        use crate::chars;
        let mut out = String::with_capacity(self.rows * self.cols + self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(op) = &self.data[row][col] {
                    out.push(op.into());
                } else {
                    out.push(chars::EMPTY_CELL);
                }
            }
            out.push('\n');
        }
        out
    }

    pub fn set_cell(&mut self, x: usize, y: usize, op: Op) {
        if y >= self.rows || x >= self.cols {
            return;
        }
        self.data[y][x] = Some(op)
    }

    pub fn del_cell(&mut self, x: usize, y: usize) {
        if y >= self.rows || x >= self.cols {
            return;
        }
        self.data[y][x] = None;
    }
}

#[cfg(test)]
mod tests {
    use crate::op::Op;

    use super::Snrkl;

    #[test]
    fn create_new_snrkl_renders_correctly() {
        let snrkl = Snrkl::new(4, 20);
        let rendered = snrkl.render();
        let expected = r#"
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
"#;
        assert_eq!(expected.trim_start(), rendered)
    }

    #[test]
    fn process_op_should_work_correctly() {
        let mut snrkl = Snrkl::new(4, 20);
        let rendered = snrkl.render();
        let expected = r#"
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
"#;
        assert_eq!(expected.trim_start(), rendered);

        snrkl.set_cell(1, 1, Op::Add);
        snrkl.set_cell(19, 3, Op::Clock);

        let rendered = snrkl.render();
        let expected = r#"
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱A⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱C
"#;
        assert_eq!(expected.trim_start(), rendered);
    }

    #[test]
    fn resize_should_work() {
        let mut snrkl = Snrkl::new(4, 4);
        snrkl.set_cell(1, 1, Op::Add);
        snrkl.set_cell(2, 2, Op::Clock);
        let rendered = snrkl.render();
        let expected = r#"
⸱⸱⸱⸱
⸱A⸱⸱
⸱⸱C⸱
⸱⸱⸱⸱
"#;
        assert_eq!(expected.trim_start(), rendered);

        snrkl.resize(10, 10);
        let rendered = snrkl.render();
        let expected = r#"
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱A⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱C⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
"#;

        assert_eq!(expected.trim_start(), rendered);

        snrkl.set_cell(9, 9, Op::Uclid);
        let rendered = snrkl.render();
        let expected = r#"
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱A⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱C⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱U
"#;

        assert_eq!(expected.trim_start(), rendered);

        snrkl.resize(4, 4);
        let rendered = snrkl.render();
        let expected = r#"
⸱⸱⸱⸱
⸱A⸱⸱
⸱⸱C⸱
⸱⸱⸱⸱
"#;

        assert_eq!(expected.trim_start(), rendered);

        snrkl.resize(10, 10);
        let rendered = snrkl.render();
        let expected = r#"
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱A⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱C⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱⸱
⸱⸱⸱⸱⸱⸱⸱⸱⸱U
"#;

        assert_eq!(expected.trim_start(), rendered);
    }
}
