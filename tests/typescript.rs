use resync::parsers::typescript::{arrow_function, multi_line_comment, get_type, get_params, match_body};
use resync::parsers::types::Span;

#[test]
fn multi_line_comment_test() {
    let (input, (start, start_pos, body, end, end_pos)) = multi_line_comment(Span::new("/*\n * comment\n * multiline\n */")).expect("Failed to parse comment");
    assert_eq!(input.fragment(), &"");
    assert_eq!(start.fragment(), &"/*");
    assert_eq!(body.fragment(), &"\n * comment\n * multiline\n ");
    assert_eq!(end.fragment(), &"*/");
}

#[test]
fn type_test() {
    let (input, element_type) = get_type(Span::new(": string")).expect("Failed to parse element type");
    assert_eq!(input.fragment(), &"");
    assert_eq!(input.fragment(), &"");
    assert_eq!(element_type.fragment(), &"string");
}

/*
#[test]
fn params_test() {
    let (input, (open_param, body, close_param)) = get_params(Span::new("(aoaidshgoidsahgasdgoi)")).expect("Failed to parse params");
    assert_eq!(input.fragment(), &"");
    assert_eq!(open_param.fragment(), &"(");
    assert_eq!(body.fragment(), &"aoaidshgoidsahgasdgoi");
    assert_eq!(close_param.fragment(), &")");
}
*/

#[test]
fn match_body_test() {
    let (input, (start, end)) = match_body(Span::new("{hello if() {ioajsdg}\n}")).expect("Failed to parse body");
    assert_eq!(input.fragment(), &"");
    assert_eq!(start.get_column(), 2);
    assert_eq!(end.location_line(), 2)
}

#[test]
fn arrow_function_test() {
    let (input, (opening, (start, end))) = arrow_function(Span::new("=> {hello my name is joe. if() {}.ok\n}")).expect("Failed to parse arrow function");
    assert_eq!(opening.fragment(), &"=>");
    assert_eq!(start.location_line(), 1);
    assert_eq!(end.location_line(), 2);
}


