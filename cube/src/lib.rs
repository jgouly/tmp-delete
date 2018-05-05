/// The corners on a 3x3x3 cube.
#[derive(Debug, PartialEq)]
pub enum Corner {
  URF,
  UFL,
  ULB,
  UBR,
  DFR,
  DLF,
  DBL,
  DRB,
}

/// The edges on a 3x3x3 cube.
#[derive(Debug, PartialEq)]
pub enum Edge {
  UR,
  UF,
  UL,
  UB,
  DR,
  DF,
  DL,
  DB,
  FR,
  FL,
  BL,
  BR,
}

/// Number of corners on a 3x3x3 cube.
pub const NUM_CORNERS: usize = 8;
/// Number of edges on a 3x3x3 cube.
pub const NUM_EDGES: usize = 12;

/// Models a 3x3x3 cube, separating permutation and orientation.
#[derive(Debug, PartialEq)]
pub struct Cube {
  cp: [Corner; NUM_CORNERS],
  co: [u8; NUM_CORNERS],
  ep: [Edge; NUM_EDGES],
  eo: [u8; NUM_EDGES],
}

impl Cube {
  /// Creates a new `Cube` with the specified permutations and orientations.
  pub fn new(
    cp: [Corner; NUM_CORNERS],
    co: [u8; NUM_CORNERS],
    ep: [Edge; NUM_EDGES],
    eo: [u8; NUM_EDGES],
  ) -> Cube {
    Cube { cp, co, ep, eo }
  }

  /// Creates a new `Cube` in the solved state.
  pub fn solved() -> Cube {
    let cp = [
      Corner::URF,
      Corner::UFL,
      Corner::ULB,
      Corner::UBR,
      Corner::DFR,
      Corner::DLF,
      Corner::DBL,
      Corner::DRB,
    ];
    let co = [0; NUM_CORNERS];
    let ep = [
      Edge::UR,
      Edge::UF,
      Edge::UL,
      Edge::UB,
      Edge::DR,
      Edge::DF,
      Edge::DL,
      Edge::DB,
      Edge::FR,
      Edge::FL,
      Edge::BL,
      Edge::BR,
    ];
    let eo = [0; NUM_EDGES];
    Cube::new(cp, co, ep, eo)
  }
}
