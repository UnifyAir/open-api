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
pub enum GetMultipleIdentifiersResponse {
	/// Expected response to a valid request
	Status200 {
		body: std::collections::HashMap<String, models::SupiInfo>,
		cache_control: Option<String>,
		etag: Option<String>,
		last_modified: Option<String>,
	} = 200,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// 406 Not Acceptable
	Status406 = 406,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Bad Gateway
	Status502(models::ProblemDetails) = 502,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for GetMultipleIdentifiersResponse {
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
				let body: std::collections::HashMap<String, models::SupiInfo> =
					serde_json::from_str(&data)?;

				Ok((status, GetMultipleIdentifiersResponse::Status200 {
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

				Ok((status, GetMultipleIdentifiersResponse::Status400(body)))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetMultipleIdentifiersResponse::Status401(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetMultipleIdentifiersResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetMultipleIdentifiersResponse::Status404(body)))
			}
			406 => {
				let data = resp.text().await?;

				Ok((status, GetMultipleIdentifiersResponse::Status406))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetMultipleIdentifiersResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetMultipleIdentifiersResponse::Status500(body)))
			}
			502 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetMultipleIdentifiersResponse::Status502(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetMultipleIdentifiersResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, GetMultipleIdentifiersResponse::Statusdefault))
			}
		}
	}
}

/// MultipleIdentifiers
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait MultipleIdentifiers {
	/// Mapping of UE Identifiers.
	///
	/// GetMultipleIdentifiers - GET /nudm-sdm/v2/multiple-identifiers
	async fn get_multiple_identifiers(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		query_params: models::GetMultipleIdentifiersQueryParams,
	) -> Result<GetMultipleIdentifiersResponse, String>;
}
