# SIMD SADS (sums of absolute differences)

Exploring performance across 3 problem variations with solutions implemented
without explicit vectorization, with SSE intrinsics, or AVX2 intrinsics.

**Important:** References to "non-vectorized" solutions means that the solutions are not **explicitly** vectorized,
the compiler will still implicitly utilize vectorization. TODO: more detail

## Problems

**Default** ([Advent of Code 2021 Day 7 Part 1](https://adventofcode.com/2021/day/7)):\
Given an input string containing a comma-separated
list of integers in the range `[0,9999]`, return the SADS (sum of absolute differences)
of every number in the input relative to the rounded median of the input values\

Input parsing: done inside solution function\
\

Implemented Solutions: SSE (`sol::default_sse`), non-vectorized (`sol::default`)\

**Relaxed (variant of Default):**

Sum of absolute differences between each element in input and first element in input
input value range: `[0,9999]`

Input parsing: is done outside solution function\
\

Implemented Solutions: AVX2 (`sol::relaxed_avx2`), SSE (`sol::relaxed_sse`), non-vectorized (`sol::relaxed`)

**Ideal (variant of Relaxed):**

Sum of absolute differences between each element in input and first element in input
input value range: `[0,255]`

Input parsing: done outside solution function

Implemented Solutions: AVX2 (`sol::ideal_avx2`, `sol::relaxed_avx2`), SSE (`sol::relaxed_sse`), non-vectorized (`sol::relaxed`)


## Benchmarks:
Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz

### Default
**Note:** The Default implementations for both non-vectorized and SSE solutions are naïve approaches to the problem and not intended to be a meaningful comparison of the differences between non-vectorized implementations and implementations explicitly using intrinsics.

In practice: parsing and calculating the median of input data dominates the running time for both solutions.

See either the Relaxed or Ideal variants of the Default problem for 

(1x input = 1000 integers `[0,9999]`)
| Input  | Time (Non-Vectorized) | Time (SSE) |
| ------ | --------------------- | ---------- |
| 1x     | 27.907 µs             | **18.675 µs**  |
| 10x    | 393.44 µs             | **357.93 µs**  |
| 100x   | 3.8568 ms             | **2.8620 ms**  |
| 1000x  | **31.098 ms**             | 31.613 ms  |
| 10000x | 372.94 ms             | **297.85 ms**           |


### Relaxed
(1x input = 1000 integers `[0,9999]`)
| Input  | Time (Non-Vectorized) | Time (SSE) | (Time AVX2) |
| ------ | --------------------- | ---------- | ----------- |
| 1x     | 352.56 ns             | 509.63 ns  | **45.695 ns**   |
| 10x    | 3.3628 µs             | 2.2539 µs  | **364.08 ns**   |
| 100x   | 38.941 µs             | 22.322 µs  | **4.4926 µs**   |
| 1000x  | 350.88 µs             | 239.62 µs  | **53.690 µs**   |
| 10000x | 4.4733 ms             | 3.7222 ms  | **1.3169 ms**            |


### Ideal
(1x input = 1000 integers `[0,256]`)
'AVX2' refers to the general AVX2 solution for the Relaxed problem
'AVX2 optimized' refers to a solution which exploits the constrained value range of the Ideal problem
| Input  | Time (AVX2) | Time (AVX2 optimized) |
| ------ | ----------- | --------------------- |
| 1x     | 40.953 ns   |  **24.813 ns**                     |
| 10x    | 356.96 ns   |  **170.89 ns**                     |
| 100x   | 4.5710 µs   |  **1.6840 µs**                     |
| 1000x  | 53.114 µs   |  **21.374 µs**                     |
| 10000x | 1.2645 ms            |  **444.64 µs**                     |



## Usage:

1. Clone this Repository.
2. `cd $CLONE_DIR/`
3. `cargo run --release -- -v ideal_avx2 -n 1000x`

## Arguments:
    -h, --help                 Print help information
    -n, --name <NAME>          Input file name: "input/{NAME}.txt" [default: ]
    -v, --variant <VARIANT>    Variant: ["default", "default_sse", "relaxed", "relaxed_sse",
                               "relaxed_avx2", "ideal_avx2"] [default: ]

### Benchmarking:

1. `cargo bench -- --quiet`

### Tests:

1. `cargo test`

