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
pub enum Call3GppSmsfDeregistrationResponse {
	/// Expected response to a valid request
	Status204 = 204,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Unprocessable Request
	Status422(models::ProblemDetails) = 422,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for Call3GppSmsfDeregistrationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, Call3GppSmsfDeregistrationResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfDeregistrationResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfDeregistrationResponse::Status404(body)))
			}
			422 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfDeregistrationResponse::Status422(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfDeregistrationResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Call3GppSmsfDeregistrationResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, Call3GppSmsfDeregistrationResponse::Statusdefault))
			}
		}
	}
}

/// SmsfDeregistrationFor3GppAccess
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SmsfDeregistrationFor3GppAccess {
	/// delete the SMSF registration for 3GPP access.
	///
	/// Call3GppSmsfDeregistration - DELETE
	/// /nudm-uecm/v1/{ueId}/registrations/smsf-3gpp-access
	async fn call3_gpp_smsf_deregistration(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		header_params: models::Call3GppSmsfDeregistrationHeaderParams,
		path_params: models::Call3GppSmsfDeregistrationPathParams,
		query_params: models::Call3GppSmsfDeregistrationQueryParams,
	) -> Result<Call3GppSmsfDeregistrationResponse, String>;
}
