use std::str::Chars;

use crate::QueryBox;

fn from_str(string: &str) -> QueryBox {
    todo!()
}

fn end_step(depth: u32) {
    if depth != 0 { panic!() }
    return;
}

fn value_step_entrance(mut iterator: Chars, depth: u32, parent_not: bool) {
    let token = match iterator.next() {
        Some(val) => val,
        None => return end_step(depth),
    };

    let (token, not) = {
        if token == '!' {
            match iterator.next() {
                Some(val) => (val, true),
                None => panic!(),
            }
        } else {
            (token, parent_not)
        }
    };

    match token {
        '\"' => return value_step_iterator(iterator, depth, not),
        '(' => {
            let group = value_step_entrance(iterator, depth + 1, not);
            todo!();
            return value_step_entrance(iterator, depth, parent_not);
        },

        // r"..." raw strings like in Rust should be implemented along with escape sequences, "eventually"
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

fn value_step_iterator(mut iterator: Chars, depth: u32, parent_not: bool) {
    let token = match iterator.next() {
        Some(val) => val,
        None => panic!(),
    };

    match token {
        '\"' => return value_step_exit(iterator, depth, parent_not), // exit

        // <https://crates.io/crates/unescape> Consider using this crate when implmenting escape sequences
        '\\' => unimplemented!("Escape sequences are unimplemented"),
        _ => todo!(), // continue
    }
}

// Expect operator or group end
fn value_step_exit(mut iterator: Chars, depth: u32, parent_not: bool) {
    let token1 = match iterator.next() {
        Some(val) => val,
        None => return end_step(depth),
    };

    match token1 {
        ')' => { return; }, 
        '&' => {/* Continue */}, // AND
        '|' => {/* Continue */}, // OR
        _ => panic!(),
    }

    let token2 = match iterator.next() {
        Some(val) => val,
        None => panic!(),
    };

    // Hacky: Currently there aren't any multi-token patterns, but it is still expected that they come in pairs.
    if token1 != token2 {
        panic!()
    }

    return value_step_entrance(iterator, depth, parent_not);
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