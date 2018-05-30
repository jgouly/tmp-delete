use cube::{Cube, Face, Move};
use std::cmp::max;
use transition_table::CPCoord;
use transition_table::Coord;
use transition_table::EPCoord;
use transition_table::UD2Coord;

#[derive(Clone, Copy)]
pub struct Phase1Coord {
  ep: usize,
  cp: usize,
  ud2: usize,
}

impl Phase1Coord {
  fn is_solved(&self) -> bool {
    self.ep == 0 && self.cp == 0 && self.ud2 == 0
  }
}

impl From<Cube> for Phase1Coord {
  fn from(val: Cube) -> Phase1Coord {
    Phase1Coord {
      ep: EPCoord::get_coord(&val),
      cp: CPCoord::get_coord(&val),
      ud2: UD2Coord::get_coord(&val),
    }
  }
}

pub struct Phase1Tables<'a> {
  ep_t: &'a [[usize; 6]],
  cp_t: &'a [[usize; 6]],
  ud2_t: &'a [[usize; 6]],
  ep_p: &'a [usize],
  cp_p: &'a [usize],
  ud2_p: &'a [usize],
}

impl<'a> Phase1Tables<'a> {
  // The new `Phase1Coord` after doing the `face` move.
  // note: This is a quarter turn for U/D and half turn for FBRL.
  fn transition(&self, coord: Phase1Coord, face: Face) -> Phase1Coord {
    let ep = self.ep_t[coord.ep][usize::from(face)];
    let cp = self.cp_t[coord.cp][usize::from(face)];
    let ud2 = self.ud2_t[coord.ud2][usize::from(face)];
    Phase1Coord { ep, cp, ud2 }
  }

  // The maximum prune depth for `coord`.
  fn prune_depth(&self, coord: Phase1Coord) -> usize {
    max(
      self.ep_p[coord.ep],
      max(self.cp_p[coord.cp], self.ud2_p[coord.ud2]),
    )
  }
}

// Check if a solution is valid.
fn solution_check(_solution: &[Move]) -> bool {
  return true;
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

/// Phase 1: Reduce a cube from G1 to solved.
pub fn phase1(
  coord: Phase1Coord,
  depth_remaining: usize,
  tables: &Phase1Tables,
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

    // FBRL are half turns only.
    let move_range = if f == Face::U || f == Face::D {
      0..3
    } else {
      1..2
    };
    let mut next = coord;
    for i in move_range {
      next = tables.transition(next, f);
      solution.push(Move(f, i + 1));
      if phase1(next, depth_remaining - 1, tables, solution) {
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
    static ref CP_T: Vec<[usize; 6]> = { get_cp_transition_table() };
    static ref EP_T: Vec<[usize; 6]> = { get_ep_transition_table() };
    static ref UD2_T: Vec<[usize; 6]> = { get_ud2_transition_table() };
    static ref CP_P: Box<[usize]> = { get_cp_prune_table(&CP_T) };
    static ref EP_P: Box<[usize]> = { get_ep_prune_table(&EP_T) };
    static ref UD2_P: Box<[usize]> = { get_ud2_prune_table(&UD2_T) };
    static ref PHASE1TABLES: Phase1Tables<'static> = {
      Phase1Tables {
        cp_t: &CP_T,
        ep_t: &EP_T,
        ud2_t: &UD2_T,
        cp_p: &CP_P,
        ep_p: &EP_P,
        ud2_p: &UD2_P,
      }
    };
  }

  fn check_is_solved(cube: Cube, solution: &[Move]) -> bool {
    let solved = solution.iter().fold(cube, |acc, &cur| acc.apply_move(cur));
    Phase1Coord::from(solved).is_solved()
  }

  #[test]
  fn basic() {
    let mut solution = vec![];
    let c = Cube::solved();
    assert!(phase1(c.into(), 0, &PHASE1TABLES, &mut solution));
    assert!(check_is_solved(c, &solution));

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::U, 1));
    assert!(!phase1(c.into(), 0, &PHASE1TABLES, &mut solution));
    assert!(phase1(c.into(), 1, &PHASE1TABLES, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::U, 3)] => true,
      _ => false,
    });
    assert!(check_is_solved(c, &solution));

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 2));
    assert!(!phase1(c.into(), 0, &PHASE1TABLES, &mut solution));
    assert!(!phase1(c.into(), 2, &PHASE1TABLES, &mut solution));
    assert!(phase1(c.into(), 1, &PHASE1TABLES, &mut solution));
    assert!(match &solution[..] {
      [Move(Face::R, 2)] => true,
      _ => false,
    });
    assert!(check_is_solved(c, &solution));

    let mut solution = vec![];
    let c = Cube::solved();
    let c = c.apply_move(Move(Face::R, 2));
    let c = c.apply_move(Move(Face::F, 2));
    assert!(!phase1(c.into(), 0, &PHASE1TABLES, &mut solution));
    assert!(!phase1(c.into(), 1, &PHASE1TABLES, &mut solution));
    assert!(phase1(c.into(), 2, &PHASE1TABLES, &mut solution));

    assert!(match &solution[..] {
      [Move(Face::F, 2), Move(Face::R, 2)] => true,
      _ => false,
    });
    assert!(check_is_solved(c, &solution));
  }
}
