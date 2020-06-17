use super::{Sink, tokenizer::{Token, SinkResult, RawToken}};
use std::borrow::Cow::{self, Borrowed};
use super::BufferQueue;


/// XML tree builder
pub struct TreeBuilder {
    errors: Vec<Cow<'static, str>>,
    line_number: u32,
    token_buffer: BufferQueue
}

impl TreeBuilder {
    /// create a new tree builder 
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            line_number: 0,
            token_buffer: BufferQueue::new()
        }
    }

    /// Process an error 
    fn process_error(&mut self, err: Cow<'static, str>){
        self.errors.push(err);
    }

    /// Add a token to the rcDOM tree
    fn process_and_complete(&mut self, token: RawToken){
        // insert into the buffer and run the step command
        self.token_buffer.push_back(token);

        self.build()
    }

    /// Build a treee using token buffer provided
    fn build(&mut self){

    }
}


impl Sink for TreeBuilder {
    /// Process a single token
    fn process(&mut self, token: Token, line_number: u32) -> SinkResult{
        self.line_number = line_number;

        // check for an error token
        match token {
            Token::Error(err) => self.process_error(err),
            Token::Token(tok) => {
                // add the token to the tree 
                self.process_and_complete(tok);

            }
        }

        SinkResult::Continue
    }

    /// Complete the processing
    fn end(&mut self){
        // loop untill nothing in the buffer
        loop {
            if self.token_buffer.is_empty(){
                break;
            }

            self.build();
        }
    }
    
}