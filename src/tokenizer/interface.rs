use std::collections::HashMap;
use std::borrow::Cow;
use tendril::StrTendril;



/// types that can be used to process tokens from tokenizer
pub trait Sink {
    /// process a single token
    fn process(&mut self, token: Token, line_number: u32) -> SinkResult; 

    /// Complete the processing
    fn end(&mut self);
}

/// Holds the result of a sink process
#[derive(Debug, Eq, PartialEq)]
pub enum SinkResult {
    Continue,
    Break
}



/// A single token emitted by the tokenizer
#[derive(Debug)]
pub enum Token {
    Error(Cow<'static, str>),
    Token(RawToken)
}

/// A single raw token emitted 
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RawToken {
    pub kind: TokenKind,
    pub name: TokenName,
    pub attributes: HashMap<StrTendril, StrTendril>,
    pub self_closing: bool,
    pub value: Option<StrTendril> 
}

/// Token kind, can be an opening or closing token
#[derive(Debug, Eq, PartialEq, Clone, Copy )]
pub enum TokenKind {
    Opening,
    Closing 
}

/// Qualified token name 
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TokenName (pub StrTendril);
