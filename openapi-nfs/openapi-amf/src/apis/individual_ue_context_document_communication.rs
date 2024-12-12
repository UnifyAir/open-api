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
pub enum CancelRelocateUeContextResponse {
	/// UE Context successfully released
	Status204 = 204,
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
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for CancelRelocateUeContextResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, CancelRelocateUeContextResponse::Status204))
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

				Ok((status, CancelRelocateUeContextResponse::Status307 {
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

				Ok((status, CancelRelocateUeContextResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelRelocateUeContextResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelRelocateUeContextResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelRelocateUeContextResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelRelocateUeContextResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelRelocateUeContextResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelRelocateUeContextResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelRelocateUeContextResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelRelocateUeContextResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CancelRelocateUeContextResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, CancelRelocateUeContextResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum CreateUeContextResponse {
	/// UE context successfully created.
	Status201 {
		body: models::UeContextCreatedData,
		location: String,
		param_3gpp_sbi_producer_id: Option<String>,
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
	/// Bad Request
	Status400(models::UeContextCreateError) = 400,
	/// Forbidden
	Status403(models::UeContextCreateError) = 403,
	/// Length Required
	Status411(models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::UeContextCreateError) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for CreateUeContextResponse {
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
				let param_3gpp_sbi_producer_id: Option<String> = resp
					.headers()
					.get(oasbi::PARAM3GPP_SBI_PRODUCER_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::UeContextCreatedData = serde_json::from_str(&data)?;

				Ok((status, CreateUeContextResponse::Status201 {
					body,
					location,
					param_3gpp_sbi_producer_id,
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

				Ok((status, CreateUeContextResponse::Status307 {
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

				Ok((status, CreateUeContextResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::UeContextCreateError = serde_json::from_str(&data)?;

				Ok((status, CreateUeContextResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::UeContextCreateError = serde_json::from_str(&data)?;

				Ok((status, CreateUeContextResponse::Status403(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateUeContextResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateUeContextResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateUeContextResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateUeContextResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::UeContextCreateError = serde_json::from_str(&data)?;

				Ok((status, CreateUeContextResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, CreateUeContextResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, CreateUeContextResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum EBiAssignmentResponse {
	/// EBI Assignment successfully performed.
	Status200(models::AssignedEbiData) = 200,
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
	/// Bad Request
	Status400(models::AssignEbiError) = 400,
	/// Forbidden
	Status403(models::AssignEbiError) = 403,
	/// Conflict
	Status409(models::AssignEbiError) = 409,
	/// Length Required
	Status411(models::ProblemDetails) = 411,
	/// Payload Too Large
	Status413(models::ProblemDetails) = 413,
	/// Unsupported Media Type
	Status415(models::ProblemDetails) = 415,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::AssignEbiError) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for EBiAssignmentResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::AssignedEbiData = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status200(body)))
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

				Ok((status, EBiAssignmentResponse::Status307 {
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

				Ok((status, EBiAssignmentResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::AssignEbiError = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::AssignEbiError = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status403(body)))
			}
			409 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::AssignEbiError = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status409(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::AssignEbiError = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, EBiAssignmentResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, EBiAssignmentResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum RegistrationStatusUpdateResponse {
	/// UE context transfer status successfully updated.
	Status200(models::UeRegStatusUpdateRspData) = 200,
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
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for RegistrationStatusUpdateResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::UeRegStatusUpdateRspData = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status200(body)))
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

				Ok((status, RegistrationStatusUpdateResponse::Status307 {
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

				Ok((status, RegistrationStatusUpdateResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RegistrationStatusUpdateResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, RegistrationStatusUpdateResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum ReleaseUeContextResponse {
	/// UE Context successfully released
	Status204 = 204,
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
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for ReleaseUeContextResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((status, ReleaseUeContextResponse::Status204))
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

				Ok((status, ReleaseUeContextResponse::Status307 {
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

				Ok((status, ReleaseUeContextResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReleaseUeContextResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReleaseUeContextResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReleaseUeContextResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReleaseUeContextResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReleaseUeContextResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReleaseUeContextResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReleaseUeContextResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReleaseUeContextResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, ReleaseUeContextResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, ReleaseUeContextResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum RelocateUeContextResponse {
	/// UE context successfully relocated.
	Status201 {
		body: models::UeContextRelocatedData,
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

impl DeserResponse for RelocateUeContextResponse {
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
				let body: models::UeContextRelocatedData = serde_json::from_str(&data)?;

				Ok((status, RelocateUeContextResponse::Status201 {
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

				Ok((status, RelocateUeContextResponse::Status307 {
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

				Ok((status, RelocateUeContextResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RelocateUeContextResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RelocateUeContextResponse::Status403(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RelocateUeContextResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RelocateUeContextResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RelocateUeContextResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RelocateUeContextResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RelocateUeContextResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, RelocateUeContextResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, RelocateUeContextResponse::Statusdefault))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum UEContextTransferResponse {
	/// UE context transfer successfully initiated.
	Status200(models::UeContextTransferRspData) = 200,
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
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Unexpected error
	Statusdefault = 0,
}

impl DeserResponse for UEContextTransferResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::UeContextTransferRspData = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status200(body)))
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

				Ok((status, UEContextTransferResponse::Status307 {
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

				Ok((status, UEContextTransferResponse::Status308 {
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status400(body)))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status403(body)))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status404(body)))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status411(body)))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status413(body)))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status415(body)))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status429(body)))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status500(body)))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((status, UEContextTransferResponse::Status503(body)))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, UEContextTransferResponse::Statusdefault))
			}
		}
	}
}

/// IndividualUeContextDocumentCommunication
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait IndividualUeContextDocumentCommunication {
	/// Namf_Communication CancelRelocateUEContext service Operation.
	///
	/// CancelRelocateUeContext - POST
	/// /namf-comm/v1/ue-contexts/{ueContextId}/cancel-relocate
	async fn cancel_relocate_ue_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::CancelRelocateUeContextPathParams,
		body: axum::body::Body,
	) -> Result<CancelRelocateUeContextResponse, String>;

	/// Namf_Communication CreateUEContext service Operation.
	///
	/// CreateUeContext - PUT /namf-comm/v1/ue-contexts/{ueContextId}
	async fn create_ue_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::CreateUeContextPathParams,
		body: axum::body::Body,
	) -> Result<CreateUeContextResponse, String>;

	/// Namf_Communication EBI Assignment service Operation.
	///
	/// EBiAssignment - POST /namf-comm/v1/ue-contexts/{ueContextId}/assign-ebi
	async fn ebi_assignment(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::EBiAssignmentPathParams,
		body: models::AssignEbiData,
	) -> Result<EBiAssignmentResponse, String>;

	/// Namf_Communication RegistrationStatusUpdate service Operation.
	///
	/// RegistrationStatusUpdate - POST
	/// /namf-comm/v1/ue-contexts/{ueContextId}/transfer-update
	async fn registration_status_update(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::RegistrationStatusUpdatePathParams,
		body: models::UeRegStatusUpdateReqData,
	) -> Result<RegistrationStatusUpdateResponse, String>;

	/// Namf_Communication ReleaseUEContext service Operation.
	///
	/// ReleaseUeContext - POST /namf-comm/v1/ue-contexts/{ueContextId}/release
	async fn release_ue_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReleaseUeContextPathParams,
		body: models::UeContextRelease,
	) -> Result<ReleaseUeContextResponse, String>;

	/// Namf_Communication RelocateUEContext service Operation.
	///
	/// RelocateUeContext - POST
	/// /namf-comm/v1/ue-contexts/{ueContextId}/relocate
	async fn relocate_ue_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::RelocateUeContextPathParams,
		body: axum::body::Body,
	) -> Result<RelocateUeContextResponse, String>;

	/// Namf_Communication UEContextTransfer service Operation.
	///
	/// UEContextTransfer - POST
	/// /namf-comm/v1/ue-contexts/{ueContextId}/transfer
	async fn ue_context_transfer(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UEContextTransferPathParams,
		body: axum::body::Body,
	) -> Result<UEContextTransferResponse, String>;
}
