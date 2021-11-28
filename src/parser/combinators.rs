use nom::error as nom_error;
pub use crate::tokens::{Position, Token, TokenKind};

pub fn fold1<I, OSeed, ONext, E, FSeed, FNext, FAcc>(
    mut seed: FSeed,
    mut next: FNext,
    acc: FAcc,
) -> impl FnMut(I) -> nom::IResult<I, OSeed, E>
where
    I: Clone,
    E: nom::error::ParseError<I>,
    FSeed: nom::Parser<I, OSeed, E>,
    FNext: nom::Parser<I, ONext, E>,
    FAcc: Fn(OSeed, ONext) -> OSeed,
{
    move |input| {
        let (rest_seed, item_seed) = seed.parse(input)?;

        let mut rest = rest_seed;
        let mut curr = item_seed;
        let mut found_none = true;
        loop {
            let r = rest.clone();
            match next.parse(rest) {
                Ok((rest_next, item_next)) => {
                    curr = acc(curr, item_next);
                    rest = rest_next;
                    found_none = false ;
                },
                Err(nom::Err::Error(_)) => {
                    if found_none {
                        return Err(nom::Err::Error(E::from_error_kind(r, nom_error::ErrorKind::Many1)))
                    }
                    return Ok((r, curr))
                },
                Err(e) => {
                    return Err(e);
                },
            }
        }
    }
}

#[allow(dead_code)]
pub fn debug<I, T, E>(
    message: &'static str,
    parser: impl nom::Parser<I, T, E>,
) -> impl FnMut(I) -> nom::IResult<I, T, E>
where 
    T: std::fmt::Debug
{
    nom::combinator::map(parser, move |found| {
        println!("{} produced {:?}", message, found);
        found
    })
}

#[allow(dead_code)]
pub fn print_tokens(context: &'static str, amount: usize, tokens: &[Token]) {
    let names = tokens
        .iter()
        .take(amount)
        .map(|t| t.kind.name())
        .collect::<Vec<_>>();
    println!(
        "{}: next {} tokens: {:?}",
        context,
        amount, 
        names,
    );
}