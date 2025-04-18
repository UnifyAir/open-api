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
pub enum UpuAckResponse {
	/// Successful acknowledgement
	Status204 = 204,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for UpuAckResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, UpuAckResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpuAckResponse::Status400(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpuAckResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpuAckResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, UpuAckResponse::Statusdefault))
			}
		}
	}
}

/// ProvidingAcknowledgementOfUeParametersUpdate
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ProvidingAcknowledgementOfUeParametersUpdate {
	/// Nudm_Sdm Info for UPU service operation.
	///
	/// UpuAck - PUT /nudm-sdm/v2/{supi}/am-data/upu-ack
	async fn upu_ack(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpuAckPathParams,
		body: Option<models::AcknowledgeInfo>,
	) -> Result<UpuAckResponse, String>;
}
