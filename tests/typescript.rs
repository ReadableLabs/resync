use resync::parsers::typescript::{arrow_function, multi_line_comment};
use resync::parsers::types::Span;

#[test]
fn arrow_body_test() {
    let (input, (opening, body)) = arrow_function(Span::new("=> {body}")).expect("Failed to parse function");
    assert_eq!(input.fragment(), &"body}");
    assert_eq!(opening.fragment(), &"=>");
    assert_eq!(body.fragment(), &"{");
}

#[test]
fn arrow_statement_test() {
    let (input, (opening, body)) = arrow_function(Span::new("=> statement; newtext")).expect("Failed to parse function");
    assert_eq!(input.fragment(), &"; newtext");
    assert_eq!(opening.fragment(), &"=>");
    assert_eq!(body.fragment(), &"statement");
}

#[test]
fn multi_line_comment_test() {
    let (input, (start, body, end)) = multi_line_comment(Span::new("/*\n * comment\n * multiline\n */")).expect("Failed to parse comment");
    assert_eq!(input.fragment(), &"");
    assert_eq!(start.fragment(), &"/*");
    assert_eq!(body.fragment(), &"\n * comment\n * multiline\n ");
    assert_eq!(end.fragment(), &"*/");
}
