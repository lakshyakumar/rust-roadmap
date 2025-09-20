// 45. How do you create zero-cost newtypes for units like Meters(f64) and Seconds(f64)?
// Implement Div to yield Speed and prevent mixing using a PhantomData marker. How does this pattern prevent unit errors?

// If you just use f64 everywhere:
// let distance: f64 = 100.0; // meters
// let time: f64 = 9.58;      // seconds
// let speed = distance / time; // ok, but it's still just f64
// ⚠️ Nothing stops you from doing time / distance, which makes no physical sense.
// We want type safety → make the units part of the type system.

use std::marker::PhantomData;
use std::ops::Div;

//
// -------- Approach 1: Newtypes --------
//

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Seconds(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Speed(f64); // Meters per Second

// Define only valid division: Meters / Seconds = Speed
impl Div<Seconds> for Meters {
    type Output = Speed;

    fn div(self, rhs: Seconds) -> Self::Output {
        Speed(self.0 / rhs.0)
    }
}

//
// -------- Approach 2: PhantomData + Generics --------
//

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Quantity<Unit> {
    value: f64,
    _marker: PhantomData<Unit>,
}

// Unit marker types
#[derive(Debug)]
struct Meter;
#[derive(Debug)]
struct Second;
#[derive(Debug)]
struct SpeedUnit; // Meter / Second

// Division: Quantity<Meter> / Quantity<Second> = Quantity<SpeedUnit>
impl Div<Quantity<Second>> for Quantity<Meter> {
    type Output = Quantity<SpeedUnit>;

    fn div(self, rhs: Quantity<Second>) -> Self::Output {
        Quantity {
            value: self.value / rhs.value,
            _marker: PhantomData,
        }
    }
}

//
// -------- Main demonstrating both --------
//

fn main() {
    println!("--- Newtype Example ---");
    let distance = Meters(100.0);
    let time = Seconds(9.58);
    let speed = distance / time; // Meters / Seconds = Speed
    println!("Speed (newtype) = {:?} m/s", speed);

    println!("\n--- PhantomData Example ---");
    let dist_q = Quantity::<Meter> {
        value: 100.0,
        _marker: PhantomData,
    };
    let time_q = Quantity::<Second> {
        value: 9.58,
        _marker: PhantomData,
    };
    let speed_q = dist_q / time_q; // Quantity<Meter> / Quantity<Second> = Quantity<SpeedUnit>
    println!("Speed (phantom) = {:?} m/s", speed_q.value);

    // ❌ Uncommenting these lines will cause compile errors
    // let wrong = time / distance;   // No Div<Seconds> for Seconds
    // let wrong_q = time_q / dist_q; // No Div<Quantity<Meter>> for Quantity<Second>
}
