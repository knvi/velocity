#![warn(clippy::pedantic)]

use velocity::Velocity;

pub mod velocity;

fn main() {
    let mut avg_time = 0.0;
    let mut velocity = Velocity::new(1);
    for j in 0..100 {
        let start = std::time::Instant::now();
        for _ in 0..1000000 {
            let v = velocity.gen();
            let _ = velocity.decode(v);
        }
        let end = std::time::Instant::now();
        let duration = end.duration_since(start).as_millis() as f64;
        avg_time += duration;
        println!("{}: {}ms", j, duration);
    }
    println!("Average: {}ms", avg_time / 100.0);
}