use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateIndividualSubcriptionResponse {
	/// Created.
	Status201_Created {
		body: models::NsmfEventExposure,
		location: String,
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

/// SubscriptionsCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait SubscriptionsCollection {
	/// Create an individual subscription for event notifications from the SMF.
	///
	/// CreateIndividualSubcription - POST
	/// /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions
	async fn create_individual_subcription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::NsmfEventExposure,
	) -> Result<CreateIndividualSubcriptionResponse, String>;
}
