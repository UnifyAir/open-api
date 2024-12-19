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
pub enum GetUeCtxInSmsfDataResponse {
	/// Expected response to a valid request
	Status200(models::UeContextInSmsfData) = 200,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for GetUeCtxInSmsfDataResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::UeContextInSmsfData = serde_json::from_str(&data)?;

				Ok((status, GetUeCtxInSmsfDataResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetUeCtxInSmsfDataResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetUeCtxInSmsfDataResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetUeCtxInSmsfDataResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, GetUeCtxInSmsfDataResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, GetUeCtxInSmsfDataResponse::Statusdefault))
			}
		}
	}
}

/// UeContextInSmsfDataRetrieval
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait UeContextInSmsfDataRetrieval {
	/// retrieve a UE's UE Context In SMSF Data.
	///
	/// GetUeCtxInSmsfData - GET /nudm-sdm/v2/{supi}/ue-context-in-smsf-data
	async fn get_ue_ctx_in_smsf_data(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::GetUeCtxInSmsfDataPathParams,
		query_params: models::GetUeCtxInSmsfDataQueryParams,
	) -> Result<GetUeCtxInSmsfDataResponse, String>;
}
