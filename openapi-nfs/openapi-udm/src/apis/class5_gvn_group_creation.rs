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
pub enum Create5GVnGroupResponse {
	/// Expected response to a valid request
	Status201 = 201,
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

impl DeserResponse for Create5GVnGroupResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			201 => {
				let data = resp.text().await?;

				Ok((status, Create5GVnGroupResponse::Status201))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Create5GVnGroupResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Create5GVnGroupResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Create5GVnGroupResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Create5GVnGroupResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Create5GVnGroupResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, Create5GVnGroupResponse::Statusdefault))
			}
		}
	}
}

/// Class5GvnGroupCreation
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Class5GvnGroupCreation {
	/// create a 5G VN Group.
	///
	/// Create5GVnGroup - PUT /nudm-pp/v1/5g-vn-groups/{extGroupId}
	async fn create5_gvn_group(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::Create5GVnGroupPathParams,
		body: models::Model5GvnGroupConfiguration,
	) -> Result<Create5GVnGroupResponse, String>;
}
