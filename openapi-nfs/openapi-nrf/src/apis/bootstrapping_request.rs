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
pub enum BootstrappingInfoRequestResponse {
	/// Successful Bootstrapping Request
	Status200 {
		body: String,
		cache_control: Option<String>,
		etag: Option<String>,
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
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for BootstrappingInfoRequestResponse {
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
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: String = data;

				Ok((status, BootstrappingInfoRequestResponse::Status200 {
					body,
					cache_control,
					etag,
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

				Ok((status, BootstrappingInfoRequestResponse::Status307 {
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

				Ok((status, BootstrappingInfoRequestResponse::Status308 {
					body,
					location,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, BootstrappingInfoRequestResponse::Status400(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, BootstrappingInfoRequestResponse::Status500(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, BootstrappingInfoRequestResponse::Statusdefault))
			}
		}
	}
}

/// BootstrappingRequest
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait BootstrappingRequest {
	/// Bootstrapping Info Request.
	///
	/// BootstrappingInfoRequest - GET //bootstrapping
	async fn bootstrapping_info_request(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		header_params: models::BootstrappingInfoRequestHeaderParams,
	) -> Result<BootstrappingInfoRequestResponse, String>;
}
