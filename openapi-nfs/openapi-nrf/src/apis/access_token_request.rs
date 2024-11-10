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
pub enum AccessTokenRequestResponse {
	/// Successful Access Token Request
	Status200 {
		body: models::AccessTokenRsp,
		cache_control: String,
		pragma: String,
		accept_encoding: Option<String>,
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
	/// Error in the Access Token Request
	Status400 {
		body: models::AccessTokenErr,
		cache_control: String,
		pragma: String,
	} = 400,
	/// Unauthorized
	Status401(models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Length Required
	Status411(models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Not Implemented
	Status501(models::ProblemDetails) = 501,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for AccessTokenRequestResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let cache_control: String = resp
					.headers()
					.get(reqwest::header::CACHE_CONTROL)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"cache_control".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let pragma: String = resp
					.headers()
					.get(reqwest::header::PRAGMA)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"pragma".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let accept_encoding: Option<String> = resp
					.headers()
					.get(reqwest::header::ACCEPT_ENCODING)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let content_encoding: Option<String> = resp
					.headers()
					.get(reqwest::header::CONTENT_ENCODING)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::AccessTokenRsp = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status200 {
					body,
					cache_control,
					pragma,
					accept_encoding,
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

				Ok((status, AccessTokenRequestResponse::Status307 {
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

				Ok((status, AccessTokenRequestResponse::Status308 {
					body,
					location,
				}))
			}
			400 => {
				let cache_control: String = resp
					.headers()
					.get(reqwest::header::CACHE_CONTROL)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"cache_control".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let pragma: String = resp
					.headers()
					.get(reqwest::header::PRAGMA)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"pragma".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::AccessTokenErr = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status400 {
					body,
					cache_control,
					pragma,
				}))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status401(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status500(body)))
			}
			501 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status501(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AccessTokenRequestResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, AccessTokenRequestResponse::Statusdefault))
			}
		}
	}
}

/// AccessTokenRequest
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait AccessTokenRequest {
	/// Access Token Request.
	///
	/// AccessTokenRequest - POST //oauth2/token
	async fn access_token_request(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		header_params: models::AccessTokenRequestHeaderParams,
		body: models::AccessTokenReq,
	) -> Result<AccessTokenRequestResponse, String>;
}
