use locatch_query::*;

pub fn check_solo(
    query: QueryIter,
    value: &str
) -> bool {
    todo!()
}

pub fn check_values(
    query: QueryIter,
    values: &[&str]
) -> bool {
    todo!()
}

/*
    How do we resolve logic properly?
    Iterate across the statement?

    Comparison made?
    It has to be recursive~
    Not truly recursive, but it has to deal with groups, and then be capable of treating that group as a single boolean.

    So it'll find a group resolve it to a boolean, and continue.

    We're always holding the previous truth and operator I believe?
    And then we evaluate against the next truth.

    Hmm...
    OR, AND, NOT
    This is what we have to deal with.

    If a statement is proceeded by an OR, if it resolves to true, then you can exit.
    What about these though?
    "1" || "2" && "3"

    How does the bodmas of it work basically?
    I suppose we can just test in Rust to find out.

    Yeah so connected AND are effectively grouped, and evaluated as one.

    Connected OR can be evaluated iteratively, well no, you just exit once you've found your OR connected clause is true.

    NOT just inverts, should be fine.

    Hmm...
    You could decouple the comparison from the boolean resolving.
    How would that work?
    You use closures or something right?
    Yeah.

    You just have the thing tell it whether value resolves to true or false.
    Same input format.
    Okay...
    Yeah.

    locatch_queryr
    Yeah.
*/

#[cfg(test)]
mod test {
    #[test]
    fn boolean_bodmas_test() {
        // How do we actually tesst assertion order?
        // Basically we just want to find out if the AND statment is rouped

        // is grouped
        assert!((false || true && false) == false, "is grouped"); 
    }
}

