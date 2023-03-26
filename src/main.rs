//! maze-generator generates a variety of mazes in the browser
//! via WebAssembly.

use maze::Rectangular;
pub mod maze;

fn main() {
    let maze = Rectangular::new(20, 5, 333);
    println!("{maze}");
}
