use locatch_query::*;
use locatch_queryr::*;

pub struct Resolver<'r>(&'r[locatch_query::Condition<'r>]);
impl<'r> ConditionResolver<()> for Resolver<'r> {
    // If any of the contents of self match the condition, return true.
    fn resolve<'a>(&self, condition: locatch_query::Condition<'a>) -> Result<bool, ()> {
        for pattern in self.0 {
            if *pattern == condition {
                return Ok(true);
            }
        }

        return Ok(false);
    }
}
impl<'r> Resolver<'r> {
    pub fn new() {
        todo!()
    }
}


pub fn check_with_resolver(
    query: QueryIter,
    resolver: Resolver,
) -> Result<bool, ResolverError<()>> {
    return resolve_with(query, &resolver);
}

// Returns true if the data was not filtered
pub fn match_filter(
    query: QueryIter,
    data: &[locatch_query::Condition]
) -> Result<bool, ResolverError<()>> {

    todo!();
}

// locatch_query::Condition
// Hmm... Is it wise to force users to convert to this?
// Or should the query resolver be converting, or holding its own types?
// I think it should hold its own types ~Kinda?
// It's either that or you standardise both to use JSON value type, probably...
// I'd much rather the system work with the base types.
// Hmm...
// This is a pain in the ass...
// JSON value doesn't really work either. You can have array types and object types with that.
// It indeed must be of type condition.

// Is there anything to it?
// Just have them convert their JSON values to conditions?
// Makes sense?
// Or does the matchfilter have its own type.
// So that it can do its own ~interpretations
// Decoupling them seems best...


mod test {

}

