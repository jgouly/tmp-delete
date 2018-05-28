use cube::{Cube, Face};
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

pub struct Phase0Tables<'a> {
  eo_t: &'a [[usize; 6]],
  co_t: &'a [[usize; 6]],
  ud1_t: &'a [[usize; 6]],
}

impl<'a> Phase0Tables<'a> {
  fn transition(&self, coord: Phase0Coord, face: Face) -> Phase0Coord {
    let eo = self.eo_t[coord.eo][usize::from(face)];
    let co = self.co_t[coord.co][usize::from(face)];
    let ud1 = self.ud1_t[coord.ud1][usize::from(face)];
    Phase0Coord { eo, co, ud1 }
  }
}

/// Phase 0: Reduce a cube from G0 to G1.
pub fn phase0(
  coord: Phase0Coord,
  depth_remaining: usize,
  tables: &Phase0Tables,
) -> bool {
  if depth_remaining == 0 {
    return coord.is_solved();
  }

  for &f in &[Face::U, Face::D, Face::F, Face::B, Face::R, Face::L] {
    let mut next = coord;
    for _ in 0..3 {
      next = tables.transition(next, f);
      if phase0(next, depth_remaining - 1, tables) {
        return true;
      }
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::Move;
  use transition_table::*;

  #[test]
  fn basic() {
    let co_t = get_co_transition_table();
    let eo_t = get_eo_transition_table();
    let ud1_t = get_ud1_transition_table();

    let tables = Phase0Tables {
      co_t: &co_t,
      eo_t: &eo_t,
      ud1_t: &ud1_t,
    };

    let c = Cube::solved();
    assert!(phase0(c.into(), 0, &tables));

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::U, 1));
    assert!(phase0(c.into(), 0, &tables));

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 1));
    assert!(!phase0(c.into(), 0, &tables));
    assert!(phase0(c.into(), 1, &tables));

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 3));
    let c = c.apply_move(Move(Face::R, 3));
    assert!(!phase0(c.into(), 0, &tables));
    assert!(!phase0(c.into(), 1, &tables));
    assert!(phase0(c.into(), 2, &tables));

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 1));
    let c = c.apply_move(Move(Face::F, 2));
    let c = c.apply_move(Move(Face::R, 1));
    assert!(phase0(c.into(), 3, &tables));
  }
}
