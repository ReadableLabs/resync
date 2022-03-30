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
    // pub declarator: Span<'a>,
    // pub arg_start:  Span<'a>, // &'a[u8],
    // pub arg_end:    Span<'a>,
    // pub end:        Span<'a>
    pub declaration:  Span<'a>,
    pub start_param:  Span<'a>,
    pub end_param:    Span<'a>,
    pub start_body:   Span<'a>,
    pub start_pos:    Span<'a>,
    pub end_body:     Span<'a>,
    pub end_pos:      Span<'a>
 // output file name with : number for easy click
}

pub fn get_fun_name(input: Span) -> IResult<Span, CStyleFunction> {
    /*
    let (input, main) = tag("main() {")(input)?;
    let (input, pos) = position(input)?;
    let (input, body) = take_until("}")(input)?;
    let (input, end) =  tag("}")(input)?;
    */
    /*
    let (input, (declarator, arg_start, arg_end, start, start_pos, end, end_pos)) = tuple((
 //       alt((const_fun, function_fun, private_fun, public_fun)),
        take_until("function"),
        take_until("("),
        take_until(")"),
        take_until("{"),
        position,
        take_until("}"),
        position
    ))(input)?;
    */
    let (input, declarator) = take_until("function")(input)?;
    let (input, arg_start) = take_until("(")(input)?;
    let (input, arg_end) = take_until(")")(input)?;
    let (input, body_start) = take_until("{")(input)?;
    let (input, start_pos) = position(input)?;
    let (input, body_end) = take_until("}")(input)?;
    let (input, end_pos) = position(input)?;
    // println!("{} - start: {} - end", std::str::from_utf8(start_pos.fragment()).unwrap(), std::str::from_utf8(end_pos.fragment()).unwrap());
    // let (input, (declaration, start_param, end_param, start_body, end_body)) = tuple((
    //   tag("function"),
    //   tag("("),
    //   tag(")"),
    //   tag("{"),
    //   tag("}"),
    // ))(input)?;
    // let (input, pos) = position(input)?;
    // println!("{}", std::str::from_utf8(input).unwrap());
    Ok((input, CStyleFunction {
      declaration: declarator,
      start_param: arg_start,
      end_param: arg_end,
      start_body: body_start,
      start_pos: start_pos,
      end_body: body_end,
      end_pos: end_pos
    }))
    // Ok((input, CStyleFunction {
    //     declarator: declarator,
    //     arg_start: arg_start,
    //     arg_end: arg_end,
    //     end: end
    // }))
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
