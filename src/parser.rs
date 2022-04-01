use nom::{
    IResult,
    error::VerboseError,
    branch::alt,
    bytes::complete::{tag, take_while, take_until},
    character::{is_hex_digit, is_alphabetic, is_space, is_alphanumeric},
    character::complete::{char, anychar},
    combinator::{map_res, value},
    error::ParseError,
    sequence::tuple};
use nom::bytes::complete::take;
use std;
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
    take_until("function") // match something for the ones in the class
  ))(input)?;

  println!("t: {}", *t.fragment());
 let function_type = match *t.fragment() {
    "=>" => FunctionType::Arrow,
    "function" => FunctionType::Normal,
    _ => FunctionType::Empty, // throw error instead
  };
  Ok((input, function_type))
}

/// Assumed you called this right after a tag of {
pub fn get_until_fn_close(input: Span) -> IResult<Span, Span> {
    let mut start = 1;
    let mut end = 0;
    while start < end {
        let (input, current_char) = anychar(input)?;
        match current_char {
            '{' => {
                start += 1;
            },
            '}' => {
                end += 1;
            }
        }
    }
    let (input, end_pos) = position(input)?;
    Ok((input, end_pos)); // end pos should be the same as input so this is useless
}

/// Gets the function range, given the whole text file
//                                          input, pos
pub fn get_fun_range(input: Span) -> IResult<Span, JsFunction> {
  let (input, fun_type) = get_fun_type(input).unwrap(); // check for error and do something if not
  match fun_type {
    FunctionType::Arrow => {
        let (input, spaces) = take_until("{")(input)?;
        if spaces.fragment().trim().is_empty() { // then there is in fact a { with whitespace
            let (input, fun_start) = tag("{")(input)?;
            let (input, fun_start) = position(input)?;
            let mut start_braces = 1;
            let mut end_braces = 0;
            while start_braces > end_braces {
                let (input, current_char) = anychar(input)?;
                match current_char { // eof might be done automatically
                    '{' => {
                        start_braces += 1;
                    },
                    '}' => {
                        end_braces += 1;
                    },
                    _ => {}
                }
            }
            println!("current input: {}", input.fragment());
            let (input, fun_end) = position(input)?;
            return Ok((input, JsFunction {
                start: fun_start,
                end: fun_end,
            }));
        }
        else {
            println!("Not empty, skipping");
        }
        // might be bad
      // parse arrow function
    },
    FunctionType::Normal => {
    },
    FunctionType::Empty => {
        println!("empty");
    }
  }
    return Ok((input, JsFunction {
        start: Span::new("did not work"),
        end:   Span::new("did not work")
    }));
}

// get comment ranges as well as function ranges so look for comments first
pub fn get_fun_name(input: Span) -> IResult<Span, CStyleFunction> { // change to result and unwrap or just frekaing stop
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
