use std::backtrace::Backtrace;

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use http::Method;
use oasbi::{DeserResponse, ReqError};
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum RetrieveCompleteSearchResponse {
	/// Expected response to a valid request
	Status200 {
		body: models::StoredSearchResult,
		cache_control: Option<String>,
		etag: Option<String>,
		content_encoding: Option<String>,
	} = 200,
	/// Temporary Redirect
	Status307 {
		body: models::RedirectResponse,
		location: String,
	} = 307,
	/// Permanent Redirect
	Status308 {
		body: models::RedirectResponse,
		location: String,
	} = 308,
}

impl DeserResponse for RetrieveCompleteSearchResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let cache_control: Option<String> = resp
					.headers()
					.get(reqwest::header::CACHE_CONTROL)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let etag: Option<String> = resp
					.headers()
					.get(reqwest::header::ETAG)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let content_encoding: Option<String> = resp
					.headers()
					.get(reqwest::header::CONTENT_ENCODING)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::StoredSearchResult = serde_json::from_str(&data)?;

				Ok((status, RetrieveCompleteSearchResponse::Status200 {
					body,
					cache_control,
					etag,
					content_encoding,
				}))
			}
			307 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, RetrieveCompleteSearchResponse::Status307 {
					body,
					location,
				}))
			}
			308 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, RetrieveCompleteSearchResponse::Status308 {
					body,
					location,
				}))
			}
			_ => Err(ReqError::UnexpectedResponseError(
				status.as_u16(),
				resp.text().await?,
				Backtrace::force_capture(),
			)),
		}
	}
}

/// CompleteStoredSearchDocument
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait CompleteStoredSearchDocument {
	/// RetrieveCompleteSearch - GET /nnrf-disc/v1/searches/{searchId}/complete
	async fn retrieve_complete_search(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		header_params: models::RetrieveCompleteSearchHeaderParams,
		path_params: models::RetrieveCompleteSearchPathParams,
	) -> Result<RetrieveCompleteSearchResponse, String>;
}
