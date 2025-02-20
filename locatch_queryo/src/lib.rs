use locatch_query::*;

struct QueryEvaluator<'a> {
    iterator: QueryIter<'a>
}

// What storage is needed?

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

// Can it all be done in the function arguments?
// That the only thing stored outside of that is the previous truth.
// That works until you come across a group.

// Or does it?
// What happens in the following?
// "tag1" && "tag2" || "tag3"
// What is the sort-of, bodmas, way of evaluating that?
// I suppose it can just be tested right here.

// Okay, it can't be done as simply as described then.
/* 
true && false || true
true || false && true
true || false && false
false && false || true
*/
// All of these evaluate to true
// In an iterative approach of storing the previous-truth, and evaluating the next previous-truth based on the next operator and value.
// Then for the bottom two examples, the 1st would evaluate to false when read from left-to-right
// And the 2nd would evaluate to false, when read from right-to-left
// AND statements implicitlly create a group for connected ANDs.
// Hmm...
// If the previous truth is true, and the next operator is an AND, then you must step forward until one of 4 is encountered:
// - OR
// - Group start
// - Group end
// - Statement end
// At each step, you're iterating to ask if each value is true, because as soon as one false is encountered, you can continue

// Okay then.
// For the storage, you just allocate something sized by the length of the query iterator
// And you don't need to ever clear the storage, but it is unlikely that you'd actually need all of it

// Purely semantics, how does this work?
// It's like a two-step iterator.
// Where you go next and await a response.
// So it'd have somekind of internal flip-flop then.

// Yep, yep.