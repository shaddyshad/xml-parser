use std::collections::VecDeque;
use super::{ RawToken};



/// A buffer ot tokenizer input that can be consumed incrementally
#[derive(Debug)]
pub struct BufferQueue {
    buffers: VecDeque<RawToken>
}


impl BufferQueue {
    /// Create an empty tokenizer input 
    pub fn new() -> Self {
        Self {
            buffers: VecDeque::with_capacity(16)   // with 16 items per consumption
        }
    }

    /// Push a buffer to the end of the buffer
    pub fn push_back(&mut self, tok: RawToken){
        self.buffers.push_back(tok)
    }

    /// check if we are out og buffers
    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    /// Get the length of remaining buffers
    pub fn len(&self) -> usize {
        self.buffers.len()
    }



    /// Retrieve the next character in the top of the buffer
    pub fn next(&mut self) -> Option<RawToken> {
        if self.is_empty(){
            return None;
        }
        
        
        self.buffers.pop_front()       
    }
}
