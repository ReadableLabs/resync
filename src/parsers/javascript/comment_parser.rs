use nom::{
    sequence::{preceded, tuple}, IResult, combinator::iterator, bytes::complete::{take_until, take_while, tag}, character::{complete::multispace0, is_alphabetic, is_newline}
};
use std::collections::HashMap;
use nom_locate::{LocatedSpan, position};

use crate::parsers::types::{SymbolSpan, LineSpan};

pub type NomSpan<'a> = LocatedSpan<&'a str>;

fn start(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    let (input, _) = preceded(take_until("/*"), tag("/*"))(input)?;
    let (input, pos) = position(input)?;
    Ok((input, pos))
}

pub fn body(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    let (input, _) = take_until("*/")(input)?;
    let (input, pos) = position(input)?;
    Ok((input, pos))
}

pub fn end(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    let (input, _) = tag("*/")(input)?;
    let (input, pos) = position(input)?;
    Ok((input, pos))
}

pub fn get_docstring(input: NomSpan) -> IResult<NomSpan, SymbolSpan> {
    let (input, (start, _, end)) = tuple((start, body, end))(input)?;
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

pub fn get_inline_start(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    let (input, start) = preceded(take_until("//"), tag("//"))(input)?;
    let (input, pos) = position(input)?;

    Ok((input, pos))
}

pub fn get_inline_end(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    let mut it = iterator(
        input,
        get_body
    );
    
    let parsed: Vec<NomSpan> = it.collect();

    let res: IResult<_, _> = it.finish();
    let output = res.unwrap().0;
    let (input, end_pos) = position(output)?;

    Ok((input, end_pos))
}

pub fn get_body(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    let (input, next_line) = preceded(take_until("\n"), tag("\n"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, comment) = tag("//")(input)?;

    let (input, pos) = position(input)?;

    Ok((input, pos))
}

pub fn get_inline(mut input: NomSpan) -> IResult<NomSpan, SymbolSpan> {
    let (input, start) = get_inline_start(input)?;
    let (input, end) = get_inline_end(input)?;

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
    let mut comments: Vec<SymbolSpan> = Vec::new();
    comments.append(&mut parse_docstring(&text));
    comments.append(&mut parse_inline(&text));

    return comments;
}

pub fn parse_docstring(text: &str) -> Vec<SymbolSpan> {
    let mut input = NomSpan::new(text);

    let it = std::iter::from_fn(move || {
        match get_docstring(input) {
            Ok((i, comment)) => {
                input = i;
                Some(comment)
            },
            _ => None,
        }
    });

    return it.collect();
}

pub fn parse_inline(text: &str) -> Vec<SymbolSpan> {
    let mut input = NomSpan::new(text);

    let it = std::iter::from_fn(move || {
        match get_inline(input) {
            Ok((i, comment)) => {
                input = i;
                Some(comment)
            },
            _ => None,
        }
    });

    return it.collect();
}
