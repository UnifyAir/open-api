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
pub enum SubscribeToSharedDataResponse {
	/// Expected response to a valid request
	Status201 {
		body: models::SdmSubscription,
		location: String,
	} = 201,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for SubscribeToSharedDataResponse {
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

				Ok((status, SubscribeToSharedDataResponse::Status201 {
					body,
					location,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SubscribeToSharedDataResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, SubscribeToSharedDataResponse::Status404(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, SubscribeToSharedDataResponse::Statusdefault))
			}
		}
	}
}

/// SubscriptionCreationForSharedData
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SubscriptionCreationForSharedData {
	/// subscribe to notifications for shared data.
	///
	/// SubscribeToSharedData - POST /nudm-sdm/v2/shared-data-subscriptions
	async fn subscribe_to_shared_data(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::SdmSubscription,
	) -> Result<SubscribeToSharedDataResponse, String>;
}
