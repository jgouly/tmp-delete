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
