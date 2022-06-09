use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::{async_trait, extract::RequestParts};
use domain::logic::stream::Range;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
pub struct HeaderRange(Option<Range>);

impl From<HeaderRange> for Option<Range> {
    fn from(e: HeaderRange) -> Self {
        e.0
    }
}

#[async_trait]
impl<B> FromRequest<B> for HeaderRange
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let range = match req.headers().get("Range") {
            Some(e) => e,
            None => return Ok(HeaderRange(None)),
        };
        let range = require(range.to_str().ok()).unwrap();
        let range = parse_range(range);
        return Ok(HeaderRange(range));
    }
}

static REGEX_NUMBERS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]+").unwrap());
fn parse_range(header_value: &str) -> Option<Range> {
    let numbers: Vec<u64> = REGEX_NUMBERS
        .find_iter(header_value)
        .map(|e| e.as_str())
        .map(|e| e.parse::<u64>().expect("invalid regex"))
        .collect();

    let start = match numbers.get(0) {
        Some(start) => start,
        None => return None,
    };

    return match numbers.get(1) {
        Some(end) => Range::new(*start, *end).ok(),
        None => Range::new_with_start(*start).ok(),
    };
}

fn require<T>(value: Option<T>) -> Result<T, StatusCode> {
    match value {
        Some(v) => Ok(v),
        None => Err(StatusCode::BAD_REQUEST),
    }
}
