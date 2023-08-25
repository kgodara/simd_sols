use criterion::measurement::Measurement;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkGroup};

use pprof::criterion::{PProfProfiler, Output};

use include_dir::{include_dir, Dir};

use simd_sads::util::parse::{ parse, parse_sse, parse_avx2, parse_avx2_ideal };

use simd_sads::sol;

#[macro_use]
extern crate lazy_static;

static IN_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/input");

static SIZE_IDX_PAIRS: [(u64, usize);5] = [
    (1,0),
    (10,1),
    (100,2),
    (1000,3),
    (10000,4),
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

/// Solution types
enum ExecModel {
    NonSIMD,
    SSE,
    AVX2,
    AVX2Ideal,
}


fn run_default_bench<M>(g: &mut BenchmarkGroup<M>, size_idx: (u64, usize), model: ExecModel)
    where M: Measurement
{

    let model_str_lookup = |m: &ExecModel| {
        match m {
            ExecModel::NonSIMD => "",
            ExecModel::SSE => "sse_",
            ExecModel::AVX2 => "avx2_",
            ExecModel::AVX2Ideal => "avx2_ideal_",
        }
    };

    let function_id = &format!("{}{}x", model_str_lookup(&model), size_idx.0);

    match model {
        ExecModel::NonSIMD => {
            g.bench_function(function_id, |b| {
                b.iter(|| black_box(unsafe { sol::default::exec(SRC_LIST[size_idx.1], false) }))
            });
        },
        ExecModel::SSE => {
            g.bench_function(function_id, |b| {
                b.iter(|| black_box(unsafe { sol::default_sse::exec(SRC_LIST[size_idx.1], false) }))
            });
        },
        _ => {panic!("AVX2 not implemented for the default problem")}
    };
}

fn run_relaxed_bench<M>(g: &mut BenchmarkGroup<M>, size_idx: (u64, usize), model: ExecModel)
where M: Measurement
{

    let model_str_lookup = |m: &ExecModel| {
        match m {
            ExecModel::NonSIMD => "",
            ExecModel::SSE => "sse_",
            ExecModel::AVX2 => "avx2_",
            ExecModel::AVX2Ideal => "avx2_ideal_",
        }
    };

    let function_id = &format!("{}{}x", model_str_lookup(&model), size_idx.0);

    match model {
        ExecModel::NonSIMD => {
            g.bench_function(function_id, |b| {
                let parsed = parse(SRC_LIST[size_idx.1]);
                b.iter(|| black_box(unsafe { sol::relaxed::exec(&parsed, false) }))
            });
        },
        ExecModel::SSE => {
            g.bench_function(function_id, |b| {
                let (parsed, trailing_zeroes) = unsafe { parse_sse(SRC_LIST[size_idx.1]) };
                b.iter(|| black_box(unsafe { sol::relaxed_sse::exec(&parsed, trailing_zeroes, false) }))
            });
        },
        ExecModel::AVX2 => {
            g.bench_function(function_id, |b| {
                let (parsed, trailing_zeroes) = unsafe { parse_avx2(SRC_LIST[size_idx.1]) };
                b.iter(|| black_box(unsafe { sol::relaxed_avx2::exec(&parsed, trailing_zeroes, false) }))
            });
        },
        _ => {panic!("AVX2Ideal not implemented for the relaxed problem")}
    }
}

fn run_ideal_bench<M>(g: &mut BenchmarkGroup<M>, size_idx: (u64, usize), model: ExecModel)
where M: Measurement
{

    let model_str_lookup = |m: &ExecModel| {
        match m {
            ExecModel::NonSIMD => "",
            ExecModel::SSE => "sse_",
            ExecModel::AVX2 => "avx2_",
            ExecModel::AVX2Ideal => "avx2_ideal_",
        }
    };

    let function_id = &format!("{}{}x", model_str_lookup(&model), size_idx.0);

    match model {
        ExecModel::AVX2 => {
            g.bench_function(function_id, |b| {
                let (parsed, trailing_zeroes) = unsafe { parse_avx2(SRC_LIST[size_idx.1]) };
                b.iter(|| black_box(unsafe { sol::relaxed_avx2::exec(&parsed, trailing_zeroes, false) }))
            });
        },
        ExecModel::AVX2Ideal => {
            g.bench_function(function_id, |b| {
                let (parsed, trailing_zeroes) = unsafe { parse_avx2_ideal(SRC_LIST[size_idx.1]) };
                b.iter(|| black_box(unsafe { sol::ideal_avx2::exec(&parsed, trailing_zeroes, false) }))
            });
        },
        _ => {panic!("NonSIMD, SSE not implemented for the ideal problem")}
    }
}



fn run_default_bench_set(c: &mut Criterion) {
    let mut g = c.benchmark_group("default");

    run_default_bench(&mut g, SIZE_IDX_PAIRS[0], ExecModel::NonSIMD);
    run_default_bench(&mut g, SIZE_IDX_PAIRS[1], ExecModel::NonSIMD);
    run_default_bench(&mut g, SIZE_IDX_PAIRS[2], ExecModel::NonSIMD);
    run_default_bench(&mut g, SIZE_IDX_PAIRS[3], ExecModel::NonSIMD);
    run_default_bench(&mut g, SIZE_IDX_PAIRS[4], ExecModel::NonSIMD);


    run_default_bench(&mut g, SIZE_IDX_PAIRS[0], ExecModel::SSE);
    run_default_bench(&mut g, SIZE_IDX_PAIRS[1], ExecModel::SSE);
    run_default_bench(&mut g, SIZE_IDX_PAIRS[2], ExecModel::SSE);
    run_default_bench(&mut g, SIZE_IDX_PAIRS[3], ExecModel::SSE);
    run_default_bench(&mut g, SIZE_IDX_PAIRS[4], ExecModel::SSE);

}

fn run_relaxed_bench_set(c: &mut Criterion) {

    let mut g = c.benchmark_group("relaxed");

    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[0], ExecModel::NonSIMD);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[1], ExecModel::NonSIMD);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[2], ExecModel::NonSIMD);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[3], ExecModel::NonSIMD);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[4], ExecModel::NonSIMD);


    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[0], ExecModel::SSE);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[1], ExecModel::SSE);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[2], ExecModel::SSE);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[3], ExecModel::SSE);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[4], ExecModel::SSE);


    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[0], ExecModel::AVX2);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[1], ExecModel::AVX2);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[2], ExecModel::AVX2);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[3], ExecModel::AVX2);
    run_relaxed_bench(&mut g, SIZE_IDX_PAIRS[4], ExecModel::AVX2);

}

fn run_ideal_bench_set(c: &mut Criterion) {

    let mut g = c.benchmark_group("ideal");

    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[0], ExecModel::AVX2);
    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[1], ExecModel::AVX2);
    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[2], ExecModel::AVX2);
    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[3], ExecModel::AVX2);
    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[4], ExecModel::AVX2);


    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[0], ExecModel::AVX2Ideal);
    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[1], ExecModel::AVX2Ideal);
    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[2], ExecModel::AVX2Ideal);
    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[3], ExecModel::AVX2Ideal);
    run_ideal_bench(&mut g, SIZE_IDX_PAIRS[4], ExecModel::AVX2Ideal);

}


criterion_group!{
    name = default_benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(997, Output::Flamegraph(None)));
    targets = run_default_bench_set
}

criterion_group!{
    name = relaxed_benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(997, Output::Flamegraph(None)));
    targets = run_relaxed_bench_set
}

criterion_group!{
    name = ideal_benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(997, Output::Flamegraph(None)));
    targets = run_ideal_bench_set
}


criterion_main!(default_benches, relaxed_benches, ideal_benches);