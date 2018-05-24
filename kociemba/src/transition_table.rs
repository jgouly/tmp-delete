use cube::{Cube, Edge, Face, Move};

enum Group {
  G0,
  G1,
}

trait Coord {
  /// Number of elements in `Coord`'s transition table.
  const NUM_ELEMS: usize;
  /// Which `Group` this `Coord` is defined for.
  const GROUP: Group;
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
  const GROUP: Group = Group::G0;

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

/// The G0 CO coordinate is 7 digit base-3 number where each digit corresponds
/// to the orientation of the corner at that index. The 8th corner's orientation
/// is calculated based on the first 7 corner orientations.
struct COCoord;

impl Coord for COCoord {
  const NUM_ELEMS: usize = 2187; // 3 ^ 7
  const GROUP: Group = Group::G0;

  fn set_coord(cube: &mut Cube, co: usize) {
    assert!(co < Self::NUM_ELEMS);
    let mut co = co;
    for i in (0..7).rev() {
      cube.co[i] = (co % 3) as u8;
      co /= 3;
      cube.co[7] = ((cube.co[7] + 3) - cube.co[i]) % 3;
    }
    cube.verify().unwrap();
  }

  fn get_coord(cube: &Cube) -> usize {
    cube.co[..7]
      .iter()
      .fold(0usize, |acc, &cur| (acc * 3) + (cur as usize))
  }
}

/// The G0 UD1 coordinate encodes the position of the four E-slice
/// edges (FR, FL, BL, BR).
/// The actual permutation of the slice edges is ignored.
struct UD1Coord;

impl Coord for UD1Coord {
  const NUM_ELEMS: usize = 495; // 12 choose 4
  const GROUP: Group = Group::G0;

  /// Setting the position of the E-slice edges based on the coordinate.
  ///
  /// Calculating the positions starts from position 11, and iterates
  /// down to position 0. At every position (N) the binomial coefficient,
  /// C(N, K), is calculated.
  /// If C(N, K) is larger than the current coordinate, N is a slice edge and K
  /// is reduced by 1.
  /// If C(N, K) is less than or equal to the coordinate, the coordinate is
  /// reduced by C(N, K).
  /// K is initially 3 and is reduced by 1 for each slide edge at
  /// a position > N. When K becomes negative, the coordinate processing is
  /// calculation is complete.
  /// This means that edges at a lower position than the 4th slice edge are
  /// ignored.
  ///
  /// Example:
  ///
  ///   Coordinate = 321
  ///
  ///   N = 11, K = 3, Coord = 321, C(11, 3) = 165
  ///   N = 10, K = 3, Coord = 156, C(10, 3) = 120
  ///   N = 9, K = 3, Coord = 36, C(9, 3) = 84, 84 > 36, N is a slice edge
  ///   N = 8, K = 2, Coord = 36, C(8, 2) = 28
  ///   N = 7, K = 2, Coord = 8, C(7, 2) = 21, 21 > 8, N is a slice edge
  ///   N = 6, K = 1, Coord = 8, C(6, 1) = 6
  ///   N = 5, K = 1, Coord = 2, C(5, 1) = 6, 6 > 2, N is a slice edge
  ///   N = 4, K = 0, Coord = 2, C(4, 0) = 1
  ///   N = 3, K = 0, Coord = 1, C(3, 0) = 1
  ///   N = 2, K = 0, Coord = 0, C(2, 0) = 1, 1 > 0, N is a slice edge
  ///
  ///   +---+---+---+---+---+---+---+---+---+---+----+----+
  ///   | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
  ///   +---+---+---+---+---+---+---+---+---+---+----+----+
  ///   | - | - | X | - | - | X | - | X | - | X |  - |  - |
  ///   +---+---+---+---+---+---+---+---+---+---+----+----+
  fn set_coord(cube: &mut Cube, coord: usize) {
    let mut coord = coord;
    cube.ep.copy_from_slice(&[Edge::UR; 12]);
    let slice_edges = [Edge::FR, Edge::FL, Edge::BL, Edge::BR];
    let mut k = 3;
    for i in (0..12).rev() {
      let binomial = choose(i, k);
      if binomial > coord {
        cube.ep[i] = slice_edges[k];
        if k == 0 {
          break;
        }
        k -= 1;
      } else {
        coord -= binomial;
      }
    }

    // Replace all `UR` edges with edges from the solved edge permutation.
    // note: This does not affect the coordinate, but creates a valid cube.
    let solved_ep = Cube::solved().ep;
    cube
      .ep
      .iter_mut()
      .filter(|&&mut e| e == Edge::UR)
      .zip(&solved_ep)
      .for_each(|(x, y)| *x = *y);

    if !cube.has_valid_parity() {
      // Swap two corners to fix parity.
      cube.cp.swap(0, 1);
    }
    cube.verify().unwrap();
  }

  /// The UD coordinate is calculated using binomial coefficients.
  ///
  /// Calculating the coordinate starts from position 11, and iterates
  /// down to position 0. At every position (N) that is not a slice edge,
  /// the binomial coefficient, C(N, K), is summed up to produce the final
  /// coordinate. K is initially 3 and is reduced by 1 for each slide edge at
  /// a position > N. When K becomes negative, the calculation is complete.
  /// This means that edges at a lower position than the 4th slice edge are
  /// ignored.
  ///
  /// Example:
  ///
  ///   +---+---+---+---+---+---+---+---+---+---+----+----+
  ///   | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
  ///   +---+---+---+---+---+---+---+---+---+---+----+----+
  ///   | - | - | X | X | - | - | - | - | X | - | X  | -  |
  ///   +---+---+---+---+---+---+---+---+---+---+----+----+
  ///
  ///   N = 11, K = 3, C(11, 3) = 165
  ///   N = 10, K -= 1, Slice edge
  ///   N = 9, K = 2, C(9, 2) = 36
  ///   N = 8, K -= 1, Slice edge
  ///   N = 7, K = 1, C(7, 1) = 7
  ///   N = 6, K = 1, C(6, 1) = 6
  ///   N = 5, K = 1, C(5, 1) = 5
  ///   N = 4, K = 1, C(4, 1) = 4
  ///   N = 3, K -= 1, Slice edge
  ///   N = 2, K -= 1, Slice edge
  ///
  ///   Coordinate = 165 + 36 + 7 + 6 + 5 + 4 = 223
  fn get_coord(cube: &Cube) -> usize {
    let mut coord = 0;
    let mut k = 3;
    for i in (0..12).rev() {
      if cube.ep[i] < Edge::FR {
        coord += choose(i, k);
      } else {
        if k == 0 {
          break;
        }
        k -= 1;
      }
    }
    coord
  }
}

pub fn get_nth_zero(val: u8, n: u8) -> u8 {
  let mut val = val;
  for _ in 0..n {
    // makes the rightmost 0 bit a 1
    val |= val + 1;
  }
  //let val = val as u8;
  return (!val).trailing_zeros() as u8;
}

struct FactorialDigits<I: Iterator<Item = usize>> {
  val: usize,
  len: usize,
  base_iter: I,
}

fn factorial_digits(
  val: usize,
  len: usize,
) -> FactorialDigits<impl Iterator<Item = usize>> {
  FactorialDigits {
    val,
    len,
    base_iter: (0..len).map(factorial).rev(),
  }
}

impl<I: Iterator<Item = usize>> Iterator for FactorialDigits<I> {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
    if self.val == 0 && self.len == 0 {
      None
    } else {
      let base = self.base_iter.next().unwrap();
      let next = self.val / base;
      self.val %= base;

      if self.len > 0 {
        self.len -= 1;
      }

      Some(next)
    }
  }
}

fn set_perm_coord<P: From<usize>>(perm: &mut [P], coord: usize) {
  //let mut used_vec = vec![7, 6, 5, 4, 3, 2, 1, 0];
  //let mut used_vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
  let mut used_bits = 0u8;

  let digits2 = factorial_digits(coord, perm.len());
  //let digits2 = FactorialDigits::new(coord, 8);
  for (i, n) in (0..perm.len()).rev().zip(digits2) {
    //for (i, n) in (0..8).zip(digits2) {
    let bit_n = get_nth_zero(used_bits, n as u8) as usize;

    used_bits |= 1 << bit_n;

    perm[i] = (perm.len() - 1 - (bit_n)).into();
    //perm[i] = (7 - used_vec.remove(n)).into();
    //println!("n = {}, used_vec = {:?}", n, used_vec);
    //perm[i] = (used_vec.remove(n)).into();
    //perm[i] = (bit_n).into();
  }
}

/// The number of inversions with regards to the element `i`.
fn num_inversions_of<P: PartialOrd>(perm: &[P], i: usize) -> usize {
  if perm.len() <= i {
    return 0;
  }
  let p = &perm[i];
  perm[..i].iter().filter(|&j| j > p).count()
  //perm[i..].iter().filter(|&j| j < p).count()
}

/// A `Iterator` over the number of inversions of each element
/// of the permutation `perm`.
fn get_perm_inversions<'a, P: PartialOrd + 'a>(
  perm: &'a [P],
) -> impl Iterator<Item = usize> + 'a {
  (0..perm.len()).map(move |i| num_inversions_of(&perm, i))
  //(0..perm.len()).rev().map(move |i| num_inversions_of(&perm, i))
}

/// TODO: sum_i  i! * |{p(j) > p(i) : j < i}|
fn get_perm_coord<P: PartialOrd + ::std::fmt::Debug>(perm: &[P]) -> usize {
  //println!("{:?}", get_perm_inversions(perm).collect::<Vec<_>>());
  get_perm_inversions(perm)
    .zip((0..).map(factorial))
    .fold(0, |acc, (f, p)| acc + f * p)
}

/// The G1 EP coordinate encodes the positions of the U and D edges.
struct EPCoord;

impl Coord for EPCoord {
  const NUM_ELEMS: usize = 40320; // 8!
  const GROUP: Group = Group::G1;

  fn set_coord(cube: &mut Cube, ep: usize) {
    set_perm_coord(&mut cube.ep[0..8], ep);

    if !cube.has_valid_parity() {
      // Swap two corners to fix parity.
      cube.cp.swap(0, 1);
    }
    debug_assert!(cube.verify().is_ok());
  }

  fn get_coord(cube: &Cube) -> usize {
    get_perm_coord(&cube.ep[0..8])
  }
}

fn init_transition_table<T: Coord>() -> Vec<[usize; 6]> {
  let mut v = vec![[0; 6]; T::NUM_ELEMS];
  let turn_counts = match T::GROUP {
    Group::G0 => [1; 6],
    Group::G1 => [1, 1, 2, 2, 2, 2],
  };
  let turns = [Face::U, Face::D, Face::F, Face::B, Face::R, Face::L];

  for i in 0..v.len() {
    let mut c = Cube::solved();
    T::set_coord(&mut c, i);
    for (&f, &dir) in turns.iter().zip(&turn_counts) {
      let nc = c.apply_move(Move(f, dir));
      let coord = T::get_coord(&nc);
      assert!(coord < T::NUM_ELEMS);
      v[i][usize::from(f)] = coord;
    }
  }
  v
}

/// Get the G0 CO transition table.
pub fn get_co_transition_table() -> Vec<[usize; 6]> {
  init_transition_table::<COCoord>()
}

/// Get the G0 EO transition table.
pub fn get_eo_transition_table() -> Vec<[usize; 6]> {
  init_transition_table::<EOCoord>()
}

/// Get the G0 UD1 transition table.
pub fn get_ud1_transition_table() -> Vec<[usize; 6]> {
  init_transition_table::<UD1Coord>()
}

/// Get the G1 EP transition table.
pub fn get_ep_transition_table() -> Vec<[usize; 6]> {
  init_transition_table::<EPCoord>()
}

fn factorial(n: usize) -> usize {
  (1..n + 1).product()
}

// The binomial coefficient: C(N, K).
fn choose(n: usize, k: usize) -> usize {
  factorial(n) / (factorial(k) * factorial(n - k))
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::Corner::*;
  use cube::Edge::*;
  use cube::{NUM_CORNERS, NUM_EDGES};

  fn exhaustive_coord_check<T: Coord>() {
    for i in 0..T::NUM_ELEMS {
      let mut c = Cube::solved();
      T::set_coord(&mut c, i);
      assert_eq!(i, T::get_coord(&c));
    }
  }

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
    assert_eq!(0, eo[EOCoord::get_coord(&c)][usize::from(Face::U)]);
  }

  #[test]
  fn eo_coord_exhaustive() {
    exhaustive_coord_check::<EOCoord>();
  }

  #[test]
  fn co_coord() {
    let c = Cube::solved();
    assert_eq!(0, COCoord::get_coord(&c));

    for i in 1..4 {
      let c = c.apply_move(Move(Face::U, i));
      assert_eq!(0, COCoord::get_coord(&c));
    }

    let c = Cube::new(
      [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
      [2, 2, 2, 2, 2, 2, 2, 1],
      [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
      [0; NUM_EDGES],
    );
    assert_eq!(COCoord::NUM_ELEMS - 1, COCoord::get_coord(&c));
  }

  #[test]
  fn co_transition() {
    let co = get_co_transition_table();

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 3));
    assert_eq!(0, co[COCoord::get_coord(&c)][usize::from(Face::F)]);
  }

  #[test]
  fn co_coord_exhaustive() {
    exhaustive_coord_check::<COCoord>();
  }

  #[test]
  fn ud1_transition() {
    let ud1 = get_ud1_transition_table();

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 3));
    assert_eq!(0, ud1[UD1Coord::get_coord(&c)][usize::from(Face::F)]);
  }

  #[test]
  fn ud1_coord_exhaustive() {
    exhaustive_coord_check::<UD1Coord>();
  }

  #[test]
  fn ep_transition() {
    let ep = get_ep_transition_table();

    let c = Cube::solved();
    let c = c.apply_move(Move(Face::F, 2));
    assert_eq!(0, ep[EPCoord::get_coord(&c)][usize::from(Face::F)]);
  }

  #[test]
  fn ep_coord_exhaustive() {
    exhaustive_coord_check::<EPCoord>();
  }

  #[test]
  fn fact_digits() {
    let digits = factorial_digits(463, 6);
    assert_eq!(vec![3, 4, 1, 0, 1, 0], digits.collect::<Vec<_>>());

    let digits = factorial_digits(719, 6);
    assert_eq!(vec![5, 4, 3, 2, 1, 0], digits.collect::<Vec<_>>());

    let digits = factorial_digits(2982, 7);
    assert_eq!(vec![4, 0, 4, 1, 0, 0, 0], digits.collect::<Vec<_>>());

    let digits = factorial_digits(40319, 8);
    assert_eq!(vec![7, 6, 5, 4, 3, 2, 1, 0], digits.collect::<Vec<_>>());
  }
}
