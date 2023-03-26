use crate::maze::stack::Stack;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

trait MazeCell<T> {
    fn neighbors(&self) -> Vec<&mut T>;
    fn num_links(&self) -> i32;
    fn link(&mut self, other: &T);
}

pub fn backtracker<T: MazeCell<T>>(start: &mut T, rng: &mut ChaCha8Rng) {
    let mut stack = Stack::new();
    stack.push(start);

    loop {
        let current = match stack.peek() {
            Some(v) => v,
            None => return,
        };
        let unlinked_neighbors: Vec<&mut T> = current
            .neighbors()
            .into_iter()
            .filter(|cell| cell.num_links() == 0)
            .collect();

        if unlinked_neighbors.len() == 0 {
            stack.pop();
            continue;
        }

        let index = rng.gen_range(0..unlinked_neighbors.len());
        let neighbor = unlinked_neighbors[index];
        current.link(neighbor);
        neighbor.link(current);
        stack.push(neighbor);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_backtracker() {
        // TODO: Create cells for testing.
        assert_eq!(false, true);
    }
}
