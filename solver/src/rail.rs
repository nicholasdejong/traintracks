use crate::Direction;

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
            //WE => "─",
            WE => "-",
            //NS => "│",
            NS => "|",
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
        matches!(self, Rail::NW | Rail::NE | Rail::NS)
    }

    pub const fn points_down(&self) -> bool {
        matches!(self, Rail::SW | Rail::SE | Rail::NS)
    }

    pub const fn points_left(&self) -> bool {
        matches!(self, Rail::NW | Rail::SW | Rail::WE)
    }

    pub const fn points_right(&self) -> bool {
        matches!(self, Rail::NE | Rail::SE | Rail::WE)
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
            directions.1
        } else if directions.1 == direction {
            directions.0
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
