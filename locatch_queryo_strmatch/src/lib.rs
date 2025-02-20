fn main() {
    println!("Hello, world!");
}

pub fn str_match_queryo(

) {

}

// The output are the items.
// The input are items each paired with tags.

// The output is items.
// The input is items, each paired with string-value(s), and the query.

// What are items though?
// They aren't anything, they're just like IDs or something.
// https://doc.rust-lang.org/std/iter/struct.Map.html
// Maps?
// That we are 'mapping' the values of the iterator, based on the results of the query operation?
// Kinda, we're also filtering them, and it isn't necessary to return any string-value data.

// Hmm...

/*
{
    tickets: [
        {
            id...
            tags...
        },
        ...
    ]
}
*/

// Do we make a trait?

/*
trait {
    fn filtering_data()...
}
*/

// Doesn't seem correct
// Okay, I think we can just start by making a function that filters one entry, and then if we do that, it makes itself.