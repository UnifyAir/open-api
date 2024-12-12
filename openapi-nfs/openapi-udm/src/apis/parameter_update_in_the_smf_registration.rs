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
pub enum UpdateSmfRegistrationResponse {
	/// Expected response to a valid request
	Status200(models::PatchResult) = 200,
	/// Expected response to a valid request
	Status204 = 204,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Unprocessable Request
	Status422(models::common_models::ProblemDetails) = 422,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for UpdateSmfRegistrationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::PatchResult = serde_json::from_str(&data)?;

				Ok((status, UpdateSmfRegistrationResponse::Status200(body)))
			}
			204 => {
				let data = resp.text().await?;

				Ok((status, UpdateSmfRegistrationResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateSmfRegistrationResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateSmfRegistrationResponse::Status404(body)))
			}
			422 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateSmfRegistrationResponse::Status422(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateSmfRegistrationResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateSmfRegistrationResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, UpdateSmfRegistrationResponse::Statusdefault))
			}
		}
	}
}

/// ParameterUpdateInTheSmfRegistration
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ParameterUpdateInTheSmfRegistration {
	/// update a parameter in the SMF registration.
	///
	/// UpdateSmfRegistration - PATCH
	/// /nudm-uecm/v1/{ueId}/registrations/smf-registrations/{pduSessionId}
	async fn update_smf_registration(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpdateSmfRegistrationPathParams,
		query_params: models::UpdateSmfRegistrationQueryParams,
		body: models::SmfRegistrationModification,
	) -> Result<UpdateSmfRegistrationResponse, String>;
}
