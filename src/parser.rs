use nom::{
    IResult,
    error::VerboseError,
    branch::alt,
    bytes::complete::{tag, take_while, take, take_until, is_not},
    character::{is_hex_digit, is_alphabetic, is_space, is_alphanumeric},
    character::complete::{char, anychar},
    combinator::{map_res, value},
    error::ParseError,
    sequence::{tuple, preceded}};
use std::{thread, time};
use std::str;
use nom_locate::{position, LocatedSpan};

/// Warning: This was extraordinarily difficult for me to wrap my head around.
/// It's using the nom library. If you don't know it, I suggest you read
/// on https://github.com/Geal/nom or else you will have a bad time.

pub type Span<'a> = LocatedSpan<&'a str>;
/*
  class myClass() {
    // for class, check all variables which start with ) and see if there's an arrow or { right after them
    myFun1() { // use till not whitespace and check char if {
    }
    myFun2 = () => {

    asdg
    sagasd
    gasd
    gsda
    gdas
    gdsa
    g
    dsagads
    g}
    myFun2 = hi => {}
  }

  function myFun2() {
  }

  function() {}

  const myFun2 = () => {}
  const myFun2 = hi => {}
*/

pub enum FunctionType {
  Arrow,
  Normal,
  Empty, // dangerous
}

pub struct JsFunction<'a> {
    pub start:  Span<'a>,
    pub end:    Span<'a>
}

/// Gets the function type of a function.
/// Currently supports normal or arrow functions
pub fn get_fun_type(input: Span) -> IResult<Span, FunctionType> {
  let (input, t) = alt(( // match
    preceded(take_until("=>"), tag("=>")), // take until and tag
    // tuple get ) and then } and make sure no white space exist between
    preceded(take_until("function"), tag("function")) // get opening brace, alt closing brace and check if there's something before the opening. The arrow function will hopefully be run before, so no need to worry about accidentally getting an arrow function
  ))(input)?;

  println!("t: {}", *t.fragment());
  println!("input {}", input.fragment());
 let function_type = match *t.fragment() { // value because t fragment is a span
    "=>" => FunctionType::Arrow,
    "function" => FunctionType::Normal,
    _ => FunctionType::Empty, // throw error instead
  };
  Ok((input, function_type))
}

/// Gets the end position of a function, assuming you're already inside a function
/// Assumed you called this right after a tag of {
pub fn get_fun_close(input: Span) -> IResult<Span, Span> {
    let mut start_braces = 1;
    let mut end_braces = 0;
    let end_pos = loop {
        let (input, end_brace_char) = alt((
                    preceded(take_until("}"), tag("}")),
                    preceded(take_until("{"), tag("{"))
                ))(input)?;
        println!("end brace {}", end_brace_char.location_offset());
        println!("input 2: {}", input.fragment());
        match *end_brace_char.fragment() { // eof might be done automatically
            "{" => {
                start_braces += 1;
            },
            "}" => {
                println!("found end brace");
                end_braces += 1;
            },
            _ => {}
        }

        if start_braces <= end_braces {
            break end_brace_char;
        }
    };
    // println!("current input: {}", input.fragment());
    let (input, fun_end) = position(end_pos)?;
    Ok((input, fun_end))
}

/// Gets the range of a single function, assumes given a text file
pub fn get_fun_range(input: Span) -> IResult<Span, JsFunction> {
  let (input, fun_type) = get_fun_type(input)?; // check for error and do something if not
  match fun_type {
    FunctionType::Arrow => {
        let (input, spaces) = take_until("{")(input)?;
        // println!("input: {}", input.fragment());
        if spaces.fragment().trim().is_empty() { // then there is in fact a { with whitespace
            let (input, fun_start) = tag("{")(input)?;
            let (input, fun_start) = position(input)?;
            let (input, fun_end) = get_fun_close(input)?;
            println!("input: {}", input.location_offset());
            println!("{}", fun_end.location_offset());
            println!("fun start line: {} - fun end line: {}", fun_start.location_offset(), fun_end.location_offset());
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
    },
    FunctionType::Empty => {
        println!("Failed to find function. Skipping");
    }
  }
  return Ok((input, JsFunction { // should never run, if it does PLEASE report a bug
      start: Span::new("did not work"),
      end:   Span::new("did not work")
  }));
}

