//! A `Retry-After` header implementation for Hyper
//!
//! This crate's repo is located at https://github.com/jwilm/retry-after.
//!
//! # Examples
//!
//! ```
//! use std::time::{Duration, SystemTime};
//! use retry_after::RetryAfter;
//!
//! # fn main() {
//! // Create a RetryAfter::Delay header
//! let retry_after_delay = RetryAfter::Delay(Duration::from_secs(300));
//!
//! // Create a RetryAfter::DateTime header
//! let retry_after_dt = RetryAfter::DateTime(SystemTime::now() + Duration::from_secs(300));
//! # }
//! ```
//!
//! For more examples, please see the _examples_ directory at the crate root.
//TODO: Fix Warnings
use std::{
    convert::TryFrom,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use chrono::{offset::Utc, DateTime, TimeZone};
use reqwest::header::HeaderValue;
use serde::{ser::Serialize, Deserialize, Deserializer, Serializer};
use thiserror::Error;

pub const HEADER_NAME: &str = "Retry-After";

#[derive(Error, Debug)]
pub enum FromHeaderValueError {
	#[error("not enough data")]
	InsufficientBytes,

	#[error("byte sequence in header was invalid")]
	InvalidByteSequence(#[from] std::str::Utf8Error),

	#[error("invalid format for time specifier")]
	ParseError,
}

/// Retry-After header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.1.3)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RetryAfter {
	/// Retry after this duration has elapsed
	///
	/// This can be coupled with a response time header to produce a DateTime.
	Delay(Duration),

	/// Retry after the given DateTime
	DateTime(SystemTime),
}

impl From<DateTime<Utc>> for RetryAfter {
	fn from(other: DateTime<Utc>) -> RetryAfter {
		let system_time: SystemTime = UNIX_EPOCH
			+ Duration::from_secs(other.timestamp() as u64)
			+ Duration::from_nanos(other.timestamp_subsec_nanos() as u64);
		RetryAfter::DateTime(system_time)
	}
}

impl TryFrom<HeaderValue> for RetryAfter {
	type Error = FromHeaderValueError;

	fn try_from(header_value: HeaderValue) -> Result<Self, Self::Error> {
		Self::try_from(&header_value)
	}
}

impl TryFrom<&HeaderValue> for RetryAfter {
	type Error = FromHeaderValueError;

	fn try_from(header_value: &HeaderValue) -> Result<Self, Self::Error> {
		if header_value.len() == 0 {
			return Err(FromHeaderValueError::InsufficientBytes);
		}

		let utf8_str = std::str::from_utf8(header_value.as_bytes())?;

		// Try and parse it as an integer, first.
		if let Ok(seconds) = utf8_str.parse::<u64>() {
			return Ok(RetryAfter::Delay(Duration::from_secs(seconds)));
		}

		// Now, try and parse it as a DateTime.
		parse_http_date(utf8_str)
			.map(From::from)
			.map_err(|_| FromHeaderValueError::ParseError)
	}
}

static RFC850_FMT: &'static str = "%A, %d-%b-%y %T GMT";
static RFC1123_FMT: &'static str = "%a, %d %b %Y %T GMT";
static ASCTIME_FMT: &'static str = "%a %b %e %T %Y";

impl Into<HeaderValue> for RetryAfter {
	fn into(self) -> HeaderValue {
		use std::io::Write;
		let mut s = Vec::new();
		match self {
			RetryAfter::Delay(duration) => {
				write!(&mut s, "{}", duration.as_secs()).expect("write to vec won't fail");
			}
			RetryAfter::DateTime(datetime) => {
				// According to RFC7231, the sender of an HTTP-date must use the RFC1123 format.
				// http://tools.ietf.org/html/rfc7231#section-7.1.1.1
				let datetime: DateTime<Utc> = datetime.into();
				write!(&mut s, "{}", datetime.format(RFC1123_FMT).to_string())
					.expect("write to vec won't fail");
			}
		}

		HeaderValue::from_bytes(&s).expect("format strings should result in ascii which is valid")
	}
}

/// Parse an HTTP-date
///
/// HTTP/1.1 servers must return HTTP-dates using RFC1123 format for
/// Retry-After. For compatibility with HTTP/1.0 servers, RFC850 and ASCTIME
/// formats are supported as well.
fn parse_http_date(raw: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
	if let Ok(dt) = Utc.datetime_from_str(raw, RFC1123_FMT) {
		Ok(dt)
	} else if let Ok(dt) = Utc.datetime_from_str(raw, RFC850_FMT) {
		Ok(dt)
	} else {
		Utc.datetime_from_str(raw, ASCTIME_FMT)
	}
}

// Custom serialization
impl Serialize for RetryAfter {
	fn serialize<S>(
		&self,
		serializer: S,
	) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match *self {
			RetryAfter::Delay(duration) => serializer.serialize_u64(duration.as_secs()),
			RetryAfter::DateTime(datetime) => {
				let datetime: DateTime<Utc> = datetime.into();
				serializer.serialize_str(&datetime.format(RFC1123_FMT).to_string())
			}
		}
	}
}

// Custom deserialization
impl<'de> Deserialize<'de> for RetryAfter {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct RetryAfterVisitor;

		impl<'de> serde::de::Visitor<'de> for RetryAfterVisitor {
			type Value = RetryAfter;

			fn expecting(
				&self,
				formatter: &mut std::fmt::Formatter,
			) -> std::fmt::Result {
				formatter.write_str("a valid Retry-After header value (integer or RFC1123 date)")
			}

			fn visit_u64<E>(
				self,
				value: u64,
			) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				Ok(RetryAfter::Delay(Duration::from_secs(value)))
			}

			fn visit_str<E>(
				self,
				value: &str,
			) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				if let Ok(dt) = parse_http_date(value) {
					Ok(RetryAfter::DateTime(dt.into()))
				} else {
					Err(E::custom("invalid date format"))
				}
			}
		}

		deserializer.deserialize_any(RetryAfterVisitor)
	}
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use chrono::{self, offset::Utc, TimeZone};
    use reqwest::header::HeaderValue;

    use super::{RetryAfter, ASCTIME_FMT, RFC1123_FMT, RFC850_FMT};

    macro_rules! test_parse_format {
		($name:ident, $fmt:ident, $dt_str:expr) => {
			#[test]
			fn $name() {
				let dt = Utc.ymd(1994, 11, 6).and_hms(8, 49, 37);

				// Check that the format is what we expect
				assert_eq!(dt.format($fmt).to_string(), $dt_str);

				// Check that it parses correctly
				assert_eq!(Ok(dt), Utc.datetime_from_str($dt_str, $fmt));
			}
		};
	}

	test_parse_format!(parse_rfc1123, RFC1123_FMT, "Sun, 06 Nov 1994 08:49:37 GMT");
	test_parse_format!(parse_rfc850, RFC850_FMT, "Sunday, 06-Nov-94 08:49:37 GMT");
	test_parse_format!(parse_asctime, ASCTIME_FMT, "Sun Nov  6 08:49:37 1994");

	#[test]
	fn parse_delay() {
		let delay = HeaderValue::from_bytes(b"1234").unwrap();
		let retry_after = RetryAfter::try_from(delay).unwrap();

		assert_eq!(
			RetryAfter::Delay(std::time::Duration::from_secs(1234)),
			retry_after
		);
	}

	macro_rules! test_retry_after_datetime {
		($name:ident, $bytes:expr) => {
			#[test]
			fn $name() {
				let raw = $bytes.to_vec();
				let header_value = HeaderValue::from_bytes(&raw[..]).unwrap();
				let dt = Utc.ymd(1994, 11, 6).and_hms(8, 49, 37);

				let retry_after = RetryAfter::try_from(header_value).expect("parse_header ok");
				assert_eq!(RetryAfter::DateTime(From::from(dt)), retry_after);
			}
		};
	}

	test_retry_after_datetime!(header_parse_rfc1123, b"Sun, 06 Nov 1994 08:49:37 GMT");
	test_retry_after_datetime!(header_parse_rfc850, b"Sunday, 06-Nov-94 08:49:37 GMT");
	test_retry_after_datetime!(header_parse_asctime, b"Sun Nov  6 08:49:37 1994");
}
