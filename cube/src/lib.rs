/// The faces on a 3x3x3 cube.
pub enum Face {
  U,
  R,
  F,
  D,
  B,
  L,
}

/// A move on a 3x3x3 cube.
pub struct Move(pub Face, pub u8);

/// The permutations and orientations representing a move.
struct MovePerm {
  cp: &'static [usize; NUM_CORNERS],
  co: &'static [u8; NUM_CORNERS],
  ep: &'static [usize; NUM_EDGES],
  eo: &'static [u8; NUM_EDGES],
}

/// An array containing the 6 basic moves on a 3x3x3.
const MOVE_PERMS: [MovePerm; 1] = [MOVE_PERM_U];

const MOVE_PERM_U: MovePerm = MovePerm {
  cp: &[3, 0, 1, 2, 4, 5, 6, 7],
  co: &[0; 8],
  ep: &[3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
  eo: &[0; 12],
};

/// The corners on a 3x3x3 cube.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

/// An error for the different invalid cube states.
#[derive(Debug, PartialEq)]
pub enum CubeStateErr {
  ErrEO,
  ErrCO,
  ErrEP,
  ErrCP,
  ErrParity,
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
    let cube = Cube { cp, co, ep, eo };
    cube.verify().unwrap();
    cube
  }

  /// Creates a new `Cube` with the specified permutations and orientations.
  /// This function does not check that the `Cube` is in a solvable state.
  pub fn new_unchecked(
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

  /// Return a new `Cube` after applying `Move` to the current `Cube`.
  pub fn apply_move(&self, move_: Move) -> Cube {
    let mp = &MOVE_PERMS[move_.0 as usize];
    let new = self.apply_move_perm(mp);
    new.verify().unwrap();
    new
  }

  /// Return a new `Cube` after applying `MovePerm` to the current `Cube`.
  fn apply_move_perm(&self, move_perm: &MovePerm) -> Cube {
    let mut cp = [Corner::URF; NUM_CORNERS];
    let mut co = [0; NUM_CORNERS];
    let mut ep = [Edge::UR; NUM_EDGES];
    let mut eo = [0; NUM_EDGES];

    for (i, &j) in move_perm.cp.iter().enumerate() {
      cp[i] = self.cp[j];
      co[i] = (self.co[j] + move_perm.co[j]) % 3;
    }

    for (i, &j) in move_perm.ep.iter().enumerate() {
      ep[i] = self.ep[j];
      eo[i] = self.eo[j] ^ move_perm.eo[i];
    }

    Cube { cp, co, ep, eo }
  }

  /// Verify that a `Cube` is in a solvable state.
  pub fn verify(&self) -> Result<(), CubeStateErr> {
    // Check that each edge is used only once.
    let mut edges = 0u16;
    for i in &self.ep {
      edges |= 1 << (*i as u16);
    }
    if edges != 0b111111111111 {
      return Err(CubeStateErr::ErrEP);
    }

    // Check that each edge orientation is 0 or 1.
    if !self.eo.iter().all(|&eo| eo <= 1) {
      return Err(CubeStateErr::ErrEO);
    }

    // Check that the total edge orientation is a multiple of 2.
    let eo: u8 = self.eo.iter().sum();
    if eo % 2 != 0 {
      return Err(CubeStateErr::ErrEO);
    }

    // Check that each corner is used only once.
    let mut corners = 0u8;
    for i in &self.cp {
      corners |= 1 << (*i as u8);
    }
    if corners != 0b11111111 {
      return Err(CubeStateErr::ErrCP);
    }

    // Check that each edge orientation is 0, 1 or 2.
    if !self.co.iter().all(|&co| co <= 2) {
      return Err(CubeStateErr::ErrCO);
    }

    // Check that the total corner orientation is a multiple of 3.
    let co: u8 = self.co.iter().sum();
    if co % 3 != 0 {
      return Err(CubeStateErr::ErrCO);
    }

    // Check that corner parity and edge parity are equal.
    if self.edge_parity() != self.corner_parity() {
      return Err(CubeStateErr::ErrParity);
    }
    Ok(())
  }

  fn corner_parity(&self) -> bool {
    num_inversions(&self.cp) % 2 != 0
  }

  fn edge_parity(&self) -> bool {
    num_inversions(&self.ep) % 2 != 0
  }
}

/// Count the number of inversions in a permutation.
fn num_inversions<P: PartialOrd>(perm: &[P]) -> usize {
  let mut num = 0;
  for i in 0..perm.len() - 1 {
    for j in i + 1..perm.len() {
      assert!(i < j);
      if perm[i] > perm[j] {
        num += 1;
      }
    }
  }
  num
}
