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
pub enum GetNon3GppRegistrationResponse {
	/// Expected response to a valid request
	Status200(models::AmfNon3GppAccessRegistration) = 200,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for GetNon3GppRegistrationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::AmfNon3GppAccessRegistration = serde_json::from_str(&data)?;

				Ok((status, GetNon3GppRegistrationResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetNon3GppRegistrationResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetNon3GppRegistrationResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetNon3GppRegistrationResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetNon3GppRegistrationResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetNon3GppRegistrationResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, GetNon3GppRegistrationResponse::Statusdefault))
			}
		}
	}
}

/// AmfNon3GppAccessRegistrationInfoRetrieval
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait AmfNon3GppAccessRegistrationInfoRetrieval {
	/// retrieve the AMF registration for non-3GPP access information.
	///
	/// GetNon3GppRegistration - GET
	/// /nudm-uecm/v1/{ueId}/registrations/amf-non-3gpp-access
	async fn get_non3_gpp_registration(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::GetNon3GppRegistrationPathParams,
		query_params: models::GetNon3GppRegistrationQueryParams,
	) -> Result<GetNon3GppRegistrationResponse, String>;
}
