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
pub enum GetLocationInfoResponse {
	/// Expected response to a valid request
	Status200(models::LocationInfo) = 200,
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

impl DeserResponse for GetLocationInfoResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::LocationInfo = serde_json::from_str(&data)?;

				Ok((status, GetLocationInfoResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetLocationInfoResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetLocationInfoResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetLocationInfoResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetLocationInfoResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetLocationInfoResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, GetLocationInfoResponse::Statusdefault))
			}
		}
	}
}

/// UeLocationInformationRetrieval
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait UeLocationInformationRetrieval {
	/// retrieve the target UE's location information.
	///
	/// GetLocationInfo - GET /nudm-uecm/v1/{ueId}/registrations/location
	async fn get_location_info(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::GetLocationInfoPathParams,
		query_params: models::GetLocationInfoQueryParams,
	) -> Result<GetLocationInfoResponse, String>;
}
