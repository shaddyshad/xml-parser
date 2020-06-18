# XML-parser library for rust 

An xml parser library for rust.

## Getting started 
Add xml parser in your `Cargo.toml` file 
````
    [dependencies]
    xml-parser = {git = "https://github.com/shaddyshad/xml-parser.git"}
````

## Usage
**`crate::from_file(filepath.xml)`** - parses a given xml file by filepath

Returns a tokenizer instance that contains the output  
````
[main.rs]

extern crate xml_parser;


use xml_parser::from_file

fn main(){
    let fp = "/home/user/path/upload.xml;

    let res = from_file(fp).unwrap();

    let tree = res.sink().output();   // contains parsed rcDOM
}
````