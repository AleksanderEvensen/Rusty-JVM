use nom::{error::ParseError, IResult, InputLength, InputTake, Parser, ToUsize};

// type IParser<I, O> = ;

pub(crate) fn length_to_vec<I, O, N, E, F, G>(
    mut length_parser: F,
    mut item_parser: G,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + InputLength + InputTake,
    N: ToUsize,
    F: Parser<I, N, E>,
    G: Parser<I, O, E>,
    E: ParseError<I>,
{
    move |i| {
        let (mut i, len) = length_parser.parse(i)?;

        let mut vec = vec![];

        for _ in 0..len.to_usize() {
            let (i2, v) = item_parser.parse(i)?;
            vec.push(v);
            i = i2;
        }
        Ok((i, vec))
    }
}
