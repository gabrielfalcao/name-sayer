use std::collections::BTreeMap;
use std::fmt::Debug;

use http::header::HeaderMap;
use http::method::Method;
use http::status::StatusCode;
use http::version::Version;
use http_body_util::{Collected, Empty, Full};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HTTPBody
{
    pub data: Vec<u8>,
}
// #[rustfmt::skip]
// impl<T> From<Collected<T>> for HTTPBody { fn from(value: Collected<T>) -> HTTPBody { let data = value.to_bytes().iter().collect::<Vec<u8>>(); HTTPBody { data } } }
// #[rustfmt::skip]
// impl<T> From<Empty<T>> for HTTPBody { fn from(value: Empty<T>) -> HTTPBody { let data = value.to_bytes().iter().collect::<Vec<u8>>(); HTTPBody { data } } }
// #[rustfmt::skip]
// impl<T> From<Full<T>> for HTTPBody { fn from(value: Full<T>) -> HTTPBody { let data = value.to_bytes().iter().collect::<Vec<u8>>(); HTTPBody { data } } }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HTTPRequest
{
    pub method: String,
    pub url: Url,
    pub headers: BTreeMap<String, Vec<String>>,
    pub version: String,
    pub body: HTTPBody,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HTTPResponse
{
    pub status: String,
    pub version: String,
    pub headers: BTreeMap<String, Vec<String>>,
    pub body: HTTPBody,
}
