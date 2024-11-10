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
pub enum PostPduSessionsResponse {
	/// successful creation of a PDU session
	Status201_SuccessfulCreationOfAPDUSession {
		body: models::PduSessionCreatedData,
		location: String,
	},
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
	/// unsuccessful creation of a PDU session
	Status400_UnsuccessfulCreationOfAPDUSession(models::PduSessionCreateError),
	/// unsuccessful creation of a PDU session
	Status403_UnsuccessfulCreationOfAPDUSession(models::PduSessionCreateError),
	/// unsuccessful creation of a PDU session
	Status404_UnsuccessfulCreationOfAPDUSession(models::PduSessionCreateError),
	/// Length Required
	Status411_LengthRequired(models::ProblemDetails),
	/// Payload Too Large
	Status413_PayloadTooLarge(models::ExtProblemDetails),
	/// Unsupported Media Type
	Status415_UnsupportedMediaType(models::ExtProblemDetails),
	/// Too Many Requests
	Status429_TooManyRequests(models::ExtProblemDetails),
	/// unsuccessful creation of a PDU session
	Status500_UnsuccessfulCreationOfAPDUSession(models::PduSessionCreateError),
	/// unsuccessful creation of a PDU session
	Status503_UnsuccessfulCreationOfAPDUSession(models::PduSessionCreateError),
	/// Generic Error
	Status0_GenericError,
}

/// PduSessionsCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait PduSessionsCollection {
	/// Create.
	///
	/// PostPduSessions - POST
	/// /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions
	async fn post_pdu_sessions(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: axum::body::Body,
	) -> Result<PostPduSessionsResponse, String>;
}
