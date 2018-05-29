use cube::{Cube, Face, Move};
use std::cmp::max;
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
  eo_p: &'a [usize],
  co_p: &'a [usize],
  ud1_p: &'a [usize],
}

impl<'a> Phase0Tables<'a> {
  // The new `Phase0Coord` after doing the `face` move.
  // note: This only does quarter turns.
  fn transition(&self, coord: Phase0Coord, face: Face) -> Phase0Coord {
    let eo = self.eo_t[coord.eo][usize::from(face)];
    let co = self.co_t[coord.co][usize::from(face)];
    let ud1 = self.ud1_t[coord.ud1][usize::from(face)];
    Phase0Coord { eo, co, ud1 }
  }

  // The maximum prune depth for `coord`.
  fn prune_depth(&self, coord: Phase0Coord) -> usize {
    max(
      self.eo_p[coord.eo],
      max(self.co_p[coord.co], self.ud1_p[coord.ud1]),
    )
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

    if len > 1 {
      // Check for A B A where A and B are opposite faces.
      match &solution[len - 2..] {
        &[Move(f1, _), Move(f2, _)] if f1.is_opposite(f2) && f1 == face => {
          return true;
        }
        _ => (),
      }
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

  if depth_remaining < tables.prune_depth(coord) {
    return false;
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
  use pruning_table::*;
  use transition_table::*;

  lazy_static! {
    static ref CO_T: Vec<[usize; 6]> = { get_co_transition_table() };
    static ref EO_T: Vec<[usize; 6]> = { get_eo_transition_table() };
    static ref UD1_T: Vec<[usize; 6]> = { get_ud1_transition_table() };
    static ref CO_P: Box<[usize]> = { get_co_prune_table(&CO_T) };
    static ref EO_P: Box<[usize]> = { get_eo_prune_table(&EO_T) };
    static ref UD1_P: Box<[usize]> = { get_ud1_prune_table(&UD1_T) };
    static ref PHASE0TABLES: Phase0Tables<'static> = {
      Phase0Tables {
        co_t: &CO_T,
        eo_t: &EO_T,
        ud1_t: &UD1_T,
        co_p: &CO_P,
        eo_p: &EO_P,
        ud1_p: &UD1_P,
      }
    };
  }

  #[test]
  fn basic() {
    let mut solution = vec![];
    let c = Cube::solved();
    assert!(phase0(c.into(), 0, &PHASE0TABLES, &mut solution));

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::U, 1));
    assert!(phase0(c.into(), 0, &PHASE0TABLES, &mut solution));

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 1));
    assert!(!phase0(c.into(), 0, &PHASE0TABLES, &mut solution));
    assert!(phase0(c.into(), 1, &PHASE0TABLES, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::F, 1)] => true,
      _ => false,
    });

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 3));
    let c = c.apply_move(Move(Face::R, 3));
    assert!(!phase0(c.into(), 0, &PHASE0TABLES, &mut solution));
    assert!(!phase0(c.into(), 1, &PHASE0TABLES, &mut solution));
    assert!(phase0(c.into(), 2, &PHASE0TABLES, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::R, 1), Move(Face::F, 1)] => true,
      _ => false,
    });

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 1));
    let c = c.apply_move(Move(Face::F, 2));
    let c = c.apply_move(Move(Face::R, 1));
    assert!(phase0(c.into(), 3, &PHASE0TABLES, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::R, 3), Move(Face::F, 2), Move(Face::R, 1)] => true,
      _ => false,
    });

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 1));
    assert!(!phase0(c.into(), 2, &PHASE0TABLES, &mut solution));

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::B, 1));
    let c = c.apply_move(Move(Face::R, 2));
    assert!(phase0(c.into(), 2, &PHASE0TABLES, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::R, 2), Move(Face::B, 1)] => true,
      _ => false,
    });
    let mut solution = vec![];
    assert!(phase0(c.into(), 4, &PHASE0TABLES, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::U, 2), Move(Face::D, 2), Move(Face::L, 2), Move(Face::F, 1)] => {
        true
      }
      _ => false,
    });

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::L, 1));
    let c = c.apply_move(Move(Face::R, 1));
    assert!(phase0(c.into(), 2, &PHASE0TABLES, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::R, 1), Move(Face::L, 1)] => true,
      _ => false,
    });
    let mut solution = vec![];
    assert!(phase0(c.into(), 5, &PHASE0TABLES, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::U, 2), Move(Face::D, 2), Move(Face::F, 2), Move(Face::R, 1), Move(Face::L, 1)] => {
        true
      }
      _ => false,
    });
  }

  #[test]
  fn prune() {
    // CO and UD1 require 2 moves.
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 1));
    let c = c.apply_move(Move(Face::U, 1));
    assert_eq!(2, PHASE0TABLES.prune_depth(c.into()));

    // CO depth is 4 moves, UD1 is 3 moves (F' U F).
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 1));
    let c = c.apply_move(Move(Face::U, 1));
    let c = c.apply_move(Move(Face::R, 3));
    let c = c.apply_move(Move(Face::U, 3));
    assert_eq!(4, PHASE0TABLES.prune_depth(c.into()));

    // CO depth is 5 moves (F2 U2 R' U F), EO and UD1 are 0 moves.
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 1));
    let c = c.apply_move(Move(Face::U, 1));
    let c = c.apply_move(Move(Face::R, 3));
    let c = c.apply_move(Move(Face::U, 1));
    let c = c.apply_move(Move(Face::R, 1));
    let c = c.apply_move(Move(Face::U, 2));
    let c = c.apply_move(Move(Face::R, 3));
    assert_eq!(5, PHASE0TABLES.prune_depth(c.into()));
  }
}
