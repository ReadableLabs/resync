use nom::{
    multi::count,
    IResult,
    error::VerboseError,
    multi::{fold_many_m_n},
    branch::alt,
    bytes::complete::{tag, take_while, take, take_until, is_not},
    character::{is_hex_digit, is_alphabetic, is_space, is_alphanumeric},
    character::complete::{char, anychar},
    combinator::{map_res, value, rest, map, map_parser, success},
    error::ParseError,
    sequence::{tuple, preceded, delimited, terminated}};
use std::{thread, time};
use std::str;
use std::vec::Vec;
use nom_locate::{position, LocatedSpan};
use crate::types::{CommentType, SymbolPosition, Span};

/*
 * If you are not familiar with nom, please check it out on GitHub. That is how the parsing is
 * done. The parser is not meant to get anything such as args, only function ranges. Args may be
 * implemented in a future update, but for now it seems like a whole lot of work for something
 * miniscule.
 */

/*
 * This file can parse the following functions
 * myFun2() {}
 * (public/private/static/return type) myFun2() {}
 * const myFun2 = () => {}
 * const myFun2 = arg => {}
*/

pub enum FunType {
    Normal,
    Arrow,
    Empty
}

pub fn get_fun(input: Span) -> IResult<Span, (SymbolPosition, SymbolPosition)> {
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
        Err(e) => ("", (FunType::Empty, CommentType::Free))
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
            let (input, results) = count(preceded(take_until("\n"), tag("\n")), 2)(input)?;
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
            let (input, fun) = delimited(
                preceded(take_until("=>"), tag("=>")), take_while(char::is_whitespace), tag("{"))(input)?;
            let (input, pos) = position(input)?;
            return Ok((input, pos));
        },
        FunType::Normal => {
            let (input, fun) = delimited(
                preceded(take_until(")"), tag(")")), take_while(char::is_whitespace), tag("{"))(input)?;
            let (input, pos) = position(input)?;
            return Ok((input, pos));
        },
        _ => unreachable!()
    };
    /*
    let (input, fun) = alt((
        delimited(
            preceded(take_until("=>"), tag("=>")), take_while(char::is_whitespace), tag("{")),
        delimited(
            preceded(take_until(")"), tag(")")), take_while(char::is_whitespace), tag("{")),
        preceded(take_until("\n"), tag("\n"))
    ))(input)?;
    */
}

pub fn get_symbol_type<'a>(input: &'a str) -> IResult<&'a str, (FunType, CommentType)> {
    println!("{}", input);
    let (input, fun) = alt((
            value("arrow",
        preceded(
            preceded(take_until("=>"), tag("=>")), preceded(take_while(char::is_whitespace), tag("{")))),
        value("normal", preceded( // char is whitespace is the reason - types
            preceded(take_until(")"), tag(")")), preceded(take_while(char::is_whitespace), tag("{")))),
            rest
    ))(input)?;
    let fun_type = match fun {
        "arrow" => (FunType::Arrow, CommentType::Docstring),
        "normal" => (FunType::Normal, CommentType::Docstring),
        _ => (FunType::Empty, CommentType::Free)
    };
    Ok((input, fun_type))
}

/// Gets the function type of a function.
/// Currently supports normal or arrow functions
pub fn get_fun_and_comment(input: Span) -> IResult<Span, Span> {
  let (input, fun) = alt(( // tuple maybe?? comment + code
    alt((
        delimited(
        preceded(take_until("=>"), tag("=>")), take_while(char::is_whitespace), tag("{")),
        delimited(
            preceded(take_until(")"), tag(")")), take_while(char::is_whitespace), tag("{")),
        )),
        rest
  ))(input)?;
  let (input, pos) = position(input)?;
  Ok((input, pos))
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

/// Gets the range of a single function, assumes given a text file
pub fn get_fun_range(input: Span) -> IResult<Span, (SymbolPosition, SymbolPosition)> {
    let (input, (comment_position, function_position)) = get_fun(input)?;
    Ok((input, (comment_position, function_position)))
}

pub fn get_all_functions(file_input: Span) -> Vec<(SymbolPosition, SymbolPosition)> {
    let mut all_funs: Vec<(SymbolPosition, SymbolPosition)> = Vec::new();
    let mut input = file_input;
    let it = std::iter::from_fn(move || {
        match get_fun_range(input) {
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
