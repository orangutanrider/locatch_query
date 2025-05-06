use locatch_query::*;

// locatch query resolver.
// Handles the resolving of boolean operators for condition resolvers

// Input?
// QueryIter + Closure for operator.
// Yeah.

pub trait ConditionResolver {
    fn resolve<'a>(condition: locatch_query::Condition<'a>) -> bool;
}

pub fn resolve_with<
    'a,
    R: ConditionResolver,
> (
    query: QueryIter<'a>,
    resolver: R
) -> bool {
    todo!()
}

fn resolve<
    'a,
    R: ConditionResolver,
> (
    mut query: QueryIter<'a>,
    resolver: R,
    trailing_truth: bool,
    trailing_operator: Operator,
) -> bool {
    let value = match query.next() {
        Some(v) => v,
        None => todo!(),
    };

    todo!()
}

// Expect:
//     Value or Group
fn entrance_step<'a, R: ConditionResolver>(
    query: &mut QueryIter<'a>,
    resolver: &R,
) {
    let token = match query.next() {
        Some(v) => v,
        None => todo!(), // Empty?
    };

    match token {
        Output::GroupEnd => todo!(), // Error
        Output::Value(value) => { // Continue
            match value.value {
                ValueType::Group => {
                    let output = entrance_step(query, resolver);
                    return operator_step(query, resolver);
                }, // Into entrance step, continue into operator step once group is exited
                ValueType::String(items) => return operator_step(query, resolver), // Into operator step
            }
        },
        Output::Operator(_) => todo!(), // Error
    }
}

// Expect:
//     Operator
//     Group end
//     END
fn operator_step<'a, R: ConditionResolver>(
    query: &mut QueryIter<'a>,
    resolver: &R,
) {
    let token = match query.next() {
        Some(v) => v,
        None => return, // Exit
    };

    match token {
        Output::GroupEnd => return, // Exit
        Output::Value(_) => todo!(), // Error
        Output::Operator(operator) => { // Continue into value step
            return value_step(query, resolver);
        },
    }

}

// Expect:
//     Value or Group
fn value_step<'a, R: ConditionResolver>(
    query: &mut QueryIter<'a>,
    resolver: &R,
) {
    let token = match query.next() {
        Some(v) => v,
        None => todo!(), // Error
    };

    match token {
        Output::GroupEnd => todo!(), // Error
        Output::Value(value) => { // Continue
            match value.value {
                ValueType::Group => {
                    let output = entrance_step(query, resolver);
                    return operator_step(query, resolver);
                }, // Into entrance step, continue into operator step once group is exited
                ValueType::String(items) => return operator_step(query, resolver), // Into operator step
            }
        },
        Output::Operator(_) => todo!(), // Error
    }
}

// No, there was a way in which I had AND statements figured out, I remember.

// --------
// Traversal only

// Entrance step
// Expect value
// If value is group, entrance step into group
// -> Operator step

// Operator step
// Expect operator or END
// -> Value step

// Value step
// Expect value
// If value is group, entrance step into group.
//  -> Operator step

// --------
// Apply Condition resolver

// Entrance step
// Expect value
// If value is group, entrance step into group
// If value is condition, execute condition resolver
// -> Operator step

// Operator step
// Expect operator or END
// -> Value step

// Value step
// Expect value
// If value is group, entrance step into group.
// If value is condition, execute condition resolver
//  -> Operator step

#[cfg(test)]
mod test {

}