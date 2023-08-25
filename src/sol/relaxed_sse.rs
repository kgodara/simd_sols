//! **SIMD(SSE)** solution for the Relaxed problem.
//!
//! Relaxed problem: Sum of absolute differences between each element in input and first element in input,\
//! input value range: `[0,9999]`\
//! \
//! Input parsing is done outside solution function



use std::{arch::x86_64::{
    __m128i,
    _mm_extract_epi16, _mm_abs_epi16, _mm_sub_epi16
}};

use std::mem::transmute;

/// Returns a solution to the Relaxed problem using SSE intrinsics.
/// 
/// `src`: single-line string of integers in the range `[0,9999]` separated by commas\
/// `trailing_zeroes`: is the number of padding zeroes in the last __m128i\
/// `print`: specifies whether to output the solution\
/// Note: can be applied to the Ideal problem.\

#[cfg(target_arch = "x86_64")]
#[target_feature(enable="ssse3")]
pub unsafe fn exec(data: &Vec<__m128i>, trailing_zeroes: usize, print: bool) -> u64 {

    let median_num: u16 = _mm_extract_epi16::<0>(data[0]) as u16;
    let median: __m128i =  transmute([median_num; 8]);

    let mut sum: u64 = 0;

    // (_mm_abs_epi16, _mm_sub_epi16) should (naïvely) be better than (_mm_or_si128, _mm_subs_epu16, _mm_subs_epu16)
    //     2 vs 3 intrinsics
    // since our values are bounded [0,9999],
    // the signed vs unsigned distinction is irrelevant for us

    // transmute::<_,[u16;8]>(), sum() seems significantly faster than successive hadd intrinsics:
    for compressed in data {
        let mut temp = _mm_sub_epi16(median, *compressed);
        temp = _mm_abs_epi16(temp);
        sum += transmute::<_,[u16;8]>(temp).iter().map(|x|*x as u64).sum::<u64>();
    }

    sum = sum.saturating_sub((median_num * (trailing_zeroes as u16)) as u64);

    if print { println!("result: {}", sum) }

    sum
}