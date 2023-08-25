//! Parsing implementations
//!
//!
//! Credit: SIMD decimal parsing from [Parsing numbers into base-10 decimals with SIMD](https://vgatherps.github.io/2022-11-28-dec/)
//!    specifically, bits involving `mul_1_10, mul_1_100, _mm_maddubs_epi16, _mm_madd_epi16`

use std::mem::transmute;

// sse
use std::arch::x86_64::{ __m128i, 
    _mm_sub_epi8,
    _mm_set1_epi8, _mm_setr_epi8, _mm_setr_epi16,
    _mm_maddubs_epi16, _mm_madd_epi16,
    _mm_sll_epi32, _mm_add_epi32
};

// avx2
use std::arch::x86_64::{ __m256i,
    _mm256_sub_epi8,
    _mm256_maddubs_epi16, _mm256_madd_epi16,
    _mm256_sll_epi32, _mm256_add_epi32,
    _mm256_sll_epi16, _mm256_add_epi16,
};

/// Parse a string of comma-separated integers(`[0-9999]`) into Vec\<u16\>.
pub fn parse(input: &str) -> Vec<u16> {
    let mut pos_list: Vec<u16> = vec![];

    for pos in input.split(',') {
        pos_list.push(pos.parse().unwrap());
    }
    pos_list
}

/// Parse a string of comma-separated integers(`[0-9999]`) into a vector of 16 bit-width integers packed into 128-bit wide integer vectors.
///
/// Returns a tuple containing:\
/// 128-bit integer vector packed with 16-bit wide integers, last vector may contain padding zeroes\
/// the number of padding zeroes present in the last integer vector
///
pub unsafe fn parse_sse(input: &str) -> (Vec<__m128i>, usize) {

    let ascii = _mm_set1_epi8(b'0' as i8);

    let mut simd_data: Vec<__m128i> = Vec::with_capacity(50);

    // see cur_num
    let mut cur_batch: [u32;4] = [808464432;4];
    let mut batch_idx: usize = 0;

    // b'0' repeated 4x, so that missing digits go to 0 when 'ascii' ([b'0';16]) is substracted, alternatively could use saturating substraction
    // This will be compiled to something like:
    //     mov     eax, 808464432
    // Elsewhere the constant 808464432 will be used
    let mut cur_num: u32 = transmute::<[u8;4],_>([b'0';4]);

    let mut shifts_remaining: u8 = 4;

    for b in input.bytes() {

        if b != b',' {
            // Add next digit in num
            cur_num <<= 8;
            cur_num |= b as u32;
        }

        if shifts_remaining == 0 || b == b',' {
            
            cur_batch[batch_idx] = cur_num;        
            batch_idx += 1;

            // 128-bit vec has filled, push and reset
            if batch_idx > 3 {
                simd_data.push(_mm_sub_epi8(transmute(cur_batch),ascii));

                cur_batch = [808464432;4];
                batch_idx = 0;
            }

            cur_num = 808464432;
            shifts_remaining = 5;
        }
        shifts_remaining -= 1;
    }

    // clean up anything remaining (if number of values %4 != 0),
    // constant is cur_num init val
    if cur_num != 808464432 {
        cur_batch[batch_idx] = cur_num;
        batch_idx += 1;
        simd_data.push(_mm_sub_epi8(transmute(cur_batch),ascii));
    }

    // Take pairs of u8s (digits) and multiply the more significant one by 10,
    // and accumulate into pairwise u16
    let mul_1_10 = _mm_setr_epi8(
        1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10
    );

    // Take pairs of u16s (not digits, but two digits each)
    // multiply the more significant by 100 and add to get pairwise u32
    let mul_1_100 = _mm_setr_epi16(1, 100, 1, 100, 1, 100, 1, 100);

    // convert remapped ascii digits to u32 binary number (1 num from 4 digits)
    for simd_vec in &mut simd_data {
        *simd_vec = _mm_maddubs_epi16(*simd_vec, mul_1_10);
        *simd_vec = _mm_madd_epi16(*simd_vec, mul_1_100);
    }





    // length of filled vecs;
    let filled_len = simd_data.len() - 1.min(batch_idx%4);
    
    // trailing_elems are those that are part of an unfilled vec,
    // plus those from the vec prior to an unfilled vec if the total number of vecs is a multiple of 2
    let trailing_elems = (filled_len%2)*4 + (batch_idx%4);

    // no longer need to use 32 bits to store a number (ascii digits are gone),
    // compact&combine vec pairs so that we store 8 elements per __m128i instead of 4 (u16 vs u32)
    let mut simd_compacted: Vec<__m128i> = Vec::with_capacity(50);
    for simd_vec in simd_data.chunks_exact_mut(2) {
        let temp = _mm_sll_epi32(simd_vec[1], transmute([16u64,0]));
        simd_compacted.push(_mm_add_epi32(simd_vec[0], temp));
    }

    // if the last iteration of the above loop was polluted by a unfilled vec, remove it
    if trailing_elems > 4 {
        simd_compacted.remove(simd_compacted.len()-1);
    }


    // now need to compact remaining 32-bit trailing_elems (in binary repr) to 16 bits
    // each element needs to move a different amount
    // using a lazy non-performant approach    
    if trailing_elems > 0 {
        let mut cleaned_trailing: [u16;8] = [0;8];

        let trailing_filled = transmute::<_,[u32;4]>(simd_data[simd_data.len()-1]);

        for i in 0..(trailing_elems as usize) {
            cleaned_trailing[i] = trailing_filled[i] as u16;
        }

        // more than one [u32;4] bit vec to incorporate
        if trailing_elems > 4 {
            let trailing_unfilled = transmute::<_,[u32;4]>(simd_data[simd_data.len()-1]);
            for i in 0..4 {
                cleaned_trailing[4+i] = trailing_unfilled[i] as u16;
            }
        }

        // push now-compacted 16-bit trailing elements to simd_compacted
        simd_compacted.push(transmute::<_,__m128i>(cleaned_trailing));
    }

    // Need this to remove trailing zeroes from median calculation later
    let trailing_zeroes = 1.min(trailing_elems)*(8-trailing_elems);

    (simd_compacted, trailing_zeroes)
}

/// Parse a string of comma-separated integers(`[0-9999]`) into a vector of 16 bit-width integers packed in 256-bit wide integer vectors.
///
/// Returns a tuple containing:\
/// 256-bit integer vector packed with 16-bit wide integers, last vector may contain padding zeroes\
/// the number of padding zeroes present in the last integer vector
///
pub unsafe fn parse_avx2(input: &str) -> (Vec<__m256i>, usize) {

    let ascii: __m256i = transmute([b'0' as i8;32]);

    let mut cleaned: Vec<__m256i> = Vec::with_capacity(50);

    let mut cur_batch: [u32;8] = [808464432;8];
    let mut batch_idx: usize = 0;

    // b'0' repeated 4x, so that missing digits go to 0 when 'ascii' ([b'0';16]) is substracted, alternatively could use saturating substraction
    // This will be compiled to something like:
    //     mov     eax, 808464432
    // Elsewhere the constant 808464432 will be used
    let mut cur_num: u32 = transmute::<[u8;4],_>([b'0';4]);

    let mut shifts_remaining: u8 = 4;

    for b in input.bytes() {

        if b != b',' {
            // Add next digit in num
            cur_num <<= 8;
            cur_num |= b as u32;
        }

        if shifts_remaining == 0 || b == b',' {
            
            cur_batch[batch_idx] = cur_num;        
            batch_idx += 1;

            if batch_idx > 7 {
                cleaned.push(_mm256_sub_epi8(transmute(cur_batch),ascii));

                cur_batch = [808464432;8];
                batch_idx = 0;
            }

            cur_num = 808464432;
            shifts_remaining = 5;
        }
        shifts_remaining -= 1;
    }

    // clean up anything remaining, constant is cur_num init val
    if cur_num != 808464432 {
        cur_batch[batch_idx] = cur_num;
        batch_idx += 1;
        cleaned.push(_mm256_sub_epi8(transmute(cur_batch),ascii));
    }
    

    // Take pairs of u8s (digits) and multiply the more significant one by 10,
    // and accumulate into pairwise u16
    let mul_1_10 = transmute::<[u8;32],__m256i>([
        1,10,1,10,1,10,1,10,
        1,10,1,10,1,10,1,10,
        1,10,1,10,1,10,1,10,
        1,10,1,10,1,10,1,10,
    ]);

    // Take pairs of u16s (not digits, but two digits each)
    // multiply the more significant by 100 and add to get pairwise u32
    let mul_1_100 = transmute::<[u16;16],__m256i>([
        1,100,1,100,
        1,100,1,100,
        1,100,1,100,
        1,100,1,100,
    ]);

    // convert remapped ascii digits to u32 binary number (1 num from 4 digits)
    for compressed in &mut cleaned {
        *compressed = _mm256_maddubs_epi16(*compressed, mul_1_10);
        *compressed = _mm256_madd_epi16(*compressed, mul_1_100);
    }

    // trailing = inp_nums.len() % 16
    // trailing = (1.min(batch_idx%8))*8 + (batch_idx%8)
    // Cases:
    //     0 < [u16;trailing].len() <= 8 --> will not interfere with chunks_exact_mut
    //         cleaned[LAST] needs to be compacted and shifted
    //     8 < [u16;trailing].len() < 16 --> will interfere with chunks_exact_mut, assume that is prevented
    //         cleaned[LAST-1], cleaned[LAST] need to be combined in such a way that all zeroes are on the right

    // length of filled vecs;
    let filled_len = cleaned.len() - 1.min(batch_idx%8);
    
    // trailing_elems are those that are part of an unfilled vec,
    // plus those from the vec prior to an unfilled vec if the total number of vecs is a multiple of 2
    let trailing_elems = (filled_len%2)*8 + (batch_idx%8);


    // no longer need to use 32 bits to store a number (ascii digits are gone),
    // compact so that we store 16 elements per __m256i instead of 8 (u16 vs u32)
    let mut compacted: Vec<__m256i> = Vec::with_capacity(50);

    for compressed in cleaned.chunks_exact_mut(2) {
        let temp = _mm256_sll_epi32(compressed[1], transmute([16u64,0]));
        compacted.push(_mm256_add_epi32(compressed[0], temp));
    }

    // last chunk was polluted by unfilled vec
    if trailing_elems > 8 {
        compacted.remove(compacted.len()-1);
    }

    // now need to compact remaining 32-bit trailing_elems (in binary repr) to 16 bits
    // each element needs to move a different amount
    // ok I will use the lazy approach
    let cleaned_trailing: [u16;16] = 
        if trailing_elems > 0 && trailing_elems <= 8 {

            let trailing = transmute::<_,[u32;8]>(cleaned[cleaned.len()-1]);

            let mut cleaned_trailing: [u16;16] = [0;16];
            for i in 0..(trailing_elems as usize) {
                cleaned_trailing[i] = trailing[i] as u16;
            }
            cleaned_trailing
        } else {

            let trailing_filled = transmute::<_,[u32;8]>(cleaned[cleaned.len()-2]);
            let trailing_unfilled = transmute::<_,[u32;8]>(cleaned[cleaned.len()-1]);

            let mut cleaned_trailing: [u16;16] = [0;16];
            for i in 0..8 {
                cleaned_trailing[i] = trailing_filled[i] as u16;
            }
            for i in 0..8 {
                cleaned_trailing[8+i] = trailing_unfilled[i] as u16;
            }
            cleaned_trailing
        };

    if trailing_elems > 0 {
        compacted.push(transmute::<_,__m256i>(cleaned_trailing));
    }

    // Need this to remove trailing zeroes from median calculation later
    let trailing_zeroes = 1.min(trailing_elems)*(16-trailing_elems);

    (compacted, trailing_zeroes)
}

/// Parse a string of comma-separated integers(`[0-256]`) into a vector of 8 bit-width integers packed in 256-bit wide integer vectors.
///
/// Returns a tuple containing:\
/// 256-bit integer vector packed with 8-bit wide integers, last vector may contain padding zeroes\
/// the number of padding zeroes present in the last integer vector
///
pub unsafe fn parse_avx2_ideal(input: &str) -> (Vec<__m256i>, usize) {
    let (mut data, trailing_zeroes) = parse_avx2(input);
    // compact data from consisting of packed 16-bit ints to 8-bit ints

    // trailing_zeroes ==> [0,7], never have a fully unfilled vec
    // so, there's one possibly unfilled vec at the end
    // if it's incorporated 

    // length of filled vecs;
    let filled_len = data.len() - 1.min(trailing_zeroes);

    // trailing_elems are those that are part of an unfilled vec,
    // plus those from the vec prior to an unfilled vec if the total number of vecs is a multiple of 2
    let trailing_elems = (filled_len%2)*16 + 1.min(trailing_zeroes)*(16-trailing_zeroes);

    // _mm256_sll_epi16
    let mut simd_compacted: Vec<__m256i> = Vec::with_capacity(50);

    for compressed in data.chunks_exact_mut(2) {
        let temp = _mm256_sll_epi16(compressed[1], transmute([8u64,0]));
        simd_compacted.push(_mm256_add_epi16(compressed[0], temp));
    }

    // last chunk was polluted by unfilled vec
    // e.g. len()%2 == 0, data[LAST] == unfilled
    if trailing_elems > 16 {
        simd_compacted.remove(simd_compacted.len()-1);
    }


    // now need to compact remaining 16-bit trailing_elems (in binary repr) to 8 bits
    // each element needs to move a different amount
    // using a lazy non-performant approach    
    if trailing_elems > 0 {
        let mut cleaned_trailing: [u8;32] = [0;32];

        let trailing_filled = transmute::<_,[u16;16]>(data[data.len()-1]);

        for i in 0..(trailing_elems as usize) {
            cleaned_trailing[i] = trailing_filled[i] as u8;
        }

        // more than one [u16;8] bit vec to incorporate
        if trailing_elems > 16 {
            let trailing_unfilled = transmute::<_,[u16;16]>(data[data.len()-1]);
            for i in 0..16 {
                cleaned_trailing[16+i] = trailing_unfilled[i] as u8;
            }
        }

        // push now-compacted 8-bit trailing elements to simd_compacted
        simd_compacted.push(transmute::<_,__m256i>(cleaned_trailing));
    }

    // Need this to remove trailing zeroes from median calculation later
    let trailing_zeroes = 1.min(trailing_elems)*(32-trailing_elems);

    (simd_compacted,trailing_zeroes)
}
