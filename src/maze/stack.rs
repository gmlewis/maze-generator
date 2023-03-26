// Modified from: https://chercher.tech/rust/stack-rust
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn push(&mut self, item: T) -> bool {
        self.items.push(item);
        return true;
    }
    pub fn size(&self) -> usize {
        self.items.len()
    }
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

#[cfg(test)]

mod tests {
    #[test]
    fn test_new() {
        let stack = super::Stack::<u32>::new();
        assert_eq!(10, stack.items.capacity());
    }

    #[test]
    fn test_pop() {
        let mut stack = super::Stack::<u32>::new();
        stack.push(1u32);
        assert_eq!(Some(1u32), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_push() {
        let mut stack = super::Stack::<u32>::new();
        stack.push(32u32);
        assert_eq!(Some(&32u32), stack.peek());
        assert_eq!(1, stack.size());
    }

    #[test]
    fn test_size() {
        let mut stack = super::Stack::<u32>::new();
        assert_eq!(0, stack.size());
        stack.push(1u32);
        assert_eq!(1, stack.size());
    }
}
