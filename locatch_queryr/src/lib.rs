use locatch_query::*;

// locatch query resolver.
// Handles the resolving of boolean operators for query operators

// Input?
// QueryIter + Closure for operator.
// Yeah.

pub trait ConditionOperator {
    fn resolve<'a>(condition: locatch_query::Condition<'a>) -> bool;
}

pub fn resolve_with_operator<
    'a,
    O: ConditionOperator,
> (
    query: QueryIter<'a>,
    operator: O
) {

}



//pub struct QueryResolver<'a>(QueryIter<'a>);
//impl<'a> QueryResolver<'a> {
//}


// Operator.
// The operator closure will use the type system.
// It just needs the closure to say yes or no.
// Okay.
// impl ? of something that outputs boolean.
// Can probably do the funtion thing right?
// It's an impl of something that outputs boolean, and it's input is variable.

// It is FnMut
// Is there a better way?
// Yeah, we just need the thing to iteratively go next, and give a boolean for the comparison, and then this thing will hand back a boolean for operator resolves.
// Iteratively until finish.

// What are the semantics of this though?

// It's going forward through the iterator.
// And then it will hand over the value data to the operator, and expect it to give it the truth.

// Is the resolver a wrapper around the query iter then?
// Seems like it.

// It's handing over value data only.
// At each step.
// And asking for boolean in return.
// Kinda like a flip-flop?

// Okay it's a wrapper and then internally it just holds its own stuf for the resolving, like previous truth?
// Or does it need that? Because it can maybe do that that via function arguments or something right?

/*
 fn resolver_next(
    previous_truth: bool,
) -> locatch_query::Value {
*/

// It needs to hold somekind of internal state, it needs to be an iterator basically.
// If not? It can't hold group state or work recursively.

// It needs to hold query iter

// A closure does make the most sense, because it allows you to have the operator only care about its self.
// And the thing comes to exit on its own.

// It just needs them to impl something.
// Value to boolean

// Okay but the value type actually contains additional information that only the resolver should care about.