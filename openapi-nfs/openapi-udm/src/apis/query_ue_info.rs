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
pub enum QueryUeInfoResponse {
	/// Expected response to a valid request
	Status200(models::UeInfo) = 200,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Not Implemented
	Status501(models::ProblemDetails) = 501,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for QueryUeInfoResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::UeInfo = serde_json::from_str(&data)?;

				Ok((status, QueryUeInfoResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, QueryUeInfoResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, QueryUeInfoResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, QueryUeInfoResponse::Status500(body)))
			}
			501 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, QueryUeInfoResponse::Status501(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, QueryUeInfoResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, QueryUeInfoResponse::Statusdefault))
			}
		}
	}
}

/// QueryUeInfo
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait QueryUeInfo {
	/// Query Information for the UE.
	///
	/// QueryUeInfo - GET /nudm-mt/v1/{supi}
	async fn query_ue_info(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::QueryUeInfoPathParams,
		query_params: models::QueryUeInfoQueryParams,
	) -> Result<QueryUeInfoResponse, String>;
}
