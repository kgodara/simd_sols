//! **SIMD(AVX2)** solution for the Relaxed problem.
//!
//! Relaxed problem: Sum of absolute differences between each element in input and first element in input,\
//! input value range: `[0,9999]`\
//! \
//! Input parsing is done outside solution function




use std::{arch::x86_64::{
    __m256i,
    _mm256_sub_epi16, _mm256_abs_epi16, _mm256_extract_epi16, _mm256_adds_epu16
}};

use std::mem::transmute;

// Separating single loop into a series of loops really hurts performance,
// what's the difference here vs simd_decimal?


/// Returns a solution to the Relaxed problem using AVX2 intrinsics.
/// 
/// `src`: a single-line string of integers in the range `[0,256]` separated by commas\
/// `trailing_zeroes`: the number of padding zeroes in the last __m128i \
/// `print`: whether to output the solution\
/// Note: can be applied to the Ideal problem.\
/// 
#[cfg(target_arch = "x86_64")]
#[target_feature(enable="avx2")]
pub unsafe fn exec(simd_data: &Vec<__m256i>, trailing_zeroes: usize, print: bool) -> u64 {

    let median_num: u16 = _mm256_extract_epi16::<0>(simd_data[0]) as u16;
    let median: __m256i =  transmute([ median_num; 16]);

    let mut sum: u64 = 0;
    let mut simd_vec_sum: __m256i = transmute::<[u16;16],_>([0;16]);


    // minimize number of horizontal additions (intrinsics or sum())
    // 6 additions is maximum number that can be done on 4-digit base10s before risk of overflow
    // (2^16)/(9999) ~= 6.5

    for simd_vec in simd_data.chunks_exact(6) {
        let x1 = _mm256_sub_epi16(median, simd_vec[0]);
        let x2 = _mm256_sub_epi16(median, simd_vec[1]);
        let x3 = _mm256_sub_epi16(median, simd_vec[2]);
        let x4 = _mm256_sub_epi16(median, simd_vec[3]);
        let x5 = _mm256_sub_epi16(median, simd_vec[4]);
        let x6 = _mm256_sub_epi16(median, simd_vec[5]);


        let x1 = _mm256_abs_epi16(x1);
        let x2 = _mm256_abs_epi16(x2);
        let x3 = _mm256_abs_epi16(x3);
        let x4 = _mm256_abs_epi16(x4);
        let x5 = _mm256_abs_epi16(x5);
        let x6 = _mm256_abs_epi16(x6);


        simd_vec_sum = _mm256_adds_epu16(simd_vec_sum, x1);
        simd_vec_sum = _mm256_adds_epu16(simd_vec_sum, x2);
        simd_vec_sum = _mm256_adds_epu16(simd_vec_sum, x3);
        simd_vec_sum = _mm256_adds_epu16(simd_vec_sum, x4);
        simd_vec_sum = _mm256_adds_epu16(simd_vec_sum, x5);
        simd_vec_sum = _mm256_adds_epu16(simd_vec_sum, x6);

        // hadd + extract likely won't move performance meaningfully since
        // profiling suggests _mm256_sub_epi16 dominates
        sum = sum.unchecked_add(transmute::<_,[u16;16]>(simd_vec_sum).iter().map(|x|*x as u64).sum::<u64>());
        simd_vec_sum = transmute::<[u16;16],_>([0;16]);
    }

    let rem_idx = simd_data.len() - (simd_data.len()%6);
    for x in simd_data[rem_idx..].iter() {
        let temp = _mm256_sub_epi16(median, *x);
        let temp = _mm256_abs_epi16(temp);
        simd_vec_sum = _mm256_adds_epu16(simd_vec_sum, temp);
    }
    sum = sum.unchecked_add(transmute::<_,[u16;16]>(simd_vec_sum).iter().map(|x|*x as u64).sum::<u64>());


    // remove effect of trailing zeroes
    sum = sum.saturating_sub((median_num as u64) * (trailing_zeroes as u64));
    
    if print { println!("result: {}", sum) }

    sum
}