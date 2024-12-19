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
pub enum UpdateSorInfoResponse {
	/// Expected response to a valid request
	Status200(models::SorInfo) = 200,
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

impl DeserResponse for UpdateSorInfoResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::SorInfo = serde_json::from_str(&data)?;

				Ok((status, UpdateSorInfoResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateSorInfoResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateSorInfoResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateSorInfoResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateSorInfoResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, UpdateSorInfoResponse::Statusdefault))
			}
		}
	}
}

/// TriggerSorInfoUpdate
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait TriggerSorInfoUpdate {
	/// Nudm_Sdm custom operation to trigger SOR info update.
	///
	/// UpdateSorInfo - POST /nudm-sdm/v2/{supi}/am-data/update-sor
	async fn update_sor_info(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpdateSorInfoPathParams,
		body: Option<models::SorUpdateInfo>,
	) -> Result<UpdateSorInfoResponse, String>;
}
