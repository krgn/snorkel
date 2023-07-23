use crate::{
    op::Op,
    state::UndoOp,
    util::{Coord, Selection},
};
use rand::Rng;
use std::{cmp, collections::HashMap};

pub struct Snorkel {
    pub rows: usize,
    pub cols: usize,
    pub frame: usize,
    vars: HashMap<char, Op>,
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
        let vars = HashMap::new();
        assert_eq!(data.len(), rows);
        Snorkel {
            rows,
            cols,
            data,
            frame,
            vars,
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
                    // ░█▀█░█▀▄░█▀▄
                    // ░█▀█░█░█░█░█
                    // ░▀░▀░▀▀░░▀▀░
                    Some(Op::Add) => {
                        if let Some(result) = self.op_add(&coord) {
                            coord.y += 1;
                            let _ignored = self.set_cell(&coord, result);
                        }
                    }
                    // ░█▀▀░█░█░█▀▄
                    // ░▀▀█░█░█░█▀▄
                    // ░▀▀▀░▀▀▀░▀▀░
                    Some(Op::Sub) => {
                        if let Some(result) = self.op_sub(&coord) {
                            coord.y += 1;
                            let _ignored = self.set_cell(&coord, result);
                        }
                    }
                    // ░█▄█░█░█░█░░
                    // ░█░█░█░█░█░░
                    // ░▀░▀░▀▀▀░▀▀▀
                    Some(Op::Mul) => {
                        let op = self.op_mul(&coord);
                        coord.y += 1;
                        let _ignored = self.set_cell(&coord, op);
                    }
                    // ░█▀▄░█▀█░█▀█░█▀▄
                    // ░█▀▄░█▀█░█░█░█░█
                    // ░▀░▀░▀░▀░▀░▀░▀▀░
                    Some(Op::Rand) => {
                        let op = self.op_rand(&coord);
                        coord.y += 1;
                        let _ignored = self.set_cell(&coord, op);
                    }
                    // ░▀█▀░█▀▀
                    // ░░█░░█▀▀
                    // ░▀▀▀░▀░░
                    Some(Op::If) => {
                        if let Some(result) = self.op_if(&coord) {
                            coord.y += 1;
                            let _ignored = self.set_cell(&coord, result);
                        }
                    }
                    // ░█▀▀░█░░░█▀█░█▀▀░█░█
                    // ░█░░░█░░░█░█░█░░░█▀▄
                    // ░▀▀▀░▀▀▀░▀▀▀░▀▀▀░▀░▀
                    Some(Op::Clock) => {
                        if let Some(result) = self.op_clock(&coord) {
                            coord.y += 1;
                            let _ignored = self.set_cell(&coord, result);
                        }
                    }
                    // ░█▀▄░█▀▀░█░░░█▀█░█░█
                    // ░█░█░█▀▀░█░░░█▀█░░█░
                    // ░▀▀░░▀▀▀░▀▀▀░▀░▀░░▀░
                    Some(Op::Delay) => {
                        let mut below = coord.clone();
                        below.y += 1;
                        let _ignored = match self.op_delay(&coord) {
                            Some(op) => self.set_cell(&below, op),
                            None => self.del_cell(&below),
                        };
                    }
                    // ░█▀▀░█▀█░█▀▀░▀█▀
                    // ░█▀▀░█▀█░▀▀█░░█░
                    // ░▀▀▀░▀░▀░▀▀▀░░▀░
                    Some(Op::East(frame)) => {
                        if frame != self.frame {
                            let op = self.op_east(&coord);
                            if op.is_bang() {
                                let _ignored = self.set_cell(&coord, op);
                            } else {
                                let _ignored = self.del_cell(&coord);
                                coord.x += 1;
                                let _ignored = self.set_cell(&coord, op);
                            };
                        }
                    }
                    // ░█░█░█▀▀░█▀▀░▀█▀
                    // ░█▄█░█▀▀░▀▀█░░█░
                    // ░▀░▀░▀▀▀░▀▀▀░░▀░
                    Some(Op::West(frame)) => {
                        if frame != self.frame {
                            let op = self.op_west(&coord);
                            if op.is_bang() {
                                let _ignored = self.set_cell(&coord, op);
                            } else {
                                let _ignored = self.del_cell(&coord);
                                coord.x -= 1;
                                let _ignored = self.set_cell(&coord, op);
                            };
                        }
                    }
                    // ░█▀█░█▀█░█▀▄░▀█▀░█░█
                    // ░█░█░█░█░█▀▄░░█░░█▀█
                    // ░▀░▀░▀▀▀░▀░▀░░▀░░▀░▀
                    Some(Op::North(frame)) => {
                        if frame != self.frame {
                            let op = self.op_north(&coord);
                            if op.is_bang() {
                                let _ignored = self.set_cell(&coord, op);
                            } else {
                                let _ignored = self.del_cell(&coord);
                                coord.y -= 1;
                                let _ignored = self.set_cell(&coord, op);
                            };
                        }
                    }
                    // ░█▀▀░█▀█░█░█░▀█▀░█░█
                    // ░▀▀█░█░█░█░█░░█░░█▀█
                    // ░▀▀▀░▀▀▀░▀▀▀░░▀░░▀░▀
                    Some(Op::South(frame)) => {
                        if frame != self.frame {
                            let op = self.op_south(&coord);
                            if op.is_bang() {
                                let _ignored = self.set_cell(&coord, op);
                            } else {
                                let _ignored = self.del_cell(&coord);
                                coord.y += 1;
                                let _ignored = self.set_cell(&coord, op);
                            };
                        }
                    }
                    // ░█▀▄░█▀█░█▀█░█▀▀
                    // ░█▀▄░█▀█░█░█░█░█
                    // ░▀▀░░▀░▀░▀░▀░▀▀▀
                    Some(Op::Bang(frame)) => {
                        if frame != self.frame {
                            let _ignored = self.del_cell(&coord);
                        }
                    }
                    // ░█▀▀░█▀▀░█▀█
                    // ░█░█░█▀▀░█░█
                    // ░▀▀▀░▀▀▀░▀░▀
                    Some(Op::Gen) => {
                        let (mut offset, ops) = self.op_gen(&coord);
                        for op in ops.into_iter() {
                            let _ignored = self.set_cell(&offset, op);
                            offset.x += 1;
                        }
                    }
                    // ░█▀▀░█▄█░█▀█░▀█▀░█░█
                    // ░█▀▀░█░█░█▀▀░░█░░░█░
                    // ░▀▀▀░▀░▀░▀░░░░▀░░░▀░
                    Some(Op::EmptyResult(ref loc)) => {
                        // if this is an orphaned empty result, delete it
                        if let None = self.get_cell(loc) {
                            let _ignored = self.del_cell(&coord);
                        }
                    }
                    // ░▀█▀░█▀█░█▀▀
                    // ░░█░░█░█░█░░
                    // ░▀▀▀░▀░▀░▀▀▀
                    Some(Op::Inc) => {
                        let op = self.op_inc(&coord);
                        coord.y += 1;
                        let _ignored = self.set_cell(&coord, op);
                    }
                    // ░█░░░█▀▀░█▀▀░█▀▀
                    // ░█░░░█▀▀░▀▀█░▀▀█
                    // ░▀▀▀░▀▀▀░▀▀▀░▀▀▀
                    Some(Op::Less) => {
                        let op = self.op_less(&coord);
                        coord.y += 1;
                        let _ignored = self.set_cell(&coord, op);
                    }
                    // ░▀▀█░█░█░█▄█░█▀█░█▀▀░█▀▄
                    // ░░░█░█░█░█░█░█▀▀░█▀▀░█▀▄
                    // ░▀▀░░▀▀▀░▀░▀░▀░░░▀▀▀░▀░▀
                    Some(Op::Jmp) => match self.above_of(&coord, 1) {
                        Some(op) => {
                            coord.y += 1;
                            let _ignored = self.set_cell(&coord, op);
                        }
                        None => {
                            coord.y += 1;
                            let _ignored = self.del_cell(&coord);
                        }
                    },
                    // ░█░█░█░█░█▄█░█▀█░█▀▀░█▀▄
                    // ░░█░░█░█░█░█░█▀▀░█▀▀░█▀▄
                    // ░░▀░░▀▀▀░▀░▀░▀░░░▀▀▀░▀░▀
                    Some(Op::Ymp) => match self.left_of(&coord, 1) {
                        Some(op) => {
                            coord.x += 1;
                            let _ignored = self.set_cell(&coord, op);
                        }
                        None => {
                            coord.x += 1;
                            let _ignored = self.del_cell(&coord);
                        }
                    },
                    // ░█░█░█▀█░█░░░█▀▄
                    // ░█▀█░█░█░█░░░█░█
                    // ░▀░▀░▀▀▀░▀▀▀░▀▀░
                    Some(Op::Hold) => {
                        let below = self.below_of(&coord, 1);
                        let next = match below {
                            Some(Op::East(_)) => Op::East(self.frame),
                            Some(Op::West(_)) => Op::West(self.frame),
                            Some(Op::North(_)) => Op::North(self.frame),
                            Some(Op::South(_)) => Op::South(self.frame),
                            Some(op) => op,
                            None => Op::EmptyResult(coord.clone()),
                        };
                        coord.y += 1;
                        let _ignored = self.set_cell(&coord, next);
                    }
                    // ░█▀▄░█▀▀░█▀█░█▀▄
                    // ░█▀▄░█▀▀░█▀█░█░█
                    // ░▀░▀░▀▀▀░▀░▀░▀▀░
                    Some(Op::Read) => {
                        let op = self.op_read(&coord);
                        coord.y += 1;
                        let _ignored = self.set_cell(&coord, op);
                    }
                    // ░█░█░█▀▄░▀█▀░▀█▀░█▀▀
                    // ░█▄█░█▀▄░░█░░░█░░█▀▀
                    // ░▀░▀░▀░▀░▀▀▀░░▀░░▀▀▀
                    Some(Op::Write) => self.op_write(&coord),
                    // ░█░█░█▀█░█▀▄
                    // ░▀▄▀░█▀█░█▀▄
                    // ░░▀░░▀░▀░▀░▀
                    Some(Op::Var) => self.op_var(&coord),
                    // ░█░█░█▀█░█▀█░█░█░█▀█░▀█▀
                    // ░█▀▄░█░█░█░█░█▀▄░█▀█░░█░
                    // ░▀░▀░▀▀▀░▀░▀░▀░▀░▀░▀░░▀░
                    Some(Op::Konkat) => self.op_konkat(&coord),
                    // ░█▀█░█░█░█▀▀░█░█
                    // ░█▀▀░█░█░▀▀█░█▀█
                    // ░▀░░░▀▀▀░▀▀▀░▀░▀
                    Some(Op::Push) => self.op_push(&coord),
                    // ░▄▀▄░█░█░█▀▀░█▀▄░█░█
                    // ░█\█░█░█░█▀▀░█▀▄░░█░
                    // ░░▀\░▀▀▀░▀▀▀░▀░▀░░▀░
                    Some(Op::Query) => self.op_query(&coord),
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
                match &row[x] {
                    Some(op) => {
                        let old = self.set_cell(&target, op.clone());
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
        if loc.y >= self.rows || loc.x >= self.cols {
            return None;
        }
        self.data[loc.y][loc.x].clone()
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
            (Some(Op::EmptyResult(_)), None) => Some(Op::Result('0')),
            (None, Some(Op::Val(c))) => Some(Op::Result(c)),
            (None, Some(Op::Result(c))) => Some(Op::Result(c)),
            (None, Some(Op::EmptyResult(_))) => Some(Op::Result('0')),
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
            (Some(Op::EmptyResult(_)), None) => Some(Op::Result('0')),
            (None, Some(Op::Val(c))) => Some(Op::Result(c)),
            (None, Some(Op::Result(c))) => Some(Op::Result(c)),
            (None, Some(Op::EmptyResult(_))) => Some(Op::Result('0')),
            _ => Some(Op::Result('0')),
        }
    }

    fn op_mul(&self, loc: &Coord) -> Op {
        let left = self.left_of(loc, 1).and_then(|op| op.extract_num());
        let right = self.right_of(loc, 1).and_then(|op| op.extract_num());
        match (left, right) {
            (Some(l), Some(r)) => Op::Result(Op::as_value_char(l * r, false)),
            _ => Op::EmptyResult(loc.clone()),
        }
    }

    fn op_rand(&self, loc: &Coord) -> Op {
        let left = self
            .left_of(loc, 1)
            .and_then(|op| op.extract_num())
            .unwrap_or(0);
        let right = self
            .right_of(loc, 1)
            .and_then(|op| op.extract_num())
            .unwrap_or(35);
        let min = cmp::min(left, right);
        let max = cmp::max(left, right);
        let val = if min == max {
            min
        } else {
            rand::thread_rng().gen_range(min..max)
        };
        Op::Result(Op::as_value_char(val, false))
    }

    fn op_if(&self, loc: &Coord) -> Option<Op> {
        let left = self.left_of(loc, 1);
        let right = self.right_of(loc, 1);
        match (left, right) {
            (Some(lhs), Some(rhs)) if lhs == rhs => Some(Op::Bang(self.frame)),
            (Some(_), Some(_)) => Some(Op::EmptyResult(loc.clone())),
            (Some(_), None) => Some(Op::EmptyResult(loc.clone())),
            (None, Some(_)) => Some(Op::EmptyResult(loc.clone())),
            (None, None) => Some(Op::Bang(self.frame)),
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
            Some(Op::Bang(self.frame))
        } else {
            Some(Op::EmptyResult(loc.clone()))
        }
    }

    pub fn op_east(&self, loc: &Coord) -> Op {
        if loc.x + 1 >= self.cols {
            return Op::Bang(self.frame);
        }
        match self.right_of(loc, 1) {
            Some(Op::EmptyResult(_)) => Op::East(self.frame),
            Some(_) => Op::Bang(self.frame),
            None => Op::East(self.frame),
        }
    }

    pub fn op_west(&self, loc: &Coord) -> Op {
        if loc.x.checked_sub(1).is_none() {
            return Op::Bang(self.frame);
        }
        match self.left_of(loc, 1) {
            Some(Op::EmptyResult(_)) => Op::West(self.frame),
            Some(_) => Op::Bang(self.frame),
            None => Op::West(self.frame),
        }
    }

    pub fn op_north(&self, loc: &Coord) -> Op {
        if loc.y.checked_sub(1).is_none() {
            return Op::Bang(self.frame);
        }
        match self.above_of(loc, 1) {
            Some(Op::EmptyResult(_)) => Op::North(self.frame),
            Some(_) => Op::Bang(self.frame),
            None => Op::North(self.frame),
        }
    }

    pub fn op_south(&self, loc: &Coord) -> Op {
        if loc.y + 1 >= self.rows {
            return Op::Bang(self.frame);
        }
        match self.below_of(loc, 1) {
            Some(Op::EmptyResult(_)) => Op::South(self.frame),
            Some(_) => Op::Bang(self.frame),
            None => Op::South(self.frame),
        }
    }

    pub fn op_inc(&self, loc: &Coord) -> Op {
        let step = self
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
            .map(|value| if value == 0 { 1 } else { value })
            .unwrap_or(36); // modulo 36, since we wrap there anyways
        let current = self
            .below_of(loc, 1)
            .and_then(|op| match op {
                Op::Result(c) | Op::Val(c) => Op::as_num(c),
                _ => Some(0),
            })
            .unwrap_or(0);
        Op::Result(Op::as_value_char((current + step) % modulo, false))
    }

    pub fn op_less(&self, loc: &Coord) -> Op {
        let left = self.left_of(loc, 1).and_then(|op| op.extract_num());
        let right = self.right_of(loc, 1).and_then(|op| op.extract_num());
        match (left, right) {
            (Some(l), Some(r)) if l > r => Op::Result(Op::as_value_char(r, false)),
            (Some(l), Some(_)) => Op::Result(Op::as_value_char(l, false)),
            _ => Op::EmptyResult(loc.clone()),
        }
    }

    pub fn op_gen(&self, loc: &Coord) -> (Coord, Vec<Op>) {
        let mut start = loc.clone();
        // enforce that this should by default be a line under current
        start.y += 1;
        start.x += self
            .left_of(loc, 3)
            .and_then(|op| match op {
                Op::Result(c) | Op::Val(c) => Op::as_num(c),
                _ => Some(0),
            })
            .unwrap_or(0);
        start.y += self
            .left_of(loc, 2)
            .and_then(|op| match op {
                Op::Result(c) | Op::Val(c) => Op::as_num(c),
                _ => Some(1),
            })
            .unwrap_or(1);
        let len = self
            .left_of(loc, 1)
            .and_then(|op| match op {
                Op::Result(c) | Op::Val(c) => Op::as_num(c),
                _ => Some(0),
            })
            .unwrap_or(0);
        let mut cursor = loc.clone();
        cursor.x += 1;
        let mut ops = vec![];
        for _ in 0..len + 1 {
            let val = self
                .get_cell(&cursor)
                .map(|op| match op {
                    Op::Val(c) => Op::Result(c),
                    other => other,
                })
                .unwrap_or(Op::EmptyResult(loc.clone()));
            ops.push(val);
            cursor.x += 1;
        }
        return (start, ops);
    }

    pub fn op_read(&self, loc: &Coord) -> Op {
        let x = self
            .left_of(loc, 2)
            .and_then(|op| op.extract_num())
            .unwrap_or(1);
        let y = self
            .left_of(loc, 1)
            .and_then(|op| op.extract_num())
            .unwrap_or(0);
        let mut source = loc.clone();
        source.x += cmp::max(x, 1);
        source.y += y;
        self.get_cell(&source)
            .map(|op| match op {
                Op::Val(ref c) => Op::Result(*c),
                op => op,
            })
            .unwrap_or(Op::EmptyResult(loc.clone()))
    }

    pub fn op_write(&mut self, loc: &Coord) {
        let mut target = loc.clone();
        target.y += 1;
        let op = self
            .right_of(&loc, 1)
            .map(|op| match op {
                Op::Val(ref c) => Op::Result(*c),
                op => op,
            })
            .unwrap_or(Op::EmptyResult(loc.clone()));
        let x = self
            .left_of(loc, 2)
            .and_then(|op| op.extract_num())
            .unwrap_or(0);
        let y = self
            .left_of(loc, 1)
            .and_then(|op| op.extract_num())
            .unwrap_or(0);
        target.x += x;
        target.y += y;
        let _ingored = self.set_cell(&target, op);
    }

    pub fn op_push(&mut self, loc: &Coord) {
        let len = self
            .left_of(loc, 1)
            .and_then(|op| op.extract_num())
            .and_then(|n| if n > 0 { Some(n) } else { None });
        if let Some(len) = len {
            let x = self
                .left_of(loc, 2)
                .and_then(|op| op.extract_num())
                .unwrap_or(0);
            let x = x % len;
            let val = self
                .right_of(loc, 1)
                .map(|op| match op {
                    Op::Val(ref c) => Op::Result(*c),
                    op => op,
                })
                .unwrap_or(Op::EmptyResult(loc.clone()));
            let mut target = loc.clone();
            target.y += 1;
            target.x += x;
            let _ignored = self.set_cell(&target, val);
        }
    }

    pub fn op_konkat(&mut self, loc: &Coord) {
        let len = self
            .left_of(loc, 1)
            .and_then(|op| op.extract_num())
            .map(|n| cmp::max(n, 1))
            .unwrap_or(1);
        let mut target = loc.clone();
        target.y += 1;
        for offset in 1..len + 1 {
            let op = self
                .right_of(loc, offset)
                .and_then(|op| match op {
                    Op::Val(ref c) | Op::Result(ref c) => self.vars.get(c),
                    _ => None,
                })
                .map(|op| match op {
                    Op::Val(ref c) => Op::Result(*c),
                    op => op.clone(),
                })
                .unwrap_or_else(|| Op::EmptyResult(loc.clone()));
            target.x += 1;
            let _ignored = self.set_cell(&target, op);
        }
    }

    pub fn op_var(&mut self, loc: &Coord) {
        let left = self.left_of(&loc, 1).and_then(|op| match op {
            Op::Val(ref c) => Some(*c),
            Op::Result(ref c) => Some(*c),
            _ => None,
        });
        let right = self.right_of(&loc, 1);
        let op = match (left, right) {
            // write mode
            (Some(key), Some(op)) => {
                let _ = self.vars.insert(key, op);
                None
            }
            (Some(key), None) => {
                let _ = self.vars.remove(&key);
                None
            }
            (None, Some(Op::Val(ref key))) | (None, Some(Op::Result(ref key))) => {
                self.vars.get(key).map(|op| op.clone())
            }
            (None, _) => Some(Op::EmptyResult(loc.clone())),
        };
        let mut below = loc.clone();
        below.y += 1;
        match op {
            Some(op) => {
                let _ignore = self.set_cell(&below, op);
            }
            None => {
                let _ignore = self.del_cell(&below);
            }
        }
    }

    pub fn op_query(&mut self, loc: &Coord) {
        let count = match self.left_of(&loc, 1).and_then(|op| op.extract_num()) {
            Some(c) => c,
            None => return,
        };
        let y_offset = match self.left_of(&loc, 2).and_then(|op| op.extract_num()) {
            Some(c) => c,
            None => return,
        };
        let x_offset = match self.left_of(&loc, 3).and_then(|op| op.extract_num()) {
            Some(c) => c,
            None => return,
        };

        for i in (0..count).rev() {
            if let Some(op) = self.get_cell(&Coord {
                x: loc.x + 1 + x_offset + i,
                y: loc.y + y_offset,
            }) {
                let dest = Coord {
                    x: loc.x.checked_sub(count).unwrap_or(0) + (i + 1),
                    y: loc.y + 1,
                };
                self.set_cell(&dest, op);
            };
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

    fn above_of(&self, loc: &Coord, offset: usize) -> Option<Op> {
        let above_y = loc.y.checked_sub(offset).and_then(|result| {
            if result >= self.rows {
                None
            } else {
                Some(result)
            }
        });
        if above_y.is_none() {
            return None;
        }
        let above_loc = Coord {
            x: loc.x,
            y: above_y.unwrap(),
        };
        self.get_cell(&above_loc)
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
        assert_eq!(
            Some(Op::Bang(snrkl.frame)),
            snrkl.get_cell(&Coord { x: 1, y: 1 })
        );
    }

    #[test]
    fn complete_inequal_if_expression_should_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 0, y: 0 }, Op::Val('1'));
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::If);
        snrkl.set_cell(&Coord { x: 2, y: 0 }, Op::Val('2'));
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(
            Some(Op::EmptyResult(Coord { x: 1, y: 0 })),
            snrkl.get_cell(&Coord { x: 1, y: 1 })
        );
    }

    #[test]
    fn incomplete_equal_if_expression_should_produce_correct_result() {
        let mut snrkl = Snorkel::new(5, 5);
        snrkl.set_cell(&Coord { x: 1, y: 0 }, Op::If);
        assert_eq!(None, snrkl.get_cell(&Coord { x: 1, y: 1 }));
        snrkl.tick();
        assert_eq!(
            Some(Op::Bang(snrkl.frame)),
            snrkl.get_cell(&Coord { x: 1, y: 1 })
        );
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

    #[test]
    fn op_query() {
        let mut snrkl = Snorkel::new(8, 8);
        snrkl.set_cell(&Coord { x: 5, y: 6 }, Op::Val('1'));
        snrkl.set_cell(&Coord { x: 6, y: 6 }, Op::Val('2'));
        snrkl.set_cell(&Coord { x: 7, y: 6 }, Op::Val('3'));
        let rendered = snrkl.render();
        let expected = r#"
········
········
········
········
········
········
·····123
········
"#;
        assert_eq!(expected.trim_start(), rendered);

        snrkl.set_cell(&Coord { x: 3, y: 1 }, Op::Query);
        snrkl.set_cell(&Coord { x: 2, y: 1 }, Op::Val('3'));
        snrkl.set_cell(&Coord { x: 1, y: 1 }, Op::Val('5'));
        snrkl.set_cell(&Coord { x: 0, y: 1 }, Op::Val('1'));

        let rendered = snrkl.render();
        let expected = r#"
········
153Q····
········
········
········
········
·····123
········
"#;
        assert_eq!(expected.trim_start(), rendered);
        snrkl.tick();

        let rendered = snrkl.render();
        let expected = r#"
········
153Q····
·123····
········
········
········
·····123
········
"#;
        assert_eq!(expected.trim_start(), rendered);
    }
}
