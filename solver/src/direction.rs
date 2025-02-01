#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub const fn offset(self) -> (isize, isize) {
        use Direction::*;
        match self {
            North => (0, -1),
            South => (0, 1),
            West => (-1, 0),
            East => (1, 0),
        }
    }
}

impl std::ops::Not for Direction {
    type Output = Self;
    fn not(self) -> Self::Output {
        use Direction::*;
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}
