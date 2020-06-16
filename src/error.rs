use std::error::Error as StdError;
use std::fmt;

use reqwest::{Url, StatusCode};

/// A `Result` alias where the `Err` case is "vault::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// The Errors that may occur when processing a `Request`.
pub struct Error {
    inner: Box<Inner>,
}

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub(crate) enum Kind {
    Builder,
    Decode,
    Reqwest,
    Status(StatusCode),
}

struct Inner {
    kind: Kind,
    source: Option<BoxError>,
    url: Option<Url>,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("reqwest::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref url) = self.inner.url {
            builder.field("url", url);
        }
        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl Error {
    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> Error
    where
        E: Into<BoxError>,
    {
        Error {
            inner: Box::new(Inner {
                kind,
                source: source.map(Into::into),
                url: None,
            }),
        }
    }

    /// Returns true if the error is from `Response::error_for_status`.
    pub fn is_status(&self) -> bool {
        match self.inner.kind {
            Kind::Status(_) => true,
            _ => false,
        }
    }

    pub fn status(&self) -> Option<StatusCode> {
        match self.inner.kind {
            Kind::Status(code) => Some(code),
            _ => None
        }
    }

    pub(crate) fn with_url(mut self, url: Url) -> Error {
        self.inner.url = Some(url);
        self
    }
}

pub(crate) fn builder<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Builder, Some(e))
}

pub(crate) fn decode<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Decode, Some(e))
}

pub(crate) fn reqwest<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Reqwest, Some(e))
}

pub(crate) fn status_code<E: Into<BoxError>>(e: E, code: StatusCode) -> Error {
    Error::new(Kind::Status(code), Some(e))
}
