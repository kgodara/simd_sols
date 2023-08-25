#![feature(unchecked_math)]

//! This crate provides implementations of non-vectorized and vectorized(x86 architectures only) solutions to a set of three problems.
//! # Problems:
//!
//! **Default** ([Advent of Code 2021 Day 7 Part 1](https://adventofcode.com/2021/day/7)):\
//! Given an input string containing a comma-separated
//! list of integers in the range `[0,9999]`, return the SADS (sum of absolute differences)
//! of every number in the input relative to the rounded median of the input values\
//!
//! Input parsing: done inside solution function (relevant for benchmarking)\
//! \
//!
//! Implemented Solutions: SSE ([`default_sse`](sol::default_sse)), non-vectorized ([`default`](sol::default))\
//!
//! **Relaxed (variant of Default):**
//!
//! Sum of absolute differences between each element in input and first element in input
//! input value range: `[0,9999]`
//!
//! Input parsing: is done outside solution function (relevant for benchmarking)\
//! \
//!
//! Implemented Solutions: AVX2 ([`relaxed_avx2`](sol::relaxed_avx2)), SSE ([`relaxed_sse`](sol::relaxed_sse)), non-vectorized ([`relaxed`](sol::relaxed))
//!
//! **Ideal (variant of Relaxed):**
//!
//! Sum of absolute differences between each element in input and first element in input
//! input value range: `[0,255]`
//!
//! Input parsing: done outside solution function (relevant for benchmarking)
//!
//! Implemented Solutions: AVX2 ([`ideal_avx2`](sol::ideal_avx2), [`relaxed_avx2`](sol::relaxed_avx2)), SSE ([`relaxed_sse`](sol::relaxed_sse)), non-vectorized ([`relaxed`](sol::relaxed))


pub mod util;
pub mod sol;