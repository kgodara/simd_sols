use include_dir::{include_dir, Dir};

#[macro_use]
extern crate lazy_static;

use simd_sads::{
    sol::{ default, default_sse, relaxed, relaxed_sse, relaxed_avx2, ideal_avx2 },
    util::parse::{ parse, parse_sse, parse_avx2, parse_avx2_ideal },
};


static IN_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/input");

static IDEAL_SRC_NAME_LIST: [&str; 5] = [
    "ideal_1x.txt",
    "ideal_10x.txt",
    "ideal_100x.txt",
    "ideal_1000x.txt",
    "ideal_10000x.txt",
];

static SRC_NAME_LIST: [&str; 5] = [
    "1x.txt",
    "10x.txt",
    "100x.txt",
    "1000x.txt",
    "10000x.txt",
];

lazy_static! {

    pub static ref IDEAL_SRC_LIST: [&'static str;5] = [
        IN_DIR.get_file("ideal_1x.txt").unwrap().contents_utf8().unwrap(),
        IN_DIR.get_file("ideal_10x.txt").unwrap().contents_utf8().unwrap(),
        IN_DIR.get_file("ideal_100x.txt").unwrap().contents_utf8().unwrap(),
        IN_DIR.get_file("ideal_1000x.txt").unwrap().contents_utf8().unwrap(),
        IN_DIR.get_file("ideal_10000x.txt").unwrap().contents_utf8().unwrap(),
    ];

    pub static ref SRC_LIST: [&'static str;5] = [
        IN_DIR.get_file("1x.txt").unwrap().contents_utf8().unwrap(),
        IN_DIR.get_file("10x.txt").unwrap().contents_utf8().unwrap(),
        IN_DIR.get_file("100x.txt").unwrap().contents_utf8().unwrap(),
        IN_DIR.get_file("1000x.txt").unwrap().contents_utf8().unwrap(),
        IN_DIR.get_file("10000x.txt").unwrap().contents_utf8().unwrap(),
    ];
}

#[test]
pub fn parse_inp_sse() {

    for src in SRC_LIST.iter() {
        let mut default_parsed = parse(*src);

        let (sse_parsed, trailing_zeroes) = unsafe { parse_sse(*src) };
        let mut sse_parsed_u16: Vec<u16> = unsafe { sse_parsed.into_iter().flat_map(|x| std::mem::transmute::<_,[u16;8]>(x)).collect() };
        
        // remove trailing zeroes from sse_parsed_u16
        for _ in 0..trailing_zeroes {
            sse_parsed_u16.remove(sse_parsed_u16.len()-1);
        }
        
        default_parsed.sort_unstable();
        sse_parsed_u16.sort_unstable();

        assert_eq!(default_parsed.len(), sse_parsed_u16.len(), "default_parsed.len() != sse_parsed_u16.len()");

        for pair in default_parsed.iter().zip(sse_parsed_u16.iter()) {
            assert_eq!(pair.0, pair.1, "parsed values differ between default_parsed and sse_parsed_u16");
        }
    }
}

#[test]
pub fn parse_inp_avx2() {

    for src in SRC_LIST.iter() {
        let mut default_parsed = parse(*src);

        let (avx2_parsed, trailing_zeroes) = unsafe { parse_avx2(*src) };
        let mut avx2_parsed_u16: Vec<u16> = unsafe { avx2_parsed.into_iter().flat_map(|x| std::mem::transmute::<_,[u16;16]>(x)).collect() };
        
        // remove trailing zeroes from sse_parsed_u16
        for _ in 0..trailing_zeroes {
            avx2_parsed_u16.remove(avx2_parsed_u16.len()-1);
        }
        
        default_parsed.sort_unstable();
        avx2_parsed_u16.sort_unstable();

        assert_eq!(default_parsed.len(), avx2_parsed_u16.len(), "default_parsed.len() != avx2_parsed_u16.len()");

        for pair in default_parsed.iter().zip(avx2_parsed_u16.iter()) {
            assert_eq!(pair.0, pair.1, "parsed values differ between default_parsed and avx2_parsed_u16");
        }
    }
}

#[test]
pub fn parse_inp_ideal_avx2() {

    for src in IDEAL_SRC_LIST.iter() {
        let mut default_parsed: Vec<u8> = parse(*src).into_iter().map(|x| x as u8).collect();

        let (avx2_parsed, trailing_zeroes) = unsafe { parse_avx2_ideal(*src) };
        let mut avx2_parsed_u8: Vec<u8> = unsafe { avx2_parsed.into_iter().flat_map(|x| std::mem::transmute::<_,[u8;32]>(x)).collect() };
        
        // remove trailing zeroes from sse_parsed_u16
        for _ in 0..trailing_zeroes {
            avx2_parsed_u8.remove(avx2_parsed_u8.len()-1);
        }

        default_parsed.sort_unstable();
        avx2_parsed_u8.sort_unstable();

        assert_eq!(default_parsed.len(), avx2_parsed_u8.len(), "default_parsed.len() != avx2_parsed_u8.len()");

        for pair in default_parsed.iter().zip(avx2_parsed_u8.iter()) {
            assert_eq!(pair.0, pair.1, "parsed values differ between default_parsed and avx2_parsed_u8");
        }
    }
}



#[test]
pub fn sol_default_sse() {

    for (idx,src) in SRC_LIST.iter().enumerate() {
        let default_res = unsafe { default::exec(*src, false) };
        let sse_res = unsafe { default_sse::exec(*src, false) };
        assert_eq!(default_res, sse_res, "Different results for {}", SRC_NAME_LIST[idx]);
    }
}

#[test]
pub fn sol_relaxed_sse() {

    for (idx,src) in SRC_LIST.iter().enumerate() {
        let mut default_parsed = parse(src);
        let default_res = unsafe { relaxed::exec(&mut default_parsed, false) };

        let (mut sse_parsed, trailing_zeroes) = unsafe { parse_sse(*src) };
        let sse_res = unsafe { relaxed_sse::exec(&mut sse_parsed, trailing_zeroes, false) };

        assert_eq!(default_res, sse_res, "Different results for {}", SRC_NAME_LIST[idx]);
    }
}

#[test]
pub fn sol_relaxed_avx2() {

    for (idx,src) in SRC_LIST.iter().enumerate() {
        let mut default_parsed = parse(src);
        let default_res = unsafe { relaxed::exec(&mut default_parsed, false) };

        let (mut avx2_parsed, trailing_zeroes) = unsafe { parse_avx2(*src) };
        let avx2_res = unsafe { relaxed_avx2::exec(&mut avx2_parsed, trailing_zeroes, false) };

        assert_eq!(default_res, avx2_res, "Different results for {}", SRC_NAME_LIST[idx]);
    }
}


#[test]
pub fn sol_ideal_avx2() {

    for (idx,src) in IDEAL_SRC_LIST.iter().enumerate() {
        let mut ideal_parsed = parse(src);
        let ideal_res = unsafe { relaxed::exec(&mut ideal_parsed, false) };

        let (mut avx2_parsed, trailing_zeroes) = unsafe { parse_avx2_ideal(*src) };
        let avx2_res = unsafe { ideal_avx2::exec(&mut avx2_parsed, trailing_zeroes, false) };

        assert_eq!(ideal_res, avx2_res, "Different results for {}", IDEAL_SRC_NAME_LIST[idx]);
    }
}