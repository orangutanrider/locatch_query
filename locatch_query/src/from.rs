use crate::QueryBox;

use ansi_parser::*;

fn from_str(string: &str) -> QueryBox {
    todo!()
}

fn ansi_test() {
    let mut iterator = "\"foo\\nbar\"".ansi_parse();

    let token = iterator.next();

    let token = match token {
        Some(val) => val,
        None => todo!(),
    };
    
    match token {
        Output::TextBlock(_) => todo!(),
        Output::Escape(ansi_sequence) => todo!(),
    }
}

// Iterate ansi parser
//  TextBlock: continue
//  Escape: error

// Iterate TextBlock
//  Expect value or group

// Value step entrance
// Valid char:
//  !
//  "
//  numeric (eventually)
//  . (eventually, numeric)  

// Value step iterator
// IF '"' EXIT
// IF END ->
// On the next iteration of the ansi parser, expect an escape.
// REPEAT

// Value step exit
//  Expect operator

// Operator step entrance
// Expect either:
//  &&
//  ||
// EXIT

// Operator step exit
//  REPEAT from "Iterate ansi parser"