extern crate cube;

use cube::Corner::*;
use cube::Edge::*;
use cube::*;

#[test]
fn solved_cube() {
  let cube = Cube::solved();
  let solved = Cube::new(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(solved, cube);
}
