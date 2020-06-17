use std::collections::VecDeque;

/// Token processor stack structure
#[derive(Debug)]
pub struct Stack<T> {
    stack: VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            stack: VecDeque::new(),
        }
    }

    // push to the stack
    pub fn push(&mut self, item: T) {
        self.stack.push_front(item)
    }

    //pop from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop_front()
    }

    //check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    //check the size of stack
    pub fn size(&self) -> usize {
        self.stack.len()
    }

    //clear the stack
    pub fn clear(&mut self) {
        self.stack.clear();
        assert!(self.stack.is_empty());
    }

    //top of the stack
    pub fn top(&self) -> Option<&T> {
        self.stack.front()
    }

    //get the top mutable
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.stack.front_mut()
    }
}