macro_rules! abort_if {
    ($x: expr) => {
        if $x {
            return false;
        }
    };
}

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

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum Rail {
    #[default]
    NW,
    SW,
    NE,
    SE,
    WE,
    NS,
}

impl std::fmt::Display for Rail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Rail::*;
        let rail = match self {
            NW => "┘",
            SW => "┐",
            NE => "└",
            SE => "┌",
            WE => "─",
            NS => "│",
        };
        write!(f, "{rail}")
    }
}

impl Rail {
    pub const NUM: usize = 6;
    pub const ALL: [Rail; Rail::NUM] = [Rail::NW, Rail::SW, Rail::NE, Rail::SE, Rail::WE, Rail::NS];

    pub const fn cycle_next(&self) -> Self {
        use Rail::*;
        match self {
            NW => SW,
            SW => NE,
            NE => SE,
            SE => WE,
            WE => NS,
            NS => NW,
        }
    }

    pub fn cycle_finished(&self) -> bool {
        *self == Rail::NS
    }

    pub const fn points_up(&self) -> bool {
        use Rail::*;
        match self {
            NW => true,
            NE => true,
            NS => true,
            _ => false,
        }
    }

    pub const fn points_down(&self) -> bool {
        use Rail::*;
        match self {
            SW => true,
            SE => true,
            NS => true,
            _ => false,
        }
    }

    pub const fn points_left(&self) -> bool {
        use Rail::*;
        match self {
            NW => true,
            SW => true,
            WE => true,
            _ => false,
        }
    }

    pub const fn points_right(&self) -> bool {
        use Rail::*;
        match self {
            NE => true,
            SE => true,
            WE => true,
            _ => false,
        }
    }

    pub const fn directions(&self) -> (Direction, Direction) {
        use Direction::*;
        use Rail::*;
        match self {
            NW => (North, West),
            SW => (South, West),
            NE => (North, East),
            SE => (South, East),
            WE => (West, East),
            NS => (North, South),
        }
    }

    pub fn points_other(&self, direction: Direction) -> Direction {
        let directions = self.directions();
        if directions.0 == direction {
            return directions.1;
        } else if directions.1 == direction {
            return directions.0;
        } else {
            panic!("This rail shares no directions with `direction`");
        }
    }

    pub fn points_to(&self, direction: Direction) -> bool {
        match direction {
            Direction::North => self.points_down(),
            Direction::South => self.points_up(),
            Direction::West => self.points_right(),
            Direction::East => self.points_left(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board<const X: usize, const Y: usize> {
    inner: [[Option<Rail>; X]; Y],
    col_hints: [u8; X],
    row_hints: [u8; Y],
}

impl<const X: usize, const Y: usize> std::fmt::Display for Board<{ X }, { Y }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        for row in self.inner {
            let mut current = String::new();
            for x in row {
                if let Some(x) = x {
                    current.push(format!("{x}").chars().next().unwrap());
                } else {
                    current.push(' ');
                }
            }
            current.push('\n');
            res.push_str(current.as_str());
        }

        write!(f, "{res}")
    }
}

impl<const X: usize, const Y: usize> Board<X, Y> {
    pub fn new(grid: [[Option<Rail>; X]; Y], col_hints: [u8; X], row_hints: [u8; Y]) -> Self {
        Self {
            inner: grid,
            col_hints,
            row_hints,
        }
    }

    pub fn x(&self) -> usize {
        X
    }

    pub fn y(&self) -> usize {
        Y
    }

    pub const fn col(&self, idx: usize) -> Option<[Option<Rail>; Y]> {
        if idx >= X {
            return None;
        }
        let mut col = [None; Y];
        let mut i = 0;
        while i < Y {
            col[i] = self.inner[i][idx];
            i += 1;
        }
        Some(col)
    }

    pub fn count_row(&self, idx: usize) -> u8 {
        assert!(idx < Y);
        self.inner[idx].iter().filter(|x| x.is_some()).count() as u8
    }

    pub fn count_col(&self, idx: usize) -> u8 {
        assert!(idx < X);
        self.col(idx)
            .unwrap()
            .iter()
            .filter(|x| x.is_some())
            .count() as u8
    }

    pub fn adheres_hints(&self) -> bool {
        for r in 0..Y {
            if self.count_row(r) > self.row_hints[r] {
                return false;
            }
        }
        for c in 0..X {
            if self.count_col(c) > self.col_hints[c] {
                return false;
            }
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        for r in 0..Y {
            if self.count_row(r) != self.row_hints[r] {
                return false;
            }
        }
        for c in 0..X {
            if self.count_col(c) != self.col_hints[c] {
                return false;
            }
        }
        true
    }

    pub fn can_place(&self, rail: Rail, x: usize, y: usize) -> bool {
        abort_if!(x >= X);
        abort_if!(y >= Y);
        abort_if!(self.inner[y][x].is_some_and(|x| x != rail));

        let (mut up, mut down, mut left, mut right) = (None, None, None, None);

        if x > 0 {
            left = self.inner[y][x - 1];
        }
        if x < X - 1 {
            right = self.inner[y][x + 1];
        }
        if y > 0 {
            up = self.inner[y - 1][x];
        }
        if y < Y - 1 {
            down = self.inner[y + 1][x];
        }

        abort_if!(up.is_some_and(|x| x.points_down() && !rail.points_up()));
        abort_if!(down.is_some_and(|x| x.points_up() && !rail.points_down()));
        abort_if!(left.is_some_and(|x| x.points_right() && !rail.points_left()));
        abort_if!(right.is_some_and(|x| x.points_left() && !rail.points_right()));

        true
    }

    pub fn find_start(&self) -> (Point, Direction) {
        for c in 0..X {
            if self.inner[0][c].is_some_and(|x| x.points_up()) {
                return (
                    (c, 0),
                    self.inner[0][c].unwrap().points_other(Direction::North),
                );
            }
            if self.inner[Y - 1][c].is_some_and(|x| x.points_down()) {
                return (
                    (c, Y - 1),
                    self.inner[c][Y - 1].unwrap().points_other(Direction::South),
                );
            }
        }
        for r in 0..Y {
            if self.inner[r][0].is_some_and(|x| x.points_left()) {
                return (
                    (0, r),
                    self.inner[r][0].unwrap().points_other(Direction::West),
                );
            }
            if self.inner[r][X - 1].is_some_and(|x| x.points_right()) {
                return (
                    (X - 1, r),
                    self.inner[r][X - 1].unwrap().points_other(Direction::East),
                );
            }
        }

        panic!("Invalid Board")
    }

    pub fn is_solved(&self) -> bool {
        let expected: u8 = self.row_hints.iter().sum();
        let mut count = 1;
        let (mut current, mut direction) = self.find_start();

        while count < expected {
            let offset = direction.offset();
            // we are trying to go out of bounds without having found all the rails
            abort_if!(current.0 == 0 && offset.0 < 0 || current.0 == X - 1 && offset.0 > 0);
            abort_if!(current.1 == 0 && offset.1 < 0 || current.1 == Y - 1 && offset.1 > 0);

            current = (
                (current.0 as isize + offset.0) as usize,
                (current.1 as isize + offset.1) as usize,
            );
            direction = self.inner[current.1][current.0]
                .unwrap()
                .points_other(!direction);

            count += 1;
        }
        true
    }

    #[rustfmt::skip]
    pub fn points_outward(&self, point: Point, rail: Rail) -> bool {
           (point.0 == 0     && rail.points_left())
        || (point.1 == 0     && rail.points_up())
        || (point.0 == X - 1 && rail.points_right())
        || (point.1 == Y - 1 && rail.points_down())
    }
}

type Point = (usize, usize);

pub struct NoSolution;
pub fn solve<const X: usize, const Y: usize>(
    board: &Board<{ X }, { Y }>,
) -> Result<(), NoSolution> {
    let original = board;
    let mut board = original.clone();
    let (cursor, direction) = board.find_start();
    //let mut path: Vec<Point> = vec![current];
    let expected: u8 = board.row_hints.iter().sum();
    //let count = 1;

    // dbg!(cursor, direction);

    fn solve<const X: usize, const Y: usize>(
        dummy: usize,
        mut board: &mut Board<{ X }, { Y }>,
        rail_count: u8,
        expected: u8,
        cursor: Point,
        direction: Direction,
    ) -> bool {
        if dummy > 55 {
            dbg!(dummy);
        }
        // dbg!(dummy);
        if !board.adheres_hints() {
            // dbg!(!board.adheres_hints());
            return false;
        }
        let offset = direction.offset();
        let out_of_bounds = cursor.0 == 0 && offset.0 < 0
            || cursor.1 == 0 && offset.1 < 0
            || cursor.0 == X - 1 && offset.0 > 0
            || cursor.1 == Y - 1 && offset.1 > 0;
        if out_of_bounds {
            // dbg!(rail_count, expected);
            //return rail_count == expected;
            if rail_count == expected {
                // We found a solution!
                // dbg!(board);
                println!("{board}");
                return true;
            } else {
                // dbg!("Out of bounds");
                // dbg!(direction, cursor);
                return false;
            }
        }
        // The usize cast won't panic as it's within bounds
        let x = (cursor.0 as isize + offset.0) as usize;
        let y = (cursor.1 as isize + offset.1) as usize;
        // dbg!(x, y);
        for rail in Rail::ALL {
            // dbg!(rail, dummy);
            if !board.can_place(rail, x, y) {
                // dbg!(!board.can_place(rail, x, y));
                continue;
            }
            // place rail, move cursor, change direction
            board.inner[y][x] = Some(rail);
            let new_cursor = (x, y);
            let new_direction = rail.points_other(!direction);
            // see if solved
            let solution = solve(
                dummy + 1,
                &mut board,
                rail_count + 1,
                expected,
                new_cursor,
                new_direction,
            );
            // undo changes
            board.inner[y][x] = None;
            if solution {
                return true;
            }
        }
        false
    }

    let solution_exists = solve(0, &mut board, 1, expected, cursor, direction);
    dbg!(solution_exists);

    if solution_exists {
        Ok(())
    } else {
        Err(NoSolution)
    }
}
