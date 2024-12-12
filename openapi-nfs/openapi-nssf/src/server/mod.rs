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
	A: apis::nf_instance_id_document::NfInstanceIdDocument
		+ apis::nssai_availability_store::NssaiAvailabilityStore
		+ apis::network_slice_information_document::NetworkSliceInformationDocument
		+ apis::subscription_id_document::SubscriptionIdDocument
		+ apis::subscriptions_collection::SubscriptionsCollection
		+ 'static,
{
	// build our application with a route
	Router::new()
		.route(
			"/nnssf-nssaiavailability/v1/nssai-availability",
			options(n_ssai_availability_options::<I, A>),
		)
		.route(
			"/nnssf-nssaiavailability/v1/nssai-availability/:nf_id",
			delete(n_ssai_availability_delete::<I, A>)
				.patch(n_ssai_availability_patch::<I, A>)
				.put(n_ssai_availability_put::<I, A>),
		)
		.route(
			"/nnssf-nssaiavailability/v1/nssai-availability/subscriptions",
			post(n_ssai_availability_post::<I, A>),
		)
		.route(
			"/nnssf-nssaiavailability/v1/nssai-availability/subscriptions/:subscription_id",
			delete(n_ssai_availability_unsubscribe::<I, A>)
				.patch(n_ssai_availability_sub_modify_patch::<I, A>),
		)
		.route(
			"/nnssf-nsselection/v2/network-slice-information",
			get(ns_selection_get::<I, A>),
		)
		.with_state(api_impl)
}

#[tracing::instrument(skip_all)]
fn n_ssai_availability_delete_validation(
	path_params: models::NSsaiAvailabilityDeletePathParams
) -> std::result::Result<(models::NSsaiAvailabilityDeletePathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// NSsaiAvailabilityDelete - DELETE
/// /nnssf-nssaiavailability/v1/nssai-availability/{nfId}
#[tracing::instrument(skip_all)]
async fn n_ssai_availability_delete<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::NSsaiAvailabilityDeletePathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nf_instance_id_document::NfInstanceIdDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || n_ssai_availability_delete_validation(path_params))
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
		.n_ssai_availability_delete(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Status307 {
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
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Status308 {
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
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Status400(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Status401(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Status404(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Status429(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Status500(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Status503(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityDeleteResponse::Statusdefault => {
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
struct NSsaiAvailabilityPatchBodyValidator<'a> {
	#[validate(length(min = 1))]
	body: &'a models::PatchDocument,
}

#[tracing::instrument(skip_all)]
fn n_ssai_availability_patch_validation(
	header_params: models::NSsaiAvailabilityPatchHeaderParams,
	path_params: models::NSsaiAvailabilityPatchPathParams,
	body: models::PatchDocument,
) -> std::result::Result<
	(
		models::NSsaiAvailabilityPatchHeaderParams,
		models::NSsaiAvailabilityPatchPathParams,
		models::PatchDocument,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	let b = NSsaiAvailabilityPatchBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, path_params, body))
}

/// NSsaiAvailabilityPatch - PATCH
/// /nnssf-nssaiavailability/v1/nssai-availability/{nfId}
#[tracing::instrument(skip_all)]
async fn n_ssai_availability_patch<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::NSsaiAvailabilityPatchPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::PatchDocument>,
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

		models::NSsaiAvailabilityPatchHeaderParams {
			content_encoding: header_content_encoding,
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		n_ssai_availability_patch_validation(header_params, path_params, body)
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
		.n_ssai_availability_patch(method, host, cookies, header_params, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status200 {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status307 {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status308 {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status400(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status401(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status403(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status404(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status411(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status413(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status415(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status429(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status500(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Status503(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPatchResponse::Statusdefault => {
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
struct NSsaiAvailabilityPutBodyValidator<'a> {
	body: &'a models::NssaiAvailabilityInfo,
}

#[tracing::instrument(skip_all)]
fn n_ssai_availability_put_validation(
	header_params: models::NSsaiAvailabilityPutHeaderParams,
	path_params: models::NSsaiAvailabilityPutPathParams,
	body: models::NssaiAvailabilityInfo,
) -> std::result::Result<
	(
		models::NSsaiAvailabilityPutHeaderParams,
		models::NSsaiAvailabilityPutPathParams,
		models::NssaiAvailabilityInfo,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	let b = NSsaiAvailabilityPutBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, path_params, body))
}

/// NSsaiAvailabilityPut - PUT
/// /nnssf-nssaiavailability/v1/nssai-availability/{nfId}
#[tracing::instrument(skip_all)]
async fn n_ssai_availability_put<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::NSsaiAvailabilityPutPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::NssaiAvailabilityInfo>,
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

		models::NSsaiAvailabilityPutHeaderParams {
			content_encoding: header_content_encoding,
			accept_encoding: header_accept_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		n_ssai_availability_put_validation(header_params, path_params, body)
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
		.n_ssai_availability_put(method, host, cookies, header_params, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status200 {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status307 {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status308 {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status400(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status401(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status403(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status404(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status411(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status413(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status415(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status429(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status500(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Status503(body) => {
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
			apis::nf_instance_id_document::NSsaiAvailabilityPutResponse::Statusdefault => {
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
fn n_ssai_availability_options_validation() -> std::result::Result<(), ValidationErrors> {
	Ok(())
}

/// NSsaiAvailabilityOptions - OPTIONS
/// /nnssf-nssaiavailability/v1/nssai-availability
#[tracing::instrument(skip_all)]
async fn n_ssai_availability_options<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nssai_availability_store::NssaiAvailabilityStore,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || n_ssai_availability_options_validation())
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
		.n_ssai_availability_options(method, host, cookies)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status200 {
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
				response.body(Body::empty())
			}
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status307 {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status308 {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status400(body) => {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status401(body) => {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status403(body) => {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status404(body) => {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status405 => {
				let mut response = response.status(405);
				response.body(Body::empty())
			}
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status429(body) => {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status500(body) => {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status501(body) => {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Status503(body) => {
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
			apis::nssai_availability_store::NSsaiAvailabilityOptionsResponse::Statusdefault => {
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
fn ns_selection_get_validation(
	query_params: models::NSSelectionGetQueryParams
) -> std::result::Result<(models::NSSelectionGetQueryParams,), ValidationErrors> {
	query_params.validate()?;

	Ok((query_params,))
}

/// NSSelectionGet - GET /nnssf-nsselection/v2/network-slice-information
#[tracing::instrument(skip_all)]
async fn ns_selection_get<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Query(query_params): Query<models::NSSelectionGetQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::network_slice_information_document::NetworkSliceInformationDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || ns_selection_get_validation(query_params))
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
		.ns_selection_get(method, host, cookies, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::network_slice_information_document::NSSelectionGetResponse::Status200(body) => {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status307 {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status308 {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status400(body) => {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status401(body) => {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status403(body) => {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status404(body) => {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::network_slice_information_document::NSSelectionGetResponse::Status414(body) => {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status429(body) => {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status500(body) => {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Status503(body) => {
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
			apis::network_slice_information_document::NSSelectionGetResponse::Statusdefault => {
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
struct NSsaiAvailabilitySubModifyPatchBodyValidator<'a> {
	#[validate(length(min = 1))]
	body: &'a models::PatchDocument,
}

#[tracing::instrument(skip_all)]
fn n_ssai_availability_sub_modify_patch_validation(
	header_params: models::NSsaiAvailabilitySubModifyPatchHeaderParams,
	path_params: models::NSsaiAvailabilitySubModifyPatchPathParams,
	body: models::PatchDocument,
) -> std::result::Result<
	(
		models::NSsaiAvailabilitySubModifyPatchHeaderParams,
		models::NSsaiAvailabilitySubModifyPatchPathParams,
		models::PatchDocument,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	let b = NSsaiAvailabilitySubModifyPatchBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, path_params, body))
}

/// NSsaiAvailabilitySubModifyPatch - PATCH
/// /nnssf-nssaiavailability/v1/nssai-availability/subscriptions/
/// {subscriptionId}
#[tracing::instrument(skip_all)]
async fn n_ssai_availability_sub_modify_patch<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::NSsaiAvailabilitySubModifyPatchPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::PatchDocument>,
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

		models::NSsaiAvailabilitySubModifyPatchHeaderParams {
			content_encoding: header_content_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		n_ssai_availability_sub_modify_patch_validation(header_params, path_params, body)
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
		.n_ssai_availability_sub_modify_patch(
			method,
			host,
			cookies,
			header_params,
			path_params,
			body,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status200
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status307
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status308
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status400
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status401
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status403
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status404
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status411
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status413
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status415
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status429
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status500
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Status503
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
			apis::subscription_id_document::NSsaiAvailabilitySubModifyPatchResponse::Statusdefault
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
fn n_ssai_availability_unsubscribe_validation(
	path_params: models::NSsaiAvailabilityUnsubscribePathParams
) -> std::result::Result<(models::NSsaiAvailabilityUnsubscribePathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// NSsaiAvailabilityUnsubscribe - DELETE
/// /nnssf-nssaiavailability/v1/nssai-availability/subscriptions/
/// {subscriptionId}
#[tracing::instrument(skip_all)]
async fn n_ssai_availability_unsubscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::NSsaiAvailabilityUnsubscribePathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_id_document::SubscriptionIdDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		n_ssai_availability_unsubscribe_validation(path_params)
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
		.n_ssai_availability_unsubscribe(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Status307 {
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
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Status308 {
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
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Status400(
				body,
			) => {
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
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Status401(
				body,
			) => {
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
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Status404(
				body,
			) => {
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
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Status429(
				body,
			) => {
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
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Status500(
				body,
			) => {
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
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Status503(
				body,
			) => {
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
			apis::subscription_id_document::NSsaiAvailabilityUnsubscribeResponse::Statusdefault => {
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
struct NSsaiAvailabilityPostBodyValidator<'a> {
	body: &'a models::NssfEventSubscriptionCreateData,
}

#[tracing::instrument(skip_all)]
fn n_ssai_availability_post_validation(
	header_params: models::NSsaiAvailabilityPostHeaderParams,
	body: models::NssfEventSubscriptionCreateData,
) -> std::result::Result<
	(
		models::NSsaiAvailabilityPostHeaderParams,
		models::NssfEventSubscriptionCreateData,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	let b = NSsaiAvailabilityPostBodyValidator { body: &body };
	b.validate()?;

	Ok((header_params, body))
}

/// NSsaiAvailabilityPost - POST
/// /nnssf-nssaiavailability/v1/nssai-availability/subscriptions
#[tracing::instrument(skip_all)]
async fn n_ssai_availability_post<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	State(api_impl): State<I>,
	Json(body): Json<models::NssfEventSubscriptionCreateData>,
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

		models::NSsaiAvailabilityPostHeaderParams {
			content_encoding: header_content_encoding,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		n_ssai_availability_post_validation(header_params, body)
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
		.n_ssai_availability_post(method, host, cookies, header_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status201 {
				body,
				location,
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status307 {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status308 {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status400(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status401(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status403(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status404(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status411(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status413(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status415(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status429(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status500(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Status503(body) => {
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
			apis::subscriptions_collection::NSsaiAvailabilityPostResponse::Statusdefault => {
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
