use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use oasbi::{DeserResponse, ReqError};
use tracing::error;
use validator::{Validate, ValidationErrors};

#[allow(unused_imports)]
use crate::{apis, models};
use crate::{header, types::*};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
							  where
								  I: AsRef<A> + Clone + Send + Sync + 'static,
								  A: apis::individual_subscription_document::IndividualSubscriptionDocument + apis::individual_ue_context_document_location::IndividualUeContextDocumentLocation + apis::individual_ue_context_document_communication::IndividualUeContextDocumentCommunication + apis::n1_n2_individual_subscription_document::N1N2IndividualSubscriptionDocument + apis::n1_n2_message_collection_collection::N1N2MessageCollectionCollection + apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2SubscriptionsCollectionForIndividualUeContextsCollection + apis::non_uen2_message_notification_individual_subscription_document::NonUen2MessageNotificationIndividualSubscriptionDocument + apis::non_uen2_messages_collection_collection::NonUen2MessagesCollectionCollection + apis::non_uen2_messages_subscriptions_collection_collection::NonUen2MessagesSubscriptionsCollectionCollection + apis::subscriptions_collection_collection::SubscriptionsCollectionCollection + apis::ue_context_document::UeContextDocument + apis::ue_contexts_collection::UeContextsCollection + apis::ue_reach_ind_document::UeReachIndDocument + 'static,
{
	// build our application with a route
	Router::new()
		.route(
			"/namf-comm/v1/non-ue-n2-messages/subscriptions",
			post(non_ue_n2_info_subscribe::<I, A>),
		)
		.route(
			"/namf-comm/v1/non-ue-n2-messages/subscriptions/:n2_notify_subscription_id",
			delete(non_ue_n2_info_un_subscribe::<I, A>),
		)
		.route(
			"/namf-comm/v1/non-ue-n2-messages/transfer",
			post(non_ue_n2_message_transfer::<I, A>),
		)
		.route(
			"/namf-comm/v1/subscriptions",
			post(amf_status_change_subscribe::<I, A>),
		)
		.route(
			"/namf-comm/v1/subscriptions/:subscription_id",
			delete(amf_status_change_un_subscribe::<I, A>)
				.put(amf_status_change_subscribe_modfy::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id",
			put(create_ue_context::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id/assign-ebi",
			post(ebi_assignment::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id/cancel-relocate",
			post(cancel_relocate_ue_context::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id/n1-n2-messages",
			post(n1_n2_message_transfer::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id/n1-n2-messages/subscriptions",
			post(n1_n2_message_subscribe::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id/n1-n2-messages/subscriptions/:\
			 subscription_id",
			delete(n1_n2_message_un_subscribe::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id/release",
			post(release_ue_context::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id/relocate",
			post(relocate_ue_context::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id/transfer",
			post(ue_context_transfer::<I, A>),
		)
		.route(
			"/namf-comm/v1/ue-contexts/:ue_context_id/transfer-update",
			post(registration_status_update::<I, A>),
		)
		.route(
			"/namf-evts/v1/subscriptions",
			post(create_subscription::<I, A>),
		)
		.route(
			"/namf-evts/v1/subscriptions/:subscription_id",
			delete(delete_subscription::<I, A>).patch(modify_subscription::<I, A>),
		)
		.route(
			"/namf-loc/v1/:ue_context_id/cancel-pos-info",
			post(cancel_location::<I, A>),
		)
		.route(
			"/namf-loc/v1/:ue_context_id/provide-loc-info",
			post(provide_location_info::<I, A>),
		)
		.route(
			"/namf-loc/v1/:ue_context_id/provide-pos-info",
			post(provide_positioning_info::<I, A>),
		)
		.route(
			"/namf-mt/v1/ue-contexts/:ue_context_id",
			get(provide_domain_selection_info::<I, A>),
		)
		.route(
			"/namf-mt/v1/ue-contexts/:ue_context_id/ue-reachind",
			put(enable_ue_reachability::<I, A>),
		)
		.route(
			"/namf-mt/v1/ue-contexts/enable-group-reachability",
			post(enable_group_reachability::<I, A>),
		)
		.with_state(api_impl)
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct AMfStatusChangeSubscribeModfyBodyValidator<'a> {
	body: &'a models::SubscriptionData,
}

#[tracing::instrument(skip_all)]
fn amf_status_change_subscribe_modfy_validation(
	path_params: models::AMfStatusChangeSubscribeModfyPathParams,
	body: models::SubscriptionData,
) -> std::result::Result<
	(
		models::AMfStatusChangeSubscribeModfyPathParams,
		models::SubscriptionData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = AMfStatusChangeSubscribeModfyBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// AMfStatusChangeSubscribeModfy - PUT
/// /namf-comm/v1/subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn amf_status_change_subscribe_modfy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::AMfStatusChangeSubscribeModfyPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SubscriptionData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_subscription_document::IndividualSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		amf_status_change_subscribe_modfy_validation(path_params, body)
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
		.amf_status_change_subscribe_modfy(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status200
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status307
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status308
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status400
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status403
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status411
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status413
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status415
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status429
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status500
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Status503
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
			apis::individual_subscription_document::AMfStatusChangeSubscribeModfyResponse::Statusdefault
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
fn amf_status_change_un_subscribe_validation(
	path_params: models::AMfStatusChangeUnSubscribePathParams
) -> std::result::Result<(models::AMfStatusChangeUnSubscribePathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// AMfStatusChangeUnSubscribe - DELETE
/// /namf-comm/v1/subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn amf_status_change_un_subscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::AMfStatusChangeUnSubscribePathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_subscription_document::IndividualSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || amf_status_change_un_subscribe_validation(path_params))
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
		.amf_status_change_un_subscribe(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_subscription_document::AMfStatusChangeUnSubscribeResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_subscription_document::AMfStatusChangeUnSubscribeResponse::Status307
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
			apis::individual_subscription_document::AMfStatusChangeUnSubscribeResponse::Status308
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
			apis::individual_subscription_document::AMfStatusChangeUnSubscribeResponse::Status400
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
			apis::individual_subscription_document::AMfStatusChangeUnSubscribeResponse::Status404
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
			apis::individual_subscription_document::AMfStatusChangeUnSubscribeResponse::Status429
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
			apis::individual_subscription_document::AMfStatusChangeUnSubscribeResponse::Status500
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
			apis::individual_subscription_document::AMfStatusChangeUnSubscribeResponse::Status503
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
			apis::individual_subscription_document::AMfStatusChangeUnSubscribeResponse::Statusdefault
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
fn delete_subscription_validation(
	path_params: models::DeleteSubscriptionPathParams
) -> std::result::Result<(models::DeleteSubscriptionPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeleteSubscription - DELETE /namf-evts/v1/subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn delete_subscription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteSubscriptionPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_subscription_document::IndividualSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || delete_subscription_validation(path_params))
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
		.delete_subscription(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status307 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status308 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status400(body) => {
				let mut response = response.status(400);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status404(body) => {
				let mut response = response.status(404);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status411(body) => {
				let mut response = response.status(411);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status413(body) => {
				let mut response = response.status(413);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status415(body) => {
				let mut response = response.status(415);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status429(body) => {
				let mut response = response.status(429);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status500(body) => {
				let mut response = response.status(500);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Status503(body) => {
				let mut response = response.status(503);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::DeleteSubscriptionResponse::Statusdefault => {
				let mut response = response.status(0);
				response.body(Body::empty())
			}
		},
		Err(_) => {
			// Application code returned an error. This should not happen, as the
			// implementation should return a valid response.
			response.status(500).body(Body::empty())
		}
	};

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct ModifySubscriptionBodyValidator<'a> {
	body: &'a models::ModifySubscriptionRequest,
}

#[tracing::instrument(skip_all)]
fn modify_subscription_validation(
	path_params: models::ModifySubscriptionPathParams,
	body: models::ModifySubscriptionRequest,
) -> std::result::Result<
	(
		models::ModifySubscriptionPathParams,
		models::ModifySubscriptionRequest,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ModifySubscriptionBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ModifySubscription - PATCH /namf-evts/v1/subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn modify_subscription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ModifySubscriptionPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::ModifySubscriptionRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_subscription_document::IndividualSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || modify_subscription_validation(path_params, body))
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
		.modify_subscription(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_subscription_document::ModifySubscriptionResponse::Status200(body) => {
				let mut response = response.status(200);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status307 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status308 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status400(body) => {
				let mut response = response.status(400);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status403(body) => {
				let mut response = response.status(403);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status404(body) => {
				let mut response = response.status(404);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status411(body) => {
				let mut response = response.status(411);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status413(body) => {
				let mut response = response.status(413);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status415(body) => {
				let mut response = response.status(415);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status429(body) => {
				let mut response = response.status(429);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status500(body) => {
				let mut response = response.status(500);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Status503(body) => {
				let mut response = response.status(503);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_subscription_document::ModifySubscriptionResponse::Statusdefault => {
				let mut response = response.status(0);
				response.body(Body::empty())
			}
		},
		Err(_) => {
			// Application code returned an error. This should not happen, as the
			// implementation should return a valid response.
			response.status(500).body(Body::empty())
		}
	};

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct CancelLocationBodyValidator<'a> {
	body: &'a models::CancelPosInfo,
}

#[tracing::instrument(skip_all)]
fn cancel_location_validation(
	path_params: models::CancelLocationPathParams,
	body: models::CancelPosInfo,
) -> std::result::Result<(models::CancelLocationPathParams, models::CancelPosInfo), ValidationErrors>
{
	path_params.validate()?;
	let b = CancelLocationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// CancelLocation - POST /namf-loc/v1/{ueContextId}/cancel-pos-info
#[tracing::instrument(skip_all)]
async fn cancel_location<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::CancelLocationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::CancelPosInfo>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_location::IndividualUeContextDocumentLocation,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || cancel_location_validation(path_params, body))
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
		.cancel_location(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_location::CancelLocationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_ue_context_document_location::CancelLocationResponse::Status307
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status308
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status400
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status401
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status403
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status404
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status411
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status413
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status415
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status429
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status500
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status503
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Status504
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
			apis::individual_ue_context_document_location::CancelLocationResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct ProvideLocationInfoBodyValidator<'a> {
	body: &'a models::RequestLocInfo,
}

#[tracing::instrument(skip_all)]
fn provide_location_info_validation(
	path_params: models::ProvideLocationInfoPathParams,
	body: models::RequestLocInfo,
) -> std::result::Result<
	(
		models::ProvideLocationInfoPathParams,
		models::RequestLocInfo,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ProvideLocationInfoBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ProvideLocationInfo - POST /namf-loc/v1/{ueContextId}/provide-loc-info
#[tracing::instrument(skip_all)]
async fn provide_location_info<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ProvideLocationInfoPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::RequestLocInfo>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_location::IndividualUeContextDocumentLocation,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || provide_location_info_validation(path_params, body))
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
		.provide_location_info(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status200
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status307
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status308
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status400
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status403
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status404
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status411
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status413
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status415
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status429
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status500
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Status503
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
			apis::individual_ue_context_document_location::ProvideLocationInfoResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct ProvidePositioningInfoBodyValidator<'a> {
	body: &'a models::RequestPosInfo,
}

#[tracing::instrument(skip_all)]
fn provide_positioning_info_validation(
	path_params: models::ProvidePositioningInfoPathParams,
	body: models::RequestPosInfo,
) -> std::result::Result<
	(
		models::ProvidePositioningInfoPathParams,
		models::RequestPosInfo,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ProvidePositioningInfoBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ProvidePositioningInfo - POST /namf-loc/v1/{ueContextId}/provide-pos-info
#[tracing::instrument(skip_all)]
async fn provide_positioning_info<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ProvidePositioningInfoPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::RequestPosInfo>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_location::IndividualUeContextDocumentLocation,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || provide_positioning_info_validation(path_params, body))
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
		.provide_positioning_info(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status200
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status307
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status308
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status400
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status403
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status411
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status413
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status415
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status429
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status500
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status503
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Status504
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
			apis::individual_ue_context_document_location::ProvidePositioningInfoResponse::Statusdefault
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
fn cancel_relocate_ue_context_validation(
	path_params: models::CancelRelocateUeContextPathParams
) -> std::result::Result<(models::CancelRelocateUeContextPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// CancelRelocateUeContext - POST
/// /namf-comm/v1/ue-contexts/{ueContextId}/cancel-relocate
/// TODO: Handle Multipart
#[tracing::instrument(skip_all)]
async fn cancel_relocate_ue_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::CancelRelocateUeContextPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_communication::IndividualUeContextDocumentCommunication,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.cancel_relocate_ue_context(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status307
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status308
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status400
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status403
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status404
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status411
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status413
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status415
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status429
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status500
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Status503
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
			apis::individual_ue_context_document_communication::CancelRelocateUeContextResponse::Statusdefault
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
fn create_ue_context_validation(
	path_params: models::CreateUeContextPathParams
) -> std::result::Result<(models::CreateUeContextPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// CreateUeContext - PUT /namf-comm/v1/ue-contexts/{ueContextId}
/// TODO: Handle Multipart
#[tracing::instrument(skip_all)]
async fn create_ue_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::CreateUeContextPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_communication::IndividualUeContextDocumentCommunication,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.create_ue_context(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status201
			{
				body,
				location,
				param_3gpp_sbi_producer_id
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
				if let Some(param_3gpp_sbi_producer_id) = param_3gpp_sbi_producer_id {
					let param_3gpp_sbi_producer_id = match header::IntoHeaderValue(param_3gpp_sbi_producer_id).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling param_3gpp_sbi_producer_id header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							param_3gpp_sbi_producer_id,
                        );
					}
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status307
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status308
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status400
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status403
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status411
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status413
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status415
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status429
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status500
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Status503
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
			apis::individual_ue_context_document_communication::CreateUeContextResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct EBiAssignmentBodyValidator<'a> {
	body: &'a models::AssignEbiData,
}

#[tracing::instrument(skip_all)]
fn ebi_assignment_validation(
	path_params: models::EBiAssignmentPathParams,
	body: models::AssignEbiData,
) -> std::result::Result<(models::EBiAssignmentPathParams, models::AssignEbiData), ValidationErrors>
{
	path_params.validate()?;
	let b = EBiAssignmentBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// EBiAssignment - POST /namf-comm/v1/ue-contexts/{ueContextId}/assign-ebi
#[tracing::instrument(skip_all)]
async fn ebi_assignment<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::EBiAssignmentPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AssignEbiData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_communication::IndividualUeContextDocumentCommunication,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || ebi_assignment_validation(path_params, body))
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
		.ebi_assignment(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status200
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status307
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status308
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status400
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status403
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status409
			(body)
			=> {
				let mut response = response.status(409);
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status411
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status413
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status415
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status429
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status500
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Status503
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
			apis::individual_ue_context_document_communication::EBiAssignmentResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct RegistrationStatusUpdateBodyValidator<'a> {
	body: &'a models::UeRegStatusUpdateReqData,
}

#[tracing::instrument(skip_all)]
fn registration_status_update_validation(
	path_params: models::RegistrationStatusUpdatePathParams,
	body: models::UeRegStatusUpdateReqData,
) -> std::result::Result<
	(
		models::RegistrationStatusUpdatePathParams,
		models::UeRegStatusUpdateReqData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = RegistrationStatusUpdateBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// RegistrationStatusUpdate - POST
/// /namf-comm/v1/ue-contexts/{ueContextId}/transfer-update
/// TODO: Handle Multipart
#[tracing::instrument(skip_all)]
async fn registration_status_update<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::RegistrationStatusUpdatePathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::UeRegStatusUpdateReqData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_communication::IndividualUeContextDocumentCommunication,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		registration_status_update_validation(path_params, body)
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
		.registration_status_update(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status200
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status307
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status308
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status400
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status403
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status404
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status411
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status413
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status415
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status429
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status500
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Status503
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
			apis::individual_ue_context_document_communication::RegistrationStatusUpdateResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct ReleaseUeContextBodyValidator<'a> {
	body: &'a models::UeContextRelease,
}

#[tracing::instrument(skip_all)]
fn release_ue_context_validation(
	path_params: models::ReleaseUeContextPathParams,
	body: models::UeContextRelease,
) -> std::result::Result<
	(models::ReleaseUeContextPathParams, models::UeContextRelease),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ReleaseUeContextBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ReleaseUeContext - POST /namf-comm/v1/ue-contexts/{ueContextId}/release
#[tracing::instrument(skip_all)]
async fn release_ue_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ReleaseUeContextPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::UeContextRelease>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_communication::IndividualUeContextDocumentCommunication,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || release_ue_context_validation(path_params, body))
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
		.release_ue_context(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status307
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status308
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status400
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status403
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status404
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status411
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status413
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status415
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status429
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status500
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Status503
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
			apis::individual_ue_context_document_communication::ReleaseUeContextResponse::Statusdefault
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
fn relocate_ue_context_validation(
	path_params: models::RelocateUeContextPathParams
) -> std::result::Result<(models::RelocateUeContextPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// RelocateUeContext - POST /namf-comm/v1/ue-contexts/{ueContextId}/relocate
/// TODO: Handle Multipart
#[tracing::instrument(skip_all)]
async fn relocate_ue_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::RelocateUeContextPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_communication::IndividualUeContextDocumentCommunication,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.relocate_ue_context(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status201
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status307
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status308
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status400
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status403
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status411
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status413
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status415
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status429
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status500
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Status503
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
			apis::individual_ue_context_document_communication::RelocateUeContextResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UEContextTransferBodyValidator<'a> {
	body: &'a models::UeContextTransferReqData,
}

#[tracing::instrument(skip_all)]
fn ue_context_transfer_validation(
	path_params: models::UEContextTransferPathParams,
	body: models::UeContextTransferReqData,
) -> std::result::Result<
	(
		models::UEContextTransferPathParams,
		models::UeContextTransferReqData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = UEContextTransferBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// UEContextTransfer - POST /namf-comm/v1/ue-contexts/{ueContextId}/transfer
#[tracing::instrument(skip_all)]
async fn ue_context_transfer<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UEContextTransferPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_context_document_communication::IndividualUeContextDocumentCommunication,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.ue_context_transfer(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status200
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status307
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status308
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status400
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status403
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status404
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status411
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status413
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status415
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status429
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status500
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Status503
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
			apis::individual_ue_context_document_communication::UEContextTransferResponse::Statusdefault
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
fn n1_n2_message_un_subscribe_validation(
	path_params: models::N1N2MessageUnSubscribePathParams
) -> std::result::Result<(models::N1N2MessageUnSubscribePathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// N1N2MessageUnSubscribe - DELETE
/// /namf-comm/v1/ue-contexts/{ueContextId}/n1-n2-messages/subscriptions/
/// {subscriptionId}
#[tracing::instrument(skip_all)]
async fn n1_n2_message_un_subscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::N1N2MessageUnSubscribePathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::n1_n2_individual_subscription_document::N1N2IndividualSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || n1_n2_message_un_subscribe_validation(path_params))
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
		.n1_n2_message_un_subscribe(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status307
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
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status308
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
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status400
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
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status411
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
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status413
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
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status415
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
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status429
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
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status500
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
			apis::n1_n2_individual_subscription_document::N1N2MessageUnSubscribeResponse::Status503
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct N1N2MessageTransferBodyValidator<'a> {
	body: &'a models::N1N2MessageTransferReqData,
}

#[tracing::instrument(skip_all)]
fn n1_n2_message_transfer_validation(
	path_params: models::N1N2MessageTransferPathParams,
	body: models::N1N2MessageTransferReqData,
) -> std::result::Result<
	(
		models::N1N2MessageTransferPathParams,
		models::N1N2MessageTransferReqData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = N1N2MessageTransferBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// N1N2MessageTransfer - POST
/// /namf-comm/v1/ue-contexts/{ueContextId}/n1-n2-messages
/// TODO: Handle Multipart
#[tracing::instrument(skip_all)]
async fn n1_n2_message_transfer<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::N1N2MessageTransferPathParams>,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::n1_n2_message_collection_collection::N1N2MessageCollectionCollection,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.n1_n2_message_transfer(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status200
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status202
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
				let mut response = response.status(202);
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status307
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status308
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status400
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status403
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status404
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status409
			(body)
			=> {
				let mut response = response.status(409);
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status411
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status413
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status415
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status429
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status500
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status503
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Status504
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
			apis::n1_n2_message_collection_collection::N1N2MessageTransferResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct N1N2MessageSubscribeBodyValidator<'a> {
	body: &'a models::UeN1N2InfoSubscriptionCreateData,
}

#[tracing::instrument(skip_all)]
fn n1_n2_message_subscribe_validation(
	path_params: models::N1N2MessageSubscribePathParams,
	body: models::UeN1N2InfoSubscriptionCreateData,
) -> std::result::Result<
	(
		models::N1N2MessageSubscribePathParams,
		models::UeN1N2InfoSubscriptionCreateData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = N1N2MessageSubscribeBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// N1N2MessageSubscribe - POST
/// /namf-comm/v1/ue-contexts/{ueContextId}/n1-n2-messages/subscriptions
#[tracing::instrument(skip_all)]
async fn n1_n2_message_subscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::N1N2MessageSubscribePathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::UeN1N2InfoSubscriptionCreateData>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2SubscriptionsCollectionForIndividualUeContextsCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || n1_n2_message_subscribe_validation(path_params, body))
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
		.n1_n2_message_subscribe(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status201
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status307
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status308
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status400
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status411
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status413
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status415
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status429
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status500
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Status503
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
			apis::n1_n2_subscriptions_collection_for_individual_ue_contexts_collection::N1N2MessageSubscribeResponse::Statusdefault
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
fn non_ue_n2_info_un_subscribe_validation(
	path_params: models::NonUeN2InfoUnSubscribePathParams
) -> std::result::Result<(models::NonUeN2InfoUnSubscribePathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// NonUeN2InfoUnSubscribe - DELETE
/// /namf-comm/v1/non-ue-n2-messages/subscriptions/{n2NotifySubscriptionId}
#[tracing::instrument(skip_all)]
async fn non_ue_n2_info_un_subscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::NonUeN2InfoUnSubscribePathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::non_uen2_message_notification_individual_subscription_document::NonUen2MessageNotificationIndividualSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || non_ue_n2_info_un_subscribe_validation(path_params))
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
		.non_ue_n2_info_un_subscribe(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::non_uen2_message_notification_individual_subscription_document::NonUeN2InfoUnSubscribeResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::non_uen2_message_notification_individual_subscription_document::NonUeN2InfoUnSubscribeResponse::Status307
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
			apis::non_uen2_message_notification_individual_subscription_document::NonUeN2InfoUnSubscribeResponse::Status308
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
			apis::non_uen2_message_notification_individual_subscription_document::NonUeN2InfoUnSubscribeResponse::Status400
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
			apis::non_uen2_message_notification_individual_subscription_document::NonUeN2InfoUnSubscribeResponse::Status404
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
			apis::non_uen2_message_notification_individual_subscription_document::NonUeN2InfoUnSubscribeResponse::Status429
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
			apis::non_uen2_message_notification_individual_subscription_document::NonUeN2InfoUnSubscribeResponse::Status500
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
			apis::non_uen2_message_notification_individual_subscription_document::NonUeN2InfoUnSubscribeResponse::Status503
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct NonUeN2MessageTransferBodyValidator<'a> {
	body: &'a models::N2InformationTransferReqData,
}

#[tracing::instrument(skip_all)]
fn non_ue_n2_message_transfer_validation(
	body: models::N2InformationTransferReqData
) -> std::result::Result<(models::N2InformationTransferReqData,), ValidationErrors> {
	let b = NonUeN2MessageTransferBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// NonUeN2MessageTransfer - POST /namf-comm/v1/non-ue-n2-messages/transfer
/// TODO: Handle Multipart
#[tracing::instrument(skip_all)]
async fn non_ue_n2_message_transfer<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	body: axum::body::Body,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::non_uen2_messages_collection_collection::NonUen2MessagesCollectionCollection,
{
	// Manually validate the request in the implementation

	let result = api_impl
		.as_ref()
		.non_ue_n2_message_transfer(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status200
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status307
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status308
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status400
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status403
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status404
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status411
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status413
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status415
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status429
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status500
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Status503
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
			apis::non_uen2_messages_collection_collection::NonUeN2MessageTransferResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct NonUeN2InfoSubscribeBodyValidator<'a> {
	body: &'a models::NonUeN2InfoSubscriptionCreateData,
}

#[tracing::instrument(skip_all)]
fn non_ue_n2_info_subscribe_validation(
	body: models::NonUeN2InfoSubscriptionCreateData
) -> std::result::Result<(models::NonUeN2InfoSubscriptionCreateData,), ValidationErrors> {
	let b = NonUeN2InfoSubscribeBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// NonUeN2InfoSubscribe - POST /namf-comm/v1/non-ue-n2-messages/subscriptions
#[tracing::instrument(skip_all)]
async fn non_ue_n2_info_subscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::NonUeN2InfoSubscriptionCreateData>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::non_uen2_messages_subscriptions_collection_collection::NonUen2MessagesSubscriptionsCollectionCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || non_ue_n2_info_subscribe_validation(body))
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
		.non_ue_n2_info_subscribe(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status201
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status307
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status308
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status400
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status403
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status411
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status413
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status415
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status429
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status500
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Status503
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
			apis::non_uen2_messages_subscriptions_collection_collection::NonUeN2InfoSubscribeResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct AMfStatusChangeSubscribeBodyValidator<'a> {
	body: &'a models::SubscriptionData,
}

#[tracing::instrument(skip_all)]
fn amf_status_change_subscribe_validation(
	body: models::SubscriptionData
) -> std::result::Result<(models::SubscriptionData,), ValidationErrors> {
	let b = AMfStatusChangeSubscribeBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// AMfStatusChangeSubscribe - POST /namf-comm/v1/subscriptions
#[tracing::instrument(skip_all)]
async fn amf_status_change_subscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::SubscriptionData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscriptions_collection_collection::SubscriptionsCollectionCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || amf_status_change_subscribe_validation(body))
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
		.amf_status_change_subscribe(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status201
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status307
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status308
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status400
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status403
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status411
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status413
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status415
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status429
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status500
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Status503
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
			apis::subscriptions_collection_collection::AMfStatusChangeSubscribeResponse::Statusdefault
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct CreateSubscriptionBodyValidator<'a> {
	body: &'a models::AmfCreateEventSubscription,
}

#[tracing::instrument(skip_all)]
fn create_subscription_validation(
	body: models::AmfCreateEventSubscription
) -> std::result::Result<(models::AmfCreateEventSubscription,), ValidationErrors> {
	let b = CreateSubscriptionBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// CreateSubscription - POST /namf-evts/v1/subscriptions
#[tracing::instrument(skip_all)]
async fn create_subscription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::AmfCreateEventSubscription>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscriptions_collection_collection::SubscriptionsCollectionCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || create_subscription_validation(body))
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
		.create_subscription(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status201
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status307
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status308
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status400
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status403
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status411
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status413
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status415
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status429
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status500
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Status503
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
			apis::subscriptions_collection_collection::CreateSubscriptionResponse::Statusdefault
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
fn provide_domain_selection_info_validation(
	path_params: models::ProvideDomainSelectionInfoPathParams,
	query_params: models::ProvideDomainSelectionInfoQueryParams,
) -> std::result::Result<
	(
		models::ProvideDomainSelectionInfoPathParams,
		models::ProvideDomainSelectionInfoQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// ProvideDomainSelectionInfo - GET /namf-mt/v1/ue-contexts/{ueContextId}
#[tracing::instrument(skip_all)]
async fn provide_domain_selection_info<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ProvideDomainSelectionInfoPathParams>,
	Query(query_params): Query<models::ProvideDomainSelectionInfoQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ue_context_document::UeContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		provide_domain_selection_info_validation(path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.provide_domain_selection_info(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status200(body) => {
				let mut response = response.status(200);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status307 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status308 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status400(body) => {
				let mut response = response.status(400);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status403(body) => {
				let mut response = response.status(403);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status404(body) => {
				let mut response = response.status(404);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status409(body) => {
				let mut response = response.status(409);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status414(body) => {
				let mut response = response.status(414);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status429(body) => {
				let mut response = response.status(429);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status500(body) => {
				let mut response = response.status(500);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Status503(body) => {
				let mut response = response.status(503);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_context_document::ProvideDomainSelectionInfoResponse::Statusdefault => {
				let mut response = response.status(0);
				response.body(Body::empty())
			}
		},
		Err(_) => {
			// Application code returned an error. This should not happen, as the
			// implementation should return a valid response.
			response.status(500).body(Body::empty())
		}
	};

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct EnableGroupReachabilityBodyValidator<'a> {
	body: &'a models::EnableGroupReachabilityReqData,
}

#[tracing::instrument(skip_all)]
fn enable_group_reachability_validation(
	body: models::EnableGroupReachabilityReqData
) -> std::result::Result<(models::EnableGroupReachabilityReqData,), ValidationErrors> {
	let b = EnableGroupReachabilityBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// EnableGroupReachability - POST
/// /namf-mt/v1/ue-contexts/enable-group-reachability
#[tracing::instrument(skip_all)]
async fn enable_group_reachability<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::EnableGroupReachabilityReqData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ue_contexts_collection::UeContextsCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || enable_group_reachability_validation(body))
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
		.enable_group_reachability(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status200(body) => {
				let mut response = response.status(200);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status307 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status308 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status400(body) => {
				let mut response = response.status(400);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status403(body) => {
				let mut response = response.status(403);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status404(body) => {
				let mut response = response.status(404);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status411(body) => {
				let mut response = response.status(411);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status413(body) => {
				let mut response = response.status(413);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status415(body) => {
				let mut response = response.status(415);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status429(body) => {
				let mut response = response.status(429);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status500(body) => {
				let mut response = response.status(500);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status503(body) => {
				let mut response = response.status(503);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Status504(body) => {
				let mut response = response.status(504);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_contexts_collection::EnableGroupReachabilityResponse::Statusdefault => {
				let mut response = response.status(0);
				response.body(Body::empty())
			}
		},
		Err(_) => {
			// Application code returned an error. This should not happen, as the
			// implementation should return a valid response.
			response.status(500).body(Body::empty())
		}
	};

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct EnableUeReachabilityBodyValidator<'a> {
	body: &'a models::EnableUeReachabilityReqData,
}

#[tracing::instrument(skip_all)]
fn enable_ue_reachability_validation(
	path_params: models::EnableUeReachabilityPathParams,
	body: models::EnableUeReachabilityReqData,
) -> std::result::Result<
	(
		models::EnableUeReachabilityPathParams,
		models::EnableUeReachabilityReqData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = EnableUeReachabilityBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// EnableUeReachability - PUT /namf-mt/v1/ue-contexts/{ueContextId}/ue-reachind
#[tracing::instrument(skip_all)]
async fn enable_ue_reachability<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::EnableUeReachabilityPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::EnableUeReachabilityReqData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ue_reach_ind_document::UeReachIndDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || enable_ue_reachability_validation(path_params, body))
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
		.enable_ue_reachability(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status200(body) => {
				let mut response = response.status(200);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status307 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status308 {
				body,
				location,
				param_3gpp_sbi_target_nf_id,
			} => {
				let location = match header::IntoHeaderValue(location).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling location header - {}",
								e
							)))
							.map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
					}
				};

				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(HeaderName::from_static(""), location);
				}
				if let Some(param_3gpp_sbi_target_nf_id) = param_3gpp_sbi_target_nf_id {
					let param_3gpp_sbi_target_nf_id =
						match header::IntoHeaderValue(param_3gpp_sbi_target_nf_id).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 param_3gpp_sbi_target_nf_id header - {}",
										e
									)))
									.map_err(|e| {
										error!(error = ?e);
										StatusCode::INTERNAL_SERVER_ERROR
									});
							}
						};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers
							.insert(HeaderName::from_static(""), param_3gpp_sbi_target_nf_id);
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
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status400(body) => {
				let mut response = response.status(400);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status403(body) => {
				let mut response = response.status(403);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status404(body) => {
				let mut response = response.status(404);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status409(body) => {
				let mut response = response.status(409);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status411(body) => {
				let mut response = response.status(411);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status413(body) => {
				let mut response = response.status(413);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status415(body) => {
				let mut response = response.status(415);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status429(body) => {
				let mut response = response.status(429);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status500(body) => {
				let mut response = response.status(500);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status503(body) => {
				let mut response = response.status(503);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Status504(body) => {
				let mut response = response.status(504);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::ue_reach_ind_document::EnableUeReachabilityResponse::Statusdefault => {
				let mut response = response.status(0);
				response.body(Body::empty())
			}
		},
		Err(_) => {
			// Application code returned an error. This should not happen, as the
			// implementation should return a valid response.
			response.status(500).body(Body::empty())
		}
	};

	resp.map_err(|e| {
		error!(error = ?e);
		StatusCode::INTERNAL_SERVER_ERROR
	})
}
