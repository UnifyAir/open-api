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
pub enum CAgAckResponse {
	/// Successful acknowledgement
	Status204 = 204,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for CAgAckResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, CAgAckResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CAgAckResponse::Status400(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CAgAckResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CAgAckResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, CAgAckResponse::Statusdefault))
			}
		}
	}
}

/// ProvidingAcknowledgementOfCagUpdate
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ProvidingAcknowledgementOfCagUpdate {
	/// Nudm_Sdm Info operation for CAG acknowledgement.
	///
	/// CAgAck - PUT /nudm-sdm/v2/{supi}/am-data/cag-ack
	async fn cag_ack(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::CAgAckPathParams,
		body: Option<models::AcknowledgeInfo>,
	) -> Result<CAgAckResponse, String>;
}
