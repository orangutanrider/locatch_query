use std::{ops::AddAssign, str::CharIndices};

use crate::{
    QueryBox, AND, GROUP, GROUP_END, NOT_BIT, OR, STRING
};

pub fn try_from_str(string: &str) -> Result<QueryBox, ReadError> {
    let mut output = Vec::with_capacity(string.len() * 2);
    let mut iterator = string.char_indices();

    match value_step_entrance(&mut output, string, &mut iterator, 0) {
        Ok(_) => {/* Continue */},
        Err(err) => return Err(err),
    }

    return Ok(QueryBox(
        output.into_boxed_slice()
    ))
}

pub enum ReadError {
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
    source: &str, iterator: &mut CharIndices, depth: u32
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
            match value_step_entrance(output, source, iterator, depth + 1) {
                Ok(_) => return operator_step(output, source, iterator, depth),
                Err(err) => return Err(err),
            }
        },

        // string
        '\"' => {
            output_type = output_type | STRING;
            output.push(output_type);
            return string_value_step(output, source, iterator, depth);
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
    let bytes = value.to_le_bytes();

    for byte in bytes {
        output[index] = byte;
        index = index + 1;
    }
}

fn string_value_step(
    output: &mut Vec<u8>,
    source: &str, iterator: &mut CharIndices, depth: u32
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
            return operator_step(output, source, iterator, depth);
        },
        '\\' => match string_escape_step(output, source, iterator, depth) {
            Ok(_) => {/* Continue */},
            Err(err) => return Err(err),
        },
        _ => match string_value_iterator(index,index, output, source, iterator, depth) {
            Ok(_) => {/* Continue */},
            Err(err) => return Err(err),
        }, 
    }

    let string_len = output.len() - usize_index;
    set_usize(output, usize_index, string_len);

    return operator_step(output, source, iterator, depth);
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
    source: &str, iterator: &mut CharIndices, depth: u32
) -> Result<(), ReadError> {
    let (index, token) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
    };

    match token {
        '\"' => { // exit
            string_value_to_output(source, i_origin, i_trailing, output);
            return Ok(());
        }, 
        '\\' => { // escape
            string_value_to_output(source, i_origin, i_trailing, output);
            return string_escape_step(output, source, iterator, depth);
        },
        _ => return string_value_iterator(i_origin, index, output, source, iterator, depth), // continue
    }
}

fn hex_step(
    iterator: &mut CharIndices,
) -> Result<u16, ReadError> {
    let (_, hex) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
    };
    
    match hex {
        '0' => return Ok(0),
        '1' => return Ok(0x1),
        '2' => return Ok(0x2),
        '3' => return Ok(0x3),
        '4' => return Ok(0x4),
        '5' => return Ok(0x5),
        '6' => return Ok(0x6),
        '7' => return Ok(0x7),
        '8' => return Ok(0x8),
        '9' => return Ok(0x9),
        'A' => return Ok(0xA),
        'B' => return Ok(0xB),
        'C' => return Ok(0xC),
        'D' => return Ok(0xD),
        'E' => return Ok(0xE),
        'F' => return Ok(0xF),
        _ => return Err(ReadError::Undefined),
    }
}

fn hex4_step(
    iterator: &mut CharIndices,
    index: &mut usize
) -> Result<String, ReadError> {
    let hex4 = match hex_step(iterator) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };
    let hex3 = match hex_step(iterator) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };
    let hex2 = match hex_step(iterator) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };
    let hex1 = match hex_step(iterator) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    index.add_assign(4);

    let char_code = (hex4 * (16^3)) + (hex3 * (16^2)) + (hex2 * 16) + hex1;
    match String::from_utf16(&[char_code]) {
        Ok(ok) => return Ok(ok),
        Err(_err) => return Err(ReadError::Undefined),
    }
}

fn string_escape_step(
    output: &mut Vec<u8>,
    source: &str, iterator: &mut CharIndices, depth: u32
) -> Result<(), ReadError> {
    let (mut index, token) = match iterator.next() {
        Some(val) => val,
        None => return Err(ReadError::Undefined),
    };

    let value = match token {
        '\"' => "\"", // quotation mark
        '\\' => "\\", // reverse solidus
        '/' => "/", // solidus (solidus and slash are the same character)
        'b' => "\x08", // backspace
        'f' => "\x0C", // formfeed
        'n' => "\n", // linefeed (linefeed and newline are the same thing)
        'r' => "\r", // carriage return
        't' => "\t", // horizontal tab
        'u' => & match hex4_step(iterator, &mut index) { // 4 hexadecimal digits
            Ok(ok) => ok,
            Err(err) => return Err(err),
        }, 
        _ => return Err(ReadError::Undefined),
    };

    for byte in value.as_bytes() {
        output.push(*byte);
    }

    return string_value_iterator(index + 1, index + 1, output, source, iterator, depth);
}

// Expect operator or group end
fn operator_step(
    output: &mut Vec<u8>,
    source: &str, iterator: &mut CharIndices, depth: u32
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

    return value_step_entrance(output, source, iterator, depth);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn str_sequence() {
        // foobar with quotations
        let sequence = stringify!(
            "foobar"
        );

        let query = match try_from_str(sequence) {
            Ok(ok) => ok,
            Err(_) => panic!("error while parsing the sequence"),
        };

        let mut query = query.iter();
        match query.next() {
            Some(val) => { match val {
                Output::GroupEnd => panic!("Unexpected group-end output"),
                Output::Value(value) => {
                    assert_eq!(value.not, false);
                    match value.value {
                        ValueType::Group => panic!("Unexpected group output"),
                        ValueType::String(items) => {
                            let string: &str = unsafe { std::mem::transmute(items) };
                            assert_eq!(string, "foobar")
                        },
                    }
                },
                Output::Operator(_) => panic!("Unexpected operator output"),
            }},
            None => panic!("Unexpected value of none"),
        }
    }
}