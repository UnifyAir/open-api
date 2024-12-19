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
pub enum UpdateNon3GppRegistrationResponse {
	/// Expected response to a valid request
	Status200(models::PatchResult) = 200,
	/// Expected response to a valid request
	Status204 = 204,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
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

impl DeserResponse for UpdateNon3GppRegistrationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::PatchResult = serde_json::from_str(&data)?;

				Ok((status, UpdateNon3GppRegistrationResponse::Status200(body)))
			}
			204 => {
				let data = resp.text().await?;

				Ok((status, UpdateNon3GppRegistrationResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateNon3GppRegistrationResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateNon3GppRegistrationResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateNon3GppRegistrationResponse::Status404(body)))
			}
			422 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateNon3GppRegistrationResponse::Status422(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateNon3GppRegistrationResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateNon3GppRegistrationResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, UpdateNon3GppRegistrationResponse::Statusdefault))
			}
		}
	}
}

/// ParameterUpdateInTheAmfRegistrationForNon3GppAccess
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ParameterUpdateInTheAmfRegistrationForNon3GppAccess {
	/// update a parameter in the AMF registration for non-3GPP access.
	///
	/// UpdateNon3GppRegistration - PATCH
	/// /nudm-uecm/v1/{ueId}/registrations/amf-non-3gpp-access
	async fn update_non3_gpp_registration(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpdateNon3GppRegistrationPathParams,
		query_params: models::UpdateNon3GppRegistrationQueryParams,
		body: models::AmfNon3GppAccessRegistrationModification,
	) -> Result<UpdateNon3GppRegistrationResponse, String>;
}
