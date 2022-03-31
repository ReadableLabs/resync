use nom::{
    IResult,
    error::VerboseError,
    branch::alt,
    bytes::complete::{tag, take_while, take_until},
    character::{is_hex_digit, is_alphabetic, is_space, is_alphanumeric},
    character::complete::char,
    combinator::{map_res, value},
    error::ParseError,
    sequence::tuple};
use nom::bytes::complete::take;
use std;
use std::str;
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;
/*
  class myClass() {
    myFun1() { // use till not whitespace and check char if {
    }
    myFun2 = () => {}
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
  None, // dangerous
}

pub struct CStyleFunction<'a> {
    pub declaration:  Span<'a>,
    pub start_param:  Span<'a>,
    pub end_param:    Span<'a>,
    pub start_body:   Span<'a>,
    pub start_pos:    Span<'a>,
    pub end_body:     Span<'a>,
    pub end_pos:      Span<'a>
 // output file name with : number for easy click
}

/// Gets the function type of a function.
/// Because ECMA functions can be two types, we need to check
/// Which type the function is before parsing it.
pub fn get_fun_type(input: Span) -> IResult<Span, FunctionType> {
  let (input, t) = alt(( // match
    take_until("=>"),
    take_until("function")
  ))(input)?;

 let function_type = match *t.fragment() {
    "=>" => FunctionType::Arrow,
    "function" => FunctionType::Normal,
    _ => FunctionType::None, // throw error instead
  };
  Ok((input, function_type))
}

/// Gets the function range, given the whole text file
pub fn get_fun_range(input: Span) {
  let (input, fun_type) = get_fun_type(input).unwrap(); // check for error and do something if not
  match fun_type {
    FunctionType::Arrow => {
      // parse arrow function
    },
    FunctionType::Normal => {
    },
    FunctionType::None => {
    }
  }
}

// get comment ranges as well as function ranges so look for comments first
pub fn get_fun_name(input: Span) -> IResult<Span, CStyleFunction> {
    let (input, declarator) = take_until("function")(input)?;
    let (input, arg_start) = take_until("(")(input)?;
    let (input, arg_end) = take_until(")")(input)?;
    let (input, body_start) = take_until("{")(input)?;
    let (input, start_pos) = position(input)?;
    let (input, body_end) = take_until("}")(input)?;
    let (input, end_pos) = position(input)?;
    Ok((input, CStyleFunction {
      declaration: declarator,
      start_param: arg_start,
      end_param: arg_end,
      start_body: body_start,
      start_pos: start_pos,
      end_body: body_end,
      end_pos: end_pos
    }))
}

/*
pub fn hex_color<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, E> {
    value(
      (), // Output is thrown away.
      tuple((
        tag("(*"),
        take_until("*)"),
        tag("*)")
      ))
    )(i)
  }
*/
