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
pub enum ReportSmDeliveryStatusResponse {
	/// Successful response
	Status204 = 204,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for ReportSmDeliveryStatusResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, ReportSmDeliveryStatusResponse::Status204))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReportSmDeliveryStatusResponse::Status400(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReportSmDeliveryStatusResponse::Status404(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReportSmDeliveryStatusResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReportSmDeliveryStatusResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, ReportSmDeliveryStatusResponse::Statusdefault))
			}
		}
	}
}

/// ReportSmDeliveryStatus
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ReportSmDeliveryStatus {
	/// Report the SM Delivery Status.
	///
	/// ReportSmDeliveryStatus - POST
	/// /nudm-rsds/v1/{ueIdentity}/sm-delivery-status
	async fn report_sm_delivery_status(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReportSmDeliveryStatusPathParams,
		body: models::SmDeliveryStatus,
	) -> Result<ReportSmDeliveryStatusResponse, String>;
}
