use cube::{Cube, Face, Move};
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

// Check if a solution is valid.
fn solution_check(solution: &[Move]) -> bool {
  let len = solution.len();
  if len > 0 {
    match solution[len - 1] {
      // Phase0 cannot end in U or D.
      Move(Face::U, _) | Move(Face::D, _) => return false,
      // Phase0 cannot end a half turn.
      Move(_, 2) => return false,
      _ => (),
    }

    if len > 1 {
      // Phase 0 cannot end in A2 B, where A and B are opposite faces.
      match &solution[len - 2..] {
        &[Move(f1, 2), Move(f2, _)] if f1.is_opposite(f2) => {
          return false;
        }
        _ => (),
      }
    }
  }
  true
}

// Check if a face should be skipped.
fn skip_face(solution: &[Move], face: Face) -> bool {
  let len = solution.len();
  if len > 0 {
    // Check for A A.
    match solution[len - 1] {
      Move(previous_face, _) if previous_face == face => return true,
      _ => (),
    }
  }
  false
}

/// Phase 0: Reduce a cube from G0 to G1.
pub fn phase0(
  coord: Phase0Coord,
  depth_remaining: usize,
  tables: &Phase0Tables,
  solution: &mut Vec<Move>,
) -> bool {
  if depth_remaining == 0 {
    if !solution_check(solution) {
      return false;
    }
    return coord.is_solved();
  }

  for &f in &[Face::U, Face::D, Face::F, Face::B, Face::R, Face::L] {
    if skip_face(solution, f) {
      continue;
    }
    let mut next = coord;
    for i in 0..3 {
      next = tables.transition(next, f);
      solution.push(Move(f, i + 1));
      if phase0(next, depth_remaining - 1, tables, solution) {
        return true;
      }
      solution.pop();
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::*;
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

    let mut solution = vec![];
    let c = Cube::solved();
    assert!(phase0(c.into(), 0, &tables, &mut solution));

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::U, 1));
    assert!(phase0(c.into(), 0, &tables, &mut solution));

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 1));
    assert!(!phase0(c.into(), 0, &tables, &mut solution));
    assert!(phase0(c.into(), 1, &tables, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::F, 1)] => true,
      _ => false,
    });

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 3));
    let c = c.apply_move(Move(Face::R, 3));
    assert!(!phase0(c.into(), 0, &tables, &mut solution));
    assert!(!phase0(c.into(), 1, &tables, &mut solution));
    assert!(phase0(c.into(), 2, &tables, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::R, 1), Move(Face::F, 1)] => true,
      _ => false,
    });

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 1));
    let c = c.apply_move(Move(Face::F, 2));
    let c = c.apply_move(Move(Face::R, 1));
    assert!(phase0(c.into(), 3, &tables, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::R, 3), Move(Face::F, 2), Move(Face::R, 1)] => true,
      _ => false,
    });

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 1));
    assert!(!phase0(c.into(), 2, &tables, &mut solution));

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::B, 1));
    let c = c.apply_move(Move(Face::R, 2));
    assert!(phase0(c.into(), 2, &tables, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::R, 2), Move(Face::B, 1)] => true,
      _ => false,
    });
    let mut solution = vec![];
    assert!(phase0(c.into(), 4, &tables, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::U, 2), Move(Face::D, 2), Move(Face::L, 2), Move(Face::F, 1)] => {
        true
      }
      _ => false,
    });
  }
}
