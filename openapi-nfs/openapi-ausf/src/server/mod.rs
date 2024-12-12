use std::collections::HashMap;

use crate::{header, types::*};
use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use oasbi::{DeserResponse, ReqError};
use tracing::error;
use validator::{Validate, ValidationErrors};

#[allow(unused_imports)]
use crate::{apis, models};


/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
							  where
								  I: AsRef<A> + Clone + Send + Sync + 'static,
								  A: apis::ausf_ueso_r_protection::AusfUesoRProtection + apis::pro_se_authentication::ProSeAuthentication + apis::rg_authentication::RgAuthentication + apis::ue_authentications::UeAuthentications + apis::ue_parameters_update_protection::UeParametersUpdateProtection + 'static,
{
	// build our application with a route
	Router::new()
		.route("/nausf-auth/v1/prose-authentications",
			post(create_pro_se_eap_session::<I, A>),
		)
		.route("/nausf-auth/v1/prose-authentications/:auth_ctx_id/prose-auth",
			delete(delete_pro_se_authentication_result::<I, A>).post(prose_auth::<I, A>),
		)
		.route("/nausf-auth/v1/rg-authentications",
			post(create_rg_authentication::<I, A>),
		)
		.route("/nausf-auth/v1/ue-authentications",
			post(create_ue_authentication::<I, A>),
		)
		.route("/nausf-auth/v1/ue-authentications/:auth_ctx_id/5g-aka-confirmation",
			delete(delete5g_aka_authentication_result::<I, A>).put(create5g_aka_authentication_result::<I, A>),
		)
		.route("/nausf-auth/v1/ue-authentications/:auth_ctx_id/eap-session",
			delete(delete_eap_authentication_result::<I, A>).post(eap_auth_method::<I, A>),
		)
		.route("/nausf-auth/v1/ue-authentications/deregister",
			post(deregister_ue::<I, A>),
		)
		.route("/nausf-sorprotection/v1/:supi/ue-sor",
			post(create_so_r_protection::<I, A>),
		)
		.route("/nausf-upuprotection/v1/:supi/ue-upu",
			post(create_upu_protection::<I, A>),
		)
		.with_state(api_impl)
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct CreateSoRProtectionBodyValidator<'a> {
	body: &'a models::SorInfo,
}


#[tracing::instrument(skip_all)]
fn create_so_r_protection_validation(
	path_params: models::CreateSoRProtectionPathParams,
	body: models::SorInfo,
) -> std::result::Result<(
	models::CreateSoRProtectionPathParams,
	models::SorInfo,
), ValidationErrors>
{
	path_params.validate()?;
	let b = CreateSoRProtectionBodyValidator { body: &body };
	b.validate()?;

	Ok((
		path_params,
		body,
	))
}

/// CreateSoRProtection - POST /nausf-sorprotection/v1/{supi}/ue-sor
#[tracing::instrument(skip_all)]
async fn create_so_r_protection<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::CreateSoRProtectionPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SorInfo>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::ausf_ueso_r_protection::AusfUesoRProtection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		create_so_r_protection_validation(
			path_params,
			body,
		)
	).await.unwrap();

	let Ok((
		path_params,
		body,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().create_so_r_protection(
		method,
		host,
		cookies,
		path_params,
		body,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ausf_ueso_r_protection::CreateSoRProtectionResponse::Status200
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
			apis::ausf_ueso_r_protection::CreateSoRProtectionResponse::Status307
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
			apis::ausf_ueso_r_protection::CreateSoRProtectionResponse::Status308
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
			apis::ausf_ueso_r_protection::CreateSoRProtectionResponse::Status503
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
struct CreateProSeEapSessionBodyValidator<'a> {
	body: &'a models::ProSeAuthenticationInfo,
}


#[tracing::instrument(skip_all)]
fn create_pro_se_eap_session_validation(
	body: models::ProSeAuthenticationInfo,
) -> std::result::Result<(
	models::ProSeAuthenticationInfo,
), ValidationErrors>
{
	let b = CreateProSeEapSessionBodyValidator { body: &body };
	b.validate()?;

	Ok((
		body,
	))
}

/// CreateProSeEapSession - POST /nausf-auth/v1/prose-authentications
#[tracing::instrument(skip_all)]
async fn create_pro_se_eap_session<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::ProSeAuthenticationInfo>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::pro_se_authentication::ProSeAuthentication,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		create_pro_se_eap_session_validation(
			body,
		)
	).await.unwrap();

	let Ok((
		body,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().create_pro_se_eap_session(
		method,
		host,
		cookies,
		body,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::pro_se_authentication::CreateProSeEapSessionResponse::Status200
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
			apis::pro_se_authentication::CreateProSeEapSessionResponse::Status201
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
						HeaderValue::from_str("application/3gppHal+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = body;
				response.body(Body::from(body_content))
			}
			apis::pro_se_authentication::CreateProSeEapSessionResponse::Status307
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
			apis::pro_se_authentication::CreateProSeEapSessionResponse::Status308
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
			apis::pro_se_authentication::CreateProSeEapSessionResponse::Status400
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
			apis::pro_se_authentication::CreateProSeEapSessionResponse::Status403
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
			apis::pro_se_authentication::CreateProSeEapSessionResponse::Status404
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
			apis::pro_se_authentication::CreateProSeEapSessionResponse::Status500
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
fn delete_pro_se_authentication_result_validation(
	path_params: models::DeleteProSeAuthenticationResultPathParams,
) -> std::result::Result<(
	models::DeleteProSeAuthenticationResultPathParams,
), ValidationErrors>
{
	path_params.validate()?;

	Ok((
		path_params,
	))
}

/// DeleteProSeAuthenticationResult - DELETE /nausf-auth/v1/prose-authentications/{authCtxId}/prose-auth
#[tracing::instrument(skip_all)]
async fn delete_pro_se_authentication_result<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteProSeAuthenticationResultPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::pro_se_authentication::ProSeAuthentication,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		delete_pro_se_authentication_result_validation(
			path_params,
		)
	).await.unwrap();

	let Ok((
		path_params,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().delete_pro_se_authentication_result(
		method,
		host,
		cookies,
		path_params,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::pro_se_authentication::DeleteProSeAuthenticationResultResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::pro_se_authentication::DeleteProSeAuthenticationResultResponse::Status307
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
			apis::pro_se_authentication::DeleteProSeAuthenticationResultResponse::Status308
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
			apis::pro_se_authentication::DeleteProSeAuthenticationResultResponse::Status400
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
			apis::pro_se_authentication::DeleteProSeAuthenticationResultResponse::Status404
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
			apis::pro_se_authentication::DeleteProSeAuthenticationResultResponse::Status500
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
			apis::pro_se_authentication::DeleteProSeAuthenticationResultResponse::Status503
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
			apis::pro_se_authentication::DeleteProSeAuthenticationResultResponse::Statusdefault
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
struct ProseAuthBodyValidator<'a> {
	body: &'a models::ProSeEapSession,
}


#[tracing::instrument(skip_all)]
fn prose_auth_validation(
	path_params: models::ProseAuthPathParams,
	body: Option<models::ProSeEapSession>,
) -> std::result::Result<(
	models::ProseAuthPathParams,
	Option<models::ProSeEapSession>,
), ValidationErrors>
{
	path_params.validate()?;
	if let Some(body) = &body {
		let b = ProseAuthBodyValidator { body };
		b.validate()?;
	}

	Ok((
		path_params,
		body,
	))
}

/// ProseAuth - POST /nausf-auth/v1/prose-authentications/{authCtxId}/prose-auth
#[tracing::instrument(skip_all)]
async fn prose_auth<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ProseAuthPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::ProSeEapSession>>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::pro_se_authentication::ProSeAuthentication,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		prose_auth_validation(
			path_params,
			body,
		)
	).await.unwrap();

	let Ok((
		path_params,
		body,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().prose_auth(
		method,
		host,
		cookies,
		path_params,
		body,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::pro_se_authentication::ProseAuthResponse::Status200
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
			apis::pro_se_authentication::ProseAuthResponse::Status307
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
			apis::pro_se_authentication::ProseAuthResponse::Status308
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
			apis::pro_se_authentication::ProseAuthResponse::Status400
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
			apis::pro_se_authentication::ProseAuthResponse::Status500
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
struct CreateRgAuthenticationBodyValidator<'a> {
	body: &'a models::RgAuthenticationInfo,
}


#[tracing::instrument(skip_all)]
fn create_rg_authentication_validation(
	body: models::RgAuthenticationInfo,
) -> std::result::Result<(
	models::RgAuthenticationInfo,
), ValidationErrors>
{
	let b = CreateRgAuthenticationBodyValidator { body: &body };
	b.validate()?;

	Ok((
		body,
	))
}

/// CreateRgAuthentication - POST /nausf-auth/v1/rg-authentications
#[tracing::instrument(skip_all)]
async fn create_rg_authentication<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::RgAuthenticationInfo>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::rg_authentication::RgAuthentication,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		create_rg_authentication_validation(
			body,
		)
	).await.unwrap();

	let Ok((
		body,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().create_rg_authentication(
		method,
		host,
		cookies,
		body,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::rg_authentication::CreateRgAuthenticationResponse::Status201
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
			apis::rg_authentication::CreateRgAuthenticationResponse::Status307
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
			apis::rg_authentication::CreateRgAuthenticationResponse::Status308
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
			apis::rg_authentication::CreateRgAuthenticationResponse::Status400
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
			apis::rg_authentication::CreateRgAuthenticationResponse::Status403
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
			apis::rg_authentication::CreateRgAuthenticationResponse::Status404
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
struct Create5gAkaAuthenticationResultBodyValidator<'a> {
	body: &'a models::ConfirmationData,
}


#[tracing::instrument(skip_all)]
fn create5g_aka_authentication_result_validation(
	path_params: models::Create5gAkaAuthenticationResultPathParams,
	body: Option<models::ConfirmationData>,
) -> std::result::Result<(
	models::Create5gAkaAuthenticationResultPathParams,
	Option<models::ConfirmationData>,
), ValidationErrors>
{
	path_params.validate()?;
	if let Some(body) = &body {
		let b = Create5gAkaAuthenticationResultBodyValidator { body };
		b.validate()?;
	}

	Ok((
		path_params,
		body,
	))
}

/// Create5gAkaAuthenticationResult - PUT /nausf-auth/v1/ue-authentications/{authCtxId}/5g-aka-confirmation
#[tracing::instrument(skip_all)]
async fn create5g_aka_authentication_result<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Create5gAkaAuthenticationResultPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::ConfirmationData>>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::ue_authentications::UeAuthentications,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		create5g_aka_authentication_result_validation(
			path_params,
			body,
		)
	).await.unwrap();

	let Ok((
		path_params,
		body,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().create5g_aka_authentication_result(
		method,
		host,
		cookies,
		path_params,
		body,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_authentications::Create5gAkaAuthenticationResultResponse::Status200
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
			apis::ue_authentications::Create5gAkaAuthenticationResultResponse::Status307
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
			apis::ue_authentications::Create5gAkaAuthenticationResultResponse::Status308
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
			apis::ue_authentications::Create5gAkaAuthenticationResultResponse::Status400
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
			apis::ue_authentications::Create5gAkaAuthenticationResultResponse::Status500
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
struct CreateUeAuthenticationBodyValidator<'a> {
	body: &'a models::AuthenticationInfo,
}


#[tracing::instrument(skip_all)]
fn create_ue_authentication_validation(
	body: models::AuthenticationInfo,
) -> std::result::Result<(
	models::AuthenticationInfo,
), ValidationErrors>
{
	let b = CreateUeAuthenticationBodyValidator { body: &body };
	b.validate()?;

	Ok((
		body,
	))
}

/// CreateUeAuthentication - POST /nausf-auth/v1/ue-authentications
#[tracing::instrument(skip_all)]
async fn create_ue_authentication<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::AuthenticationInfo>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::ue_authentications::UeAuthentications,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		create_ue_authentication_validation(
			body,
		)
	).await.unwrap();

	let Ok((
		body,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().create_ue_authentication(
		method,
		host,
		cookies,
		body,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_authentications::CreateUeAuthenticationResponse::Status201
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
						HeaderValue::from_str("application/3gppHal+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = body;
				response.body(Body::from(body_content))
			}
			apis::ue_authentications::CreateUeAuthenticationResponse::Status307
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
			apis::ue_authentications::CreateUeAuthenticationResponse::Status308
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
			apis::ue_authentications::CreateUeAuthenticationResponse::Status400
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
			apis::ue_authentications::CreateUeAuthenticationResponse::Status403
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
			apis::ue_authentications::CreateUeAuthenticationResponse::Status404
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
			apis::ue_authentications::CreateUeAuthenticationResponse::Status500
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
			apis::ue_authentications::CreateUeAuthenticationResponse::Status501
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
fn delete5g_aka_authentication_result_validation(
	path_params: models::Delete5gAkaAuthenticationResultPathParams,
) -> std::result::Result<(
	models::Delete5gAkaAuthenticationResultPathParams,
), ValidationErrors>
{
	path_params.validate()?;

	Ok((
		path_params,
	))
}

/// Delete5gAkaAuthenticationResult - DELETE /nausf-auth/v1/ue-authentications/{authCtxId}/5g-aka-confirmation
#[tracing::instrument(skip_all)]
async fn delete5g_aka_authentication_result<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Delete5gAkaAuthenticationResultPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::ue_authentications::UeAuthentications,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		delete5g_aka_authentication_result_validation(
			path_params,
		)
	).await.unwrap();

	let Ok((
		path_params,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().delete5g_aka_authentication_result(
		method,
		host,
		cookies,
		path_params,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_authentications::Delete5gAkaAuthenticationResultResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::ue_authentications::Delete5gAkaAuthenticationResultResponse::Status307
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
			apis::ue_authentications::Delete5gAkaAuthenticationResultResponse::Status308
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
			apis::ue_authentications::Delete5gAkaAuthenticationResultResponse::Status400
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
			apis::ue_authentications::Delete5gAkaAuthenticationResultResponse::Status404
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
			apis::ue_authentications::Delete5gAkaAuthenticationResultResponse::Status500
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
			apis::ue_authentications::Delete5gAkaAuthenticationResultResponse::Status503
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
			apis::ue_authentications::Delete5gAkaAuthenticationResultResponse::Statusdefault
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
fn delete_eap_authentication_result_validation(
	path_params: models::DeleteEapAuthenticationResultPathParams,
) -> std::result::Result<(
	models::DeleteEapAuthenticationResultPathParams,
), ValidationErrors>
{
	path_params.validate()?;

	Ok((
		path_params,
	))
}

/// DeleteEapAuthenticationResult - DELETE /nausf-auth/v1/ue-authentications/{authCtxId}/eap-session
#[tracing::instrument(skip_all)]
async fn delete_eap_authentication_result<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteEapAuthenticationResultPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::ue_authentications::UeAuthentications,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		delete_eap_authentication_result_validation(
			path_params,
		)
	).await.unwrap();

	let Ok((
		path_params,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().delete_eap_authentication_result(
		method,
		host,
		cookies,
		path_params,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_authentications::DeleteEapAuthenticationResultResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::ue_authentications::DeleteEapAuthenticationResultResponse::Status307
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
			apis::ue_authentications::DeleteEapAuthenticationResultResponse::Status308
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
			apis::ue_authentications::DeleteEapAuthenticationResultResponse::Status400
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
			apis::ue_authentications::DeleteEapAuthenticationResultResponse::Status404
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
			apis::ue_authentications::DeleteEapAuthenticationResultResponse::Status500
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
			apis::ue_authentications::DeleteEapAuthenticationResultResponse::Status503
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
			apis::ue_authentications::DeleteEapAuthenticationResultResponse::Statusdefault
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
struct DeregisterUeBodyValidator<'a> {
	body: &'a models::DeregistrationInfo,
}


#[tracing::instrument(skip_all)]
fn deregister_ue_validation(
	body: models::DeregistrationInfo,
) -> std::result::Result<(
	models::DeregistrationInfo,
), ValidationErrors>
{
	let b = DeregisterUeBodyValidator { body: &body };
	b.validate()?;

	Ok((
		body,
	))
}

/// DeregisterUe - POST /nausf-auth/v1/ue-authentications/deregister
#[tracing::instrument(skip_all)]
async fn deregister_ue<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::DeregistrationInfo>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::ue_authentications::UeAuthentications,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		deregister_ue_validation(
			body,
		)
	).await.unwrap();

	let Ok((
		body,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().deregister_ue(
		method,
		host,
		cookies,
		body,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_authentications::DeregisterUeResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::ue_authentications::DeregisterUeResponse::Status307
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
			apis::ue_authentications::DeregisterUeResponse::Status308
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
			apis::ue_authentications::DeregisterUeResponse::Status404
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
struct EapAuthMethodBodyValidator<'a> {
	body: &'a models::EapSession,
}


#[tracing::instrument(skip_all)]
fn eap_auth_method_validation(
	path_params: models::EapAuthMethodPathParams,
	body: Option<models::EapSession>,
) -> std::result::Result<(
	models::EapAuthMethodPathParams,
	Option<models::EapSession>,
), ValidationErrors>
{
	path_params.validate()?;
	if let Some(body) = &body {
		let b = EapAuthMethodBodyValidator { body };
		b.validate()?;
	}

	Ok((
		path_params,
		body,
	))
}

/// EapAuthMethod - POST /nausf-auth/v1/ue-authentications/{authCtxId}/eap-session
#[tracing::instrument(skip_all)]
async fn eap_auth_method<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::EapAuthMethodPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::EapSession>>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::ue_authentications::UeAuthentications,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		eap_auth_method_validation(
			path_params,
			body,
		)
	).await.unwrap();

	let Ok((
		path_params,
		body,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().eap_auth_method(
		method,
		host,
		cookies,
		path_params,
		body,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_authentications::EapAuthMethodResponse::Status200
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
			apis::ue_authentications::EapAuthMethodResponse::Status307
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
			apis::ue_authentications::EapAuthMethodResponse::Status308
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
			apis::ue_authentications::EapAuthMethodResponse::Status400
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
			apis::ue_authentications::EapAuthMethodResponse::Status500
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
struct CreateUpuProtectionBodyValidator<'a> {
	body: &'a models::UpuInfo,
}


#[tracing::instrument(skip_all)]
fn create_upu_protection_validation(
	path_params: models::CreateUpuProtectionPathParams,
	body: models::UpuInfo,
) -> std::result::Result<(
	models::CreateUpuProtectionPathParams,
	models::UpuInfo,
), ValidationErrors>
{
	path_params.validate()?;
	let b = CreateUpuProtectionBodyValidator { body: &body };
	b.validate()?;

	Ok((
		path_params,
		body,
	))
}

/// CreateUpuProtection - POST /nausf-upuprotection/v1/{supi}/ue-upu
#[tracing::instrument(skip_all)]
async fn create_upu_protection<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::CreateUpuProtectionPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::UpuInfo>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::ue_parameters_update_protection::UeParametersUpdateProtection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move ||
		create_upu_protection_validation(
			path_params,
			body,
		)
	).await.unwrap();

	let Ok((
		path_params,
		body,
	)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl.as_ref().create_upu_protection(
		method,
		host,
		cookies,
		path_params,
		body,
	).await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_parameters_update_protection::CreateUpuProtectionResponse::Status200
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
			apis::ue_parameters_update_protection::CreateUpuProtectionResponse::Status307
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
			apis::ue_parameters_update_protection::CreateUpuProtectionResponse::Status308
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
			apis::ue_parameters_update_protection::CreateUpuProtectionResponse::Status503
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

