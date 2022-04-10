use nom::{
    multi::count,
    IResult,
    multi::{fold_many_m_n},
    branch::alt,
    bytes::complete::{tag, take, take_while, take_while1, take_until},
    combinator::{value, rest},
    character::complete::alphanumeric1,
    combinator::opt,
    sequence::{tuple, preceded, delimited, terminated}};
use std::str;
use std::vec::Vec;
use nom_locate::{position};
use crate::parsers::{
    types::{CommentType, SymbolPosition, Span},
    base::Parser};

pub struct TsParser;

impl Parser for TsParser {
    fn parse<'a>(&self, file_input: Span<'a>) -> Vec<(SymbolPosition<'a>, SymbolPosition<'a>)> {
        println!("using ts parser");
        let mut all_funs: Vec<(SymbolPosition, SymbolPosition)> = Vec::new();
        let mut input = file_input;
        let it = std::iter::from_fn(move || {
            match get_symbol_pair(input) {
                Ok((i, fun)) => {
                    input = i;
                    Some(fun)
                }
                _ => None,
            }
        });
        all_funs.extend(it.into_iter());
        for (comment_position, function_position) in &all_funs {
            println!("start - {}, end - {}, fun_start - {}, fun_end - {}", comment_position.start.location_line(), comment_position.end.location_line(), function_position.start.location_line(), function_position.end.location_line());
        }
        return all_funs;
    }
}

#[derive(Debug)]
pub enum FunType {
    Normal,
    Arrow,
    Empty
}

pub fn multi_line_comment(input: Span) -> IResult<Span, (Span, Span, Span, Span, Span)> {

    let (input, start) = tag("/*")(input)?;

    let (input, start_pos) = position(input)?;

    let (input, body) = take_until("*/")(input)?;
    let (input, end) = tag("*/")(input)?;

    let (input, end_pos) = position(input)?;
    return Ok((input, (start, start_pos, body, end, end_pos)));
}

pub fn arrow_function(input: Span) -> IResult<Span, (Span, Span)> {
    let (input, (opening, _, body)) = tuple((
            tag("=>"),
            take_while(char::is_whitespace),
            alt((
                match_body_start,
                match_statement
                ))
            ))(input)?;

    Ok((input, (opening, body)))
}

pub fn normal_function(input: Span) -> IResult<Span, (Span, Span, Span, Span, Span, Span)> {
    let (input, declaration) = tag("function")(input)?;
    let (input, _) = take_while1(char::is_whitespace)(input)?;
    let (input, (param_start, param_body, param_end)) = get_params(input)?;
    let (input, _) = take_while(char::is_whitespace)(input)?;
    let (input, _element_type) = opt(get_type)(input)?;
    let (input, (body_start, body_end)) = match_body(input)?;

    Ok((input, (declaration, param_start, param_body, param_end, body_start, body_end)))
}

pub fn get_type(input: Span) -> IResult<Span, Span> {
    let (input, (_type_specifier, _, element_type, _)) = tuple((
        tag(":"),
        take_while(char::is_whitespace),
        take_while1(char::is_alphanumeric),
        take_while(char::is_whitespace)
    ))(input)?;
    Ok((input, element_type))
}

pub fn get_params(input: Span) -> IResult<Span, (Span, Span, Span)> {
    tuple((
        tag("("),
        take_until(")"),
        tag(")")
        ))(input)
}

pub fn match_statement(input: Span) -> IResult<Span, Span> {
    alphanumeric1(input)
}

pub fn match_body(input: Span) -> IResult<Span, (Span, Span)> {
    let (input, start) = match_body_start(input)?;
    let (input, start_pos) = position(input)?;
    let mut start_braces = 1;
    let mut end_braces = 0;

    let (input, end) = loop {
        let (input, end_brace_char) = alt((
                match_body_start,
                match_body_end,
                take(1usize)
                ))(input)?;
        println!("{}", end_brace_char.fragment());
        match end_brace_char.fragment() {
            &"{" => {
                start_braces += 1;
            },
            &"}" => {
                end_braces += 1;
            },
            _ => {}
        }

        if start_braces <= end_braces {
            let (input, pos) = position(input)?;
            break (input, pos);
        }
    };

    Ok((input, (start, end)))
}

pub fn match_body_end(input: Span) -> IResult<Span, Span> {
    tag("}")(input)
}

pub fn match_body_start(input: Span) -> IResult<Span, Span> {
    tag("{")(input)
}

pub fn get_symbol_pair(input: Span) -> IResult<Span, (SymbolPosition, SymbolPosition)> {
    let (input, (comment_start, comment_end)) = tuple((
        preceded(take_until("/*"), tag("/*")),
        preceded(take_until("*/"), tag("*/"))
    ))(input)?;

    let (_input, new_lines) =
        match fold_many_m_n(
           0,
           8, // search lines
           terminated(take_until("\n"), tag("\n")), // use newline combinator
           String::new,
           |mut joined_lines: String, line: Span| {
               joined_lines = format!("{}{}", joined_lines, line.fragment());
               joined_lines
           }
           )(input) {
            Ok((input, new_lines)) => {
                (input, new_lines)
            },
            Err(e) => {
                return Err(e);
            }
        };

    let (_input, (fun_type, comment_type)) = match get_symbol_type(new_lines.as_str()) {
        Ok((input, (fun_type, comment_type))) => (input, (fun_type, comment_type)),
        Err(_) => ("", (FunType::Empty, CommentType::Free))
        // Err(e) => ("", CommentType::Free)
    };
    println!("symbol type: {:?}", comment_type);

    let (input, (fun_start, fun_end)) = match comment_type {
        CommentType::Docstring => {
            println!("doing docstring check");
            let (input, fun_start) = get_symbol_start(input, fun_type)?;
            let (input, fun_end) = get_fun_close(input)?;
            (input, (fun_start, fun_end))
        }
        CommentType::Free => {
            let (input, _) = take_until("\n")(input)?;
            let (input, code_start) = position(input)?;
            let (input, _) = count(preceded(take_until("\n"), tag("\n")), 4)(input)?;
            let (input, code_end) = position(input)?;
            (input, (code_start, code_end))
        }
    };
    let comment_position = SymbolPosition {
        start: comment_start,
        end: comment_end
    };
    let function_position = SymbolPosition {
        start: fun_start,
        end: fun_end
    };
    Ok((input, (comment_position, function_position)))
}

// FunctionType
pub fn get_symbol_start(input: Span, fun_type: FunType) -> IResult<Span, Span> {
    match fun_type {
        FunType::Arrow => {
            let (input, _) = delimited(
                preceded(take_until("=>"), tag("=>")), take_while(char::is_whitespace), tag("{"))(input)?;
            let (input, pos) = position(input)?;
            return Ok((input, pos));
        },
        FunType::Normal => {
            let (input, _) = preceded(
                preceded(take_until(")"), tag(")")), preceded(take_until("{"), tag("{")))(input)?;
            let (input, pos) = position(input)?;
            return Ok((input, pos));
        },
        _ => unreachable!()
    };
}

pub fn get_symbol_type<'a>(input: &'a str) -> IResult<&'a str, (FunType, CommentType)> {
    println!("{}", input);
    let (input, fun) = alt((
            value("arrow",
        preceded(
            preceded(take_until("=>"), tag("=>")), preceded(take_while(char::is_whitespace), tag("{")))),
        value("normal", preceded( // char is whitespace is the reason - types
            preceded(take_until(")"), tag(")")), preceded(take_until("{"), tag("{")))),
            rest
    ))(input)?;
    let fun_type = match fun {
        "arrow" => (FunType::Arrow, CommentType::Docstring),
        "normal" => (FunType::Normal, CommentType::Docstring),
        _ => (FunType::Empty, CommentType::Free)
    };
    Ok((input, fun_type))
}

/// Gets the end position of a function, assuming you're already inside a function
/// Assumed you called this right after a tag of {
pub fn get_fun_close(input: Span) -> IResult<Span, Span> {
    let mut start_braces = 1;
    let mut end_braces = 0;
    let (input, end_pos) = loop {
        let (input, end_brace_char) = alt((
                    preceded(take_until("}"), tag("}")),
                    preceded(take_until("{"), tag("{"))
                ))(input)?;
        match *end_brace_char.fragment() {
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

