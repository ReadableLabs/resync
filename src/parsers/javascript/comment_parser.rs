use nom::{
    sequence::{preceded, tuple, delimited}, IResult, bytes::complete::{take_until, tag},
    branch::alt, combinator::rest
};
use nom_locate::{LocatedSpan, position};
use syn::PredicateLifetime;

use crate::parsers::types::{SymbolSpan, LineSpan};

pub type NomSpan<'a> = LocatedSpan<&'a str>;

fn start(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    let (input, start) = preceded(take_until("/*"), tag("/*"))(input)?;
    let (input, pos) = position(input)?;
    Ok((input, pos))
}

pub fn body(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    let (input, body) = take_until("*/")(input)?;
    let (input, pos) = position(input)?;
    Ok((input, pos))
}
pub fn end(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    let (input, end) = tag("*/")(input)?;
    let (input, pos) = position(input)?;
    Ok((input, pos))
}

pub fn tag_start(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    preceded(take_until("/"), tag("/"))(input)
}

pub fn tag_body(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    alt((
        // match since you need start pos
        preceded(tag("/"), take_until("\n")),
        preceded(preceded(tag("*"), take_until("*/")), tag("*/")),
        // two preceded
        rest
    ))(input)
}

pub fn get_comment(input: NomSpan) -> IResult<NomSpan, SymbolSpan> {
    let (input, (start, body, end)) = tuple((start, body, end))(input)?;
    let start_line = start.location_line();
    let start_char = start.get_column();

    let end_line = end.location_line();
    let end_char = end.get_column();

    let symbol = SymbolSpan {
        start: LineSpan {
            line: usize::try_from(start_line).unwrap(),
            character: start_char
        },
        end: LineSpan {
            line: usize::try_from(end_line).unwrap(),
            character: end_char
        }
    };

    Ok((input, symbol))
}

pub fn parse_comments(text: &str) -> Vec<SymbolSpan> {
    let mut input = NomSpan::new(text);

    let it = std::iter::from_fn(move || {
        match get_comment(input) {
            Ok((i, comment)) => {
                input = i;
                Some(comment)
            },
            _ => None,
        }
    });

    return it.collect();
}