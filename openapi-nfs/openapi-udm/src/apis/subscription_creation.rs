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
pub enum SubscribeResponse {
	/// Expected response to a valid request
	Status201 {
		body: models::SdmSubscription,
		location: String,
	} = 201,
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

impl DeserResponse for SubscribeResponse {
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
				let body: models::SdmSubscription = serde_json::from_str(&data)?;

				Ok((status, SubscribeResponse::Status201 { body, location }))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SubscribeResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SubscribeResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SubscribeResponse::Status500(body)))
			}
			501 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SubscribeResponse::Status501(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SubscribeResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, SubscribeResponse::Statusdefault))
			}
		}
	}
}

/// SubscriptionCreation
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SubscriptionCreation {
	/// subscribe to notifications.
	///
	/// Subscribe - POST /nudm-sdm/v2/{ueId}/sdm-subscriptions
	async fn subscribe(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::SubscribePathParams,
		body: models::SdmSubscription,
	) -> Result<SubscribeResponse, String>;
}
