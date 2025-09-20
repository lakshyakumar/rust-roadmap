// 74. How do you implement an embedded-style fixed-point decimal type using an i64 backing?
// Implement arithmetic operations with scaling. Why is fixed-point arithmetic used in embedded systems?

// Why fixed-point in embedded?
// No FPU: integer ops are much cheaper / sometimes the only option.
// Determinism: exact control over scale and rounding; predictable behavior for real-time systems.
// Smaller code & lower power: avoids floating-point library/code.
// Reproducibility: same results across platforms lacking IEEE FP variations.

// Design choices
// Backing: i64 raw stores value = real_value * SCALE.
// SCALE: const SCALE: i64 = 1_000_000 ⇒ 6 decimal places.
// Arithmetic uses i128 intermediates to avoid overflow during mul/div.
// Implement both checked_* and saturating_* strategies.
// Rounding: truncating toward zero by default; you can add rounding modes if needed.

fn main() {}
