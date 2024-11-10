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
pub enum Get5GMbsGroupResponse {
	/// Expected response to a valid request
	Status200(models::MulticastMbsGroupMemb) = 200,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// 406 Not Acceptable
	Status406 = 406,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Bad Gateway
	Status502(models::ProblemDetails) = 502,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for Get5GMbsGroupResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::MulticastMbsGroupMemb = serde_json::from_str(&data)?;

				Ok((status, Get5GMbsGroupResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GMbsGroupResponse::Status400(body)))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GMbsGroupResponse::Status401(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GMbsGroupResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GMbsGroupResponse::Status404(body)))
			}
			406 => {
				let data = resp.text().await?;

				Ok((status, Get5GMbsGroupResponse::Status406))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GMbsGroupResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GMbsGroupResponse::Status500(body)))
			}
			502 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GMbsGroupResponse::Status502(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GMbsGroupResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, Get5GMbsGroupResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum Modify5GMbsGroupResponse {
	/// Expected response to a valid request
	Status200(models::PatchResult) = 200,
	/// Expected response to a valid request
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
	/// Bad Gateway
	Status502(models::ProblemDetails) = 502,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for Modify5GMbsGroupResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::PatchResult = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status200(body)))
			}
			204 => {
				let data = resp.text().await?;

				Ok((status, Modify5GMbsGroupResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status400(body)))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status401(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status500(body)))
			}
			502 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status502(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GMbsGroupResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, Modify5GMbsGroupResponse::Statusdefault))
			}
		}
	}
}

/// Class5GmbsGroupModification
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Class5GmbsGroupModification {
	/// get 5G MBS Group.
	///
	/// Get5GMbsGroup - GET /nudm-pp/v1/mbs-group-membership/{extGroupId}
	async fn get5_g_mbs_group(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::Get5GMbsGroupPathParams,
	) -> Result<Get5GMbsGroupResponse, String>;

	/// modify a 5G MBS Group.
	///
	/// Modify5GMbsGroup - PATCH /nudm-pp/v1/mbs-group-membership/{extGroupId}
	async fn modify5_g_mbs_group(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::Modify5GMbsGroupPathParams,
		query_params: models::Modify5GMbsGroupQueryParams,
		body: models::MulticastMbsGroupMemb,
	) -> Result<Modify5GMbsGroupResponse, String>;
}
