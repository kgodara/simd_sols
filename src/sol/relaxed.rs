//! **Non-vectorized** solution for the Relaxed problem.
//!
//! Relaxed problem: Sum of absolute differences between each element in input and first element in input,\
//! input value range: `[0,9999]`\
//! \
//! Input parsing is done outside solution function


/// Returns a solution to the Relaxed problem using without explicit vectorization.
/// `src`: single-line string of integers in the range `[0,9999]` separated by commas\
/// `print`: whether to output the solution\
/// Note: can be applied to the Ideal problem.\

pub unsafe fn exec(pos_list: &Vec<u16>, print: bool) -> u64 {

    let ideal_pos: u16 = pos_list[0];

    let mut total_fuel: u64 = 0;

    for pos in pos_list {
        // Casting to i16 like this case does not seem to change generated asm
        // vs changing function signature such that pos_list is Vec<i16>
        total_fuel = total_fuel.unchecked_add(((ideal_pos as i16)-(*pos as i16)).abs() as u64);
    }
    if print { println!("result: {}", total_fuel) }

    total_fuel
}