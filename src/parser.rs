use nom::{
    IResult,
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

pub type Span<'a> = LocatedSpan<&'a [u8]>;
/// Possible functions I'm parsing for. Maybe it's possible to get typescript and js in one parser
/// const hi = () => {}
/// function myFun2(arg1, arg2) {}
/// private myFun2() {}
/// public myFun2() {} make sure it does not include a semicolon, and the next character is an
/// opening brace
/// myFun2() {}


pub fn const_fun(input: Span) -> IResult<Span, Span> {
    tag("const")(input)
}

pub fn function_fun(input: Span) -> IResult<Span, Span> {
    tag("function")(input)
}

pub fn private_fun(input: Span) -> IResult<Span, Span> {
    tag("private")(input)
}

pub fn public_fun(input: Span) -> IResult<Span, Span> {
    tag("public")(input)
}

pub fn beginning_args(input: Span) -> IResult<Span, Span> {
    tag("(")(input)
}


pub struct CStyleFunction<'a> {
    pub declarator: Span<'a>,
    pub arg_start:  Span<'a>, // &'a[u8],
    pub arg_end:    Span<'a>,
    pub end:        Span<'a>
 // output file name with : number for easy click
}

pub fn get_fun_name(input: Span) -> IResult<Span, CStyleFunction> {
    /*
    let (input, main) = tag("main() {")(input)?;
    let (input, pos) = position(input)?;
    let (input, body) = take_until("}")(input)?;
    let (input, end) =  tag("}")(input)?;
    */
    let (input, (declarator, arg_start, arg_end, end)) = tuple((
 //       alt((const_fun, function_fun, private_fun, public_fun)),
        tag("function"),
        tag("("),
        tag(")"),
        tag("}")
    ))(input)?;
    let (input, pos) = position(input)?;
    Ok((input, CStyleFunction {
        declarator: declarator,
        arg_start: arg_start,
        arg_end: arg_end,
        end: end
    }))
    // println!("{} {} {}", str::from_utf8( start).unwrap(), str::from_utf8(mid).unwrap(), str::from_utf8(end).unwrap());
    // tag("main")(input)
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
