#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OrdinalDirection {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}
