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
pub enum Call3GppRegistrationResponse {
	/// OK
	Status200(models::Amf3GppAccessRegistration) = 200,
	/// Created
	Status201 {
		body: models::Amf3GppAccessRegistration,
		location: String,
	} = 201,
	/// No content
	Status204 = 204,
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

impl DeserResponse for Call3GppRegistrationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::Amf3GppAccessRegistration = serde_json::from_str(&data)?;

				Ok((status, Call3GppRegistrationResponse::Status200(body)))
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
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::Amf3GppAccessRegistration = serde_json::from_str(&data)?;

				Ok((status, Call3GppRegistrationResponse::Status201 {
					body,
					location,
				}))
			}
			204 => {
				let data = resp.text().await?;

				Ok((status, Call3GppRegistrationResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppRegistrationResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppRegistrationResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppRegistrationResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppRegistrationResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppRegistrationResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, Call3GppRegistrationResponse::Statusdefault))
			}
		}
	}
}

/// AmfRegistrationFor3GppAccess
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait AmfRegistrationFor3GppAccess {
	/// register as AMF for 3GPP access.
	///
	/// Call3GppRegistration - PUT
	/// /nudm-uecm/v1/{ueId}/registrations/amf-3gpp-access
	async fn call3_gpp_registration(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::Call3GppRegistrationPathParams,
		body: models::Amf3GppAccessRegistration,
	) -> Result<Call3GppRegistrationResponse, String>;
}
