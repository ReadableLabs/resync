use nom::{
    multi::count,
    IResult,
    multi::{fold_many_m_n},
    branch::alt,
    bytes::complete::{tag, take_while, take_until},
    combinator::{value, rest},
    sequence::{tuple, preceded, delimited, terminated}};
use std::str;
use std::vec::Vec;
use nom_locate::{position};
use crate::parsers::{
    types::{CommentType, SymbolPosition, Span},
    base::Parser};

pub struct TsParser;

impl Parser for TsParser {
    fn parse<'a>(&self, file_input: Span<'a>) -> Vec<(SymbolPosition<'a>, SymbolPosition<'a>)> {
        let mut all_funs: Vec<(SymbolPosition, SymbolPosition)> = Vec::new();
        let mut input = file_input;
        let it = std::iter::from_fn(move || {
            match get_symbol_pair(input) {
                Ok((i, fun)) => {
                    input = i;
                    Some(fun)
                }
                _ => None,
            }
        });
        all_funs.extend(it.into_iter());
        for (comment_position, function_position) in &all_funs {
            println!("start - {}, end - {}, fun_start - {}, fun_end - {}", comment_position.start.location_line(), comment_position.end.location_line(), function_position.start.location_line(), function_position.end.location_line());
        }
        return all_funs;
    }
}

pub enum FunType {
    Normal,
    Arrow,
    Empty
}

pub fn get_symbol_pair(input: Span) -> IResult<Span, (SymbolPosition, SymbolPosition)> {
    let (input, (comment_start, comment_end)) = tuple((
        preceded(take_until("/*"), tag("/*")),
        preceded(take_until("*/"), tag("*/"))
    ))(input)?;

    let (_input, new_lines) =
        match fold_many_m_n(
           0,
           4, // search lines
           terminated(take_until("\n"), tag("\n")), // use newline combinator
           String::new,
           |mut joined_lines: String, line: Span| {
               joined_lines = format!("{}{}", joined_lines, line.fragment());
               joined_lines
           }
           )(input) {
            Ok((input, new_lines)) => {
                (input, new_lines)
            },
            Err(e) => {
                return Err(e);
            }
        };

    let (_input, (fun_type, comment_type)) = match get_symbol_type(new_lines.as_str()) {
        Ok((input, (fun_type, comment_type))) => (input, (fun_type, comment_type)),
        Err(_) => ("", (FunType::Empty, CommentType::Free))
        // Err(e) => ("", CommentType::Free)
    };

    let (input, (fun_start, fun_end)) = match comment_type {
        CommentType::Docstring => {
            let (input, fun_start) = get_symbol_start(input, fun_type)?;
            let (input, fun_end) = get_fun_close(input)?;
            (input, (fun_start, fun_end))
        }
        CommentType::Free => {
            let (input, _) = take_until("\n")(input)?;
            let (input, code_start) = position(input)?;
            let (input, _) = count(preceded(take_until("\n"), tag("\n")), 2)(input)?;
            let (input, code_end) = position(input)?;
            (input, (code_start, code_end))
        }
    };
    let comment_position = SymbolPosition {
        start: comment_start,
        end: comment_end
    };
    let function_position = SymbolPosition {
        start: fun_start,
        end: fun_end
    };
    Ok((input, (comment_position, function_position)))
}

// FunctionType
pub fn get_symbol_start(input: Span, fun_type: FunType) -> IResult<Span, Span> {
    match fun_type {
        FunType::Arrow => {
            let (input, _) = delimited(
                preceded(take_until("=>"), tag("=>")), take_while(char::is_whitespace), tag("{"))(input)?;
            let (input, pos) = position(input)?;
            return Ok((input, pos));
        },
        FunType::Normal => {
            let (input, _) = preceded(
                preceded(take_until(")"), tag(")")), take_until("{"))(input)?;
            let (input, pos) = position(input)?;
            return Ok((input, pos));
        },
        _ => unreachable!()
    };
}

pub fn get_symbol_type<'a>(input: &'a str) -> IResult<&'a str, (FunType, CommentType)> {
    println!("{}", input);
    let (input, fun) = alt((
            value("arrow",
        preceded(
            preceded(take_until("=>"), tag("=>")), preceded(take_while(char::is_whitespace), tag("{")))),
        value("normal", preceded( // char is whitespace is the reason - types
            preceded(take_until(")"), tag(")")), tag("{"))),
            rest
    ))(input)?;
    let fun_type = match fun {
        "arrow" => (FunType::Arrow, CommentType::Docstring),
        "normal" => (FunType::Normal, CommentType::Docstring),
        _ => (FunType::Empty, CommentType::Free)
    };
    Ok((input, fun_type))
}

/// Gets the end position of a function, assuming you're already inside a function
/// Assumed you called this right after a tag of {
pub fn get_fun_close(input: Span) -> IResult<Span, Span> {
    let mut start_braces = 1;
    let mut end_braces = 0;
    let (input, end_pos) = loop {
        let (input, end_brace_char) = alt((
                    preceded(take_until("}"), tag("}")),
                    preceded(take_until("{"), tag("{"))
                ))(input)?;
        match *end_brace_char.fragment() {
            "}" => {
                end_braces += 1;
            },
            "{" => {
                start_braces += 1;
            },
            _ => {} // replace with unreachable
        }

        if start_braces <= end_braces {
            break (input, end_brace_char);
        }
    };
    let (_input, fun_end) = position(end_pos)?; // we don't take this input because it returns the fragment of the end brace char
    Ok((input, fun_end))
}

