use bytes::*;

// Gotta change it to not use an iterator, I think.
// That way string_reader can return a slice, rather than a newly created String.

// This is internal stuff.
// The way it'd work for the end user, is that they'd have an iterator, that'd return a union type. I think.

fn funcbox(data: Box<[u8]>) {
    let e = data.iter();
}

fn type_reader(mut iter: core::slice::Iter<u8>) {
    let byte = match iter.next() {
        Some(val) => val,
        None => return,
    };

    match *byte {
        ValueType::STRING => todo!(),
        _ => return,
    }
}

const USIZE_BYTE_LEN: u32 = usize::BITS / 8;

fn string_len_reader(len_val: usize, len_index: u32, mut iter: core::slice::Iter<u8>) -> Result<usize, ()> {
    let byte = match iter.next() {
        Some(val) => val,
        None => return Err(()),
    };

    let mask: usize = *byte as usize;
    let mask = mask >> (len_index * USIZE_BYTE_LEN);

    let len_val = len_val | mask;

    let len_index = len_index + 1;
    
    if len_index == USIZE_BYTE_LEN { // end len_reader step
        return Ok(len_val);
    } else { // continue
        return string_len_reader(len_val, len_index, iter);
    }
}

fn string_reader(str_len: usize, mut iter: core::slice::Iter<u8>) {

}

// The len reader implementation is the same though anywas, it seems.
// If we're storing length, then theoretically we don't need a pointer to where the next value is, because we expect it to all be contiguous
/* 
/// entrance: ```string_ptr_reader(0, 0, usize::BITS / 8, iter)```
fn string_ptr_reader(ptr: usize, ptr_index: u32, ptr_len: u32, mut iter: core::slice::Iter<u8>) -> Result<usize, ()> {
    let byte = match iter.next() {
        Some(val) => val,
        None => return Err(()),
    };

    let mask: usize = *byte as usize;
    let mask = mask >> (ptr_index * ptr_len);

    let ptr = ptr | mask;

    let ptr_index = ptr_index + 1;
    
    if ptr_index == ptr_len { // end ptr_reader step
        return Ok(ptr);
    } else { // continue
        return string_ptr_reader(ptr, ptr_index, ptr_len, iter);
    }
}
*/

fn pointer_size() -> u32 {
    return usize::BITS / 8
}

struct ValueType(u8);
impl ValueType {
    const STRING: u8 = 0;

    // Support for number value types can be added later (it would need more types than just number (float, integer)), string is all I need for now.
    //const NUMBER: u8 = 1; 
}

// struct Value {
//     value: [u8],
//     terminator: *const u8
// }

// struct Value {
//     len: usize,
//     value: [u8],
// }

// fn test() {
//     let string = "el-gringo";
//     string.bytes()
// }

/* 
pub struct LocatchQuery(Vec<Token>);

pub enum Token{
    Group(LocatchQuery),
    Value((bool, Value)),
    Operator(Operator)
}

pub enum Value {
    String(String),
    Number(u32),
}

pub enum Operator{
    And,
    Or
}
*/