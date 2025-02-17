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
        // r"..." raw strings like in Rust should be implemented along with escape sequences
        'r' => unimplemented!("Raw strings are unimplemented"), 
        // numeric
        '.' => unimplemented!("Numeric values are unimplemented"),
        '0' => unimplemented!("Numeric values are unimplemented"),
        '1' => unimplemented!("Numeric values are unimplemented"),
        '2' => unimplemented!("Numeric values are unimplemented"),
        '3' => unimplemented!("Numeric values are unimplemented"),
        '4' => unimplemented!("Numeric values are unimplemented"),
        '5' => unimplemented!("Numeric values are unimplemented"),
        '6' => unimplemented!("Numeric values are unimplemented"),
        '7' => unimplemented!("Numeric values are unimplemented"),
        '8' => unimplemented!("Numeric values are unimplemented"),
        '9' => unimplemented!("Numeric values are unimplemented"),
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
        // <https://crates.io/crates/unescape> Consider using this when implmenting escape sequences
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

// Value step entrance
// Valid char:
//  !
//  "
//  ( = +1 depth
// If empty 
//  Depth must equal 0
//  END

// Value step iterator
// IF '"' EXIT
// IF END ERROR
// REPEAT "Value step iterator"

// Value step exit
// IF ')', -1 depth, REPEAT value step entrance
// Operator step entrance
// Expect either: 
//  &&
//  ||
// EXIT

// Operator step exit
//  into value step entrance