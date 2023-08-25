#![feature(unchecked_math)]

use include_dir::{include_dir, Dir};

use clap::Parser;

mod sol;
mod util;

use sol::{
    default, default_sse,
    relaxed, relaxed_sse, relaxed_avx2,
    ideal_avx2,
};

/// Execute Problem Solution with given variant
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file name: "input/{NAME}.txt"
    #[clap(short, long, default_value = "")]
    name: String,

    /// Variant: ["default", "default_sse", "relaxed", "relaxed_sse", "relaxed_avx2", "ideal_avx2"]
    #[clap(short, long, default_value = "")]
    variant: String
}

enum Variant {
    Default,
    DefaultSSE,
    Relaxed,
    RelaxedSSE,
    RelaxedAVX2,
    IdealAVX2,
}


fn main() {

    let args = Args::parse();

    let variant: Variant = match args.variant.as_str() {
        "default" => Variant::Default,
        "default_sse" => Variant::DefaultSSE,
        "relaxed" => Variant::Relaxed,
        "relaxed_sse" => Variant::RelaxedSSE,
        "relaxed_avx2" => Variant::RelaxedAVX2,
        "ideal_avx2" => Variant::IdealAVX2,
        _ => panic!("Invalid variant!"),
    };

    static IN_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/input");
    let src = IN_DIR.get_file(&format!("{}.txt", args.name)).unwrap().contents_utf8().unwrap();

    use std::time::Instant;
    let now = Instant::now();

    match variant {
        Variant::Default => unsafe {
            default::exec(src, true)
        },

        Variant::DefaultSSE => unsafe {
            default_sse::exec(src, true)
        },

        Variant::Relaxed => unsafe {
            relaxed::exec(&mut util::parse::parse(src), true)
        },

        Variant::RelaxedSSE => unsafe {
            let (mut data, trailing_zeroes) = util::parse::parse_sse(src);
            relaxed_sse::exec(&mut data, trailing_zeroes, true)
        },

        Variant::RelaxedAVX2 => unsafe {
            let (mut data, trailing_zeroes) = util::parse::parse_avx2(src);
            relaxed_avx2::exec(&mut data, trailing_zeroes, true)
        },

        Variant::IdealAVX2 => unsafe {
            let (mut data, trailing_zeroes) = util::parse::parse_avx2_ideal(src);
            ideal_avx2::exec(&mut data, trailing_zeroes, true)
        }
    };

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}