use nom::bytes::complete::take_while1;
use nom::error::ErrorKind;
use nom::IResult;
use std::str::FromStr;

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    let (input, number_str) = take_while1(|c: char| c.is_ascii_digit())(input)?;
    Ok((
        input,
        number_str
            .parse()
            .map_err(|_| nom::Err::Failure(nom::error::Error::new(number_str, ErrorKind::Fail)))?,
    ))
}
