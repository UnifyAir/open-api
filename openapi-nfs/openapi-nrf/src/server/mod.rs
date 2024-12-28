use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header::CONTENT_TYPE};
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
								  A: apis::access_token_request::AccessTokenRequest + apis::bootstrapping_request::BootstrappingRequest + apis::complete_stored_search_document::CompleteStoredSearchDocument + apis::individual_scp_domain_routing_information_subscription_document::IndividualScpDomainRoutingInformationSubscriptionDocument + apis::nf_instance_id_document::NfInstanceIdDocument + apis::nf_instances_store::NfInstancesStore + apis::scp_domain_routing_information_document::ScpDomainRoutingInformationDocument + apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInformationSubscriptionsCollection + apis::stored_search_document::StoredSearchDocument + apis::subscription_id_document::SubscriptionIdDocument + apis::subscriptions_collection::SubscriptionsCollection + 'static,
{
	// build our application with a route
	Router::new()
		.route("/bootstrapping", get(bootstrapping_info_request::<I, A>))
		.route("/oauth2/token", post(access_token_request::<I, A>))
		.route(
			"/nnrf-disc/v1/nf-instances",
			get(search_nf_instances::<I, A>),
		)
		.route(
			"/nnrf-disc/v1/scp-domain-routing-info",
			get(scp_domain_routing_info_get::<I, A>),
		)
		.route(
			"/nnrf-disc/v1/scp-domain-routing-info-subs",
			post(scp_domain_routing_info_subscribe::<I, A>),
		)
		.route(
			"/nnrf-disc/v1/scp-domain-routing-info-subs/:subscription_id",
			delete(scp_domain_routing_info_unsubscribe::<I, A>),
		)
		.route(
			"/nnrf-disc/v1/searches/:search_id",
			get(retrieve_stored_search::<I, A>),
		)
		.route(
			"/nnrf-disc/v1/searches/:search_id/complete",
			get(retrieve_complete_search::<I, A>),
		)
		.route(
			"/nnrf-nfm/v1/nf-instances",
			get(get_nf_instances::<I, A>).options(options_nf_instances::<I, A>),
		)
		.route(
			"/nnrf-nfm/v1/nf-instances/:nf_instance_id",
			delete(deregister_nf_instance::<I, A>)
				.get(get_nf_instance::<I, A>)
				.patch(update_nf_instance::<I, A>)
				.put(register_nf_instance::<I, A>),
		)
		.route(
			"/nnrf-nfm/v1/subscriptions",
			post(create_subscription::<I, A>),
		)
		.route(
			"/nnrf-nfm/v1/subscriptions/:subscription_id",
			delete(remove_subscription::<I, A>).patch(update_subscription::<I, A>),
		)
		.with_state(api_impl)
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct AccessTokenRequestBodyValidator<'a> {
	body: &'a models::AccessTokenReq,
}

#[tracing::instrument(skip_all)]
fn access_token_request_validation(
	header_params: models::AccessTokenRequestHeaderParams,
	body: models::AccessTokenReq,
) -> std::result::Result<
	(
		models::AccessTokenRequestHeaderParams,
		models::AccessTokenReq,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	let b = AccessTokenRequestBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, body))
}

/// AccessTokenRequest - POST //oauth2/token
#[tracing::instrument(skip_all)]
async fn access_token_request<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	State(api_impl): State<I>,
	Form(body): Form<models::AccessTokenReq>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::access_token_request::AccessTokenRequest,
{
	// Header parameters
	let header_params = {
		let header_content_encoding = headers.get(HeaderName::from_static("content_encoding"));

		let header_content_encoding = match header_content_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Content-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::AccessTokenRequestHeaderParams {
			content_encoding: header_content_encoding,
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || access_token_request_validation(header_params, body))
			.await
			.unwrap();

	let Ok((header_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.access_token_request(method, host, cookies, header_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::access_token_request::AccessTokenRequestResponse::Status200 {
				body,
				cache_control,
				pragma,
				accept_encoding,
				content_encoding,
			} => {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling cache_control header \
								 - {}",
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
						response_headers.insert(HeaderName::from_static(""), cache_control);
					}
				}

				if let Some(pragma) = pragma {
					let pragma = match header::IntoHeaderValue(pragma).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling pragma header - {}",
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
						response_headers.insert(HeaderName::from_static(""), pragma);
					}
				};

				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding =
						match header::IntoHeaderValue(content_encoding).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 content_encoding header - {}",
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
						response_headers.insert(HeaderName::from_static(""), content_encoding);
					}
				}
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
			apis::access_token_request::AccessTokenRequestResponse::Status307 {
				body,
				location,
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
			apis::access_token_request::AccessTokenRequestResponse::Status308 {
				body,
				location,
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
			apis::access_token_request::AccessTokenRequestResponse::Status400 {
				body,
				cache_control,
				pragma,
			} => {
				let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling cache_control header \
								 - {}",
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
					response_headers.insert(HeaderName::from_static(""), cache_control);
				}
				let pragma = match header::IntoHeaderValue(pragma).try_into() {
					Ok(val) => val,
					Err(e) => {
						return Response::builder()
							.status(StatusCode::INTERNAL_SERVER_ERROR)
							.body(Body::from(format!(
								"An internal server error occurred handling pragma header - {}",
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
					response_headers.insert(HeaderName::from_static(""), pragma);
				}
				let mut response = response.status(400);
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
			apis::access_token_request::AccessTokenRequestResponse::Status401(body) => {
				let mut response = response.status(401);
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
			apis::access_token_request::AccessTokenRequestResponse::Status403(body) => {
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
			apis::access_token_request::AccessTokenRequestResponse::Status404(body) => {
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
			apis::access_token_request::AccessTokenRequestResponse::Status411(body) => {
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
			apis::access_token_request::AccessTokenRequestResponse::Status413(body) => {
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
			apis::access_token_request::AccessTokenRequestResponse::Status415(body) => {
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
			apis::access_token_request::AccessTokenRequestResponse::Status429(body) => {
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
			apis::access_token_request::AccessTokenRequestResponse::Status500(body) => {
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
			apis::access_token_request::AccessTokenRequestResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::access_token_request::AccessTokenRequestResponse::Status503(body) => {
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
			apis::access_token_request::AccessTokenRequestResponse::Statusdefault => {
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

#[tracing::instrument(skip_all)]
fn bootstrapping_info_request_validation(
	header_params: models::BootstrappingInfoRequestHeaderParams
) -> std::result::Result<(models::BootstrappingInfoRequestHeaderParams,), ValidationErrors> {
	header_params.validate()?;

	Ok((header_params,))
}

/// BootstrappingInfoRequest - GET //bootstrapping
#[tracing::instrument(skip_all)]
async fn bootstrapping_info_request<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::bootstrapping_request::BootstrappingRequest,
{
	// Header parameters
	let header_params = {
		let header_if_none_match = headers.get(HeaderName::from_static("if_none_match"));

		let header_if_none_match = match header_if_none_match {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-None-Match - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::BootstrappingInfoRequestHeaderParams {
			if_none_match: header_if_none_match,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || bootstrapping_info_request_validation(header_params))
			.await
			.unwrap();

	let Ok((header_params,)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.bootstrapping_info_request(method, host, cookies, header_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::bootstrapping_request::BootstrappingInfoRequestResponse::Status200 {
				body,
				cache_control,
				etag,
			} => {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling cache_control \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), cache_control);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling etag header - {}",
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
						response_headers.insert(HeaderName::from_static(""), etag);
					}
				}
				let mut response = response.status(200);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/3gppHal+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = body;
				response.body(Body::from(body_content))
			}
			apis::bootstrapping_request::BootstrappingInfoRequestResponse::Status307 {
				body,
				location,
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
			apis::bootstrapping_request::BootstrappingInfoRequestResponse::Status308 {
				body,
				location,
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
			apis::bootstrapping_request::BootstrappingInfoRequestResponse::Status400(body) => {
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
			apis::bootstrapping_request::BootstrappingInfoRequestResponse::Status500(body) => {
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
			apis::bootstrapping_request::BootstrappingInfoRequestResponse::Statusdefault => {
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

#[tracing::instrument(skip_all)]
fn retrieve_complete_search_validation(
	header_params: models::RetrieveCompleteSearchHeaderParams,
	path_params: models::RetrieveCompleteSearchPathParams,
) -> std::result::Result<
	(
		models::RetrieveCompleteSearchHeaderParams,
		models::RetrieveCompleteSearchPathParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;

	Ok((header_params, path_params))
}

/// RetrieveCompleteSearch - GET /nnrf-disc/v1/searches/{searchId}/complete
#[tracing::instrument(skip_all)]
async fn retrieve_complete_search<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::RetrieveCompleteSearchPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::complete_stored_search_document::CompleteStoredSearchDocument,
{
	// Header parameters
	let header_params = {
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::RetrieveCompleteSearchHeaderParams {
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		retrieve_complete_search_validation(header_params, path_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.retrieve_complete_search(method, host, cookies, header_params, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::complete_stored_search_document::RetrieveCompleteSearchResponse::Status200 {
				body,
				cache_control,
				etag,
				content_encoding,
			} => {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling cache_control \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), cache_control);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling etag header - {}",
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
						response_headers.insert(HeaderName::from_static(""), etag);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding =
						match header::IntoHeaderValue(content_encoding).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 content_encoding header - {}",
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
						response_headers.insert(HeaderName::from_static(""), content_encoding);
					}
				}
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
			apis::complete_stored_search_document::RetrieveCompleteSearchResponse::Status307 {
				body,
				location,
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
			apis::complete_stored_search_document::RetrieveCompleteSearchResponse::Status308 {
				body,
				location,
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

#[tracing::instrument(skip_all)]
fn scp_domain_routing_info_unsubscribe_validation(
	path_params: models::ScpDomainRoutingInfoUnsubscribePathParams
) -> std::result::Result<(models::ScpDomainRoutingInfoUnsubscribePathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// ScpDomainRoutingInfoUnsubscribe - DELETE
/// /nnrf-disc/v1/scp-domain-routing-info-subs/{subscriptionID}
#[tracing::instrument(skip_all)]
async fn scp_domain_routing_info_unsubscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ScpDomainRoutingInfoUnsubscribePathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_scp_domain_routing_information_subscription_document::IndividualScpDomainRoutingInformationSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		scp_domain_routing_info_unsubscribe_validation(path_params)
	})
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
		.scp_domain_routing_info_unsubscribe(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status400
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status401
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status403
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status404
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status411
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status413
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status415
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status429
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status500
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status501
			(body)
			=> {
				let mut response = response.status(501);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Status503
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
			apis::individual_scp_domain_routing_information_subscription_document::ScpDomainRoutingInfoUnsubscribeResponse::Statusdefault
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
fn deregister_nf_instance_validation(
	path_params: models::DeregisterNfInstancePathParams
) -> std::result::Result<(models::DeregisterNfInstancePathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeregisterNfInstance - DELETE /nnrf-nfm/v1/nf-instances/{nfInstanceID}
#[tracing::instrument(skip_all)]
async fn deregister_nf_instance<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeregisterNfInstancePathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nf_instance_id_document::NfInstanceIdDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || deregister_nf_instance_validation(path_params))
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
		.deregister_nf_instance(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status307 {
				body,
				location,
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status308 {
				body,
				location,
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status400(body) => {
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status401(body) => {
				let mut response = response.status(401);
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status403(body) => {
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status404(body) => {
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status411(body) => {
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status429(body) => {
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status500(body) => {
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Status503(body) => {
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
			apis::nf_instance_id_document::DeregisterNfInstanceResponse::Statusdefault => {
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

#[tracing::instrument(skip_all)]
fn get_nf_instance_validation(
	path_params: models::GetNfInstancePathParams,
	query_params: models::GetNfInstanceQueryParams,
) -> std::result::Result<
	(
		models::GetNfInstancePathParams,
		models::GetNfInstanceQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetNfInstance - GET /nnrf-nfm/v1/nf-instances/{nfInstanceID}
#[tracing::instrument(skip_all)]
async fn get_nf_instance<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetNfInstancePathParams>,
	Query(query_params): Query<models::GetNfInstanceQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nf_instance_id_document::NfInstanceIdDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || get_nf_instance_validation(path_params, query_params))
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
		.get_nf_instance(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instance_id_document::GetNfInstanceResponse::Status200 { body, etag } => {
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling etag header - {}",
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
						response_headers.insert(HeaderName::from_static(""), etag);
					}
				}
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status307 { body, location } => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status308 { body, location } => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status400(body) => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status401(body) => {
				let mut response = response.status(401);
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status403(body) => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status404(body) => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::nf_instance_id_document::GetNfInstanceResponse::Status411(body) => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status413(body) => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status415(body) => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status429(body) => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status500(body) => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Status503(body) => {
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
			apis::nf_instance_id_document::GetNfInstanceResponse::Statusdefault => {
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
struct RegisterNfInstanceBodyValidator<'a> {
	body: &'a models::NfProfile1,
}

#[tracing::instrument(skip_all)]
fn register_nf_instance_validation(
	header_params: models::RegisterNfInstanceHeaderParams,
	path_params: models::RegisterNfInstancePathParams,
	body: models::NfProfile1,
) -> std::result::Result<
	(
		models::RegisterNfInstanceHeaderParams,
		models::RegisterNfInstancePathParams,
		models::NfProfile1,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	let b = RegisterNfInstanceBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, path_params, body))
}

/// RegisterNfInstance - PUT /nnrf-nfm/v1/nf-instances/{nfInstanceID}
#[tracing::instrument(skip_all)]
async fn register_nf_instance<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::RegisterNfInstancePathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::NfProfile1>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nf_instance_id_document::NfInstanceIdDocument,
{
	// Header parameters
	let header_params = {
		let header_content_encoding = headers.get(HeaderName::from_static("content_encoding"));

		let header_content_encoding = match header_content_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Content-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::RegisterNfInstanceHeaderParams {
			content_encoding: header_content_encoding,
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		register_nf_instance_validation(header_params, path_params, body)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.register_nf_instance(method, host, cookies, header_params, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status200 {
				body,
				accept_encoding,
				content_encoding,
				etag,
			} => {
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding =
						match header::IntoHeaderValue(content_encoding).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 content_encoding header - {}",
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
						response_headers.insert(HeaderName::from_static(""), content_encoding);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling etag header - {}",
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
						response_headers.insert(HeaderName::from_static(""), etag);
					}
				}
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status201 {
				body,
				location,
				accept_encoding,
				content_encoding,
				etag,
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
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding =
						match header::IntoHeaderValue(content_encoding).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 content_encoding header - {}",
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
						response_headers.insert(HeaderName::from_static(""), content_encoding);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling etag header - {}",
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
						response_headers.insert(HeaderName::from_static(""), etag);
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status307 {
				body,
				location,
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status308 {
				body,
				location,
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status400(body) => {
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status401(body) => {
				let mut response = response.status(401);
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status403(body) => {
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status404(body) => {
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status411(body) => {
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status413(body) => {
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status415(body) => {
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status429(body) => {
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status500(body) => {
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Status503(body) => {
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
			apis::nf_instance_id_document::RegisterNfInstanceResponse::Statusdefault => {
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
struct UpdateNfInstanceBodyValidator<'a> {
	#[validate(length(min = 1))]
	body: &'a Vec<models::PatchItem>,
}

#[tracing::instrument(skip_all)]
fn update_nf_instance_validation(
	header_params: models::UpdateNfInstanceHeaderParams,
	path_params: models::UpdateNfInstancePathParams,
	body: Vec<models::PatchItem>,
) -> std::result::Result<
	(
		models::UpdateNfInstanceHeaderParams,
		models::UpdateNfInstancePathParams,
		Vec<models::PatchItem>,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	let b = UpdateNfInstanceBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, path_params, body))
}

/// UpdateNfInstance - PATCH /nnrf-nfm/v1/nf-instances/{nfInstanceID}
#[tracing::instrument(skip_all)]
async fn update_nf_instance<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::UpdateNfInstancePathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Vec<models::PatchItem>>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nf_instance_id_document::NfInstanceIdDocument,
{
	// Header parameters
	let header_params = {
		let header_content_encoding = headers.get(HeaderName::from_static("content_encoding"));

		let header_content_encoding = match header_content_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Content-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};
		let header_if_match = headers.get(HeaderName::from_static("if_match"));

		let header_if_match = match header_if_match {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!("Invalid header If-Match - {}", err)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::UpdateNfInstanceHeaderParams {
			content_encoding: header_content_encoding,
			accept_encoding: header_accept_encoding,
			if_match: header_if_match,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		update_nf_instance_validation(header_params, path_params, body)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.update_nf_instance(method, host, cookies, header_params, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status200 {
				body,
				accept_encoding,
				etag,
				content_encoding,
			} => {
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling etag header - {}",
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
						response_headers.insert(HeaderName::from_static(""), etag);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding =
						match header::IntoHeaderValue(content_encoding).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 content_encoding header - {}",
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
						response_headers.insert(HeaderName::from_static(""), content_encoding);
					}
				}
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status204 {
				accept_encoding,
			} => {
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status307 {
				body,
				location,
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status308 {
				body,
				location,
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status400(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status403(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status404(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status409(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status411(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status412(body) => {
				let mut response = response.status(412);
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status413(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status415(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status429(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status500(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Status503(body) => {
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
			apis::nf_instance_id_document::UpdateNfInstanceResponse::Statusdefault => {
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

#[tracing::instrument(skip_all)]
fn get_nf_instances_validation(
	query_params: models::GetNfInstancesQueryParams
) -> std::result::Result<(models::GetNfInstancesQueryParams,), ValidationErrors> {
	query_params.validate()?;

	Ok((query_params,))
}

/// GetNfInstances - GET /nnrf-nfm/v1/nf-instances
#[tracing::instrument(skip_all)]
async fn get_nf_instances<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Query(query_params): Query<models::GetNfInstancesQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nf_instances_store::NfInstancesStore,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || get_nf_instances_validation(query_params))
		.await
		.unwrap();

	let Ok((query_params,)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_nf_instances(method, host, cookies, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instances_store::GetNfInstancesResponse::Status200 { body, etag } => {
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling etag header - {}",
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
						response_headers.insert(HeaderName::from_static(""), etag);
					}
				}
				let mut response = response.status(200);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/3gppHal+json").map_err(|e| {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status307 { body, location } => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status308 { body, location } => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status400(body) => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status401(body) => {
				let mut response = response.status(401);
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
			apis::nf_instances_store::GetNfInstancesResponse::Status403(body) => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status404(body) => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::nf_instances_store::GetNfInstancesResponse::Status411(body) => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status413(body) => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status415(body) => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status429(body) => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status500(body) => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::nf_instances_store::GetNfInstancesResponse::Status503(body) => {
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
			apis::nf_instances_store::GetNfInstancesResponse::Statusdefault => {
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

#[tracing::instrument(skip_all)]
fn options_nf_instances_validation() -> std::result::Result<(), ValidationErrors> {
	Ok(())
}

/// OptionsNfInstances - OPTIONS /nnrf-nfm/v1/nf-instances
#[tracing::instrument(skip_all)]
async fn options_nf_instances<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nf_instances_store::NfInstancesStore,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || options_nf_instances_validation())
		.await
		.unwrap();

	let Ok(()) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.options_nf_instances(method, host, cookies)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instances_store::OptionsNfInstancesResponse::Status200 {
				body,
				accept_encoding,
			} => {
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status204 { accept_encoding } => {
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::nf_instances_store::OptionsNfInstancesResponse::Status307 { body, location } => {
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status308 { body, location } => {
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status400(body) => {
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status401(body) => {
				let mut response = response.status(401);
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status403(body) => {
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status404(body) => {
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status405 => {
				let mut response = response.status(405);
				response.body(Body::empty())
			}
			apis::nf_instances_store::OptionsNfInstancesResponse::Status429(body) => {
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status500(body) => {
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Status503(body) => {
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
			apis::nf_instances_store::OptionsNfInstancesResponse::Statusdefault => {
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

#[tracing::instrument(skip_all)]
fn search_nf_instances_validation(
	header_params: models::SearchNfInstancesHeaderParams,
	query_params: models::SearchNfInstancesQueryParams,
) -> std::result::Result<
	(
		models::SearchNfInstancesHeaderParams,
		models::SearchNfInstancesQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	query_params.validate()?;

	Ok((header_params, query_params))
}

/// SearchNfInstances - GET /nnrf-disc/v1/nf-instances
#[tracing::instrument(skip_all)]
async fn search_nf_instances<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Query(query_params): Query<models::SearchNfInstancesQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nf_instances_store::NfInstancesStore,
{
	// Header parameters
	let header_params = {
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};
		let header_if_none_match = headers.get(HeaderName::from_static("if_none_match"));

		let header_if_none_match = match header_if_none_match {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-None-Match - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::SearchNfInstancesHeaderParams {
			accept_encoding: header_accept_encoding,
			if_none_match: header_if_none_match,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		search_nf_instances_validation(header_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.search_nf_instances(method, host, cookies, header_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instances_store::SearchNfInstancesResponse::Status200 {
				body,
				cache_control,
				etag,
				content_encoding,
			} => {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling cache_control \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), cache_control);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling etag header - {}",
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
						response_headers.insert(HeaderName::from_static(""), etag);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding =
						match header::IntoHeaderValue(content_encoding).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 content_encoding header - {}",
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
						response_headers.insert(HeaderName::from_static(""), content_encoding);
					}
				}
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status307 { body, location } => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status308 { body, location } => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status400(body) => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status401(body) => {
				let mut response = response.status(401);
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status403(body) => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status404(body) => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::nf_instances_store::SearchNfInstancesResponse::Status411(body) => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status413(body) => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status415(body) => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status429(body) => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status500(body) => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::nf_instances_store::SearchNfInstancesResponse::Status503(body) => {
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
			apis::nf_instances_store::SearchNfInstancesResponse::Statusdefault => {
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

#[tracing::instrument(skip_all)]
fn scp_domain_routing_info_get_validation(
	header_params: models::SCpDomainRoutingInfoGetHeaderParams,
	query_params: models::SCpDomainRoutingInfoGetQueryParams,
) -> std::result::Result<
	(
		models::SCpDomainRoutingInfoGetHeaderParams,
		models::SCpDomainRoutingInfoGetQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	query_params.validate()?;

	Ok((header_params, query_params))
}

/// SCpDomainRoutingInfoGet - GET /nnrf-disc/v1/scp-domain-routing-info
#[tracing::instrument(skip_all)]
async fn scp_domain_routing_info_get<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Query(query_params): Query<models::SCpDomainRoutingInfoGetQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::scp_domain_routing_information_document::ScpDomainRoutingInformationDocument,
{
	// Header parameters
	let header_params = {
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::SCpDomainRoutingInfoGetHeaderParams {
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		scp_domain_routing_info_get_validation(header_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.scp_domain_routing_info_get(method, host, cookies, header_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status200
			{
				body,
				content_encoding
			}
			=> {
				if let Some(content_encoding) = content_encoding {
					let content_encoding = match header::IntoHeaderValue(content_encoding).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling content_encoding header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							content_encoding,
						);
					}
				}
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status307
			{
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
				let mut response = response.status(307);
				response.body(Body::empty())
			}
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status400
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status401
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status403
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status404
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status406
			=> {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status411
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status413
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status415
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status429
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status500
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status501
			(body)
			=> {
				let mut response = response.status(501);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Status503
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
			apis::scp_domain_routing_information_document::SCpDomainRoutingInfoGetResponse::Statusdefault
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
struct ScpDomainRoutingInfoSubscribeBodyValidator<'a> {
	body: &'a models::ScpDomainRoutingInfoSubscription,
}

#[tracing::instrument(skip_all)]
fn scp_domain_routing_info_subscribe_validation(
	header_params: models::ScpDomainRoutingInfoSubscribeHeaderParams,
	body: models::ScpDomainRoutingInfoSubscription,
) -> std::result::Result<
	(
		models::ScpDomainRoutingInfoSubscribeHeaderParams,
		models::ScpDomainRoutingInfoSubscription,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	let b = ScpDomainRoutingInfoSubscribeBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, body))
}

/// ScpDomainRoutingInfoSubscribe - POST
/// /nnrf-disc/v1/scp-domain-routing-info-subs
#[tracing::instrument(skip_all)]
async fn scp_domain_routing_info_subscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	State(api_impl): State<I>,
	Json(body): Json<models::ScpDomainRoutingInfoSubscription>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInformationSubscriptionsCollection,
{
	// Header parameters
	let header_params = {
		let header_content_encoding = headers.get(HeaderName::from_static("content_encoding"));

		let header_content_encoding = match header_content_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Content-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::ScpDomainRoutingInfoSubscribeHeaderParams {
			content_encoding: header_content_encoding,
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		scp_domain_routing_info_subscribe_validation(header_params, body)
	})
	.await
	.unwrap();

	let Ok((header_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.scp_domain_routing_info_subscribe(method, host, cookies, header_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status201
			{
				body,
				location,
				accept_encoding,
				content_encoding
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
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling accept_encoding header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							accept_encoding,
						);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding = match header::IntoHeaderValue(content_encoding).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling content_encoding header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							content_encoding,
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status400
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status401
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status403
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status404
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status411
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status413
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status415
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status429
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status500
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status501
			(body)
			=> {
				let mut response = response.status(501);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Status503
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
			apis::scp_domain_routing_information_subscriptions_collection::ScpDomainRoutingInfoSubscribeResponse::Statusdefault
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
fn retrieve_stored_search_validation(
	header_params: models::RetrieveStoredSearchHeaderParams,
	path_params: models::RetrieveStoredSearchPathParams,
) -> std::result::Result<
	(
		models::RetrieveStoredSearchHeaderParams,
		models::RetrieveStoredSearchPathParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;

	Ok((header_params, path_params))
}

/// RetrieveStoredSearch - GET /nnrf-disc/v1/searches/{searchId}
#[tracing::instrument(skip_all)]
async fn retrieve_stored_search<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::RetrieveStoredSearchPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::stored_search_document::StoredSearchDocument,
{
	// Header parameters
	let header_params = {
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::RetrieveStoredSearchHeaderParams {
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		retrieve_stored_search_validation(header_params, path_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.retrieve_stored_search(method, host, cookies, header_params, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::stored_search_document::RetrieveStoredSearchResponse::Status200 {
				body,
				cache_control,
				etag,
				content_encoding,
			} => {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling cache_control \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), cache_control);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling etag header - {}",
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
						response_headers.insert(HeaderName::from_static(""), etag);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding =
						match header::IntoHeaderValue(content_encoding).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 content_encoding header - {}",
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
						response_headers.insert(HeaderName::from_static(""), content_encoding);
					}
				}
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
			apis::stored_search_document::RetrieveStoredSearchResponse::Status307 {
				body,
				location,
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
			apis::stored_search_document::RetrieveStoredSearchResponse::Status308 {
				body,
				location,
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

#[tracing::instrument(skip_all)]
fn remove_subscription_validation(
	path_params: models::RemoveSubscriptionPathParams
) -> std::result::Result<(models::RemoveSubscriptionPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// RemoveSubscription - DELETE /nnrf-nfm/v1/subscriptions/{subscriptionID}
#[tracing::instrument(skip_all)]
async fn remove_subscription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::RemoveSubscriptionPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_id_document::SubscriptionIdDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || remove_subscription_validation(path_params))
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
		.remove_subscription(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_id_document::RemoveSubscriptionResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::subscription_id_document::RemoveSubscriptionResponse::Status307 {
				body,
				location,
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status308 {
				body,
				location,
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status400(body) => {
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status401(body) => {
				let mut response = response.status(401);
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status403(body) => {
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status404(body) => {
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status411(body) => {
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status413(body) => {
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status415(body) => {
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status429(body) => {
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status500(body) => {
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Status503(body) => {
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
			apis::subscription_id_document::RemoveSubscriptionResponse::Statusdefault => {
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
struct UpdateSubscriptionBodyValidator<'a> {
	body: &'a Vec<models::PatchItem>,
}

#[tracing::instrument(skip_all)]
fn update_subscription_validation(
	header_params: models::UpdateSubscriptionHeaderParams,
	path_params: models::UpdateSubscriptionPathParams,
	body: Vec<models::PatchItem>,
) -> std::result::Result<
	(
		models::UpdateSubscriptionHeaderParams,
		models::UpdateSubscriptionPathParams,
		Vec<models::PatchItem>,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	let b = UpdateSubscriptionBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, path_params, body))
}

/// UpdateSubscription - PATCH /nnrf-nfm/v1/subscriptions/{subscriptionID}
#[tracing::instrument(skip_all)]
async fn update_subscription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::UpdateSubscriptionPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Vec<models::PatchItem>>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_id_document::SubscriptionIdDocument,
{
	// Header parameters
	let header_params = {
		let header_content_encoding = headers.get(HeaderName::from_static("content_encoding"));

		let header_content_encoding = match header_content_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Content-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::UpdateSubscriptionHeaderParams {
			content_encoding: header_content_encoding,
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		update_subscription_validation(header_params, path_params, body)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.update_subscription(method, host, cookies, header_params, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_id_document::UpdateSubscriptionResponse::Status200 {
				body,
				accept_encoding,
				content_encoding,
			} => {
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding =
						match header::IntoHeaderValue(content_encoding).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 content_encoding header - {}",
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
						response_headers.insert(HeaderName::from_static(""), content_encoding);
					}
				}
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status204 {
				accept_encoding,
			} => {
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::subscription_id_document::UpdateSubscriptionResponse::Status307 {
				body,
				location,
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status308 {
				body,
				location,
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status400(body) => {
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status403(body) => {
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status404(body) => {
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status411(body) => {
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status413(body) => {
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status415(body) => {
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status429(body) => {
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status500(body) => {
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Status503(body) => {
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
			apis::subscription_id_document::UpdateSubscriptionResponse::Statusdefault => {
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
struct CreateSubscriptionBodyValidator<'a> {
	body: &'a models::SubscriptionData,
}

#[tracing::instrument(skip_all)]
fn create_subscription_validation(
	header_params: models::CreateSubscriptionHeaderParams,
	body: models::SubscriptionData,
) -> std::result::Result<
	(
		models::CreateSubscriptionHeaderParams,
		models::SubscriptionData,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	let b = CreateSubscriptionBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, body))
}

/// CreateSubscription - POST /nnrf-nfm/v1/subscriptions
#[tracing::instrument(skip_all)]
async fn create_subscription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	State(api_impl): State<I>,
	Json(body): Json<models::SubscriptionData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscriptions_collection::SubscriptionsCollection,
{
	// Header parameters
	let header_params = {
		let header_content_encoding = headers.get(HeaderName::from_static("content_encoding"));

		let header_content_encoding = match header_content_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Content-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};
		let header_accept_encoding = headers.get(HeaderName::from_static("accept_encoding"));

		let header_accept_encoding = match header_accept_encoding {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header Accept-Encoding - {}",
							err
						)))
						.map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						});
				}
			},
			None => None,
		};

		models::CreateSubscriptionHeaderParams {
			content_encoding: header_content_encoding,
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || create_subscription_validation(header_params, body))
			.await
			.unwrap();

	let Ok((header_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.create_subscription(method, host, cookies, header_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscriptions_collection::CreateSubscriptionResponse::Status201 {
				body,
				location,
				accept_encoding,
				content_encoding,
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
				if let Some(accept_encoding) = accept_encoding {
					let accept_encoding = match header::IntoHeaderValue(accept_encoding).try_into()
					{
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling accept_encoding \
									 header - {}",
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
						response_headers.insert(HeaderName::from_static(""), accept_encoding);
					}
				}
				if let Some(content_encoding) = content_encoding {
					let content_encoding =
						match header::IntoHeaderValue(content_encoding).try_into() {
							Ok(val) => val,
							Err(e) => {
								return Response::builder()
									.status(StatusCode::INTERNAL_SERVER_ERROR)
									.body(Body::from(format!(
										"An internal server error occurred handling \
										 content_encoding header - {}",
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
						response_headers.insert(HeaderName::from_static(""), content_encoding);
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status307 {
				body,
				location,
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status308 {
				body,
				location,
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status400(body) => {
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status401(body) => {
				let mut response = response.status(401);
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status403(body) => {
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status404(body) => {
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status411(body) => {
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status413(body) => {
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status415(body) => {
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status429(body) => {
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status500(body) => {
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status501(body) => {
				let mut response = response.status(501);
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Status503(body) => {
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
			apis::subscriptions_collection::CreateSubscriptionResponse::Statusdefault => {
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
