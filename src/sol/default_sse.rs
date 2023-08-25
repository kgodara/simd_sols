//! **SIMD(SSE)** solution for Default problem.
//!
//! Default problem, source: [Advent of Code 2021 Day 7 Part 1](https://adventofcode.com/2021/day/7)\
//! \
//! Problem: Given an input string containing a comma-separated
//! list of integers in the range `[0,9999]`, return the SADS (sum of absolute differences)
//! of every number in the input relative to the rounded median of the input values


use std::{arch::x86_64::{
    __m128i,
    _mm_sub_epi16, _mm_abs_epi16,
}};

use crate::util::parse::parse_sse;


/// Returns a solution to the Default problem using SSE intrinsics.
/// 
/// `src`: single-line string of integers in the range `[0,9999]` separated by commas\
/// `print`: whether to output the solution\
/// 

#[cfg(target_arch = "x86_64")]
#[target_feature(enable="ssse3")]
pub unsafe fn exec(src: &str, print: bool) -> u64 {

    // Note: many shuffle instructions require compile-time constants
    // When conditions allow, can generate all possibilities and pick at run-time?
    // e.g. _mm_shuffle_epi32::<imm8>(x);

    let (mut data_sse, trailing_zeroes) = parse_sse(src);

    let mut data: Vec<u16> = data_sse.iter().flat_map(|x| std::mem::transmute::<_,[u16;8]>(*x)).collect();

   // so compiler doesn't complain about immutable borrow in working_slice declaration
   let data_len = data.len();

    // exclude any trailing zeroes
    let working_slice = &mut data[..data_len - (trailing_zeroes as usize)];

    working_slice.sort_unstable();

    // rounded median value
    let median_num: u16 = if working_slice.len() % 2 != 0 {
            working_slice[working_slice.len() / 2]
        } else {
            ( working_slice[working_slice.len() / 2] + working_slice[(working_slice.len() / 2) - 1] ) / 2 +
            ( working_slice[working_slice.len() / 2] + working_slice[(working_slice.len() / 2) - 1] ) % 2
        }
    ;

    let median: __m128i =  std::mem::transmute([median_num;8]);
    let mut sum: u64 = 0;

    // calculate SADS (sum of absolute differences) between each vector and the median, add result to sum
    for compressed in &mut *data_sse {
        *compressed = _mm_sub_epi16(median, *compressed);
        *compressed = _mm_abs_epi16(*compressed);
        sum += std::mem::transmute::<_,[u16;8]>(*compressed).iter().map(|x|*x as u64).sum::<u64>();
    }

    // remove effect of padding zeroes at the end
    sum = sum.saturating_sub((median_num * (trailing_zeroes as u16)) as u64);

    if print { println!("result: {}", sum) }

    sum

}