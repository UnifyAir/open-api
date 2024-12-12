use std::backtrace::Backtrace;

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
pub enum PostPcEventExposureSubscResponse {
	/// Success
	Status201 {
		body: models::PcEventExposureSubsc,
		location: String,
	} = 201,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::common_models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::common_models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Length Required
	Status411(models::common_models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::common_models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::common_models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::common_models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for PostPcEventExposureSubscResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			201 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::PcEventExposureSubsc = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status201 {
					body,
					location,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status400(body)))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status401(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostPcEventExposureSubscResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, PostPcEventExposureSubscResponse::Statusdefault))
			}
		}
	}
}

/// PolicyControlEventsSubscriptionCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait PolicyControlEventsSubscriptionCollection {
	/// Creates a new Individual Policy Control Events Subscription resource.
	///
	/// PostPcEventExposureSubsc - POST /npcf-eventexposure/v1/subscriptions
	async fn post_pc_event_exposure_subsc(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::PcEventExposureSubsc,
	) -> Result<PostPcEventExposureSubscResponse, String>;
}
