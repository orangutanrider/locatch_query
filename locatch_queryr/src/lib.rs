use locatch_query::*;

// locatch query resolver.
// Handles the resolving of boolean operators for condition resolvers

// Input?
// QueryIter + Closure for operator.
// Yeah.

pub trait ConditionResolver {
    fn resolve<'a>(&self, condition: locatch_query::Condition<'a>) -> bool;
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
) -> bool {
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
                    return operator_step(query, resolver, output);
                }, // Into entrance step, continue into operator step once group is exited
                ValueType::String(items) => { // Into operator step
                    let truth = resolver.resolve(Condition::String(items));
                    return operator_step(query, resolver, truth)
                }, 
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
    previous_truth: bool,
) -> bool {
    let token = match query.next() {
        Some(v) => v,
        None => return previous_truth, // Exit
    };

    match token {
        Output::GroupEnd => return previous_truth, // Exit
        Output::Value(_) => todo!(), // Error
        Output::Operator(operator) => { // Continue into value step
            match operator {
                Operator::And => todo!(),
                Operator::Or => { if previous_truth {
                    // Continue until the exit of current depth
                    return exit_current_with_truth(query, true);

                } else {
                    return value_step(query, resolver, previous_truth, Operator::Or);
                } },
            }
        },
    }
}

fn exit_current_with_truth<'a>(
    query: &mut QueryIter<'a>,
    truth: bool,
) -> bool {
    loop {
        let token = match query.next() {
            Some(v) => v,
            None => return truth, // Exit
        };

        todo!(); // Preform traversal validation

        match token {
            Output::GroupEnd => return truth,
            Output::Value(value) => continue,
            Output::Operator(operator) => continue,
        }
    }
}

// Expect:
//     Value or Group
fn value_step<'a, R: ConditionResolver>(
    query: &mut QueryIter<'a>,
    resolver: &R,
    previous_truth: bool,
    previous_operator: Operator, // only matters if AND
) -> bool {
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
                    return operator_step(query, resolver, output);
                }, // Into entrance step, continue into operator step once group is exited
                ValueType::String(items) => { // Into operator step
                    let truth = resolver.resolve(Condition::String(items));
                    return operator_step(query, resolver, truth)
                }, 
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

// --------
// OR statement logic
// During operator step, If previous truth true, and OR is detected.
// Then exit with true, continuing till end or group end
// IF group end 
//     exit with previous truth
// ...

// Entrance step
// Expect value
// If value is group, entrance step into group
// If value is condition, execute condition resolver
// -> Operator step

// Operator step
// Expect operator or END
// If OR
//     If previous truth was true, exit currennt depth with true, and progress iterator until group end or statement end
// -> Value step

// Value step
// Expect value
// If value is group, entrance step into group.
// If value is condition, execute condition resolver
//  -> Operator step

#[cfg(test)]
mod test {

}