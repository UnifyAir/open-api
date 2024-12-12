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
pub enum GetLcsBcaDataResponse {
	/// Expected response to a valid request
	Status200 {
		body: models::LcsBroadcastAssistanceTypesData,
		cache_control: Option<String>,
		etag: Option<String>,
		last_modified: Option<String>,
	} = 200,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for GetLcsBcaDataResponse {
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
				let body: models::LcsBroadcastAssistanceTypesData = serde_json::from_str(&data)?;

				Ok((status, GetLcsBcaDataResponse::Status200 {
					body,
					cache_control,
					etag,
					last_modified,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetLcsBcaDataResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetLcsBcaDataResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetLcsBcaDataResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetLcsBcaDataResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, GetLcsBcaDataResponse::Statusdefault))
			}
		}
	}
}

/// LcsBroadcastAssistanceDataTypesRetrieval
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait LcsBroadcastAssistanceDataTypesRetrieval {
	/// retrieve a UE's LCS Broadcast Assistance Data Types Subscription Data.
	///
	/// GetLcsBcaData - GET /nudm-sdm/v2/{supi}/lcs-bca-data
	async fn get_lcs_bca_data(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		header_params: models::GetLcsBcaDataHeaderParams,
		path_params: models::GetLcsBcaDataPathParams,
		query_params: models::GetLcsBcaDataQueryParams,
	) -> Result<GetLcsBcaDataResponse, String>;
}
