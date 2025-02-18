use std::str::CharIndices;

use crate::{
    QueryBox,
    NOT_MASK,
    GROUP_END,
    AND,
    OR,
    STRING,
};

fn from_str(string: &str) -> QueryBox {
    todo!()
}

enum ReadError {
    Undefined
}

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

fn end_step(depth: u32) -> Result<(), ReadError> {
    if depth != 0 { return Err(ReadError::Undefined) }
    return Ok(());
}

fn value_step_entrance(
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32, parent_not: bool,
) -> Result<(), ReadError> {
    let (_, token) = match skip_whitespace(iterator) {
        Some(val) => val,
        None => return end_step(depth),
    };

    let (token, not) = {
        if token == '!' {
            match skip_whitespace(iterator) {
                Some((_, val)) => (val, true),
                None => return Err(ReadError::Undefined),
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

        _ => return Err(ReadError::Undefined),
    }
}

fn string_value_entrance(
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) -> Result<(), ReadError> {
    let (index, token) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
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
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) -> Result<(), ReadError> {
    let (index, token) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
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
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) -> Result<(), ReadError> {
    let (_, token) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
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
        _ => return Err(ReadError::Undefined),
    }

    todo!(); // construct and push escape value onto output

    return string_value_entrance(output, iterator, depth, parent_not);
}

fn hex4_step(
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) -> Result<(), ReadError> {
    //hex_step(&mut output, &mut iterator);
    //hex_step(&mut output, &mut iterator);
    //hex_step(&mut output, &mut iterator);
    //hex_step(&mut output, &mut iterator);
    todo!()
}

fn hex_step(
    output: &mut Vec<u8>,
    iterator: &mut CharIndices
) -> Result<(), ReadError> {
    let (_, hex) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
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
        _ => return Err(ReadError::Undefined),
    }
}

// push collection to output
// continue to operator step
fn string_value_to_output(
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) -> Result<(), ReadError> {
    todo!()
}

// Expect operator or group end
fn operator_step(
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32, parent_not: bool
) -> Result<(), ReadError> {
    let (_, token1) = match skip_whitespace(iterator) {
        Some(val) => val,
        None => return end_step(depth),
    };

    match token1 {
        ')' => { 
            output.push(GROUP_END);
            // depth reduction (implicit by returning)
            // return to operator step in higher group (implicit by returning)
            return Ok(()); 
        }, 
        '&' => { // AND
            output.push(AND);
            /* Continue */
        },
        '|' => { // OR
            output.push(OR)
            /* Continue */
        },
        _ => return Err(ReadError::Undefined),
    }

    let (_, token2) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
    };

    // Hacky: Currently there aren't any multi-token patterns, but it is still expected that they come in pairs.
    if token1 != token2 {
        return Err(ReadError::Undefined)
    }

    return value_step_entrance(output, iterator, depth, parent_not);
}