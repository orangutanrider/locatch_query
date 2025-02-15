mod from;

pub struct QueryBox(Box<[u8]>); 
impl<'a> QueryBox {
    pub fn iter(&'a self) -> QueryIter<'a> {
        return QueryIter::<'a>{
            slice: &self.0,
            index: 0
        }
    }

    pub fn from_str(string: &str) -> Self {
        todo!()
    }
}
impl From<&str> for QueryBox {
    fn from(value: &str) -> Self {
        todo!()
    }
}

pub struct QueryIter<'a> {
    slice: &'a [u8],
    index: usize
}
impl<'a> QueryIter<'a> {
    pub fn next(&'a mut self) -> Option<Output<'a>>{
        return iterate(self, self.slice.len())
    }
}
// Internal
impl<'a> QueryIter<'a> {
    fn increment(&mut self) -> u8 {
        let value = self.slice[self.index];
        self.index = self.index + 1;
        return value
    }

    fn manual_step(&mut self, amount: usize) {
        self.index = self.index + amount;
    }
}

pub enum Output<'a> {
    Value(Value<'a>),
    Operator(Operator),
}

pub enum Operator {
    And,
    Or
}

pub struct Value<'a> {
    pub value: ValueType<'a>,
    pub not: bool,    
}

pub enum ValueType<'a> {
    Group(Group<'a>),
    String(&'a [u8]),
}

// Group value storage
// TYPE u8 | TERMINUS usize 
pub struct Group<'a>{
    iterator: &'a mut QueryIter<'a>,
    terminus: usize,
}
impl<'a> Group<'a> {
    pub fn next(&'a mut self) -> Option<Output<'a>>{
        return iterate(self.iterator, self.terminus)
    }
}

// const NOT_BIT: u8 = 128; // The final bit of the type value is used as a NOT flag for following value data.
const NOT_MASK: u8 = 127; // A negative mask for the NOT bit
// Byte IDs
const AND: u8 = 0;
const OR: u8 = 1;
const GROUP: u8 = 2;
const STRING: u8 = 3;

#[inline]
fn iterate<'a>(iterator: &'a mut QueryIter<'a>, end: usize) -> Option<Output<'a>> {
    if iterator.index >= end {
        return None
    }

    // type step
    let increment = iterator.increment();
    match increment & NOT_MASK { // mask out the NOT_BIT
        AND => return Some(Output::Operator(Operator::And)),
        OR  => return Some(Output::Operator(Operator::Or)),
        GROUP => return group_step(iterator, increment),
        STRING => return string_step(iterator, increment),
        _ => panic!("Unexpected type value of {} during iteration", increment), // It is expected that QueryBox and QueryIter will be constructed correctly.
    }
}

// String value storage
// TYPE u8 | LEN usize | STRING...
// TYPE has already been read during the type-step

// It is expected to be entered after the type step, meaning those bytes have already been incremented past in the iterator.
// QueryBox and QueryIter are expected to be created as valid data, so no error checking is done here.
fn string_step<'a>(iterator: &'a mut QueryIter, type_increment: u8) -> Option<Output<'a>> {
    let string_len = step_usize(iterator, 0, 0);
    let string = &iterator.slice[iterator.index..iterator.index+string_len];
    iterator.manual_step(string_len);

    let not = type_increment > NOT_MASK; // is NOT_BIT present?

    return Some(Output::Value(
        Value { value: ValueType::String(string), not }
    ))
}

// Group value storage
// TYPE u8 | TERMINUS usize 
fn group_step<'a>(iterator: &'a mut QueryIter<'a>, type_increment: u8) -> Option<Output<'a>> {
    let terminus = step_usize(iterator, 0, 0);

    let not = type_increment > NOT_MASK; // is NOT_BIT present?

    return Some(Output::Value(
        Value{ value: ValueType::Group( Group{ iterator, terminus }), not }
    ))
}

/// Entrance: `step_usize(iterator, 0, 0)`
fn step_usize(iterator: &mut QueryIter, step_index: u32, output: usize) -> usize {
    let byte = iterator.increment();

    // place the byte into a usize
    // usize [ BYTE | empty bytes ]
    let mask = byte as usize;
    // bitshift the value by the step_index
    let mask = mask >> step_index;

    // use a bitwise OR to place the value into the output usize 
    let output = output | mask;

    // increment the step_index by a byte
    let step_index = step_index + 8;
    if step_index == usize::BITS { // end has been reached
        return output
    } else { // continue
        return step_usize(iterator, step_index, output)
    }
}