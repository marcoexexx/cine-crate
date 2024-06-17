use std::fmt::Display;

use scraper::error::SelectorErrorKind;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  ScrapeFail(SelectorErrorKind<'static>),
  RequestFail(reqwest::Error),
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

impl From<SelectorErrorKind<'static>> for Error {
  fn from(value: SelectorErrorKind<'static>) -> Self {
    Self::ScrapeFail(value)
  }
}

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Self::RequestFail(value)
  }
}

impl std::error::Error for Error {}
