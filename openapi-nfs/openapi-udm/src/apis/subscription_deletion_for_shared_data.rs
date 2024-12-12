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
pub enum UnsubscribeForSharedDataResponse {
	/// Successful response
	Status204 = 204,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for UnsubscribeForSharedDataResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, UnsubscribeForSharedDataResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UnsubscribeForSharedDataResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UnsubscribeForSharedDataResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UnsubscribeForSharedDataResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UnsubscribeForSharedDataResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, UnsubscribeForSharedDataResponse::Statusdefault))
			}
		}
	}
}

/// SubscriptionDeletionForSharedData
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SubscriptionDeletionForSharedData {
	/// unsubscribe from notifications for shared data.
	///
	/// UnsubscribeForSharedData - DELETE
	/// /nudm-sdm/v2/shared-data-subscriptions/{subscriptionId}
	async fn unsubscribe_for_shared_data(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UnsubscribeForSharedDataPathParams,
	) -> Result<UnsubscribeForSharedDataResponse, String>;
}
