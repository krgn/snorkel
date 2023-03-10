#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
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
    Val(char),
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

impl Op {
    pub fn from(value: char) -> Option<Op> {
        match value {
            'A' => Some(Op::Add),
            'B' => Some(Op::Sub),
            'C' => Some(Op::Clock),
            'E' => Some(Op::East),
            'F' => Some(Op::If),
            'G' => Some(Op::Gen),
            'H' => Some(Op::Hold),
            'I' => Some(Op::Inc),
            'J' => Some(Op::Jmp),
            'K' => Some(Op::Konkat),
            'L' => Some(Op::Less),
            'M' => Some(Op::Mul),
            'N' => Some(Op::North),
            'O' => Some(Op::Read),
            'P' => Some(Op::Push),
            'Q' => Some(Op::Query),
            'R' => Some(Op::Rand),
            'S' => Some(Op::South),
            'T' => Some(Op::Track),
            'U' => Some(Op::Uclid),
            'V' => Some(Op::Var),
            'W' => Some(Op::West),
            'X' => Some(Op::Write),
            'Y' => Some(Op::Ymp),
            'Z' => Some(Op::Lerp),

            '*' => Some(Op::Bang),
            '#' => Some(Op::Comment),

            // TODO: I'm lazy, but this should work for now
            c if c.is_alphanumeric() => Some(Op::Val(c)),

            _ => None,
        }
    }

    pub fn is_primop(&self) -> bool {
        match self {
            Op::Bang | Op::Comment | Op::Val(_) => false,
            _ => true,
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            Op::Comment => true,
            _ => false,
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Op::Val(_) => true,
            _ => false,
        }
    }
}

impl From<&Op> for String {
    fn from(value: &Op) -> Self {
        let c: char = value.into();
        c.to_string()
    }
}

impl From<&Op> for char {
    fn from(value: &Op) -> Self {
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
            Op::Val(c) => *c,
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
