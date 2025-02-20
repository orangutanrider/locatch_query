use locatch_query::*;

struct QueryEvaluator<'a> {
    iterator: QueryIter<'a>,

}

// How does the storage work?

// And statements essentially create a group, maybe they don't need storage?
// It's a group that is created and progresses until one of the prescribed break-points.
// The idea of storing previous truth seems useless doesn't it?

// Hmm, getting confused it feels.

// The previous truth is more like the overall truth up until the next iteration.
// Yeah I'm not sure, I had an idea with the previous algorithm but now I feel confused.

// Okay no, yeah we still can do the previous truth.
// It is just that when the next statement is proceeded by an AND operator, then we must continue?
// No.
// true || false && false (evaluates to true)
// Each statement must be continued until a group, group-end, or the end of the iterator.
// And then we evaluated joined AND statements.
// But we still must step through it iterativley, to get the answer from the user as to whether a value means true or false.
// So logically, we collect the entire statement as this first?
/* 
Evaluator(Vec<Token>)
Token {
    Value(bool)
    Operator {
        And,
        Or
    }
}
*/

// Yeah, but maybe there are some optimzations you can do during the process of insertion.
// Insertion as being the process where the user tells the evaluator what values are true or false.

// Would it matter though, given that we're already allocating something sized by the query iterator len.

// Evaluating the store statement then.
// Values connected by AND operators are evaluated as if they were a group.
// The previous truth is stored, and is used by values proceeding OR operators to determine the next previous truth.
// Groups are evaluated internally through recursion.
// Okay, I should test the thing with AND and OR though.
// What if this bascially:
// true && false || true && false
// ...
// Yeah it holds true to the algorithm prescribed.

// Is there a more optimal way of doing it though?
// For some of these operations, you can kinda imagine bitwise operations being useful.
// For a chain of OR operators, if you put all of the truths into a byte, you'd just check "if greater than 0"
// For AND operations, you put all the truths in a byte, and then you just do a comparison for if it equals a byte of all true of the same length
// Hmm...

// This can be done during construction too, putting them into bytes like this.
// Hmm...

// It's the same algorithm but with bit-packing I think.

// Okay.

// OperatorType+BitLen
// Groups break it up, but redundancy can be handled beforehand.

// 00 Group end
// 01 Group start
// 10 AND block
// 11 OR block

// 6 bits for length?

// hmm...
// 0-63
// 63 connected statements?

// It's not a maximum, it's a maximum for the bit-packing optimsation.

// Doesn't seem ideal.
// I think slices seem more true?
// But describing the operators and groups is where the difficulty lies.

// But yeah, we can imagine the truths as just being bytes or usize packed together with no context.
// hmm...

// 00 Group end
// 01 Group start
// 10 AND
// 11 OR 
// A parralel storage of double the length, storing these packed together, describing the inbetweens?
// That we could infer where the next value is suppsoed to be.

// Yeah I think that works.
// Should be highly compact.

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