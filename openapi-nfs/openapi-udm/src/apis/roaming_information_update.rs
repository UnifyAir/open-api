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
pub enum UpdateRoamingInformationResponse {
	/// Created
	Status201 {
		body: models::RoamingInfoUpdate,
		location: String,
	} = 201,
	/// No content
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

impl DeserResponse for UpdateRoamingInformationResponse {
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
				let body: models::RoamingInfoUpdate = serde_json::from_str(&data)?;

				Ok((status, UpdateRoamingInformationResponse::Status201 {
					body,
					location,
				}))
			}
			204 => {
				let data = resp.text().await?;

				Ok((status, UpdateRoamingInformationResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateRoamingInformationResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateRoamingInformationResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateRoamingInformationResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateRoamingInformationResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UpdateRoamingInformationResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, UpdateRoamingInformationResponse::Statusdefault))
			}
		}
	}
}

/// RoamingInformationUpdate
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait RoamingInformationUpdate {
	/// Update the Roaming Information.
	///
	/// UpdateRoamingInformation - POST
	/// /nudm-uecm/v1/{ueId}/registrations/amf-3gpp-access/roaming-info-update
	async fn update_roaming_information(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpdateRoamingInformationPathParams,
		body: models::RoamingInfoUpdate,
	) -> Result<UpdateRoamingInformationResponse, String>;
}
