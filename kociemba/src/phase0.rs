use cube::Cube;
use transition_table::COCoord;
use transition_table::Coord;
use transition_table::EOCoord;
use transition_table::UD1Coord;

#[derive(Clone, Copy)]
pub struct Phase0Coord {
  eo: usize,
  co: usize,
  ud1: usize,
}

impl Phase0Coord {
  fn is_solved(&self) -> bool {
    self.eo == 0 && self.co == 0 && self.ud1 == 0
  }
}

impl From<Cube> for Phase0Coord {
  fn from(val: Cube) -> Phase0Coord {
    Phase0Coord {
      eo: EOCoord::get_coord(&val),
      co: COCoord::get_coord(&val),
      ud1: UD1Coord::get_coord(&val),
    }
  }
}

/// Phase 0: Reduce a cube from G0 to G1.
pub fn phase0(coord: Phase0Coord) -> bool {
  coord.is_solved()
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::{Face, Move};

  #[test]
  fn basic() {
    let c = Cube::solved();
    assert!(phase0(c.into()));

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::U, 1));
    assert!(phase0(c.into()));

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 1));
    assert!(!phase0(c.into()));
  }
}
