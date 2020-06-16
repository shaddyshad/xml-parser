use std::default::Default;
 
/// Tracks the current state the tokenizer is in
/// Each state implies a different processing for the different tokens different tokens
#[derive(Debug, Eq, PartialEq)]
pub enum States {
    /// Initial and final processing state
    Document,
    /// An opening token
    OpeningToken,
    /// A closing token
    ClosingToken,
    /// name of the token
    TokenName,
    /// State before processing an attribute name
    BeforeAttributeName,
    /// Before processing an attribute value
    BeforeAttributeValue,
    /// Attribute value
    AttributeValue,
    /// Meta/ processing instructions for the tokenizer
    ProcessingInstruction,
    /// A sequence of characters encountered
    Characters
}

/// the default state 
impl Default for States {
    fn default() -> Self {
        Self:: Document
    }
}