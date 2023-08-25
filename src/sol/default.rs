//! **Non-vectorized** solution for Default problem.
//!
//! Problem: Given an input string containing a comma-separated
//! list of integers in the range `[0,9999]`, return the SADS (sum of absolute differences)
//! of every number in the input relative to the rounded median of the input values


/// Returns a solution to the Default problem without explicit vectorization.
/// 
/// `src`: single-line string of integers in the range `[0,9999]` separated by commas\
/// `print` specifies whether to output the solution\
/// \
/// `export RUSTFLAGS="-C opt-level=3 -C no-vectorize-loops"; cargo build --release`
pub unsafe fn exec(src: &str, print: bool) -> u64 {

    let mut pos_list: Vec<u16> = vec![];

    for pos in src.split(',') {
        pos_list.push(pos.parse().unwrap());
    }

    pos_list.sort_unstable();

    let ideal_pos: u16 =
        if pos_list.len() % 2 != 0 {
            pos_list[pos_list.len() / 2]
        } else {
            ( pos_list[pos_list.len() / 2] + pos_list[(pos_list.len() / 2) - 1] ) / 2 +
            ( pos_list[pos_list.len() / 2] + pos_list[(pos_list.len() / 2) - 1] ) % 2
        }
    ;

    let mut total_fuel: u64 = 0;

    for pos in pos_list {
        // Casting to i16 like this does not seem to change generated asm
        // vs changing function signature such that pos_list is Vec<i16>
        total_fuel = total_fuel.unchecked_add(((ideal_pos as i16)-(pos as i16)).abs() as u64);
    }

    if print { println!("result: {}", total_fuel) }

    total_fuel

}