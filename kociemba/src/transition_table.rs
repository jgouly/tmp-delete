use cube::{Cube, Face, Move};

trait Coord {
  /// Number of elements in `Coord`'s transition table.
  const NUM_ELEMS: usize;
  /// Modify `Cube` to have the given coordinate.
  fn set_coord(cube: &mut Cube, coord: usize);
  /// Get the coordinate for a given `Cube`.
  fn get_coord(cube: &Cube) -> usize;
}

/// The G0 EO coordinate is an 11-bit number where each bit corresponds
/// to the orientation of the edge at that index. The 12th edge's orientation
/// is calculated based on the first 11 edge orientations.
struct EOCoord;

impl Coord for EOCoord {
  const NUM_ELEMS: usize = 2048; // 2 ^ 11

  fn set_coord(cube: &mut Cube, eo: usize) {
    assert!(eo < Self::NUM_ELEMS);
    let mut eo = eo;
    for i in (0..11).rev() {
      cube.eo[i] = (eo & 1) as u8;
      cube.eo[11] ^= (eo & 1) as u8;
      eo >>= 1;
    }
    cube.verify().unwrap();
  }

  fn get_coord(cube: &Cube) -> usize {
    cube.eo[..11]
      .iter()
      .fold(0, |acc, &cur| (acc | cur as usize) << 1) >> 1
  }
}

fn init_transition_table<T: Coord>() -> Vec<[usize; 6]> {
  let mut v = vec![[0; 6]; T::NUM_ELEMS];
  let turn_counts = [1; 6];
  let turns = [Face::U, Face::D, Face::F, Face::B, Face::R, Face::L];

  for i in 0..v.len() {
    let mut c = Cube::solved();
    T::set_coord(&mut c, i);
    for (&f, &dir) in turns.iter().zip(&turn_counts) {
      let nc = c.apply_move(Move(f, dir));
      let coord = T::get_coord(&nc);
      assert!(coord < T::NUM_ELEMS);
      v[i][f as usize] = coord;
    }
  }
  v
}

/// Get the G0 EO transition table.
pub fn get_eo_transition_table() -> Vec<[usize; 6]> {
  init_transition_table::<EOCoord>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::Corner::*;
  use cube::Edge::*;
  use cube::{NUM_CORNERS, NUM_EDGES};

  #[test]
  fn eo_coord() {
    let c = Cube::solved();
    assert_eq!(0, EOCoord::get_coord(&c));

    for i in 1..4 {
      let c = c.apply_move(Move(Face::U, i));
      assert_eq!(0, EOCoord::get_coord(&c));
    }

    let c = Cube::new(
      [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
      [0; NUM_CORNERS],
      [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
      [1; NUM_EDGES],
    );
    assert_eq!(EOCoord::NUM_ELEMS - 1, EOCoord::get_coord(&c));
  }

  #[test]
  fn eo_transition() {
    let eo = get_eo_transition_table();

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::U, 3));
    assert_eq!(0, eo[EOCoord::get_coord(&c)][Face::U as usize]);
  }
}
