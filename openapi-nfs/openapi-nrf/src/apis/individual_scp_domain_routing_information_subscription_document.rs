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
pub enum ScpDomainRoutingInfoUnsubscribeResponse {
	/// Expected response to a successful subscription removal
	Status204 = 204,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Length Required
	Status411(models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Not Implemented
	Status501(models::ProblemDetails) = 501,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for ScpDomainRoutingInfoUnsubscribeResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, ScpDomainRoutingInfoUnsubscribeResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status400(body),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status401(body),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status403(body),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status404(body),
				))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status411(body),
				))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status413(body),
				))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status415(body),
				))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status429(body),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status500(body),
				))
			}
			501 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status501(body),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Status503(body),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((
					status,
					ScpDomainRoutingInfoUnsubscribeResponse::Statusdefault,
				))
			}
		}
	}
}

/// IndividualScpDomainRoutingInformationSubscriptionDocument
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait IndividualScpDomainRoutingInformationSubscriptionDocument {
	/// Deletes a subscription.
	///
	/// ScpDomainRoutingInfoUnsubscribe - DELETE
	/// /nnrf-disc/v1/scp-domain-routing-info-subs/{subscriptionID}
	async fn scp_domain_routing_info_unsubscribe(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ScpDomainRoutingInfoUnsubscribePathParams,
	) -> Result<ScpDomainRoutingInfoUnsubscribeResponse, String>;
}
