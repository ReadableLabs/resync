use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_until},
    character::{is_hex_digit, is_alphabetic},
    combinator::{map_res, value},
    error::ParseError,
    sequence::tuple};
use nom::bytes::complete::take;
use std::str;

pub struct CStyleFunction<'a> {
    pub start:  &'a[u8],
    pub mid:    &'a[u8],
    pub end:    &'a[u8],
 // output file name with : number for easy click
}

pub fn get_fun_name<'a>(input: &'a [u8]) -> IResult<&'a [u8], CStyleFunction> {
    let (input, (main, body, end)) = tuple((
        tag("main() {"),
        take_until("}"),
        tag("}")
    ))(input)?;
    Ok((input, CStyleFunction {start: main, mid: body, end: end}))
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