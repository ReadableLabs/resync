use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_until},
    character::{is_hex_digit},
    combinator::{map_res, value},
    error::ParseError,
    sequence::tuple};
use nom::bytes::complete::take;
use std::str;

pub struct Color<'a> {
    pub hashtag:    &'a str,
    pub color:      &'a str,
}

pub fn get_fun_name(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("public")(input)
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