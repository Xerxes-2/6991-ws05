#[derive(Debug, Clone)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl From<CardinalDirection> for Direction {
    fn from(dir: CardinalDirection) -> Direction {
        match dir {
            CardinalDirection::North => Direction { x: 0, y: -1 },
            CardinalDirection::East => Direction { x: 1, y: 0 },
            CardinalDirection::South => Direction { x: 0, y: 1 },
            CardinalDirection::West => Direction { x: -1, y: 0 },
        }
    }
}
