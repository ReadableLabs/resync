use nom::{
    multi::count,
    IResult,
    error::VerboseError,
    multi::{fold_many_m_n},
    branch::alt,
    bytes::complete::{tag, take_while, take, take_until, is_not},
    character::{is_hex_digit, is_alphabetic, is_space, is_alphanumeric},
    character::complete::{char, anychar},
    combinator::{map_res, value, rest, map, map_parser, success},
    error::ParseError,
    sequence::{tuple, preceded, delimited, terminated}};
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

pub enum CommentType {
    WithParams,
    Normal // just a random multi line comment
}

pub enum FunctionType {
  Arrow,
  Normal,
  Empty, // dangerous
}

pub struct SymbolPosition<'a> {
    pub start:  Span<'a>,
    pub end:    Span<'a>
}

#[derive(Debug)]
pub enum FunType {
    Docstring,
    Free
}

pub struct SymbolRange<'a> {
    pub comment:    SymbolPosition<'a>,
    pub function:   SymbolPosition<'a>
}

pub struct JsFunction<'a> {
    pub start:  Span<'a>,
    pub end:    Span<'a>
}

pub fn get_fun(input: Span) -> IResult<Span, (SymbolPosition, SymbolPosition)> {
    /* input (comment.0|1)
     * tuple((
     *  tuple(("/**/"))
     *  alt((
     *    map((
     *    map((take("\n\n")), get_fun_start) // see if position is the same,
     *    |s: Span| (s, type::Function)
     *    ))
     *    map((
     *      take_next_five_lines_up,
     *      |s: Span| (s, type::Free)
     *    ))
     *  ))
     * ))
     */
    let (input, (comment_start, comment_end)) = tuple((
        preceded(take_until("/*"), tag("/*")),
        preceded(take_until("*/"), tag("*/"))
    ))(input)?;
     // failed here
     // just take 2 and joined the results of the vec. Match the result and return ok or no

     /*
     let (test, nput) = terminated(take_until("\n"), tag("\n"))(input)?;
     let (test, nput) = terminated(take_until("\n"), tag("\n"))(test)?;
     println!("input test - {}", nput.fragment());
     println!("input 2 {}", input);
     */
     /*
    let (_input, new_lines) = match count(take_until("\n"), 2)(input) {
        Ok((input, lines)) => {
            println!("input - {}", input);
            let mut joined = String::from("");
            for line in lines {
                println!("fragment {}", line.fragment());
                joined = format!("{}{}", joined, line.fragment());
                println!("{}", joined);
            }
            println!("joined - {}", joined);
            (input, joined)
        },
        Err(e) => {
            println!("Error");
            return Err(e);
        }
    };
    */

    let (_input, new_lines) =
        match fold_many_m_n(
           0,
           2,
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
                println!("error");
                return Err(e);
            }
        };

    let (_input, fun_type) = match get_symbol_type(new_lines.as_str()) {
        Ok((input, fun_type)) => (input, fun_type),
        Err(e) => ("", FunType::Free)
    };
    println!("symbol type - {:?}", fun_type);
    let (input, (fun_start, fun_end)) = match fun_type {
        FunType::Docstring => {
            let (input, fun_start) = get_symbol_start(input)?;
            let (input, fun_end) = get_fun_close(input)?;
            (input, (fun_start, fun_end))
        }
        FunType::Free => {
            let (input, _) = take_until("\n")(input)?; // get the next line since we're currently on comment
            let (input, code_start) = position(input)?;
            let (input, results) = count(preceded(take_until("\n"), tag("\n")), 2)(input)?;
            let (input, code_end) = position(input)?;
            (input, (code_start, code_end))
            // start is just next line
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
    /*
    let (input, ((comment_start, comment_end), code_start)) = tuple((
        tuple((
            take_until("/*"),
            take_until("*/")
        )),
        alt((
            map(
                preceded(
                    map_parser(
                        count(take_until("\n"), 2),
                        check_symbol
                    ),
                    get_symbol_start // so it actually gets the start - not just on result
                ),
                |s: Span| (s, FunType::Docstring)
            ),
            preceded((
                take_until("\n"), // get the next newline
                |s: Span| (s, FunType::Free) // if it's on last line of file there's no new line
            ))
        ))
    ))(input)?;
    Ok((input, (comment_start, comment_end), code_start))
        */
}

pub fn get_symbol_start(input: Span) -> IResult<Span, Span> {
    let (input, fun) = alt((
        delimited(
            preceded(take_until("=>"), tag("=>")), take_while(char::is_whitespace), tag("{")),
        delimited(
            preceded(take_until(")"), tag(")")), take_while(char::is_whitespace), tag("{")), // do the 5 lines here
        preceded(take_until("\n"), tag("\n"))
    ))(input)?;
    let (input, pos) = position(input)?;
    // let (input, pos) = position(input)?;
    Ok((input, pos))
}

pub fn get_symbol_type<'a>(input: &'a str) -> IResult<&'a str, FunType> {
    let (input, fun) = alt((
        preceded(
            preceded(take_until("=>"), tag("=>")), preceded(take_while(char::is_whitespace), tag("{"))),
        preceded( // maybe use value
            preceded(take_until(")"), tag(")")), preceded(take_while(char::is_whitespace), tag("{"))), // do the 5 lines here
            rest
    ))(input)?;
    let fun_type = match fun {
        "{" => FunType::Docstring,
        _ => FunType::Free
    };
    // let (input, pos) = position(input)?;
    Ok((input, fun_type))
}

/// Gets the function type of a function.
/// Currently supports normal or arrow functions
pub fn get_fun_and_comment(input: Span) -> IResult<Span, Span> {
    // see if it tags function or comment first, then if function check for whitespace.
  let (input, fun) = alt(( // tuple maybe?? comment + code
    alt((
        delimited(
        preceded(take_until("=>"), tag("=>")), take_while(char::is_whitespace), tag("{")), // maybe make it delimited so you can tag for { here
        delimited(
            preceded(take_until(")"), tag(")")), take_while(char::is_whitespace), tag("{")),
        )),
        rest // not a function
  ))(input)?;
/*
  let trimmed_fragment = t.fragment().trim();
 let function_type = match trimmed_fragment { // the reason for trim, is because the take_while returns a ton of spaces, which we need to check for
    "=>" => FunctionType::Arrow,
    "" => FunctionType::Normal, // match space for function because of delimited returning the second arg
  };
  */
  let (input, pos) = position(input)?;
  Ok((input, pos))
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
pub fn get_fun_range(input: Span) -> IResult<Span, (SymbolPosition, SymbolPosition)> {
    let (input, (comment_position, function_position)) = get_fun(input)?;
    // println!("start - {}, end - {}, fun_start - {}, fun_end - {}", comment_position.start.location_line(), comment_position.end.location_line(), function_position.start.location_line(), function_position.end.location_line());
    Ok((input, (comment_position, function_position)))
  // let (input, fun_start) = get_fun_and_comment(input)?; // check for error and do something if not
  /*
  match fun_type { // try return match function type and just doing Ok(())
    FunctionType::Arrow => {
        let (input, spaces) = take_until("{")(input)?;
        if spaces.fragment().trim().is_empty() { // then there is in fact a { with whitespace
            let (input, fun_start) = tag("{")(input)?;
            let (input, fun_start) = position(input)?;
            let (input, fun_end) = get_fun_close(input)?;
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
        let (input, fun_start) = position(input)?;
        let (input, fun_end) = get_fun_close(input)?;
        return Ok((input, JsFunction {
            start: fun_start,
            end: fun_end,
        }));
    },
    FunctionType::Empty => {
        return Ok((input, JsFunction {
            start:  Span::new("Empty"),
            end:    Span::new("Empty"),
        }));
    }
  }
  */
  /*
  let (input, fun_end) = get_fun_close(input)?;
  Ok((input, JsFunction {
      start: fun_start,
      end: fun_end
  }))
  */
    /*
    Ok((input, JsFunction {
        start: comment_start,
        end: comment_end
    }))
    */
}

pub fn contains_comment(/* get all comments and check if end range of comment is above start range of this */) { // option
    // get comment from start range of the function
}

pub fn get_all_functions(file_input: Span) {
    let mut input = file_input;
    let it = std::iter::from_fn(move || {
        // global state, if comment if adds to global vec. the function will always be comparing to
        // the last item in that linked list, to see if the function has a comment on top of it.
        // If it does, it will be added to the vector because it will be Ok will return something
        match get_fun_range(input) {
            Ok((i, fun)) => {
                // make it check for commnet here, and get one if there is any from the file_input
                // span
                input = i;
                Some(fun)
            }
            _ => None,
        }
    });
    for (comment_position, function_position) in it {
        println!("start - {}, end - {}, fun_start - {}, fun_end - {}", comment_position.start.location_line(), comment_position.end.location_line(), function_position.start.location_line(), function_position.end.location_line());
    }
    let (input, (comment, function)) = get_fun_range(file_input).unwrap();
}

