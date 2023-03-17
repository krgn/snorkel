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
    pub fn as_num(value: char) -> Option<usize> {
        match value {
            '0' => Some(0),
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            // lower case
            'a' => Some(10),
            'b' => Some(11),
            'c' => Some(12),
            'd' => Some(13),
            'e' => Some(14),
            'f' => Some(15),
            'g' => Some(16),
            'h' => Some(17),
            'i' => Some(18),
            'j' => Some(19),
            'k' => Some(20),
            'l' => Some(21),
            'm' => Some(22),
            'n' => Some(23),
            'o' => Some(24),
            'p' => Some(25),
            'q' => Some(26),
            'r' => Some(27),
            's' => Some(28),
            't' => Some(29),
            'u' => Some(30),
            'v' => Some(31),
            'w' => Some(32),
            'x' => Some(33),
            'y' => Some(34),
            'z' => Some(35),
            // capitalized
            'A' => Some(10),
            'B' => Some(11),
            'C' => Some(12),
            'D' => Some(13),
            'E' => Some(14),
            'F' => Some(15),
            'G' => Some(16),
            'H' => Some(17),
            'I' => Some(18),
            'J' => Some(19),
            'K' => Some(20),
            'L' => Some(21),
            'M' => Some(22),
            'N' => Some(23),
            'O' => Some(24),
            'P' => Some(25),
            'Q' => Some(26),
            'R' => Some(27),
            'S' => Some(28),
            'T' => Some(29),
            'U' => Some(30),
            'V' => Some(31),
            'W' => Some(32),
            'X' => Some(33),
            'Y' => Some(34),
            'Z' => Some(35),
            _ => None,
        }
    }

    pub fn to_char(value: usize, is_cap: bool) -> char {
        match value % 36 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 if is_cap => 'A',
            11 if is_cap => 'B',
            12 if is_cap => 'C',
            13 if is_cap => 'D',
            14 if is_cap => 'E',
            15 if is_cap => 'F',
            16 if is_cap => 'G',
            17 if is_cap => 'H',
            18 if is_cap => 'I',
            19 if is_cap => 'J',
            20 if is_cap => 'K',
            21 if is_cap => 'L',
            22 if is_cap => 'M',
            23 if is_cap => 'N',
            24 if is_cap => 'O',
            25 if is_cap => 'P',
            26 if is_cap => 'Q',
            27 if is_cap => 'R',
            28 if is_cap => 'S',
            29 if is_cap => 'T',
            30 if is_cap => 'U',
            31 if is_cap => 'V',
            32 if is_cap => 'W',
            33 if is_cap => 'X',
            34 if is_cap => 'Y',
            35 if is_cap => 'Y',
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            16 => 'g',
            17 => 'h',
            18 => 'i',
            19 => 'j',
            20 => 'k',
            21 => 'l',
            22 => 'm',
            23 => 'n',
            24 => 'o',
            25 => 'p',
            26 => 'q',
            27 => 'r',
            28 => 's',
            29 => 't',
            30 => 'u',
            31 => 'v',
            32 => 'w',
            33 => 'x',
            34 => 'y',
            35 => 'z',
            _ => unreachable!(),
        }
    }

    pub fn is_capital(value: char) -> bool {
        match value {
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N'
            | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' => true,
            _ => false,
        }
    }

    pub fn add(lhs: Op, rhs: Op) -> Option<Op> {
        use Op::*;

        // parse chars
        let (lhs_chr, rhs_chr) = if let (Val(lhs), Val(rhs)) = (lhs, rhs) {
            (lhs, rhs)
        } else {
            return None;
        };

        // parse num
        let (lhs_num, rhs_num) =
            if let (Some(lhs), Some(rhs)) = (Self::as_num(lhs_chr), Self::as_num(rhs_chr)) {
                (lhs, rhs)
            } else {
                return None;
            };

        // perform addition
        Some(Val(Self::to_char(
            lhs_num.wrapping_add(rhs_num),
            Self::is_capital(rhs_chr),
        )))
    }

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
