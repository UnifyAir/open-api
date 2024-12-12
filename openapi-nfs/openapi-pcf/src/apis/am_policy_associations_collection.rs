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
pub enum CreateIndividualAmPolicyAssociationResponse {
	/// Created
	Status201 {
		body: models::PolicyAssociation,
		location: String,
	} = 201,
	/// Bad request
	Status400(models::common_models::ProblemDetails) = 400,
	/// Unauthorized
	Status401(models::common_models::ProblemDetails) = 401,
	/// Forbidden
	Status403(models::common_models::ProblemDetails) = 403,
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

impl DeserResponse for CreateIndividualAmPolicyAssociationResponse {
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
				let body: models::PolicyAssociation = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status201 { body, location },
				))
			}
			400 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status400(body),
				))
			}
			401 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status401(body),
				))
			}
			403 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status403(body),
				))
			}
			404 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status404(body),
				))
			}
			411 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status411(body),
				))
			}
			413 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status413(body),
				))
			}
			415 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status415(body),
				))
			}
			429 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status429(body),
				))
			}
			500 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status500(body),
				))
			}
			503 => {
				let data = resp.text().await?;
				// Deserialize body only when dataType is present and no headers
				let body: models::common_models::ProblemDetails = serde_json::from_str(&data)?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Status503(body),
				))
			}
			_ => {
				let data = resp.text().await?;

				Ok((
					status,
					CreateIndividualAmPolicyAssociationResponse::Statusdefault,
				))
			}
		}
	}
}

/// AmPolicyAssociationsCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait AmPolicyAssociationsCollection {
	/// Create individual AM policy association..
	///
	/// CreateIndividualAmPolicyAssociation - POST
	/// /npcf-am-policy-control/v1/policies
	async fn create_individual_am_policy_association(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::PolicyAssociationRequest,
	) -> Result<CreateIndividualAmPolicyAssociationResponse, String>;
}
