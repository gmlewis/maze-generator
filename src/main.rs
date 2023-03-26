//! maze-generator generates a variety of mazes in the browser
//! via WebAssembly.

use std::time::SystemTime;

use maze::Rectangular;
pub mod maze;

fn main() {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error");
    let seed = now.as_millis() as u64;
    let maze = Rectangular::build(20, 5, seed);
    println!("{maze}");
}
