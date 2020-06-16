use std::collections::VecDeque;
use tendril::StrTendril;
use super::SmallCharSet;



/// A buffer ot tokenizer input that can be consumed incrementally
#[derive(Debug)]
pub struct TokenizerInput {
    buffers: VecDeque<StrTendril>
}

/// used to hold the result of `pop_except_from()`passing an input character set
/// It identifies characters found in the input that belong to the set the user request
/// or alternatively a set of character mot within the set
#[derive(Debug, PartialEq, Eq)]
pub enum SetResult {
    FromSet(char),
    NotFromSet(StrTendril)
}


impl TokenizerInput {
    /// Create an empty tokenizer input 
    pub fn new() -> Self {
        Self {
            buffers: VecDeque::with_capacity(16)   // with 16 items per consumption
        }
    }

    /// Push a buffer to the end of the buffer
    pub fn push_back(&mut self, buf: StrTendril){
        if buf.len32() == 0 {
            return;
        }

        self.buffers.push_back(buf)
    }

    /// check if we are out og buffers
    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    /// Get the length of remaining buffers
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    /// Pops and return either a single character from a given set
    /// or a buffer of character not within the set
    pub fn pop_except_from(&mut self, set: SmallCharSet) -> Option<SetResult> {
        let (result, empty) = match self.buffers.front_mut(){
            None => (None, false),
            Some(buf) => {
                let n = set.nonmember_prefix_len(&buf);

                if n > 0 {
                    let out;

                    unsafe {
                        out = buf.unsafe_subtendril(0, n);
                        buf.unsafe_pop_front(n);
                    }

                    (Some(SetResult::NotFromSet(out)), buf.is_empty())
                }else{
                    let c = buf.pop_front_char().expect("empty buffer in the queue");
                    (Some(SetResult::FromSet(c)), buf.is_empty())
                }
            }
        };

        if empty {
            self.buffers.pop_front();
        }

        result

    }


    /// Retrieve the next character in the top of the buffer
    pub fn next(&mut self) -> Option<char> {
        let (result, empty) = match self.buffers.front_mut(){
            None => (None, false),
            Some(buf) => {
                let c = buf.pop_front_char().expect("empty buffer in the queue");
                (Some(c), buf.is_empty())
            }
        };

        if empty {
            self.buffers.pop_front();
        }

        result
    }
}
