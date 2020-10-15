pub mod endpoints;

use exitfailure::ExitFailure;
use reqwest::{
    blocking::Client,
    header::{self, HeaderMap, HeaderValue},
    Url,
};
use serde::de::DeserializeOwned;

pub type ApiResult<T> = Result<T, ExitFailure>;

pub fn get<T>(url: &Url) -> T
where
    T: DeserializeOwned,
{
    Client::new()
        .get(url.clone())
        .headers(headers())
        .send()
        .unwrap()
        .json::<T>()
        .unwrap()
}

pub fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append(
        header::USER_AGENT,
        HeaderValue::from_str("curl/7.64.1").unwrap(),
    );
    headers
}
