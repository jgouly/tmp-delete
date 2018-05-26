use cube::Face;

fn init_prune_table_inner(
  coord: usize,
  prune_table: &mut [usize],
  trans_table: &[[usize; 6]],
  max_depth: usize,
  depth: usize,
) {
  // End the current search branch if max_depth is reached or the current
  // coordinate was already reached at a lower depth.
  if depth == max_depth || prune_table[coord] <= depth {
    return;
  }
  // Save the current depth for this coordinate.
  prune_table[coord] = depth;
  for &f in &[Face::U, Face::D, Face::F, Face::B, Face::R, Face::L] {
    let mut new_coord = coord;
    for _ in 0..3 {
      new_coord = trans_table[new_coord][usize::from(f)];
      init_prune_table_inner(
        new_coord,
        prune_table,
        trans_table,
        max_depth,
        depth + 1,
      );
    }
  }
}

/// Initialise a pruning table from a transition table. The pruning table
/// stores the depth of each coordinate.
fn init_prune_table(
  trans_table: &[[usize; 6]],
  max_depth: usize,
  table_size: usize,
) -> Box<[usize]> {
  let mut table = vec![table_size; table_size];
  init_prune_table_inner(0, &mut table, trans_table, max_depth, 0);
  table.into_boxed_slice()
}

/// Get the G0 CO prune table.
pub fn get_co_prune_table(co_trans: &[[usize; 6]]) -> Box<[usize]> {
  init_prune_table(&co_trans[..], 7, co_trans.len())
}

/// Get the G0 EO prune table.
pub fn get_eo_prune_table(eo_trans: &[[usize; 6]]) -> Box<[usize]> {
  init_prune_table(&eo_trans[..], 8, eo_trans.len())
}

/// Get the G0 UD1 prune table.
pub fn get_ud1_prune_table(ud1_trans: &[[usize; 6]]) -> Box<[usize]> {
  init_prune_table(&ud1_trans[..], 6, ud1_trans.len())
}

/// Get the G1 CP prune table.
pub fn get_cp_prune_table(cp_trans: &[[usize; 6]]) -> Box<[usize]> {
  init_prune_table(&cp_trans[..], 14, cp_trans.len())
}

/// Get the G1 EP prune table.
pub fn get_ep_prune_table(ep_trans: &[[usize; 6]]) -> Box<[usize]> {
  init_prune_table(&ep_trans[..], 9, ep_trans.len())
}

/// Get the G1 UD2 prune table.
pub fn get_ud2_prune_table(ud2_trans: &[[usize; 6]]) -> Box<[usize]> {
  init_prune_table(&ud2_trans[..], 5, ud2_trans.len())
}

#[cfg(test)]
mod tests {
  use super::*;
  use transition_table::*;

  #[test]
  fn co_prune() {
    let co_t = get_co_transition_table();
    let co_p = get_co_prune_table(&co_t);
    assert!(co_p.iter().all(|&depth| depth < co_t.len()));
    assert_eq!(&6, co_p.iter().max().unwrap());
  }

  #[test]
  fn eo_prune() {
    let eo_t = get_eo_transition_table();
    let eo_p = get_eo_prune_table(&eo_t);
    assert!(eo_p.iter().all(|&depth| depth < eo_t.len()));
    assert_eq!(&7, eo_p.iter().max().unwrap());
  }

  #[test]
  fn ud1_prune() {
    let ud1_t = get_ud1_transition_table();
    let ud1_p = get_ud1_prune_table(&ud1_t);
    assert!(ud1_p.iter().all(|&depth| depth < ud1_t.len()));
    assert_eq!(&5, ud1_p.iter().max().unwrap());
  }

  #[test]
  fn cp_prune() {
    let cp_t = get_cp_transition_table();
    let cp_p = get_cp_prune_table(&cp_t);
    assert!(cp_p.iter().all(|&depth| depth < cp_t.len()));
    assert_eq!(&13, cp_p.iter().max().unwrap());
  }

  #[test]
  fn ep_prune() {
    let ep_t = get_ep_transition_table();
    let ep_p = get_ep_prune_table(&ep_t);
    assert!(ep_p.iter().all(|&depth| depth < ep_t.len()));
    assert_eq!(&8, ep_p.iter().max().unwrap());
  }

  #[test]
  fn ud2_prune() {
    let ud2_t = get_ud2_transition_table();
    let ud2_p = get_ud2_prune_table(&ud2_t);
    assert!(ud2_p.iter().all(|&depth| depth < ud2_t.len()));
    assert_eq!(&4, ud2_p.iter().max().unwrap());
  }
}
