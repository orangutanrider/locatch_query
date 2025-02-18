use std::str::CharIndices;

use crate::{
    QueryBox, AND, GROUP, GROUP_END, NOT_BIT, NOT_MASK, OR, STRING
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
    iterator: &mut CharIndices, depth: u32
) -> Result<(), ReadError> {
    let (_, token) = match skip_whitespace(iterator) {
        Some(val) => val,
        None => return end_step(depth),
    };

    let mut output_type: u8 = 0;

    let token = {
        if token == '!' {
            match skip_whitespace(iterator) {
                Some((_, val)) => {
                    output_type = output_type | NOT_BIT;
                    val
                },
                None => return Err(ReadError::Undefined),
            }
        } else {
            token
        }
    };

    match token {
        // group
        '(' => { 
            output_type = output_type | GROUP;
            output.push(output_type);
            match value_step_entrance(output, iterator, depth + 1) {
                Ok(_) => return operator_step(output, iterator, depth),
                Err(err) => return Err(err),
            }
        },

        // string
        '\"' => {
            output_type = output_type | STRING;
            output.push(output_type);
            return string_value_step(output, iterator, depth);
        },

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

// returns the number of bits pushed
fn push_empty_usize(
    output: &mut Vec<u8>,
) {
    let mut bit_index: u32 = 0;
    while bit_index < usize::BITS {
        bit_index = bit_index + 8;
        output.push(0);
    }
}

fn set_usize(
    output: &mut Vec<u8>,
    mut index: usize,
    value: usize,
) {
    let bytes = value.to_be_bytes();

    for byte in bytes {
        output[index] = byte;
        index = index + 1;
    }
}

fn string_value_step(
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32
) -> Result<(), ReadError> {
    let (index, token) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
    };

    let usize_index = output.len();
    push_empty_usize(output);

    match token {
        '\"' => {
            set_usize(output, usize_index, 0);
            return operator_step(output, iterator, depth);
        },
        '\\' => match escaped_string_step(output, iterator, depth) {
            Ok(_) => {/* Continue */},
            Err(err) => return Err(err),
        },
        _ => match string_value_iterator(index,index, output, iterator, depth) {
            Ok(_) => {/* Continue */},
            Err(err) => return Err(err),
        }, 
    }

    let string_len = output.len() - usize_index;
    set_usize(output, usize_index, string_len);

    return operator_step(output, iterator, depth);
}

fn string_value_to_output(
    source: &str,
    i_origin: usize, i_trailing: usize,
    output: &mut Vec<u8>,
) {
    let value = source[i_origin..i_trailing].as_bytes();
    for byte in value {
        output.push(*byte);
    }
}

fn string_value_iterator(
    i_origin: usize, i_trailing: usize,
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32
) -> Result<(), ReadError> {
    let (index, token) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
    };

    match token {
        '\"' => { // exit
            string_value_to_output(todo!(), i_origin, i_trailing, output);
            return Ok(());
        }, 
        '\\' => { // escape
            string_value_to_output(todo!(), i_origin, i_trailing, output);
            return escaped_string_step(output, iterator, depth);
        },
        _ => return string_value_iterator(i_origin, index, output, iterator, depth), // continue
    }
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

fn escaped_string_step(
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32
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

    return string_value_iterator(todo!(), todo!(), output, iterator, depth);
}

// Expect operator or group end
fn operator_step(
    output: &mut Vec<u8>,
    iterator: &mut CharIndices, depth: u32
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

    return value_step_entrance(output, iterator, depth);
}