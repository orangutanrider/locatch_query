struct QueryBox(Box<[u8]>); 
impl<'a> QueryBox {
    fn iter(&'a self) -> QueryIter<'a> {
        return QueryIter::<'a>{
            slice: &self.0,
            index: 0
        }
    }
}

struct QueryIter<'a>{
    slice: &'a [u8],
    index: usize
}
impl<'a> QueryIter<'a> {
    // todo
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

enum Output<'a> {
    Group,
    Operator(Operator),
    Value(Value<'a>),
}

enum Operator {
    And,
    Or
}

struct Value<'a> {
    value: ValueType<'a>,
    not: bool,    
}

enum ValueType<'a> {
    String(&'a [u8])
}

const NOT_BIT: u8 = 128; // The final bit of the type value is used as a NOT flag for following value data.
const NOT_MASK: u8 = 127; // A negative mask for the NOT bit
// Byte IDs
const GROUP: u8 = 0;
const AND: u8 = 1;
const OR: u8 = 2;
const STRING: u8 = 3;

fn iterate<'a>(iterator: &'a mut QueryIter) -> Option<Output<'a>> {
    if iterator.index >= iterator.slice.len() {
        return None
    }

    // type step
    let increment = iterator.increment();
    match increment & NOT_MASK { // mask out the NOT_BIT
        GROUP => return Some(Output::Group),
        AND => return Some(Output::Operator(Operator::And)),
        OR  => return Some(Output::Operator(Operator::Or)),
        STRING => return string_step(iterator, increment),
        _ => panic!(), // It is expected that QueryBox and QueryIter will be constructed correctly.
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

    let not = type_increment > 127; // is not bit present?

    return Some(Output::Value(
        Value { value: ValueType::String(string), not }
    ))
}

/// Entrance: `step_usize(iterator, 0, 0)`
fn step_usize<'a>(iterator: &mut QueryIter, step_index: u32, output: usize) -> usize {
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