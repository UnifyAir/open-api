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
pub enum GetSupiOrGpsiResponse {
	/// Expected response to a valid request
	Status200 {
		body: models::IdTranslationResult,
		cache_control: Option<String>,
		etag: Option<String>,
		last_modified: Option<String>,
	} = 200,
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

impl DeserResponse for GetSupiOrGpsiResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let cache_control: Option<String> = resp
					.headers()
					.get(reqwest::header::CACHE_CONTROL)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let etag: Option<String> = resp
					.headers()
					.get(reqwest::header::ETAG)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let last_modified: Option<String> = resp
					.headers()
					.get(reqwest::header::LAST_MODIFIED)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::IdTranslationResult = serde_json::from_str(&data)?;

				Ok((status, GetSupiOrGpsiResponse::Status200 {
					body,
					cache_control,
					etag,
					last_modified,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetSupiOrGpsiResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetSupiOrGpsiResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetSupiOrGpsiResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetSupiOrGpsiResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetSupiOrGpsiResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, GetSupiOrGpsiResponse::Statusdefault))
			}
		}
	}
}

/// GpsitoSupiTranslationOrSupitoGpsiTranslation
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait GpsitoSupiTranslationOrSupitoGpsiTranslation {
	/// retrieve a UE's SUPI or GPSI.
	///
	/// GetSupiOrGpsi - GET /nudm-sdm/v2/{ueId}/id-translation-result
	async fn get_supi_or_gpsi(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		header_params: models::GetSupiOrGpsiHeaderParams,
		path_params: models::GetSupiOrGpsiPathParams,
		query_params: models::GetSupiOrGpsiQueryParams,
	) -> Result<GetSupiOrGpsiResponse, String>;
}
