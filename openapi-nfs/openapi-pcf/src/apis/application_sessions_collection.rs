use std::backtrace::Backtrace;

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use http::Method;
use oasbi::{DeserResponse, ReqError};
use retry_after::RetryAfter;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum PostAppSessionsResponse {
	/// Successful creation of the resource
	Status201 {
		body: models::AppSessionContext,
		location: String,
	} = 201,
	/// See Other. The result of the HTTP POST request would be equivalent to
	/// the existing Application Session Context.
	Status303 { location: String } = 303,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::common_models::ProblemDetails) = 401,
	/// Forbidden
	Status403 {
		body: models::ExtendedProblemDetails,
		retry_after: Option<RetryAfter>,
	} = 403,
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

impl DeserResponse for PostAppSessionsResponse {
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
				let body: models::AppSessionContext = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status201 {
					body,
					location,
				}))
			}
			303 => {
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

				Ok((status, PostAppSessionsResponse::Status303 { location }))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status400(body)))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status401(body)))
			}
			403 => {
				let retry_after: Option<RetryAfter> = resp
					.headers()
					.get(reqwest::header::RETRY_AFTER)
					.map(|v| {
						v.try_into().map_err(|e| {
							ReqError::ParsingRetryHeaderError(
								v.as_bytes().to_owned(),
								e,
								Backtrace::force_capture(),
							)
						})
					})
					.transpose()?;
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ExtendedProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status403 {
					body,
					retry_after,
				}))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, PostAppSessionsResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, PostAppSessionsResponse::Statusdefault))
			}
		}
	}
}

/// ApplicationSessionsCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ApplicationSessionsCollection {
	/// Creates a new Individual Application Session Context resource.
	///
	/// PostAppSessions - POST /npcf-policyauthorization/v1/app-sessions
	async fn post_app_sessions(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::AppSessionContext,
	) -> Result<PostAppSessionsResponse, String>;
}
