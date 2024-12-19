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
pub enum ScpDomainRoutingInfoSubscribeResponse {
	/// Expected response to a valid request
	Status201 {
		body: models::ScpDomainRoutingInfoSubscription,
		location: String,
		accept_encoding: Option<String>,
		content_encoding: Option<String>,
	} = 201,
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

impl DeserResponse for ScpDomainRoutingInfoSubscribeResponse {
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
				let accept_encoding: Option<String> = resp
					.headers()
					.get(reqwest::header::ACCEPT_ENCODING)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let content_encoding: Option<String> = resp
					.headers()
					.get(reqwest::header::CONTENT_ENCODING)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ScpDomainRoutingInfoSubscription = serde_json::from_str(&data)?;

				Ok((status, ScpDomainRoutingInfoSubscribeResponse::Status201 {
					body,
					location,
					accept_encoding,
					content_encoding,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status400(body),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status401(body),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status403(body),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status404(body),
				))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status411(body),
				))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status413(body),
				))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status415(body),
				))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status429(body),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status500(body),
				))
			}
			501 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status501(body),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ScpDomainRoutingInfoSubscribeResponse::Status503(body),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, ScpDomainRoutingInfoSubscribeResponse::Statusdefault))
			}
		}
	}
}

/// ScpDomainRoutingInformationSubscriptionsCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ScpDomainRoutingInformationSubscriptionsCollection {
	/// Create a new subscription.
	///
	/// ScpDomainRoutingInfoSubscribe - POST
	/// /nnrf-disc/v1/scp-domain-routing-info-subs
	async fn scp_domain_routing_info_subscribe(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		header_params: models::ScpDomainRoutingInfoSubscribeHeaderParams,
		body: models::ScpDomainRoutingInfoSubscription,
	) -> Result<ScpDomainRoutingInfoSubscribeResponse, String>;
}
