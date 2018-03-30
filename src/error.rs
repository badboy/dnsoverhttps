use trust_dns_proto::error::ProtoError;
use reqwest;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Failed to parse DNS protocol")]
    DnsProtoError,
    #[fail(display = "{}", _0)]
    Url(#[cause] reqwest::UrlError),
    #[fail(display = "{}", _0)]
    Reqwest(#[cause] reqwest::Error),
}

impl From<ProtoError> for Error {
    fn from(_error: ProtoError) -> Error {
        Error::DnsProtoError
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(error: reqwest::UrlError) -> Error {
        Error::Url(error)
    }
}
