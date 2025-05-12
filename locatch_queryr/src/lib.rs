use locatch_query::*;

// locatch query resolver.
// Handles the resolving of boolean operators for condition resolvers

pub trait ConditionResolver<E> {
    fn resolve<'a>(&self, condition: locatch_query::Condition<'a>) -> Result<bool, E>;
}

pub enum ResolverError<E> {
    Undefined,
    ConditionResolver(E)
}

pub fn resolve_with<'a, E, R: ConditionResolver<E>> (
    mut query: QueryIter<'a>,
    resolver: &R
) -> Result<bool, ResolverError<E>> {
    return entrance_step(&mut query, resolver)
}

// Expect:
//     Value or Group
fn entrance_step<'a, E, R: ConditionResolver<E>>(
    query: &mut QueryIter<'a>,
    resolver: &R,
) -> Result<bool, ResolverError<E>> {
    let token = match query.next() {
        Some(v) => v,
        None => return Err(ResolverError::Undefined), // Empty? Unsure as to whether this should be error
    };

    match token {
        Output::GroupEnd => return Err(ResolverError::Undefined), // Error
        Output::Value(value) => { // Continue
            match value.value {
                ValueType::Group => {
                    let truth = entrance_step(query, resolver) ?;
                    return operator_step(query, resolver, truth);
                }, // Into entrance step, continue into operator step once group is exited
                ValueType::String(items) => { // Into operator step
                    let truth = match resolver.resolve(Condition::String(items)) {
                        Ok(ok) => ok,
                        Err(err) =>  return Err(ResolverError::ConditionResolver(err)),
                    };
                    return operator_step(query, resolver, truth)
                }, 
            }
        },
        Output::Operator(_) => return Err(ResolverError::Undefined), // Error
    }
}

// Expect:
//     Operator
//     Group end
//     END
fn operator_step<'a, E, R: ConditionResolver<E>>(
    query: &mut QueryIter<'a>,
    resolver: &R,
    previous_truth: bool,
) -> Result<bool, ResolverError<E>> {
    let token = match query.next() {
        Some(v) => v,
        None => return Ok(previous_truth), // Exit
    };

    match token {
        Output::GroupEnd => return Ok(previous_truth), // Exit
        Output::Value(_) => return Err(ResolverError::Undefined), // Error
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

fn or_truth_exit<'a, E>(
    query: &mut QueryIter<'a>,
) -> Result<bool, ResolverError<E>> {
    let mut previous = OutputType::Operator;
    let mut relative_depth: usize = 0;
    
    loop { // The rest of this function is looped
    let token = match query.next() {
        Some(v) => v,
        None => { // Exit
            // (("val")
            // Isn't valid
            if relative_depth != 0 {
                return Err(ResolverError::Undefined)
            }

            match previous {
                OutputType::GroupEnd => return Ok(true),
                OutputType::Value => return Ok(true),
                // ... &&
                // Isn't valid
                OutputType::Operator => return Err(ResolverError::Undefined),
            }
        }, 
    };

    match token {
        Output::GroupEnd => {
            relative_depth = relative_depth - 1;

            match previous {
                // && )
                // Isn't valid
                OutputType::Operator => return Err(ResolverError::Undefined),
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
                OutputType::GroupEnd => return Err(ResolverError::Undefined), 
                // "val" "val"
                // Isn't valid
                OutputType::Value => return Err(ResolverError::Undefined),
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
                OutputType::Operator => return Err(ResolverError::Undefined),
                // && )
                // Isn't valid
                OutputType::GroupEnd => { 
                    return Err(ResolverError::Undefined)
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
fn value_step<'a, E, R: ConditionResolver<E>>(
    query: &mut QueryIter<'a>,
    resolver: &R,
    previous_truth: bool,
    previous_operator: Operator, // only matters if AND
) -> Result<bool, ResolverError<E>> {
    let token = match query.next() {
        Some(v) => v,
        None => return Err(ResolverError::Undefined), // Error
    };

    match token {
        Output::GroupEnd => return Err(ResolverError::Undefined), // Error
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
                    let mut truth = match resolver.resolve(Condition::String(items)) {
                        Ok(ok) => ok,
                        Err(err) =>  return Err(ResolverError::ConditionResolver(err)),
                    };
                    if value.not { truth = !truth };
                    match previous_operator {
                        Operator::And => return operator_step(query, resolver, previous_truth && truth),
                        Operator::Or => return operator_step(query, resolver, previous_truth || truth),
                    }
                }, 
            }
        },
        Output::Operator(_) => return Err(ResolverError::Undefined), // Error
    }
}

#[cfg(test)]
mod test {
    // Basic AND
    // Basic OR
    // OR, AND
    // OR, AND with groups

    use locatch_query::QueryBox;

    use crate::{resolve_with, ConditionResolver};

    struct TestResolver;
    impl ConditionResolver<()> for TestResolver {
        fn resolve<'a>(&self, condition: locatch_query::Condition<'a>) -> Result<bool, ()> {
            match condition {
                locatch_query::Condition::String(items) => {
                    let string: &str = unsafe { std::mem::transmute(items) };
                    match string {
                        "true" => return Ok(true),
                        "false" => return Ok(false),
                        _ => return Err(()),
                    }
                },
            }
        }
    }

    #[test]
    fn basic_and() {
        let resolver = TestResolver;

        // True and True
        let statement: &str = stringify!("true" && "true");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, true, "true and true");

        // True and False
        let statement: &str = stringify!("true" && "false");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, false, "true and false");

        // False and True
        let statement: &str = stringify!("false" && "true");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, false, "false and true");

        // False and False
        let statement: &str = stringify!("false" && "false");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, false, "false and false");
    }

    #[test]
    fn basic_or() {
        let resolver = TestResolver;

        // True or True
        let statement: &str = stringify!("true" || "true");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, true, "true or true");

        // True or False
        let statement: &str = stringify!("true" || "false");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, true, "true or false");

        // False or True
        let statement: &str = stringify!("false" || "true");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, true, "false or true");

        // False or False
        let statement: &str = stringify!("false" || "false");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, false, "false or false");
    }

    #[test]
    fn or_and() {
        let resolver = TestResolver;

        let statement: &str = stringify!("true" || "true" && "false");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, true, "true or true and false");

        let statement: &str = stringify!("true" || "false" && "true");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, true, "true or false and true");

        let statement: &str = stringify!("false" || "true" && "false");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, false, "false or true and false");

        let statement: &str = stringify!("false" || "false" && "true");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, false, "false or false and true");
    }

    #[test]
    fn and_or() {
        let resolver = TestResolver;

        let statement: &str = stringify!("true" && "false" || "false");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, false, "true and false or false");

        let statement: &str = stringify!("false" && "true" || "false");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, false, "false and true or false");

        let statement: &str = stringify!("true" && "false" || "true");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, true, "true and false or true");


        let statement: &str = stringify!("false" && "true" || "true");
        let query = match QueryBox::try_from_str(statement) {
            Ok(ok) => ok,
            Err(_) => panic!("Failed to create query (indicates issue with locatch_query)"),
        };
        let query = query.iter();
        let resolved = match resolve_with(query, &resolver) {
            Ok(ok) => ok,
            Err(_) => panic!("Unexpected resolver error"),
        };
        assert_eq!(resolved, true, "false and true or true");
    }

    // True AND (group = false)
    // False AND (group = true)
    // True ADN (group = true)

    // (group with OR = true) AND false 
    // (group with OR = false) AND true
    // (group with OR = true) AND true
    // (group with OR = false) AND false
}