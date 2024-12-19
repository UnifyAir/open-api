use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReleasePduSessionResponse {
	/// successful release of a PDU session with content in the response
	Status200_SuccessfulReleaseOfAPDUSessionWithContentInTheResponse(models::ReleasedData),
	/// successful release of a PDU session
	Status204_SuccessfulReleaseOfAPDUSession,
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
pub enum RetrievePduSessionResponse {
	/// successful information retrieval
	Status200_SuccessfulInformationRetrieval(models::RetrievedData),
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
pub enum TransferMoDataResponse {
	/// successful transfering of MO data
	Status204_SuccessfulTransferingOfMOData,
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
pub enum UpdatePduSessionResponse {
	/// successful update of a PDU session with content in the response
	Status200_SuccessfulUpdateOfAPDUSessionWithContentInTheResponse(models::HsmfUpdatedData),
	/// successful update of a PDU session without content in the response
	Status204_SuccessfulUpdateOfAPDUSessionWithoutContentInTheResponse,
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
	/// unsuccessful update of a PDU session
	Status400_UnsuccessfulUpdateOfAPDUSession(models::HsmfUpdateError),
	/// unsuccessful update of a PDU session
	Status403_UnsuccessfulUpdateOfAPDUSession(models::HsmfUpdateError),
	/// unsuccessful update of a PDU session
	Status404_UnsuccessfulUpdateOfAPDUSession(models::HsmfUpdateError),
	/// Length Required
	Status411_LengthRequired(models::ProblemDetails),
	/// Payload Too Large
	Status413_PayloadTooLarge(models::ExtProblemDetails),
	/// Unsupported Media Type
	Status415_UnsupportedMediaType(models::ExtProblemDetails),
	/// Too Many Requests
	Status429_TooManyRequests(models::ExtProblemDetails),
	/// unsuccessful update of a PDU session
	Status500_UnsuccessfulUpdateOfAPDUSession(models::HsmfUpdateError),
	/// unsuccessful update of a PDU session
	Status503_UnsuccessfulUpdateOfAPDUSession(models::HsmfUpdateError),
	/// Generic Error
	Status0_GenericError,
}

/// IndividualPduSessionHsmforSmf
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait IndividualPduSessionHsmforSmf {
	/// Release.
	///
	/// ReleasePduSession - POST
	/// /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/
	/// release
	async fn release_pdu_session(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReleasePduSessionPathParams,
		body: axum::body::Body,
	) -> Result<ReleasePduSessionResponse, String>;

	/// Retrieve.
	///
	/// RetrievePduSession - POST
	/// /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/
	/// retrieve
	async fn retrieve_pdu_session(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::RetrievePduSessionPathParams,
		body: models::RetrieveData,
	) -> Result<RetrievePduSessionResponse, String>;

	/// Transfer MO Data.
	///
	/// TransferMoData - POST
	/// /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/
	/// transfer-mo-data
	async fn transfer_mo_data(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::TransferMoDataPathParams,
		body: axum::body::Body,
	) -> Result<TransferMoDataResponse, String>;

	/// Update (initiated by V-SMF or I-SMF).
	///
	/// UpdatePduSession - POST
	/// /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/
	/// modify
	async fn update_pdu_session(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpdatePduSessionPathParams,
		body: axum::body::Body,
	) -> Result<UpdatePduSessionResponse, String>;
}
