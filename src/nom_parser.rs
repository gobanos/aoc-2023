use nom::bytes::complete::take_while1;
use nom::error::ErrorKind;
use nom::IResult;
use std::error::Error;
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

pub trait ErrToOwned {
    type Owned;

    fn err_to_owned(self) -> nom::Err<Self::Owned>;
}

impl ErrToOwned for nom::Err<(&[u8], ErrorKind)> {
    type Owned = (Vec<u8>, ErrorKind);

    fn err_to_owned(self) -> nom::Err<Self::Owned> {
        self.to_owned()
    }
}

impl ErrToOwned for nom::Err<(&str, ErrorKind)> {
    type Owned = (String, ErrorKind);

    fn err_to_owned(self) -> nom::Err<Self::Owned> {
        self.to_owned()
    }
}

impl ErrToOwned for nom::Err<nom::error::Error<&[u8]>> {
    type Owned = nom::error::Error<Vec<u8>>;

    fn err_to_owned(self) -> nom::Err<Self::Owned> {
        self.to_owned()
    }
}

impl ErrToOwned for nom::Err<nom::error::Error<&str>> {
    type Owned = nom::error::Error<String>;

    fn err_to_owned(self) -> nom::Err<Self::Owned> {
        self.to_owned()
    }
}

pub fn to_result<I, O, E>(i_result: IResult<I, O, E>) -> anyhow::Result<O>
where
    nom::Err<E>: ErrToOwned,
    nom::Err<<nom::Err<E> as ErrToOwned>::Owned>: Error + Send + Sync + 'static,
{
    Ok(i_result.map_err(|err| err.err_to_owned())?.1)
}
