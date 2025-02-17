use std::str::Chars;

use crate::QueryBox;


fn from_str(string: &str) -> QueryBox {
    todo!()
}

fn value_step_entrance(mut iterator: Chars) {
    let token = match iterator.next() {
        Some(val) => val,
        None => panic!(),
    };

    match token {
        '!' => todo!(),
        '\"' => todo!(),
        '(' => todo!(),
        '.' => unimplemented!(),
        '0' => unimplemented!(),
        '1' => unimplemented!(),
        '2' => unimplemented!(),
        '3' => unimplemented!(),
        '4' => unimplemented!(),
        '5' => unimplemented!(),
        '6' => unimplemented!(),
        '7' => unimplemented!(),
        '8' => unimplemented!(),
        '9' => unimplemented!(),
        _ => panic!(),
    }
}

fn value_step_iterator(mut iterator: Chars) {
    let token = match iterator.next() {
        Some(val) => val,
        None => panic!(),
    };

    match token {
        '\"' => todo!(), // exit
        '\\' => unimplemented!("Escape sequences are unimplemented"),
        _ => todo!(), // continue
    }
}

fn operator_step(mut iterator: Chars) {
    let token1 = match iterator.next() {
        Some(val) => val,
        None => todo!(), // iterate ansi parser (expect end of iterator)
    };

    let token2 = match iterator.next() {
        Some(val) => val,
        None => todo!(), // iterate ansi parser (expect end of iterator)
    };

    // hack
    // Currently there aren't any multi-token patterns, but it is still expected that they come in pairs.
    if token1 != token2 {
        panic!()
    }

    match token1 {
        '&' => todo!(), // AND
        '|' => todo!(), // OR
        _ => panic!(),
    }
    
    // loop back to text step (that step needs to accept an char iterator instead of str)
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
// IF END: Iterate ansi parser
//  TextBlock: error
//  Escape: continue
// Iterate ansi parser until TextBlock appears
// REPEAT "Value step iterator"

// Value step exit
//  Expect operator

// Operator step entrance
// Expect either: 
//  &&
//  ||
// EXIT

// Operator step exit
//  REPEAT from "Iterate ansi parser"