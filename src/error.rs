
/*
error_chain! {
    foreign_links {
        ReqwestError(reqwest::Error);
        //ParseUrlError(reqwest::ParseError);
        IoError(std::io::Error);
        EnvError(std::env::VarError);
        InvalidMethodError(http::method::InvalidMethod);
    }

    errors {

    }
}
*/
use std::error::Error as StdError;
use std::fmt;
use std::io;

use reqwest::{StatusCode, Url};

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
    Request,
    Redirect,
    Status(StatusCode),
    Body,
    Decode,
    Reqwest,
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
}

pub(crate) fn builder<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Builder, Some(e))
}

pub(crate) fn body<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Body, Some(e))
}


pub(crate) fn decode<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Decode, Some(e))
}

pub(crate) fn reqwest<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Reqwest, Some(e))
}

