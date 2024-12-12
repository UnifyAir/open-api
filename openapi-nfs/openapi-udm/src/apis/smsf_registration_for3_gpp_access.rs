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
pub enum Call3GppSmsfRegistrationResponse {
	/// Expected response to a valid request
	Status200 {
		body: models::SmsfRegistration,
		etag: Option<String>,
	} = 200,
	/// Created
	Status201 {
		body: models::SmsfRegistration,
		location: String,
		etag: Option<String>,
	} = 201,
	/// No content
	Status204 { etag: Option<String> } = 204,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Forbidden
	Status403(models::common_models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for Call3GppSmsfRegistrationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let etag: Option<String> = resp
					.headers()
					.get(reqwest::header::ETAG)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::SmsfRegistration = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfRegistrationResponse::Status200 {
					body,
					etag,
				}))
			}
			201 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let etag: Option<String> = resp
					.headers()
					.get(reqwest::header::ETAG)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::SmsfRegistration = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfRegistrationResponse::Status201 {
					body,
					location,
					etag,
				}))
			}
			204 => {
				let etag: Option<String> = resp
					.headers()
					.get(reqwest::header::ETAG)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;

				Ok((status, Call3GppSmsfRegistrationResponse::Status204 { etag }))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfRegistrationResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfRegistrationResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfRegistrationResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfRegistrationResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfRegistrationResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, Call3GppSmsfRegistrationResponse::Statusdefault))
			}
		}
	}
}

/// SmsfRegistrationFor3GppAccess
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SmsfRegistrationFor3GppAccess {
	/// register as SMSF for 3GPP access.
	///
	/// Call3GppSmsfRegistration - PUT
	/// /nudm-uecm/v1/{ueId}/registrations/smsf-3gpp-access
	async fn call3_gpp_smsf_registration(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::Call3GppSmsfRegistrationPathParams,
		body: models::SmsfRegistration,
	) -> Result<Call3GppSmsfRegistrationResponse, String>;
}
