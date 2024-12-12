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
pub enum GenerateAuthDataResponse {
	/// Expected response to a valid request
	Status200(models::AuthenticationInfoResult) = 200,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Forbidden
	Status403(models::common_models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Not Implemented
	Status501(models::common_models::ProblemDetails) = 501,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for GenerateAuthDataResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::AuthenticationInfoResult = serde_json::from_str(&data)?;

				Ok((status, GenerateAuthDataResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateAuthDataResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateAuthDataResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateAuthDataResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateAuthDataResponse::Status500(body)))
			}
			501 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateAuthDataResponse::Status501(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateAuthDataResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, GenerateAuthDataResponse::Statusdefault))
			}
		}
	}
}

/// GenerateAuthData
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait GenerateAuthData {
	/// Generate authentication data for the UE.
	///
	/// GenerateAuthData - POST
	/// /nudm-ueau/v1/{supiOrSuci}/security-information/generate-auth-data
	async fn generate_auth_data(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::GenerateAuthDataPathParams,
		body: models::AuthenticationInfoRequest,
	) -> Result<GenerateAuthDataResponse, String>;
}
