use maze::rectangular::Rectangular;
pub mod maze;

fn main() {
    let maze = Rectangular::new(20, 5, 333);
    println!("{maze}");
}
