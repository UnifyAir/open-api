#![feature(adt_const_params)]
#![feature(error_generic_member_access)]
#![allow(incomplete_features)]

use std::backtrace::Backtrace;

use retry_after::FromHeaderValueError;
use thiserror::Error;

pub mod amf;
pub mod ausf;
pub mod chf;
pub mod common;
mod modified;
pub mod nrf;
pub mod nssf;
pub mod pcf;
pub mod progenitor_client;
pub mod service_properties;
pub mod smf;
pub mod udm;
pub mod udr;
pub mod upf;

#[derive(Debug, Error)]
pub enum ReqError {
	#[error("Reqwest: Reqwest Error Received")]
	Reqwest(
		#[from]
		#[backtrace]
		reqwest::Error,
	),
	#[error("Serde: Response Parsing Error Received")]
	Serde(
		#[from]
		#[backtrace]
		serde_json::Error,
	),
	#[error("Io: Response Io Error Received")]
	Io(
		#[from]
		#[backtrace]
		std::io::Error,
	),
	#[error("RequiredHeaderNotFound: Required Header Not Found: {0}")]
	RequiredHeaderNotFound(String, #[backtrace] Backtrace),
	#[error("ToStrErrorReqwest: Header decoding Error")]
	ToStrErrorReqwest(
		#[from]
		#[backtrace]
		reqwest::header::ToStrError,
	),
	//    #[error("Header decoding Error")]
	//    ToStrErrorHttp(#[from]#[backtrace] http::header::ToStrError),
	#[error("UnexpectedResponseError: Got Error Which was not expected: {0}")]
	UnexpectedResponseError(u16, String, #[backtrace] Backtrace),
	#[error("ParsingRetryHeaderError: Invalid Retry Header")]
	ParsingRetryHeaderError(
		Vec<u8>,
		#[source] FromHeaderValueError,
		#[backtrace] Backtrace,
	),
}

pub trait DeserResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError>
	where
		Self: Sized;
}

use reqwest::header::HeaderName;

pub const PARAM3GPP_SBI_TARGET_NF_ID: HeaderName = HeaderName::from_static("3gpp-sbi-target-nf-id");
pub const PARAM3GPP_SBI_PRODUCER_ID: HeaderName = HeaderName::from_static("3gpp-sbi-producer-id");
