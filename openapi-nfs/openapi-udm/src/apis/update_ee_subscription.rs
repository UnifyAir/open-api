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
pub enum UpdateEeSubscriptionResponse {
	/// Expected response to a valid request
	Status200(models::PatchResult) = 200,
	/// Successful response
	Status204 = 204,
	/// Forbidden
	Status403(models::common_models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for UpdateEeSubscriptionResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::PatchResult = serde_json::from_str(&data)?;

				Ok((status, UpdateEeSubscriptionResponse::Status200(body)))
			}
			204 => {
				let data = resp.text().await?;

				Ok((status, UpdateEeSubscriptionResponse::Status204))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateEeSubscriptionResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateEeSubscriptionResponse::Status404(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, UpdateEeSubscriptionResponse::Statusdefault))
			}
		}
	}
}

/// UpdateEeSubscription
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait UpdateEeSubscription {
	/// Patch.
	///
	/// UpdateEeSubscription - PATCH
	/// /nudm-ee/v1/{ueIdentity}/ee-subscriptions/{subscriptionId}
	async fn update_ee_subscription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpdateEeSubscriptionPathParams,
		query_params: models::UpdateEeSubscriptionQueryParams,
		body: Vec<models::PatchItem>,
	) -> Result<UpdateEeSubscriptionResponse, String>;
}
