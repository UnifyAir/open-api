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
pub enum PostSmContextsResponse {
	/// successful creation of an SM context
	Status201_SuccessfulCreationOfAnSMContext {
		body: models::SmContextCreatedData,
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
	/// unsuccessful creation of an SM context - bad request
	Status400_UnsuccessfulCreationOfAnSMContext(models::SmContextCreateError),
	/// unsuccessful creation of an SM context - forbidden
	Status403_UnsuccessfulCreationOfAnSMContext(models::SmContextCreateError),
	/// unsuccessful creation of an SM context - not found
	Status404_UnsuccessfulCreationOfAnSMContext(models::SmContextCreateError),
	/// Length Required
	Status411_LengthRequired(models::common_models::ProblemDetails),
	/// Payload Too Large
	Status413_PayloadTooLarge(models::ExtProblemDetails),
	/// Unsupported Media Type
	Status415_UnsupportedMediaType(models::ExtProblemDetails),
	/// Too Many Requests
	Status429_TooManyRequests(models::ExtProblemDetails),
	/// unsuccessful creation of an SM context - internal server error
	Status500_UnsuccessfulCreationOfAnSMContext(models::SmContextCreateError),
	/// unsuccessful creation of an SM context - service unavailable
	Status503_UnsuccessfulCreationOfAnSMContext(models::SmContextCreateError),
	/// unsuccessful creation of an SM context - gateway timeout
	Status504_UnsuccessfulCreationOfAnSMContext(models::SmContextCreateError),
	/// Generic Error
	Status0_GenericError,
}

/// SmContextsCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SmContextsCollection {
	/// Create SM Context.
	///
	/// PostSmContexts - POST /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts
	async fn post_sm_contexts(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: axum::body::Body,
	) -> Result<PostSmContextsResponse, String>;
}
