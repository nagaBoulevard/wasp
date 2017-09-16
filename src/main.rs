#![feature(slice_patterns)]

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


mod lexer;
mod parser;
mod core;
use core::Core;
use lexer::Lexer;
use parser::Parser;


fn main() {

    /// read the input wasp file, and put it in the `input` string
    let input_handle = File::open("../input/test.ws").expect("Fail to open ../input/test.lp");
    let mut buf_reader = BufReader::new(input_handle);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).expect("Fail to read buffer data to string");
    // TODO exit if the input is empty
    println!["---------------------"];

    println!["{}", input];

    println!["---------------------"]; // XXX seems to be needed due to IO flushing problems


    let core = Core::new(input);
    let mut lexer = Lexer::new(&core);
    lexer.main_parser();

    let mut parser = Parser::new(&core, &lexer);
    parser.parse();

    Core::eval_all(&parser);
}
