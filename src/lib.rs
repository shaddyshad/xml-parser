#[macro_use]
pub mod utils;

#[macro_use]
pub mod macros;


pub mod tokenizer;
pub mod tree_builder;

use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

pub use tendril::StrTendril;
// Sink trait implemented by tree_builder 
pub use tokenizer::{Sink, Tokenizer, RawToken, TokenKind};
pub use tree_builder::{TreeBuilder, Handle};
pub use utils::{SmallCharSet, BufferQueue};

// read some text
pub fn parse_text(input: String) -> Result<Tokenizer<TreeBuilder>, String> {
    // create tree builder instance
    let tb = TreeBuilder::new();
    let mut tok = Tokenizer::new(tb);       // tokenize 

    // split the content by lines 
    let buf = StrTendril::from(input);

    // feed and finish 
    let _ = tok.feed(buf);
    tok.end();
    

    return Ok(tok);
}

/// Read a file and feed the tokenizer 
pub fn from_file(filepath: &str) -> Result<Tokenizer<TreeBuilder>, String>{
    // create a treee builder instance
    let tb = TreeBuilder::new();
    let mut tok = Tokenizer::new(tb);

    // open the file
    if let Ok(lines) = read_lines(filepath){
        // read the file line by line, feeding it to the tokenizer
        for line in lines {
            if let Ok(l) = line {
                let buf = StrTendril::from(l.trim());

                // feed to the tokenizer
                // we need to check if the 
                let _ = tok.feed(buf);
            }
        }

        tok.end();
        return Ok(tok);
    }

    Err(format!("{:?} not found", filepath))
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
