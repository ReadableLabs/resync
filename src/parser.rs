use nom::{
    IResult,
    error::VerboseError,
    branch::alt,
    bytes::complete::{tag, take_while, take, take_until, is_not},
    character::{is_hex_digit, is_alphabetic, is_space, is_alphanumeric},
    character::complete::{char, anychar},
    combinator::{map_res, value, rest},
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

pub struct JsFunction<'a> { // make a c style comment parser (maybe)
    pub start:  Span<'a>,
    pub end:    Span<'a>,
    pub is_empty:  bool
}

/// Gets the function type of a function.
/// Currently supports normal or arrow functions
pub fn get_fun_type(input: Span) -> IResult<Span, FunctionType> {
  let (input, t) = alt((
    preceded(take_until("=>"), tag("=>")), // maybe make it delimited so you can tag for { here
    delimited(
        preceded(take_until(")"), tag(")")), take_while(char::is_whitespace), tag("{")),
    rest, // return rest of input as output - no function has been found

  ))(input)?;

  let trimmed_fragment = t.fragment().trim();
 let function_type = match trimmed_fragment { // the reason for trim, is because the take_while returns a ton of spaces, which we need to check for
    "=>" => FunctionType::Arrow,
    "" => FunctionType::Normal, // match space for function because of delimited returning the second arg
    _ => FunctionType::Empty, // throw error instead
  };
  Ok((input, function_type))
}

pub fn is_space_or_newline(chr: u8) -> bool { // made because I didn't feel like using the alt tag
    chr == b'\n' || chr == b' ' || chr == b'\t'
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
  let (input, fun_type) = get_fun_type(input)?; // check for error and do something if not
  match fun_type {
    FunctionType::Arrow => {
        let (input, spaces) = take_until("{")(input)?;
        if spaces.fragment().trim().is_empty() { // then there is in fact a { with whitespace
            let (input, fun_start) = tag("{")(input)?;
            let (input, fun_start) = position(input)?;
            let (input, fun_end) = get_fun_close(input)?;
            return Ok((input, JsFunction {
                start: fun_start,
                end: fun_end,
                is_empty: false,
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
            is_empty: false,
        }));
    },
    FunctionType::Empty => {
        return Ok((input, JsFunction {
            start:  Span::new("Empty"),
            end:    Span::new("Empty"),
            is_empty: true,
        }));
    }
  }
  return Ok((input, JsFunction { // should never run, if it does PLEASE report a bug
      start: Span::new("did not work"),
      end:   Span::new("did not work"),
      is_empty: true,
  }));
}

pub fn get_all_functions(file_input: Span) {
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
    for value in it {
        println!("function: {} - {}", value.start.location_line(), value.end.location_line());
    }
}

