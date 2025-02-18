use std::str::Chars;

use crate::QueryBox;

fn from_str(string: &str) -> QueryBox {
    todo!()
}

struct QueryConstructor();



fn end_step(depth: u32) {
    if depth != 0 { panic!() }
    return;
}

fn value_step_entrance(
    output: QueryConstructor,
    mut iterator: Chars, depth: u32, parent_not: bool,
) {
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
        // group
        '(' => { 
            let group = value_step_entrance(output, iterator, depth + 1, not);
            todo!();
            return value_step_entrance(output, iterator, depth, parent_not);
        },

        // string
        '\"' => return string_value_iterator(output, iterator, depth, not),

        // numerical
        '-' => todo!(), 
        '0' => todo!(), // fraction
        'e' => todo!(), // exponent
        'E' => todo!(), // exponent
        // digits 1-9
        '1' => todo!(),
        '2' => todo!(),
        '3' => todo!(),
        '4' => todo!(),
        '5' => todo!(),
        '6' => todo!(),
        '7' => todo!(),
        '8' => todo!(),
        '9' => todo!(),

        _ => panic!(),
    }
}

fn string_value_iterator(
    output: QueryConstructor,
    mut iterator: Chars, depth: u32, parent_not: bool
) {
    let token = match iterator.next() {
        Some(val) => val,
        None => panic!(),
    };

    match token {
        '\"' => return value_step_exit(output, iterator, depth, parent_not), // exit
        '\\' => todo!(),
        _ => todo!(), // continue
    }
}

fn escape_step(
    output: QueryConstructor,
    mut iterator: Chars, depth: u32, parent_not: bool
) {
    let token = match iterator.next() {
        Some(val) => val,
        None => panic!(),
    };

    match token {
        '\"' => todo!(), // quotation mark
        '\\' => todo!(), // reverse solidus
        '/' => todo!(), // solidus
        'b' => todo!(), // backspace
        'f' => todo!(), // formfeed
        'n' => todo!(), // linefeed
        'r' => todo!(), // carriage return
        't' => todo!(), // horizontal tab
        'u' => todo!(), // 4 hex digits
        _ => panic!(),
    }
}

fn hex4_step(
    mut output: QueryConstructor,
    mut iterator: Chars, depth: u32, parent_not: bool
) {
    hex_step(&mut output, &mut iterator);
    hex_step(&mut output, &mut iterator);
    hex_step(&mut output, &mut iterator);
    hex_step(&mut output, &mut iterator);
}

fn hex_step(
    output: &mut QueryConstructor,
    iterator: &mut Chars
) {
    let hex = match iterator.next() {
        Some(val) => val,
        None => panic!(),
    };

    match hex {
        '0' => todo!(),
        '1' => todo!(),
        '2' => todo!(),
        '3' => todo!(),
        '4' => todo!(),
        '5' => todo!(),
        '6' => todo!(),
        '7' => todo!(),
        '8' => todo!(),
        '9' => todo!(),
        'A' => todo!(),
        'B' => todo!(),
        'C' => todo!(),
        'D' => todo!(),
        'E' => todo!(),
        'F' => todo!(),
        _ => panic!(),
    }
}

// Expect operator or group end
fn value_step_exit(
    output: QueryConstructor,
    mut iterator: Chars, depth: u32, parent_not: bool
) {
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

    return value_step_entrance(output, iterator, depth, parent_not);
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