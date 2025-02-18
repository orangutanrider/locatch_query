use std::str::CharIndices;

use crate::QueryBox;

fn from_str(string: &str) -> QueryBox {
    todo!()
}

struct QueryConstructor(Vec<u8>);

fn skip_whitespace(
    iterator: &mut CharIndices,
) -> Option<(usize, char)> {
    for (index, token) in iterator {
        if token.is_whitespace() {
            continue;
        }

        return Some((index, token));
    }

    return None;
}

fn end_step(depth: u32) {
    if depth != 0 { panic!() }
    return;
}

fn value_step_entrance(
    output: &mut QueryConstructor,
    iterator: &mut CharIndices, depth: u32, parent_not: bool,
) {
    let (_, token) = match skip_whitespace(iterator) {
        Some(val) => val,
        None => return end_step(depth),
    };

    let (token, not) = {
        if token == '!' {
            match iterator.next() {
                Some((_, val)) => (val, true),
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
            return operator_step(output, iterator, depth, parent_not);
        },

        // string
        '\"' => return string_value_entrance(output, iterator, depth, not),

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

fn string_value_entrance(
    output: &mut QueryConstructor,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) {
    let (index, token) = match iterator.next() {
        Some(val) => val,
        None => panic!(),
    };

    match token {
        '\"' => {
            string_value_to_output(output, iterator, depth, parent_not);
            return operator_step(output, iterator, depth, parent_not);
        },
        '\\' => return escaped_string_step(output, iterator, depth, parent_not),
        _ => return string_value_iterator(index,index, output, iterator, depth, parent_not), 
    }
}

fn string_value_iterator(
    i_origin: usize, i_trailing: usize,
    output: &mut QueryConstructor,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) {
    let (index, token) = match iterator.next() {
        Some(val) => val,
        None => panic!(),
    };

    match token {
        '\"' => { // exit
            string_value_to_output(output, iterator, depth, parent_not);
            return operator_step(output, iterator, depth, parent_not);
        }, 
        '\\' => { // escape
            string_value_to_output(output, iterator, depth, parent_not);
            return escaped_string_step(output, iterator, depth, parent_not);
        },
        _ => return string_value_iterator(i_origin, index, output, iterator, depth, parent_not), // continue
    }
}

fn escaped_string_step(
    output: &mut QueryConstructor,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
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

    todo!(); // construct and push escape value onto output

    return string_value_entrance(output, iterator, depth, parent_not);
}

fn hex4_step(
    output: &mut QueryConstructor,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) {
    //hex_step(&mut output, &mut iterator);
    //hex_step(&mut output, &mut iterator);
    //hex_step(&mut output, &mut iterator);
    //hex_step(&mut output, &mut iterator);
    todo!()
}

fn hex_step(
    output: &mut QueryConstructor,
    iterator: &mut CharIndices
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

// push collection to output
// continue to operator step
fn string_value_to_output(
    output: &mut QueryConstructor,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) {
    todo!()
}

// Expect operator or group end
fn operator_step(
    output: &mut QueryConstructor,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) {
    let token1 = match skip_whitespace(iterator) {
        Some(val) => val,
        None => return end_step(depth),
    };

    match token1 {
        ')' => { 
            todo!(); // push value to output
            todo!(); // depth reduction
            return; 
        }, 
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