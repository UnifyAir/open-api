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
pub enum ReleaseSmContextResponse {
	/// successful release of a PDU session with content in the response
	Status200_SuccessfulReleaseOfAPDUSessionWithContentInTheResponse(models::SmContextReleasedData),
	/// successful release of an SM context without content in the response
	Status204_SuccessfulReleaseOfAnSMContextWithoutContentInTheResponse,
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
	/// Forbidden
	Status403_Forbidden(models::ExtProblemDetails),
	/// Not Found
	Status404_NotFound(models::ExtProblemDetails),
	/// Length Required
	Status411_LengthRequired(models::ProblemDetails),
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

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum RetrieveSmContextResponse {
	/// successful retrieval of an SM context
	Status200_SuccessfulRetrievalOfAnSMContext(models::SmContextRetrievedData),
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
	/// Forbidden
	Status403_Forbidden(models::ExtProblemDetails),
	/// Not Found
	Status404_NotFound(models::ExtProblemDetails),
	/// Length Required
	Status411_LengthRequired(models::ProblemDetails),
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
	/// Gateway Timeout
	Status504_GatewayTimeout(models::ProblemDetails),
	/// Generic Error
	Status0_GenericError,
}

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SendMoDataResponse {
	/// successful sending of MO data
	Status204_SuccessfulSendingOfMOData,
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
	Status411_LengthRequired(models::ProblemDetails),
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

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateSmContextResponse {
	/// successful update of an SM context with content in the response
	Status200_SuccessfulUpdateOfAnSMContextWithContentInTheResponse(models::SmContextUpdatedData),
	/// successful update of an SM context without content in the response
	Status204_SuccessfulUpdateOfAnSMContextWithoutContentInTheResponse,
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
	/// unsuccessful update of an SM context - bad request
	Status400_UnsuccessfulUpdateOfAnSMContext(models::SmContextUpdateError),
	/// unsuccessful update of an SM context - forbidden
	Status403_UnsuccessfulUpdateOfAnSMContext(models::SmContextUpdateError),
	/// unsuccessful update of an SM context - not found
	Status404_UnsuccessfulUpdateOfAnSMContext(models::SmContextUpdateError),
	/// Length Required
	Status411_LengthRequired(models::ProblemDetails),
	/// Payload Too Large
	Status413_PayloadTooLarge(models::ExtProblemDetails),
	/// Unsupported Media Type
	Status415_UnsupportedMediaType(models::ExtProblemDetails),
	/// Too Many Requests
	Status429_TooManyRequests(models::ExtProblemDetails),
	/// unsuccessful update of an SM context - Internal server error
	Status500_UnsuccessfulUpdateOfAnSMContext(models::SmContextUpdateError),
	/// unsuccessful update of an SM context - Service Unavailable
	Status503_UnsuccessfulUpdateOfAnSMContext(models::SmContextUpdateError),
	/// unsuccessful update of an SM context - gateway timeout
	Status504_UnsuccessfulUpdateOfAnSMContext(models::SmContextUpdateError),
	/// Generic Error
	Status0_GenericError,
}

/// IndividualSmContext
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait IndividualSmContext {
	/// Release SM Context.
	///
	/// ReleaseSmContext - POST
	/// /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/
	/// release
	async fn release_sm_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReleaseSmContextPathParams,
		body: axum::body::Body,
	) -> Result<ReleaseSmContextResponse, String>;

	/// Retrieve SM Context.
	///
	/// RetrieveSmContext - POST
	/// /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/
	/// retrieve
	async fn retrieve_sm_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::RetrieveSmContextPathParams,
		body: Option<models::SmContextRetrieveData>,
	) -> Result<RetrieveSmContextResponse, String>;

	/// Send MO Data.
	///
	/// SendMoData - POST
	/// /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/
	/// send-mo-data
	async fn send_mo_data(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::SendMoDataPathParams,
		body: axum::body::Body,
	) -> Result<SendMoDataResponse, String>;

	/// Update SM Context.
	///
	/// UpdateSmContext - POST
	/// /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/modify
	async fn update_sm_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpdateSmContextPathParams,
		body: axum::body::Body,
	) -> Result<UpdateSmContextResponse, String>;
}
