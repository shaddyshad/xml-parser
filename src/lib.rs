mod tokenizer;
mod tree_builder;

use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use tendril::StrTendril;
// Sink trait implemented by tree_builder 
pub use tokenizer::{Sink, Tokenizer};
pub use tree_builder::TreeBuilder;


/// Read a file and feed the tokenizer 
pub fn from_file(filepath: &str) -> Result<(), String>{
    // create a treee builder instance
    let tb = TreeBuilder::new();
    let mut tok = Tokenizer::new(tb);

    // open the file
    if let Ok(lines) = read_lines(filepath){
        // read the file line by line, feeding it to the tokenizer
        for line in lines {
            if let Ok(l) = line {
                let buf = StrTendril::from(l);

                // feed to the tokenizer
                // we need to check if the 
                let _ = tok.feed(buf);
            }
        }

        tok.end();
        return Ok(());
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