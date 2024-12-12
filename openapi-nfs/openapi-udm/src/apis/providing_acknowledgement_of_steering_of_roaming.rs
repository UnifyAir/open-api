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
pub enum SorAckInfoResponse {
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

impl DeserResponse for SorAckInfoResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, SorAckInfoResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SorAckInfoResponse::Status400(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SorAckInfoResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SorAckInfoResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, SorAckInfoResponse::Statusdefault))
			}
		}
	}
}

/// ProvidingAcknowledgementOfSteeringOfRoaming
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ProvidingAcknowledgementOfSteeringOfRoaming {
	/// Nudm_Sdm Info service operation.
	///
	/// SorAckInfo - PUT /nudm-sdm/v2/{supi}/am-data/sor-ack
	async fn sor_ack_info(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::SorAckInfoPathParams,
		body: Option<models::AcknowledgeInfo>,
	) -> Result<SorAckInfoResponse, String>;
}
