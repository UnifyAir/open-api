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
pub enum SendRoutingInfoSmResponse {
	/// Expected response to a valid request
	Status200(models::RoutingInfoSmResponse) = 200,
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

impl DeserResponse for SendRoutingInfoSmResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RoutingInfoSmResponse = serde_json::from_str(&data)?;

				Ok((status, SendRoutingInfoSmResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SendRoutingInfoSmResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SendRoutingInfoSmResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SendRoutingInfoSmResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SendRoutingInfoSmResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SendRoutingInfoSmResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, SendRoutingInfoSmResponse::Statusdefault))
			}
		}
	}
}

/// SendRoutingInfoSmCustomOperation
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SendRoutingInfoSmCustomOperation {
	/// Retreive addressing information for SMS delivery.
	///
	/// SendRoutingInfoSm - POST
	/// /nudm-uecm/v1/{ueId}/registrations/send-routing-info-sm
	async fn send_routing_info_sm(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::SendRoutingInfoSmPathParams,
		body: models::RoutingInfoSmRequest,
	) -> Result<SendRoutingInfoSmResponse, String>;
}
