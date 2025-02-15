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

// Expect value or group
// Expect operator
// Repeat

// Value step entrance
// Valid char:
//  !
//  "
//  numeric (eventually)
//  . (eventually, numeric)  

// Value step iterator
// Collect until '\' OR '"'
// IF '\' the next step of iteration will ignore '"'
// IF '"' EXIT

// Operator step entrance
// Expect either:
//  &&
//  ||
// EXIT

