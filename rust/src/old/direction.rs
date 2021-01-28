use rand::distributions::Standard;
use rand::prelude::*;

/// Direction encodes a North, South, East, West, direction as a `u8` value.
/// These directions are stored in the cells of a maze and used to indicate which walls were
/// "carved" out (removed).

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Direction {
    // North
    N = 1,
    // South
    S = 2,
    // East
    E = 4,
    // West
    W = 8,
}

impl Direction {
    pub fn dx(&self) -> i8 {
        match self {
            Direction::E => 1,
            Direction::W => -1,
            _ => 0,
        }
    }

    pub fn dy(&self) -> i8 {
        match self {
            Direction::N => -1,
            Direction::S => 1,
            _ => 0,
        }
    }

    /// returns the opposite direction to this direction
    pub fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}

impl Distribution<Direction> for Standard {
    /// returns a random direction using the supplied `rng` random number generator
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::N,
            1 => Direction::S,
            2 => Direction::E,
            _ => Direction::W,
        }
    }
}
