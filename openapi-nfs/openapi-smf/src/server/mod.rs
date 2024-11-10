use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header::CONTENT_TYPE};
use tracing::error;
use validator::{Validate, ValidationErrors};

#[allow(unused_imports)]
use crate::{apis, models};
use crate::{header, types::*};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
	I: AsRef<A> + Clone + Send + Sync + 'static,
	A: apis::individual_pdu_session_hsmfor_smf::IndividualPduSessionHsmforSmf
		+ apis::individual_pdu_session_nsmf_nidd::IndividualPduSessionNsmfNidd
		+ apis::individual_sm_context::IndividualSmContext
		+ apis::individual_subscription_document::IndividualSubscriptionDocument
		+ apis::pdu_sessions_collection::PduSessionsCollection
		+ apis::sm_contexts_collection::SmContextsCollection
		+ apis::subscriptions_collection::SubscriptionsCollection
		+ 'static,
{
	// build our application with a route
	Router::new()
		.route(
			"/nsmf-event-exposure/v1/subscriptions",
			post(create_individual_subcription::<I, A>),
		)
		.route(
			"/nsmf-event-exposure/v1/subscriptions/:sub_id",
			delete(delete_individual_subcription::<I, A>)
				.get(get_individual_subcription::<I, A>)
				.put(replace_individual_subcription::<I, A>),
		)
		.route(
			"/nsmf-nidd/v1/pdu-sessions/:pdu_session_ref/deliver",
			post(deliver::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/pdu-sessions",
			post(post_pdu_sessions::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/pdu-sessions/:pdu_session_ref/modify",
			post(update_pdu_session::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions/:pdu_session_ref/release",
			post(release_pdu_session::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/pdu-sessions/:pdu_session_ref/retrieve",
			post(retrieve_pdu_session::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/pdu-sessions/:pdu_session_ref/transfer-mo-data",
			post(transfer_mo_data::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/sm-contexts",
			post(post_sm_contexts::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/sm-contexts/:sm_context_ref/modify",
			post(update_sm_context::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/sm-contexts/:sm_context_ref/release",
			post(release_sm_context::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/sm-contexts/:sm_context_ref/retrieve",
			post(retrieve_sm_context::<I, A>),
		)
		.route(
			"/nsmf-pdusession/v1/sm-contexts/:sm_context_ref/send-mo-data",
			post(send_mo_data::<I, A>),
		)
		.with_state(api_impl)
}

// #[derive(validator::Validate)]
// #[allow(dead_code)]
// struct ReleasePduSessionBodyValidator<'a> {
//         #[validate(nested)]
//       body: &'a models::ReleaseData,
// }

#[tracing::instrument(skip_all)]
fn release_pdu_session_validation(
	path_params: models::ReleasePduSessionPathParams,
	body: Option<models::ReleaseData>,
) -> std::result::Result<
	(
		models::ReleasePduSessionPathParams,
		Option<models::ReleaseData>,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	// if let Some(body) = &body {
	//   let b = ReleasePduSessionBodyValidator { body };
	//   b.validate()?;
	// }

	Ok((path_params, body))
}
/// ReleasePduSession - POST
/// /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/release
#[tracing::instrument(skip_all)]
async fn release_pdu_session<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ReleasePduSessionPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_pdu_session_hsmfor_smf::IndividualPduSessionHsmforSmf,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.release_pdu_session(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status200_SuccessfulReleaseOfAPDUSessionWithContentInTheResponse
            (body)
            => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status204_SuccessfulReleaseOfAPDUSession
            => {
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::ReleasePduSessionResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

// #[derive(validator::Validate)]
// #[allow(dead_code)]
// struct RetrievePduSessionBodyValidator<'a> {
//         #[validate(nested)]
//       body: &'a models::RetrieveData,
// }

#[tracing::instrument(skip_all)]
fn retrieve_pdu_session_validation(
	path_params: models::RetrievePduSessionPathParams,
	body: models::RetrieveData,
) -> std::result::Result<
	(models::RetrievePduSessionPathParams, models::RetrieveData),
	ValidationErrors,
> {
	path_params.validate()?;
	// let b = RetrievePduSessionBodyValidator { body: &body };
	// b.validate()?;

	Ok((path_params, body))
}
/// RetrievePduSession - POST
/// /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/retrieve
#[tracing::instrument(skip_all)]
async fn retrieve_pdu_session<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::RetrievePduSessionPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::RetrieveData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_pdu_session_hsmfor_smf::IndividualPduSessionHsmforSmf,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || retrieve_pdu_session_validation(path_params, body))
			.await
			.unwrap();

	let Ok((path_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.retrieve_pdu_session(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status200_SuccessfulInformationRetrieval
            (body)
            => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status504_GatewayTimeout
            (body)
            => {
                let mut response = response.status(504);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::RetrievePduSessionResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[tracing::instrument(skip_all)]
fn transfer_mo_data_validation(
	path_params: models::TransferMoDataPathParams
) -> std::result::Result<(models::TransferMoDataPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}
/// TransferMoData - POST
/// /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/
/// transfer-mo-data
#[tracing::instrument(skip_all)]
async fn transfer_mo_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::TransferMoDataPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_pdu_session_hsmfor_smf::IndividualPduSessionHsmforSmf,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.transfer_mo_data(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status204_SuccessfulTransferingOfMOData
            => {
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status401_Unauthorized
            (body)
            => {
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::TransferMoDataResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

// #[derive(validator::Validate)]
// #[allow(dead_code)]
// struct UpdatePduSessionBodyValidator<'a> {
//         #[validate(nested)]
//       body: &'a models::HsmfUpdateData,
// }

#[tracing::instrument(skip_all)]
fn update_pdu_session_validation(
	path_params: models::UpdatePduSessionPathParams,
	body: models::HsmfUpdateData,
) -> std::result::Result<
	(models::UpdatePduSessionPathParams, models::HsmfUpdateData),
	ValidationErrors,
> {
	path_params.validate()?;
	// let b = UpdatePduSessionBodyValidator { body: &body };
	// b.validate()?;

	Ok((path_params, body))
}
/// UpdatePduSession - POST
/// /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/modify
#[tracing::instrument(skip_all)]
async fn update_pdu_session<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdatePduSessionPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_pdu_session_hsmfor_smf::IndividualPduSessionHsmforSmf,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.update_pdu_session(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status200_SuccessfulUpdateOfAPDUSessionWithContentInTheResponse
            (body)
            => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status204_SuccessfulUpdateOfAPDUSessionWithoutContentInTheResponse
            => {
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status400_UnsuccessfulUpdateOfAPDUSession
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status403_UnsuccessfulUpdateOfAPDUSession
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status404_UnsuccessfulUpdateOfAPDUSession
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status500_UnsuccessfulUpdateOfAPDUSession
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status503_UnsuccessfulUpdateOfAPDUSession
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_hsmfor_smf::UpdatePduSessionResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[tracing::instrument(skip_all)]
fn deliver_validation(
	path_params: models::DeliverPathParams
) -> std::result::Result<(models::DeliverPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}
/// Deliver - POST
/// /nsmf-pdusession/v1/nsmf-nidd/v1/pdu-sessions/{pduSessionRef}/deliver
#[tracing::instrument(skip_all)]
async fn deliver<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeliverPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_pdu_session_nsmf_nidd::IndividualPduSessionNsmfNidd,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.deliver(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status204_SuccessfulTransferingOfDelivery
            => {
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status401_Unauthorized
            (body)
            => {
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status504_UnsuccessfulDeliveryOfMobileTerminatedData
            (body)
            => {
                let mut response = response.status(504);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_pdu_session_nsmf_nidd::DeliverResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

// #[derive(validator::Validate)]
// #[allow(dead_code)]
// struct ReleaseSmContextBodyValidator<'a> {
//         #[validate(nested)]
//       body: &'a models::SmContextReleaseData,
// }

#[tracing::instrument(skip_all)]
fn release_sm_context_validation(
	path_params: models::ReleaseSmContextPathParams,
	body: Option<models::SmContextReleaseData>,
) -> std::result::Result<
	(
		models::ReleaseSmContextPathParams,
		Option<models::SmContextReleaseData>,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	// if let Some(body) = &body {
	//   let b = ReleaseSmContextBodyValidator { body };
	//   b.validate()?;
	// }

	Ok((path_params, body))
}
/// ReleaseSmContext - POST
/// /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/release
#[tracing::instrument(skip_all)]
async fn release_sm_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ReleaseSmContextPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_sm_context::IndividualSmContext,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.release_sm_context(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_sm_context::ReleaseSmContextResponse::Status200_SuccessfulReleaseOfAPDUSessionWithContentInTheResponse
            (body)
            => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status204_SuccessfulReleaseOfAnSMContextWithoutContentInTheResponse
            => {
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::ReleaseSmContextResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

// #[derive(validator::Validate)]
// #[allow(dead_code)]
// struct RetrieveSmContextBodyValidator<'a> {
//         #[validate(nested)]
//       body: &'a models::SmContextRetrieveData,
// }

#[tracing::instrument(skip_all)]
fn retrieve_sm_context_validation(
	path_params: models::RetrieveSmContextPathParams,
	body: Option<models::SmContextRetrieveData>,
) -> std::result::Result<
	(
		models::RetrieveSmContextPathParams,
		Option<models::SmContextRetrieveData>,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	// if let Some(body) = &body {
	//   let b = RetrieveSmContextBodyValidator { body };
	//   b.validate()?;
	// }

	Ok((path_params, body))
}
/// RetrieveSmContext - POST
/// /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/retrieve
#[tracing::instrument(skip_all)]
async fn retrieve_sm_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::RetrieveSmContextPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::SmContextRetrieveData>>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_sm_context::IndividualSmContext,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || retrieve_sm_context_validation(path_params, body))
			.await
			.unwrap();

	let Ok((path_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.retrieve_sm_context(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_sm_context::RetrieveSmContextResponse::Status200_SuccessfulRetrievalOfAnSMContext
            (body)
            => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status504_GatewayTimeout
            (body)
            => {
                let mut response = response.status(504);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::RetrieveSmContextResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[tracing::instrument(skip_all)]
fn send_mo_data_validation(
	path_params: models::SendMoDataPathParams
) -> std::result::Result<(models::SendMoDataPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}
/// SendMoData - POST
/// /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/
/// send-mo-data
#[tracing::instrument(skip_all)]
async fn send_mo_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::SendMoDataPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_sm_context::IndividualSmContext,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.send_mo_data(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_sm_context::SendMoDataResponse::Status204_SuccessfulSendingOfMOData
            => {
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::individual_sm_context::SendMoDataResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status401_Unauthorized
            (body)
            => {
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::SendMoDataResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

// #[derive(validator::Validate)]
// #[allow(dead_code)]
// struct UpdateSmContextBodyValidator<'a> {
//         #[validate(nested)]
//       body: &'a models::SmContextUpdateData,
// }

#[tracing::instrument(skip_all)]
fn update_sm_context_validation(
	path_params: models::UpdateSmContextPathParams,
	body: models::SmContextUpdateData,
) -> std::result::Result<
	(
		models::UpdateSmContextPathParams,
		models::SmContextUpdateData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	// let b = UpdateSmContextBodyValidator { body: &body };
	// b.validate()?;

	Ok((path_params, body))
}
/// UpdateSmContext - POST
/// /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/modify
#[tracing::instrument(skip_all)]
async fn update_sm_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateSmContextPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_sm_context::IndividualSmContext,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.update_sm_context(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_sm_context::UpdateSmContextResponse::Status200_SuccessfulUpdateOfAnSMContextWithContentInTheResponse
            (body)
            => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status204_SuccessfulUpdateOfAnSMContextWithoutContentInTheResponse
            => {
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status400_UnsuccessfulUpdateOfAnSMContext
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status403_UnsuccessfulUpdateOfAnSMContext
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status404_UnsuccessfulUpdateOfAnSMContext
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status500_UnsuccessfulUpdateOfAnSMContext
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status503_UnsuccessfulUpdateOfAnSMContext
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status504_UnsuccessfulUpdateOfAnSMContext
            (body)
            => {
                let mut response = response.status(504);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_sm_context::UpdateSmContextResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[tracing::instrument(skip_all)]
fn delete_individual_subcription_validation(
	path_params: models::DeleteIndividualSubcriptionPathParams
) -> std::result::Result<(models::DeleteIndividualSubcriptionPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}
/// DeleteIndividualSubcription - DELETE
/// /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions/{subId}
#[tracing::instrument(skip_all)]
async fn delete_individual_subcription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteIndividualSubcriptionPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_subscription_document::IndividualSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || delete_individual_subcription_validation(path_params))
			.await
			.unwrap();

	let Ok((path_params,)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.delete_individual_subcription(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status204_NoContent
            => {
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status401_Unauthorized
            (body)
            => {
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::DeleteIndividualSubcriptionResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[tracing::instrument(skip_all)]
fn get_individual_subcription_validation(
	path_params: models::GetIndividualSubcriptionPathParams
) -> std::result::Result<(models::GetIndividualSubcriptionPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}
/// GetIndividualSubcription - GET
/// /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions/{subId}
#[tracing::instrument(skip_all)]
async fn get_individual_subcription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetIndividualSubcriptionPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_subscription_document::IndividualSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || get_individual_subcription_validation(path_params))
			.await
			.unwrap();

	let Ok((path_params,)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_individual_subcription(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status200_OK
            (body)
            => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status401_Unauthorized
            (body)
            => {
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status406
            => {
                let mut response = response.status(406);
                response.body(Body::empty())
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::GetIndividualSubcriptionResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

// #[derive(validator::Validate)]
// #[allow(dead_code)]
// struct ReplaceIndividualSubcriptionBodyValidator<'a> {
//         #[validate(nested)]
//       body: &'a models::NsmfEventExposure,
// }

#[tracing::instrument(skip_all)]
fn replace_individual_subcription_validation(
	path_params: models::ReplaceIndividualSubcriptionPathParams,
	body: models::NsmfEventExposure,
) -> std::result::Result<
	(
		models::ReplaceIndividualSubcriptionPathParams,
		models::NsmfEventExposure,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	// let b = ReplaceIndividualSubcriptionBodyValidator { body: &body };
	// b.validate()?;

	Ok((path_params, body))
}
/// ReplaceIndividualSubcription - PUT
/// /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions/{subId}
#[tracing::instrument(skip_all)]
async fn replace_individual_subcription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ReplaceIndividualSubcriptionPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::NsmfEventExposure>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_subscription_document::IndividualSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		replace_individual_subcription_validation(path_params, body)
	})
	.await
	.unwrap();

	let Ok((path_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.replace_individual_subcription(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status200_OK
            (body)
            => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status204_NoContent
            => {
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status401_Unauthorized
            (body)
            => {
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::individual_subscription_document::ReplaceIndividualSubcriptionResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

// #[derive(validator::Validate)]
// #[allow(dead_code)]
// struct PostPduSessionsBodyValidator<'a> {
//         #[validate(nested)]
//       body: &'a models::PduSessionCreateData,
// }

#[tracing::instrument(skip_all)]
fn post_pdu_sessions_validation(
	body: models::PduSessionCreateData
) -> std::result::Result<(models::PduSessionCreateData,), ValidationErrors> {
	// let b = PostPduSessionsBodyValidator { body: &body };
	// b.validate()?;

	Ok((body,))
}
/// PostPduSessions - POST /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions
#[tracing::instrument(skip_all)]
async fn post_pdu_sessions<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::pdu_sessions_collection::PduSessionsCollection,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.post_pdu_sessions(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status201_SuccessfulCreationOfAPDUSession
            {
                body,
                location
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                let mut response = response.status(201);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status400_UnsuccessfulCreationOfAPDUSession
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status403_UnsuccessfulCreationOfAPDUSession
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status404_UnsuccessfulCreationOfAPDUSession
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status500_UnsuccessfulCreationOfAPDUSession
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status503_UnsuccessfulCreationOfAPDUSession
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pdu_sessions_collection::PostPduSessionsResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[tracing::instrument(skip_all)]
fn post_sm_contexts_validation() -> std::result::Result<(), ValidationErrors> {
	Ok(())
}
/// PostSmContexts - POST /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts
#[tracing::instrument(skip_all)]
async fn post_sm_contexts<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::sm_contexts_collection::SmContextsCollection,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.post_sm_contexts(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::sm_contexts_collection::PostSmContextsResponse::Status201_SuccessfulCreationOfAnSMContext
            {
                body,
                location
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                let mut response = response.status(201);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status307_TemporaryRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(307);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status308_PermanentRedirect
            {
                body,
                location,
                param_3gpp_sbi_target_nf_id
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
                    let param_3gpp_sbi_target_nf_id = match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_target_nf_id header - {}", e))).map_err(|e| {
                                error!(error = ?e);
                                StatusCode::INTERNAL_SERVER_ERROR
                            });
                        }
                    };


                    {
                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                            HeaderName::from_static(""),
                            param_3gpp_sbi_target_nf_id,
                        );
                    }
                }
                let mut response = response.status(308);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status400_UnsuccessfulCreationOfAnSMContext
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status403_UnsuccessfulCreationOfAnSMContext
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status404_UnsuccessfulCreationOfAnSMContext
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status500_UnsuccessfulCreationOfAnSMContext
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status503_UnsuccessfulCreationOfAnSMContext
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status504_UnsuccessfulCreationOfAnSMContext
            (body)
            => {
                let mut response = response.status(504);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sm_contexts_collection::PostSmContextsResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

// #[derive(validator::Validate)]
// #[allow(dead_code)]
// struct CreateIndividualSubcriptionBodyValidator<'a> {
//         #[validate(nested)]
//       body: &'a models::NsmfEventExposure,
// }

#[tracing::instrument(skip_all)]
fn create_individual_subcription_validation(
	body: models::NsmfEventExposure
) -> std::result::Result<(models::NsmfEventExposure,), ValidationErrors> {
	// let b = CreateIndividualSubcriptionBodyValidator { body: &body };
	// b.validate()?;

	Ok((body,))
}
/// CreateIndividualSubcription - POST
/// /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions
#[tracing::instrument(skip_all)]
async fn create_individual_subcription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::NsmfEventExposure>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscriptions_collection::SubscriptionsCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || create_individual_subcription_validation(body))
			.await
			.unwrap();

	let Ok((body,)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.create_individual_subcription(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
        Ok(rsp) => match rsp {
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status201_Created
            {
                body,
                location
            }
            => {
                let location = match header::IntoHeaderValue(location).try_into() {
                    Ok(val) => val,
                    Err(e) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("An internal server error occurred handling location header - {}", e))).map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        });
                    }
                };


                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static(""),
                        location,
                    );
                }
                let mut response = response.status(201);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status400_BadRequest
            (body)
            => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status401_Unauthorized
            (body)
            => {
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status403_Forbidden
            (body)
            => {
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status404_NotFound
            (body)
            => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status411_LengthRequired
            (body)
            => {
                let mut response = response.status(411);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status413_PayloadTooLarge
            (body)
            => {
                let mut response = response.status(413);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status415_UnsupportedMediaType
            (body)
            => {
                let mut response = response.status(415);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status429_TooManyRequests
            (body)
            => {
                let mut response = response.status(429);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status500_InternalServerError
            (body)
            => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status503_ServiceUnavailable
            (body)
            => {
                let mut response = response.status(503);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?);
                }

                let body_content = tokio::task::spawn_blocking(move ||
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })).await.unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::subscriptions_collection::CreateIndividualSubcriptionResponse::Status0_GenericError
            => {
                let mut response = response.status(0);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}
