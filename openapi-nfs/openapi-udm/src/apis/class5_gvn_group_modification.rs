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
pub enum Get5GVnGroupResponse {
	/// Expected response to a valid request
	Status200(models::Model5GvnGroupConfiguration) = 200,
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

impl DeserResponse for Get5GVnGroupResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::Model5GvnGroupConfiguration = serde_json::from_str(&data)?;

				Ok((status, Get5GVnGroupResponse::Status200(body)))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GVnGroupResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GVnGroupResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GVnGroupResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GVnGroupResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Get5GVnGroupResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, Get5GVnGroupResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum Modify5GVnGroupResponse {
	/// Expected response to a valid request
	Status200(models::PatchResult) = 200,
	/// Expected response to a valid request
	Status204 = 204,
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

impl DeserResponse for Modify5GVnGroupResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::PatchResult = serde_json::from_str(&data)?;

				Ok((status, Modify5GVnGroupResponse::Status200(body)))
			}
			204 => {
				let data = resp.text().await?;

				Ok((status, Modify5GVnGroupResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GVnGroupResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GVnGroupResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GVnGroupResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GVnGroupResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, Modify5GVnGroupResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, Modify5GVnGroupResponse::Statusdefault))
			}
		}
	}
}

/// Class5GvnGroupModification
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Class5GvnGroupModification {
	/// get 5G VN Group.
	///
	/// Get5GVnGroup - GET /nudm-pp/v1/5g-vn-groups/{extGroupId}
	async fn get5_gvn_group(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::Get5GVnGroupPathParams,
	) -> Result<Get5GVnGroupResponse, String>;

	/// modify a 5G VN Group.
	///
	/// Modify5GVnGroup - PATCH /nudm-pp/v1/5g-vn-groups/{extGroupId}
	async fn modify5_gvn_group(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::Modify5GVnGroupPathParams,
		query_params: models::Modify5GVnGroupQueryParams,
		body: models::Model5GvnGroupConfiguration,
	) -> Result<Modify5GVnGroupResponse, String>;
}
