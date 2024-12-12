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
pub enum AMfStatusChangeSubscribeResponse {
	/// N1N2 Message Subscription successfully created.
	Status201 {
		body: models::SubscriptionData,
		location: String,
	} = 201,
	/// Temporary Redirect
	Status307 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 307,
	/// Permanent Redirect
	Status308 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 308,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
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
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for AMfStatusChangeSubscribeResponse {
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
				let body: models::SubscriptionData = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status201 {
					body,
					location,
				}))
			}
			307 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status307 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			308 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status403(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, AMfStatusChangeSubscribeResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, AMfStatusChangeSubscribeResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum CreateSubscriptionResponse {
	/// Subsription Created
	Status201 {
		body: models::AmfCreatedEventSubscription,
		location: String,
	} = 201,
	/// Temporary Redirect
	Status307 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 307,
	/// Permanent Redirect
	Status308 {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	} = 308,
	/// Bad request
	Status400(models::ProblemDetails) = 400,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
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
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for CreateSubscriptionResponse {
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
				let body: models::AmfCreatedEventSubscription = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status201 {
					body,
					location,
				}))
			}
			307 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status307 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			308 => {
				let location: String = resp
					.headers()
					.get(reqwest::header::LOCATION)
					.ok_or(ReqError::RequiredHeaderNotFound(
						"location".to_string(),
						Backtrace::force_capture(),
					))?
					.to_str()?
					.to_owned();
				let param_3gpp_sbi_target_nf_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status403(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateSubscriptionResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, CreateSubscriptionResponse::Statusdefault))
			}
		}
	}
}

/// SubscriptionsCollectionCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SubscriptionsCollectionCollection {
	/// Namf_Communication AMF Status Change Subscribe service Operation.
	///
	/// AMfStatusChangeSubscribe - POST /namf-comm/v1/subscriptions
	async fn amf_status_change_subscribe(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::SubscriptionData,
	) -> Result<AMfStatusChangeSubscribeResponse, String>;

	/// Namf_EventExposure Subscribe service Operation.
	///
	/// CreateSubscription - POST /namf-evts/v1/subscriptions
	async fn create_subscription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::AmfCreateEventSubscription,
	) -> Result<CreateSubscriptionResponse, String>;
}
