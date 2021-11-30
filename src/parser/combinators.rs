use nom::error as nom_error;
pub use crate::tokens::{Position, Token, TokenKind};

pub fn map_into<I, O1, O2, O3, E, F, G>(
    mut parser: F, 
    mut f: G,
) -> impl FnMut(I) -> nom::IResult<I, O3, E>
where
  F: nom::Parser<I, O1, E>,
  G: FnMut(O1) -> O2,
  O2: Into<O3>,
{
  move |input: I| {
    let (input, o1) = parser.parse(input)?;
    Ok((input, f(o1).into()))
  }
}

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

pub fn optional_delimited_list<I, O1, O2, O3, O4, E, P1, P2, P3, P4>(
    left_delim: P1,
    sep: P2,
    element: P3,
    right_delim: P4,
) -> impl FnMut(I) -> nom::IResult<I, Vec<O3>, E>
where
    I: Clone + nom::InputLength,
    E: nom::error::ParseError<I>,
    P1: nom::Parser<I, O1, E>,
    P2: nom::Parser<I, O2, E>,
    P3: nom::Parser<I, O3, E>,
    P4: nom::Parser<I, O4, E>,
{
    opt_or(
        nom::sequence::delimited(
            left_delim,
            nom::multi::separated_list1(sep, element),
            right_delim,
        ),
        Option::unwrap_or_default,
    )
}

// Note that unlike optional_delimited_list, this combinator
// will accept 0 or more params.
pub fn delimited_list<I, O1, O2, O3, O4, E, P1, P2, P3, P4>(
    left_delim: P1,
    sep: P2,
    element: P3,
    right_delim: P4,
) -> impl FnMut(I) -> nom::IResult<I, Vec<O3>, E>
where
    I: Clone + nom::InputLength,
    E: nom::error::ParseError<I>,
    P1: nom::Parser<I, O1, E>,
    P2: nom::Parser<I, O2, E>,
    P3: nom::Parser<I, O3, E>,
    P4: nom::Parser<I, O4, E>,
{
    nom::sequence::delimited(
        left_delim,
        nom::multi::separated_list0(sep, element),
        right_delim,
    )
}

pub fn opt_or<I, O, E, P, D>(
    parser: P,
    default: D,
) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    I: Clone,
    E: nom::error::ParseError<I>,
    P: nom::Parser<I, O, E>,
    D: FnMut(Option<O>) -> O
{
    nom::combinator::map(
        nom::combinator::opt(parser),
        default,
    )
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