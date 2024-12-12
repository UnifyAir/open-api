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
pub enum DeleteAuthResponse {
	/// Expected response to a successful authentication result removal
	Status204 = 204,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Not Found
	Status404(models::common_models::ProblemDetails) = 404,
	/// Internal Server Error
	Status500(models::common_models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::common_models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for DeleteAuthResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, DeleteAuthResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, DeleteAuthResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, DeleteAuthResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, DeleteAuthResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, DeleteAuthResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, DeleteAuthResponse::Statusdefault))
			}
		}
	}
}

/// DeleteAuth
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait DeleteAuth {
	/// Deletes the authentication result in the UDM.
	///
	/// DeleteAuth - PUT /nudm-ueau/v1/{supi}/auth-events/{authEventId}
	async fn delete_auth(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::DeleteAuthPathParams,
		body: models::AuthEvent,
	) -> Result<DeleteAuthResponse, String>;
}
