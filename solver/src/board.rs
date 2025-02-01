use crate::direction::Direction;
use crate::helpers::abort_if;
use crate::rail::Rail;
use crate::Point;

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
                    current.push('·');
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

    pub fn rail_count(&self) -> u8 {
        self.row_hints.iter().sum()
    }

    pub fn grid_mut(&mut self) -> &mut [[Option<Rail>; X]; Y] {
        &mut self.inner
    }

    pub const fn row(&self, idx: usize) -> Option<[Option<Rail>; X]> {
        if idx >= Y {
            return None;
        }
        Some(self.inner[idx])
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

    pub fn adheres_hints_at(&self, point: Point) -> bool {
        self.count_row(point.1) <= self.row_hints[point.1]
            && self.count_col(point.0) <= self.col_hints[point.0]
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
        //abort_if!(self.inner[y][x].is_some_and(|x| x != rail));
        if let Some(x) = self.inner[y][x] {
            return x == rail;
        }

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
                    self.inner[Y - 1][c].unwrap().points_other(Direction::South),
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
