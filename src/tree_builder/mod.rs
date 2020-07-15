use super::{Sink, tokenizer::{Token, SinkResult, RawToken}};
use std::borrow::Cow::{self};
use super::{BufferQueue, TokenKind};
mod node;
mod stack;

pub use node::{Node, Handle};
use stack::Stack;


/// XML tree builder
#[derive(Debug)]
pub struct TreeBuilder {
    errors: Vec<Cow<'static, str>>,
    line_number: u32,
    token_buffer: BufferQueue,
    processing_stack: Stack<Handle>,
    currently_processing: Option<RawToken>,
    reconsume: bool,
    tree: Option<Handle>
}


impl TreeBuilder {
    /// create a new tree builder 
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            line_number: 0,
            token_buffer: BufferQueue::new(),
            processing_stack: Stack::new(),
            currently_processing: None,
            reconsume: false,
            tree: None
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
        if self.processing_stack.is_empty() && self.tree.is_some(){
            if !self.token_buffer.is_empty(){
                self.token_buffer.clear() ;

                return;
            }   
        }

        self.next();

        if let Some(ref tok) = self.currently_processing{
            // process the token 
            if let TokenKind::Opening = tok.kind {
                // create and push a node on top of the stack
                let t = self.get_handle(tok.clone());
                // if the tag is self closing, then create and insert it 
                if tok.self_closing {
                    self.add_to_previous(t);
                }else{
                    self.processing_stack.push(t);
                }
                
            }else{
                // finish processing and pop from the top of the stack
                self.finish_processing(tok.clone())
            }
        }
    }

    /// Complete the processing of a token
    fn finish_processing(&mut self, token: RawToken){
        let t = self.processing_stack.pop().unwrap();

        if token.value.is_some(){
            let v = token.value.unwrap();

            // add it ot tokens  t 
            t.data.borrow_mut().set_value(v);
        }

        if self.processing_stack.is_empty(){
            // set the tree 
            self.tree = Some(t);
        }else{
            // append to previous 
            self.add_to_previous(t);
        }
       
    }

    /// get the output 
    pub fn output(&self) -> Option<&Handle>{
        if let Some(ref handle) = self.tree {
            return Some(handle)
        }

        None
    }


    /// Retrieve a handle to add to the stack
    pub fn get_handle(&self, data: RawToken) -> Handle {
        Node::new(data)
    }

    /// Get the next token to process 
    fn next(&mut self){
        if self.reconsume {
            self.reconsume = false;
            return;
        }

        let n = self.token_buffer.next();

        self.currently_processing = n;

        if self.processing_stack.is_empty(){
            self.tree = self.processing_stack.pop();
        }
    }

    /// Add a node to the node on the top of stack 
    fn add_to_previous(&mut self, child: Handle){
        let previous = self.processing_stack.top().unwrap();

        Node::append_node(previous, child);
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

        if self.tree.is_none(){
            let head = self.processing_stack.pop();
            // 
            self.tree = head;
        }

        


    }
    
}