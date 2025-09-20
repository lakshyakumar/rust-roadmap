// 76. How do you use SIMD(Single Instruction Multiple data) in Rust to sum f32 slices with SSE/AVX when available, and fallback to scalar code?
// Use cfg(target_feature) for conditional compilation. Why is SIMD important for performance?

// SIMD (Single Instruction, Multiple Data) lets the CPU process multiple values in parallel with one instruction.
// E.g. with AVX, you can add 8 f32 numbers at once instead of looping one by one.

// Benefits:
// Massive speedups in numerics (image/audio processing, ML inference, physics sims).
// Better cache use, fewer instructions, less branch overhead.
// Critical in high-performance Rust code (e.g., rayon, ndarray, tch-rs).

use core::arch::x86_64::*;

/// Fallback scalar sum
fn sum_scalar(slice: &[f32]) -> f32 {
    slice.iter().copied().sum()
}

/// AVX version: process 8 floats at a time
#[target_feature(enable = "avx")]
unsafe fn sum_avx(slice: &[f32]) -> f32 {
    let mut acc = _mm256_setzero_ps();
    let mut i = 0;
    while i + 8 <= slice.len() {
        let chunk = _mm256_loadu_ps(slice.as_ptr().add(i));
        acc = _mm256_add_ps(acc, chunk);
        i += 8;
    }
    // Horizontal add: reduce 8 lanes to 1
    let mut tmp = [0.0f32; 8];
    _mm256_storeu_ps(tmp.as_mut_ptr(), acc);
    let mut total: f32 = tmp.iter().copied().sum();
    // leftover
    total += slice[i..].iter().copied().sum::<f32>();
    total
}

/// SSE version: process 4 floats at a time
#[target_feature(enable = "sse")]
unsafe fn sum_sse(slice: &[f32]) -> f32 {
    let mut acc = _mm_setzero_ps();
    let mut i = 0;
    while i + 4 <= slice.len() {
        let chunk = _mm_loadu_ps(slice.as_ptr().add(i));
        acc = _mm_add_ps(acc, chunk);
        i += 4;
    }
    // Horizontal add 4 lanes
    let mut tmp = [0.0f32; 4];
    _mm_storeu_ps(tmp.as_mut_ptr(), acc);
    let mut total: f32 = tmp.iter().copied().sum();
    total += slice[i..].iter().copied().sum::<f32>();
    total
}

/// Public wrapper: chooses best available at runtime
pub fn sum_f32(slice: &[f32]) -> f32 {
    // Runtime detection (works on stable Rust)
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx") {
            unsafe {
                return sum_avx(slice);
            }
        } else if is_x86_feature_detected!("sse") {
            unsafe {
                return sum_sse(slice);
            }
        }
    }
    sum_scalar(slice)
}

fn main() {
    let data: Vec<f32> = (0..1_000).map(|x| x as f32).collect();
    let sum = sum_f32(&data);
    println!("Sum = {}", sum);
}
