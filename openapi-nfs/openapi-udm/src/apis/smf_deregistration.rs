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
pub enum SmfDeregistrationResponse {
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

impl DeserResponse for SmfDeregistrationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, SmfDeregistrationResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SmfDeregistrationResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SmfDeregistrationResponse::Status404(body)))
			}
			422 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SmfDeregistrationResponse::Status422(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SmfDeregistrationResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SmfDeregistrationResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, SmfDeregistrationResponse::Statusdefault))
			}
		}
	}
}

/// SmfDeregistration
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SmfDeregistration {
	/// delete an SMF registration.
	///
	/// SmfDeregistration - DELETE
	/// /nudm-uecm/v1/{ueId}/registrations/smf-registrations/{pduSessionId}
	async fn smf_deregistration(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::SmfDeregistrationPathParams,
		query_params: models::SmfDeregistrationQueryParams,
	) -> Result<SmfDeregistrationResponse, String>;
}
