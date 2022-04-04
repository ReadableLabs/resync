use nom::{
    multi::count,
    IResult,
    error::VerboseError,
    branch::alt,
    bytes::complete::{tag, take_while, take, take_until, is_not},
    character::{is_hex_digit, is_alphabetic, is_space, is_alphanumeric},
    character::complete::{char, anychar},
    combinator::{map_res, value, rest, map_parser},
    error::ParseError,
    sequence::{tuple, preceded, delimited}};
use std::{thread, time};
use std::str;
use std::vec::Vec;
use nom_locate::{position, LocatedSpan};

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

pub type Span<'a> = LocatedSpan<&'a str>;

pub enum FunctionType {
  Arrow,
  Normal,
  Empty, // dangerous
}

pub struct SymbolPosition<'a> {
    pub start:  Span<'a>,
    pub end:    Span<'a>
}

pub struct SymbolRange<'a> {
    pub comment:    SymbolPosition<'a>,
    pub function:   SymbolPosition<'a>
}

pub struct JsFunction<'a> {
    pub start:  Span<'a>,
    pub end:    Span<'a>
}

/// Gets the function type of a function.
/// Currently supports normal or arrow functions
pub fn get_fun_and_comment(input: Span) -> IResult<Span, Span> {
  let (input, (comment_start, comment_end) = tuple(take_until("/*"), take_until("*/"))(input)?;
  let (input, new_line) = count(take_until("\n", 2))(input)?;
  let (_input, _) =
  alt((
    delimited(
      preceded(take_until("=>", tag("=>")), take_while(char::is_whitespace), tag("{")),
      delimited(
        preceded(take_until(")"), tag(")")), take_while(char::is_whitespace), tag("{"))
      )
  ))(new_line)?; // may have to check if position is right
  let (input, fun_start) =
  alt((
    delimited(
      preceded(take_until("=>", tag("=>")), take_while(char::is_whitespace), tag("{")),
      delimited(
        preceded(take_until(")"), tag(")")), take_while(char::is_whitespace), tag("{"))
      )
  ))(input)?; // may have to check if position is right
  let (input, fun_end) = get_fun_close(input)?;
  // do it to main input if that succeeded
  /*
  let (input, (comment, fun)) = alt(( // tuple maybe?? comment + code
    delimited(
    preceded(take_until("=>"), tag("=>")), take_while(char::is_whitespace), tag("{")), // maybe make it delimited so you can tag for { here
    delimited(
        preceded(take_until(")"), tag(")")), take_while(char::is_whitespace), tag("{")),
  ))(input)?;
  */
/*
  let trimmed_fragment = t.fragment().trim();
 let function_type = match trimmed_fragment { // the reason for trim, is because the take_while returns a ton of spaces, which we need to check for
    "=>" => FunctionType::Arrow,
    "" => FunctionType::Normal, // match space for function because of delimited returning the second arg
  };
  */
  let (input, pos) = position(input)?;
  Ok((input, pos))
}

/// Gets the end position of a function, assuming you're already inside a function
/// Assumed you called this right after a tag of {
pub fn get_fun_close(input: Span) -> IResult<Span, Span> {
    let mut start_braces = 1;
    let mut end_braces = 0;
    let (input, end_pos) = loop {
        let (input, end_brace_char) = alt(( // right here, input isn't passed back
                    preceded(take_until("}"), tag("}")),
                    preceded(take_until("{"), tag("{"))
                ))(input)?;
        match *end_brace_char.fragment() { // eof might be done automatically
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
pub fn get_fun_range(input: Span) -> IResult<Span, JsFunction> {
  let (input, fun_start) = get_fun_and_comment(input)?; // check for error and do something if not
  /*
  match fun_type { // try return match function type and just doing Ok(())
    FunctionType::Arrow => {
        let (input, spaces) = take_until("{")(input)?;
        if spaces.fragment().trim().is_empty() { // then there is in fact a { with whitespace
            let (input, fun_start) = tag("{")(input)?;
            let (input, fun_start) = position(input)?;
            let (input, fun_end) = get_fun_close(input)?;
            return Ok((input, JsFunction {
                start: fun_start,
                end: fun_end,
            }));
        }
        else {
            println!("There was something between the arrow and the opening brace. Skipping");
        }
    },
    FunctionType::Normal => {
        let (input, fun_start) = position(input)?;
        let (input, fun_end) = get_fun_close(input)?;
        return Ok((input, JsFunction {
            start: fun_start,
            end: fun_end,
        }));
    },
    FunctionType::Empty => {
        return Ok((input, JsFunction {
            start:  Span::new("Empty"),
            end:    Span::new("Empty"),
        }));
    }
  }
  */
  let (input, fun_end) = get_fun_close(input)?;
  Ok((input, JsFunction {
      start: fun_start,
      end: fun_end
  }))
}

pub fn get_all_functions(file_input: Span) {
    let mut input = file_input;
    let it = std::iter::from_fn(move || {
        match get_fun_range(input) {
            Ok((i, fun)) => {
                // make it check for commnet here, and get one if there is any from the file_input
                // span
                input = i;
                Some(fun)
            }
            _ => None,
        }
    });
    for value in it {
        println!("function: {} - {}", value.start.location_line(), value.end.location_line());
    }
}

