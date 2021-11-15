use itertools::Itertools;
use std::convert::Into;

pub trait StringJoinExt<S>: Iterator<Item=S>
where
    S: Into<String>
{
    fn join(self, separator: impl Into<String>) -> String;
}

impl<S, I> StringJoinExt<S> for I 
where 
    S: Into<String>,
    I: Iterator<Item=S>,
{
    fn join(self, separator: impl Into<String>) -> String {
        Itertools::intersperse(self.map(Into::into), separator.into()).collect()
    }
}