use nom::IResult;

pub trait Parser {
  fn parse(input: &str) -> IResult<&str, Self>
  where
    Self: Sized;
}

macro_rules! parse_alt_enum {
  ($s:expr) => {
    value($s.into(), tag($s))
  };
}

pub(crate) use parse_alt_enum;
