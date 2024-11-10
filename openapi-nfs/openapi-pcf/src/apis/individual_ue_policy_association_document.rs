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
pub enum DeleteIndividualUePolicyAssociationResponse {
	/// No Content. Resource was successfully deleted
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
	/// Unauthorized
	Status401(models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::ProblemDetails) = 403,
	/// Not Found
	Status404(models::ProblemDetails) = 404,
	/// Too Many Requests
	Status429(models::ProblemDetails) = 429,
	/// Internal Server Error
	Status500(models::ProblemDetails) = 500,
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for DeleteIndividualUePolicyAssociationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			204 => {
				let data = resp.text().await?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status204,
				))
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
					.get(crate::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status307 {
						body,
						location,
						param_3gpp_sbi_target_nf_id,
					},
				))
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
					.get(crate::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status308 {
						body,
						location,
						param_3gpp_sbi_target_nf_id,
					},
				))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status400(body),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status401(body),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status403(body),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status404(body),
				))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status429(body),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status500(body),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Status503(body),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((
					status,
					DeleteIndividualUePolicyAssociationResponse::Statusdefault,
				))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum ReadIndividualUePolicyAssociationResponse {
	/// OK. Resource representation is returned
	Status200(models::PolicyAssociation1) = 200,
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
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for ReadIndividualUePolicyAssociationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::PolicyAssociation1 = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status200(body),
				))
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
					.get(crate::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status307 {
						body,
						location,
						param_3gpp_sbi_target_nf_id,
					},
				))
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
					.get(crate::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status308 {
						body,
						location,
						param_3gpp_sbi_target_nf_id,
					},
				))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status400(body),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status401(body),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status403(body),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status404(body),
				))
			}
			406 => {
				let data = resp.text().await?;

				Ok((status, ReadIndividualUePolicyAssociationResponse::Status406))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status429(body),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status500(body),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Status503(body),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((
					status,
					ReadIndividualUePolicyAssociationResponse::Statusdefault,
				))
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
#[repr(u16)]
#[serde(untagged)]
pub enum ReportObservedEventTriggersForIndividualUePolicyAssociationResponse {
	/// OK. Updated policies are returned
	Status200(models::PolicyUpdate1) = 200,
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
	/// Service Unavailable
	Status503(models::ProblemDetails) = 503,
	/// Generic Error
	Statusdefault = 0,
}

impl DeserResponse for ReportObservedEventTriggersForIndividualUePolicyAssociationResponse {
	async fn deserialize(resp: reqwest::Response) -> Result<(reqwest::StatusCode, Self), ReqError> {
		let status = resp.status();

		match status.as_u16() {
			200 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::PolicyUpdate1 = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status200(
						body,
					),
				))
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
					.get(crate::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status307
				{
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}
				))
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
					.get(crate::PARAM3GPP_SBI_TARGET_NF_ID)
					.map(|v| v.to_str().ok().map(|s| s.to_owned()))
					.flatten();
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::RedirectResponse = serde_json::from_str(&data)?;

				Ok((status, ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status308
				{
					body,
					location,
					param_3gpp_sbi_target_nf_id,
				}
				))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status400(
						body,
					),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status401(
						body,
					),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status403(
						body,
					),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status404(
						body,
					),
				))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status411(
						body,
					),
				))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status413(
						body,
					),
				))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status415(
						body,
					),
				))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status429(
						body,
					),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status500(
						body,
					),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status503(
						body,
					),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((status, ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Statusdefault
				))
			}
		}
	}
}

/// IndividualUePolicyAssociationDocument
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait IndividualUePolicyAssociationDocument {
	/// Delete individual UE policy association..
	///
	/// DeleteIndividualUePolicyAssociation - DELETE
	/// /npcf-ue-policy-control/v1/policies/{polAssoId}
	async fn delete_individual_ue_policy_association(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::DeleteIndividualUePolicyAssociationPathParams,
	) -> Result<DeleteIndividualUePolicyAssociationResponse, String>;

	/// Read individual UE policy association..
	///
	/// ReadIndividualUePolicyAssociation - GET
	/// /npcf-ue-policy-control/v1/policies/{polAssoId}
	async fn read_individual_ue_policy_association(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReadIndividualUePolicyAssociationPathParams,
	) -> Result<ReadIndividualUePolicyAssociationResponse, String>;

	/// Report observed event triggers and possibly obtain updated policies for
	/// an individual UE policy association. .
	///
	/// ReportObservedEventTriggersForIndividualUePolicyAssociation - POST
	/// /npcf-ue-policy-control/v1/policies/{polAssoId}/update
	async fn report_observed_event_triggers_for_individual_ue_policy_association(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReportObservedEventTriggersForIndividualUePolicyAssociationPathParams,
		body: models::PolicyAssociationUpdateRequest1,
	) -> Result<ReportObservedEventTriggersForIndividualUePolicyAssociationResponse, String>;
}
