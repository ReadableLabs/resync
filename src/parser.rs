use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_until},
    character::{is_hex_digit, is_alphabetic},
    combinator::{map_res, value},
    error::ParseError,
    sequence::tuple};
use nom::bytes::complete::take;
use std::str;
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a [u8]>;

pub struct CStyleFunction<'a> {
    pub position: Span<'a>,
    pub start:  Span<'a>, // &'a[u8],
    pub mid:    Span<'a>,
    pub end:    Span<'a>,
 // output file name with : number for easy click
}

pub fn get_fun_name(input: Span) -> IResult<Span, CStyleFunction> {
    let (input, main) = tag("main() {")(input)?;
    let (input, pos) = position(input)?;
    let (input, body) = take_until("}")(input)?;
    let (input, end) =  tag("}")(input)?;
    /*
    let (input, (main, body, end,)) = tuple((
        tag("main() {"),
        take_until("}"),
        tag("}")
    ))(input)?;
    */
    Ok((input, CStyleFunction {position: pos, start: main, mid: body, end: end}))
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
