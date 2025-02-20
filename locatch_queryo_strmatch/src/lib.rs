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

// How does the query algorithm work then?
// check_solo

/* 
Traversal

Entrance
 Group
 Value (string)
Value step
 Operator
 End of iterator
Group step (into entrance)
Operator step (repeat entrance)
*/

/*
Logic

Value
  If equal ->
  If NOT
    return false
  Else
    return true
*/

// Are we re-construting the query as booleans?
// Can that be built into the query?
// That you ask it to evaluate, given booleans in-place of its values.

// How do we do that?
// How is the data stored, basically.

// 00
// 10
// 01
// 11
// eh...

// The most convinient way, in my head right now, would be to allow it to be done during iteration somehow.
// Hmm... Yeah, but that doesn't mean we have to store this stuff in the data being iterated on.
// Rather, we produce something that can be evaluated by iterating.

// Yeah...
// I sigh, because, it'll be a lot of work I think.

/* 
locatch_queryo
Evaluator(Vec<Token>)
Token {
    Group
    GroupEnd
    Value(bool)
    Operator {
        And,
        Or
    }
}
*/

// hmm...
// And you can give the evaluator an iterator in order to construct itself and iterate onto only the things you care about (the values (str, numerical))

// And then evaluating
// It's hierachical/recursive because of groups.
// The evaluator doesn't need to store groups then does it? Does it?
// Yeah it's technically collapsing a group into a value once it comes to the group end.
// It temporarly stores the Not above the group while the contents are being evaluated.
// And then once it returns, the not can be used, and the group is evaluated, and collapsed.

/* 
locatch_queryo
Evaluator(Vec<Token>)
Token {
    Value(bool)
    Operator {
        And,
        Or
    }
}
*/

// Yeah lots of work...

// In the meta-iterator that it gives the user.
// Does it need to give them the not?
// No, it technically just iterates, and then awaits their response of a boolean?
// Does that make sense?
// !"tag"
// If the tag isn't detected we return false, and so it becomes true.
// Yeah it works.

/* 
locatch_queryo
Evaluator(QueryIter)
*/

// It's technically just that I think.
// The whole thing is one big group, so you just collapse it to a single boolean.

// signifigant "todo"

// No, but it does have to store things to evaluate the group, but yes once it has been evaluated the evaluator becomes useless.
// And all that matters is the boolean you're left with.
