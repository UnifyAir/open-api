use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteIndividualSubcriptionResponse {
	/// No Content. Resource was successfully deleted
	Status204_NoContent,
	/// Temporary Redirect
	Status307_TemporaryRedirect {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	},
	/// Permanent Redirect
	Status308_PermanentRedirect {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	},
	/// Bad request
	Status400_BadRequest(models::ExtProblemDetails),
	/// Unauthorized
	Status401_Unauthorized(models::ExtProblemDetails),
	/// Forbidden
	Status403_Forbidden(models::ExtProblemDetails),
	/// Not Found
	Status404_NotFound(models::ExtProblemDetails),
	/// Too Many Requests
	Status429_TooManyRequests(models::ExtProblemDetails),
	/// Internal Server Error
	Status500_InternalServerError(models::ExtProblemDetails),
	/// Service Unavailable
	Status503_ServiceUnavailable(models::ExtProblemDetails),
	/// Generic Error
	Status0_GenericError,
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetIndividualSubcriptionResponse {
	/// OK. Resource representation is returned
	Status200_OK(models::NsmfEventExposure),
	/// Temporary Redirect
	Status307_TemporaryRedirect {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	},
	/// Permanent Redirect
	Status308_PermanentRedirect {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	},
	/// Bad request
	Status400_BadRequest(models::ExtProblemDetails),
	/// Unauthorized
	Status401_Unauthorized(models::ExtProblemDetails),
	/// Forbidden
	Status403_Forbidden(models::ExtProblemDetails),
	/// Not Found
	Status404_NotFound(models::ExtProblemDetails),
	/// 406 Not Acceptable
	Status406,
	/// Too Many Requests
	Status429_TooManyRequests(models::ExtProblemDetails),
	/// Internal Server Error
	Status500_InternalServerError(models::ExtProblemDetails),
	/// Service Unavailable
	Status503_ServiceUnavailable(models::ExtProblemDetails),
	/// Generic Error
	Status0_GenericError,
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReplaceIndividualSubcriptionResponse {
	/// OK. Resource was successfully modified and representation is returned
	Status200_OK(models::NsmfEventExposure),
	/// No Content. Resource was successfully modified
	Status204_NoContent,
	/// Temporary Redirect
	Status307_TemporaryRedirect {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	},
	/// Permanent Redirect
	Status308_PermanentRedirect {
		body: models::RedirectResponse,
		location: String,
		param_3gpp_sbi_target_nf_id: Option<String>,
	},
	/// Bad request
	Status400_BadRequest(models::ExtProblemDetails),
	/// Unauthorized
	Status401_Unauthorized(models::ExtProblemDetails),
	/// Forbidden
	Status403_Forbidden(models::ExtProblemDetails),
	/// Not Found
	Status404_NotFound(models::ExtProblemDetails),
	/// Length Required
	Status411_LengthRequired(models::common_models::ProblemDetails),
	/// Payload Too Large
	Status413_PayloadTooLarge(models::ExtProblemDetails),
	/// Unsupported Media Type
	Status415_UnsupportedMediaType(models::ExtProblemDetails),
	/// Too Many Requests
	Status429_TooManyRequests(models::ExtProblemDetails),
	/// Internal Server Error
	Status500_InternalServerError(models::ExtProblemDetails),
	/// Service Unavailable
	Status503_ServiceUnavailable(models::ExtProblemDetails),
	/// Generic Error
	Status0_GenericError,
}

/// IndividualSubscriptionDocument
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait IndividualSubscriptionDocument {
	/// Delete an individual subscription for event notifications from the SMF.
	///
	/// DeleteIndividualSubcription - DELETE
	/// /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions/{subId}
	async fn delete_individual_subcription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::DeleteIndividualSubcriptionPathParams,
	) -> Result<DeleteIndividualSubcriptionResponse, String>;

	/// Read an individual subscription for event notifications from the SMF.
	///
	/// GetIndividualSubcription - GET
	/// /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions/{subId}
	async fn get_individual_subcription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::GetIndividualSubcriptionPathParams,
	) -> Result<GetIndividualSubcriptionResponse, String>;

	/// Replace an individual subscription for event notifications from the SMF.
	///
	/// ReplaceIndividualSubcription - PUT
	/// /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions/{subId}
	async fn replace_individual_subcription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReplaceIndividualSubcriptionPathParams,
		body: models::NsmfEventExposure,
	) -> Result<ReplaceIndividualSubcriptionResponse, String>;
}
