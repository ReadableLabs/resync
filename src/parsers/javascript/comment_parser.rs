use nom::{
    sequence::preceded, IResult, bytes::complete::{take_until, tag},
};
use nom_locate::LocatedSpan;

pub type NomSpan<'a> = LocatedSpan<&'a str>;

fn start(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    preceded(take_until("/*"), tag("/*"))(input)
}

pub fn body(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    take_until("*/")(input)
}
pub fn end(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    tag("*/")(input)
}

pub fn get_comment(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    panic!("Not implemented");
}

pub fn parse_comments(text: &str) {
    let mut input = NomSpan::new(text);

    // let it = std::iter::from_fn(move || {
    //     match get_comment(input) {
    //     }
    // });
}