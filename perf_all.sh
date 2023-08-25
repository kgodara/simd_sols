#! /bin/bash

export RUSTFLAGS="-C opt-level=3 -C no-vectorize-loops"; cargo build --release > perf_out.txt

cargo bench default/10000x -- --quiet >> perf_out.txt
perf stat --append -o perf_out.txt cargo bench default/10000x -- --profile-time 10 --quiet

export RUSTFLAGS="-C opt-level=3"; cargo build --release

cargo bench default/10000x -- --quiet >> perf_out.txt
perf stat --append -o perf_out.txt cargo bench default/10000x -- --profile-time 10 --quiet

cargo bench default/sse_10000x -- --quiet >> perf_out.txt
perf stat --append -o perf_out.txt  cargo bench default/sse_10000x -- --profile-time 10 --quiet


cargo bench relaxed/10000x -- --quiet >> perf_out.txt
perf stat --append -o perf_out.txt  cargo bench relaxed/10000x -- --profile-time 10 --quiet

cargo bench relaxed/sse_10000x -- --quiet >> perf_out.txt
perf stat --append -o perf_out.txt  cargo bench relaxed/sse_10000x -- --profile-time 10 --quiet

cargo bench relaxed/avx2_10000x -- --quiet >> perf_out.txt
perf stat --append -o perf_out.txt  cargo bench relaxed/avx2_10000x -- --profile-time 10 --quiet


cargo bench ideal/avx2_10000x -- --quiet >> perf_out.txt
perf stat --append -o perf_out.txt  cargo bench ideal/avx2_10000x -- --profile-time 10 --quiet

cargo bench ideal/avx2_ideal_10000x -- --quiet >> perf_out.txt
perf stat --append -o perf_out.txt  cargo bench default/avx2_ideal_10000x -- --profile-time 10 --quiet



