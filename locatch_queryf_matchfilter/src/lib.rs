use locatch_query::*;
use locatch_queryr::*;

pub enum Comparable<'a>{
    String(&'a [u8]),
}

pub struct Resolver<'r>(Box<[Comparable<'r>]>);
impl<'r> ConditionResolver<()> for Resolver<'r> {
    // If any of the contents of self match the condition, return true.
    fn resolve<'a>(&self, condition: locatch_query::Condition<'a>) -> Result<bool, ()> {
        match condition {
            Condition::String(items) => {
                for pattern in self.0.as_ref() {
                    #[allow(irrefutable_let_patterns)] // New patterns would be added, but is unimplemented
                    let Comparable::String(pattern) = *pattern else { continue; };

                    if items == pattern { return Ok(true) }
                }
            },
        }

        return Ok(false);
    }
}
impl<'r> Resolver<'r> {
    fn new(data: ) {

    }

    fn from_str(data: &'r [&str]) -> Self {
        let mut comparables = Vec::with_capacity(data.len());
        for value in data {
            comparables.push(Comparable::String(value.as_bytes()));
        }

        return Resolver(comparables.into_boxed_slice())
    }
}

pub fn check_with_resolver(
    query: QueryIter,
    resolver: Resolver,
) -> Result<bool, ResolverError<()>> {
    return resolve_with(query, &resolver);
}

// Returns true if the data passed the query, via comparisons of the contents of the data against the query operation
pub fn match_filter(
    query: QueryIter,
    data: &[Comparable]
) -> Result<bool, ResolverError<()>> {
    return check_with_resolver(query, Resolver::new(data));
}

// Returns true if the data passed the query, via comparisons of the contents of the data against the query operation
pub fn match_filter_str(
    query: QueryIter,
    data: &[&str]
) -> Result<bool, ResolverError<()>> {
    todo!()
}

mod test {

}

