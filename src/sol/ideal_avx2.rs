//! **Optimized SIMD(AVX2)** solution for the Ideal problem.
//!
//! Ideal problem: Sum of absolute differences between each element in input and first element in input,\
//! input value range: `[0,255]`\
//! \
//! Input parsing is done outside solution function


use std::{arch::x86_64::{
    __m256i,
    _mm256_extract_epi8, _mm256_sad_epu8, _mm256_add_epi64
}};

use std::mem::transmute;

// Separating single loop into a series of loops really hurts performance,
// what's the difference here vs simd_decimal?
 
/// Returns a solution to the Ideal problem using AVX2 intrinsics.
/// 
/// `src`: single-line string of integers in the range `[0,256]` separated by commas\
/// `print`: whether to output the solution\
/// Note: ([`relaxed`](crate::sol::relaxed), [`relaxed_avx2`](crate::sol::relaxed_avx2), [`relaxed_sse`](crate::sol::relaxed_sse)) also
/// can be applied to the Ideal problem, due to it being a subset of the Relaxed problem.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable="avx2")]
pub unsafe fn exec(data: &Vec<__m256i>, trailing_zeroes: usize, print: bool) -> u64 {

    let median_num: u8 = _mm256_extract_epi8::<0>(data[0]) as u8;
    let median: __m256i =  transmute([ median_num; 32]);

    let mut vec_sum: __m256i = transmute::<[u64;4],_>([0;4]);

    for compressed in data {
        let x = _mm256_sad_epu8(*compressed, median);
        vec_sum = _mm256_add_epi64(vec_sum,x);
    }

    let mut sum: u64 = transmute::<_,[u64;4]>(vec_sum).iter().sum::<u64>();

    // remove effect of trailing zeroes
    sum = sum.saturating_sub((median_num as u64) * (trailing_zeroes as u64));

    if print { println!("result: {}", sum) }

    sum
}