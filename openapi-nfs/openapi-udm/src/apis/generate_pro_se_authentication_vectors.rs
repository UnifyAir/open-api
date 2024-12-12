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
pub enum GenerateProseAvResponse {
	/// Expected response to a valid request
	Status200(models::ProSeAuthenticationInfoResult) = 200,
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

impl DeserResponse for GenerateProseAvResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProSeAuthenticationInfoResult = serde_json::from_str(&data)?;

				Ok((status, GenerateProseAvResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateProseAvResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateProseAvResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateProseAvResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateProseAvResponse::Status500(body)))
			}
			501 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateProseAvResponse::Status501(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GenerateProseAvResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, GenerateProseAvResponse::Statusdefault))
			}
		}
	}
}

/// GenerateProSeAuthenticationVectors
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait GenerateProSeAuthenticationVectors {
	/// Generate authentication data for ProSe.
	///
	/// GenerateProseAv - POST
	/// /nudm-ueau/v1/{supiOrSuci}/prose-security-information/generate-av
	async fn generate_prose_av(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::GenerateProseAvPathParams,
		body: models::ProSeAuthenticationInfoRequest,
	) -> Result<GenerateProseAvResponse, String>;
}
