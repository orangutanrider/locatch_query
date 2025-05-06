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

// Expect:
//     Value or Group
fn entrance_step<'a, R: ConditionResolver>(
    query: &mut QueryIter<'a>,
    resolver: &R,
) -> Result<bool, ()> {
    let token = match query.next() {
        Some(v) => v,
        None => return Err(()), // Empty? Unsure as to whether this should be error
    };

    match token {
        Output::GroupEnd => return Err(()), // Error
        Output::Value(value) => { // Continue
            match value.value {
                ValueType::Group => {
                    let truth = entrance_step(query, resolver) ?;
                    return operator_step(query, resolver, truth);
                }, // Into entrance step, continue into operator step once group is exited
                ValueType::String(items) => { // Into operator step
                    let truth = resolver.resolve(Condition::String(items));
                    return operator_step(query, resolver, truth)
                }, 
            }
        },
        Output::Operator(_) => return Err(()), // Error
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
) -> Result<bool, ()> {
    let token = match query.next() {
        Some(v) => v,
        None => return Ok(previous_truth), // Exit
    };

    match token {
        Output::GroupEnd => return Ok(previous_truth), // Exit
        Output::Value(_) => return Err(()), // Error
        Output::Operator(operator) => { // Continue into value step
            match operator {
                Operator::And => return value_step(query, resolver, previous_truth, Operator::And),
                Operator::Or => { if previous_truth {
                    // Continue until the exit of current depth
                    return or_truth_exit(query);

                } else {
                    return value_step(query, resolver, previous_truth, Operator::Or);
                } },
            }
        },
    }
}

enum OutputType {
    Operator,
    GroupEnd,
    Value,
}

fn or_truth_exit<'a>(
    query: &mut QueryIter<'a>,
) -> Result<bool, ()> {
    let mut previous = OutputType::Operator;
    let mut relative_depth: usize = 0;
    
    loop { // The rest of this function is looped
    let token = match query.next() {
        Some(v) => v,
        None => { // Exit
            // (("val")
            // Isn't valid
            if relative_depth != 0 {
                return Err(())
            }

            match previous {
                OutputType::GroupEnd => return Ok(true),
                OutputType::Value => return Ok(true),
                // ... &&
                // Isn't valid
                OutputType::Operator => return Err(()),
            }
        }, 
    };

    match token {
        Output::GroupEnd => {
            relative_depth = relative_depth - 1;

            match previous {
                // && )
                // Isn't valid
                OutputType::Operator => return Err(()),
                OutputType::GroupEnd | OutputType::Value => { 
                    if relative_depth == 0 {
                        return Ok(true)
                    } else {
                        previous = OutputType::GroupEnd;
                        continue;
                }}
            }
        }, 
        Output::Value(val) => {
            if let ValueType::Group = val.value {
                relative_depth = relative_depth + 1;
            };

            match previous {
                // ) "val"
                // Isn't valid
                OutputType::GroupEnd => return Err(()), 
                // "val" "val"
                // Isn't valid
                OutputType::Value => return Err(()),
                OutputType::Operator => {
                    previous = OutputType::Value;
                    continue;
                },
            }
        },
        Output::Operator(_) => {
            match previous {
                // && &&
                // Isn't valid
                OutputType::Operator => return Err(()),
                // && )
                // Isn't valid
                OutputType::GroupEnd => { 
                    return Err(())
                },
                OutputType::Value => {
                    previous = OutputType::Operator;
                    continue;
                },
            }
        },
    }
}}

// Expect:
//     Value or Group
fn value_step<'a, R: ConditionResolver>(
    query: &mut QueryIter<'a>,
    resolver: &R,
    previous_truth: bool,
    previous_operator: Operator, // only matters if AND
) -> Result<bool, ()> {
    let token = match query.next() {
        Some(v) => v,
        None => return Err(()), // Error
    };

    match token {
        Output::GroupEnd => return Err(()), // Error
        Output::Value(value) => { // Continue
            match value.value {
                ValueType::Group => { // Into entrance step, continue into operator step once group is exited
                    let mut truth = entrance_step(query, resolver) ?;
                    if value.not { truth = !truth };
                    match previous_operator {
                        Operator::And => return operator_step(query, resolver, previous_truth && truth),
                        Operator::Or => return operator_step(query, resolver, previous_truth || truth),
                    }
                }, 
                ValueType::String(items) => { // Into operator step
                    let mut truth = resolver.resolve(Condition::String(items));
                    if value.not { truth = !truth };
                    match previous_operator {
                        Operator::And => return operator_step(query, resolver, previous_truth && truth),
                        Operator::Or => return operator_step(query, resolver, previous_truth || truth),
                    }
                }, 
            }
        },
        Output::Operator(_) => return Err(()), // Error
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

// --------
// AND statement logic
// If previous operator was AND
// And self or previous truth is false
// False

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
    // If previous operator was AND, and group resolved to false OR previous truth was false.
    // Then next step previous truth is false
    // Else true
// If value is condition, execute condition resolver
    // If previous operator was AND, and condition resolved to false OR previous truth was false.
    // Then next step previous truth is false
    // Else true
//  -> Operator step

// --------
// NOT clause 


#[cfg(test)]
mod test {

}