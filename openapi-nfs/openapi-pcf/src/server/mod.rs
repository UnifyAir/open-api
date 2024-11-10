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
								  A: apis::am_policy_associations_collection::AmPolicyAssociationsCollection + apis::am_policy_events_subscription_document::AmPolicyEventsSubscriptionDocument + apis::application_am_contexts_collection::ApplicationAmContextsCollection + apis::application_sessions_collection::ApplicationSessionsCollection + apis::bdt_policies_collection::BdtPoliciesCollection + apis::events_subscription_document::EventsSubscriptionDocument + apis::individual_am_policy_association_document::IndividualAmPolicyAssociationDocument + apis::individual_application_am_context_document::IndividualApplicationAmContextDocument + apis::individual_application_session_context_document::IndividualApplicationSessionContextDocument + apis::individual_bdt_policy_document::IndividualBdtPolicyDocument + apis::individual_mbs_application_session_context_document::IndividualMbsApplicationSessionContextDocument + apis::individual_mbs_policy_document::IndividualMbsPolicyDocument + apis::individual_policy_control_events_subscription_document::IndividualPolicyControlEventsSubscriptionDocument + apis::individual_sm_policy_document::IndividualSmPolicyDocument + apis::individual_ue_policy_association_document::IndividualUePolicyAssociationDocument + apis::mbs_application_session_contexts_collection::MbsApplicationSessionContextsCollection + apis::mbs_policies_collection::MbsPoliciesCollection + apis::pcscf_restoration_indication::PcscfRestorationIndication + apis::policy_control_events_subscription_collection::PolicyControlEventsSubscriptionCollection + apis::sm_policies_collection::SmPoliciesCollection + apis::ue_policy_associations_collection::UePolicyAssociationsCollection + 'static,
{
	// build our application with a route
	Router::new()
		.route(
			"/npcf-am-policy-control/v1/policies",
			post(create_individual_am_policy_association::<I, A>),
		)
		.route(
			"/npcf-am-policy-control/v1/policies/:pol_asso_id",
			delete(delete_individual_am_policy_association::<I, A>)
				.get(read_individual_am_policy_association::<I, A>),
		)
		.route(
			"/npcf-am-policy-control/v1/policies/:pol_asso_id/update",
			post(report_observed_event_triggers_for_individual_am_policy_association::<I, A>),
		)
		.route(
			"/npcf-am-policyauthorization/v1/app-am-contexts",
			post(post_app_am_contexts::<I, A>),
		)
		.route(
			"/npcf-am-policyauthorization/v1/app-am-contexts/:app_am_context_id",
			delete(delete_app_am_context::<I, A>)
				.get(get_app_am_context::<I, A>)
				.patch(mod_app_am_context::<I, A>),
		)
		.route(
			"/npcf-am-policyauthorization/v1/app-am-contexts/:app_am_context_id/\
			 events-subscription",
			delete(delete_am_events_subsc::<I, A>).put(update_am_events_subsc::<I, A>),
		)
		.route(
			"/npcf-bdtpolicycontrol/v1/bdtpolicies",
			post(create_bdt_policy::<I, A>),
		)
		.route(
			"/npcf-bdtpolicycontrol/v1/bdtpolicies/:bdt_policy_id",
			get(get_bdt_policy::<I, A>).patch(update_bdt_policy::<I, A>),
		)
		.route(
			"/npcf-eventexposure/v1/subscriptions",
			post(post_pc_event_exposure_subsc::<I, A>),
		)
		.route(
			"/npcf-eventexposure/v1/subscriptions/:subscription_id",
			delete(delete_pc_event_exposure_subsc::<I, A>)
				.get(get_pc_event_exposure_subsc::<I, A>)
				.put(put_pc_event_exposure_subsc::<I, A>),
		)
		.route(
			"/npcf-mbspolicyauth/v1/contexts",
			post(create_mbs_app_session_ctxt::<I, A>),
		)
		.route(
			"/npcf-mbspolicyauth/v1/contexts/:context_id",
			delete(delete_mbs_app_session_ctxt::<I, A>)
				.get(get_mbs_app_session_ctxt::<I, A>)
				.patch(modify_mbs_app_session_ctxt::<I, A>),
		)
		.route(
			"/npcf-mbspolicycontrol/v1/mbs-policies",
			post(create_mbs_policy::<I, A>),
		)
		.route(
			"/npcf-mbspolicycontrol/v1/mbs-policies/:mbs_policy_id",
			delete(delete_ind_mbs_policy::<I, A>).get(get_ind_mbs_policy::<I, A>),
		)
		.route(
			"/npcf-mbspolicycontrol/v1/mbs-policies/:mbs_policy_id/update",
			post(update_ind_mbs_policy::<I, A>),
		)
		.route(
			"/npcf-policyauthorization/v1/app-sessions",
			post(post_app_sessions::<I, A>),
		)
		.route(
			"/npcf-policyauthorization/v1/app-sessions/:app_session_id",
			get(get_app_session::<I, A>).patch(mod_app_session::<I, A>),
		)
		.route(
			"/npcf-policyauthorization/v1/app-sessions/:app_session_id/delete",
			post(delete_app_session::<I, A>),
		)
		.route(
			"/npcf-policyauthorization/v1/app-sessions/:app_session_id/events-subscription",
			delete(delete_events_subsc::<I, A>).put(update_events_subsc::<I, A>),
		)
		.route(
			"/npcf-policyauthorization/v1/app-sessions/pcscf-restoration",
			post(pcscf_restoration::<I, A>),
		)
		.route(
			"/npcf-smpolicycontrol/v1/sm-policies",
			post(create_sm_policy::<I, A>),
		)
		.route(
			"/npcf-smpolicycontrol/v1/sm-policies/:sm_policy_id",
			get(get_sm_policy::<I, A>),
		)
		.route(
			"/npcf-smpolicycontrol/v1/sm-policies/:sm_policy_id/delete",
			post(delete_sm_policy::<I, A>),
		)
		.route(
			"/npcf-smpolicycontrol/v1/sm-policies/:sm_policy_id/update",
			post(update_sm_policy::<I, A>),
		)
		.route(
			"/npcf-ue-policy-control/v1/policies",
			post(create_individual_ue_policy_association::<I, A>),
		)
		.route(
			"/npcf-ue-policy-control/v1/policies/:pol_asso_id",
			delete(delete_individual_ue_policy_association::<I, A>)
				.get(read_individual_ue_policy_association::<I, A>),
		)
		.route(
			"/npcf-ue-policy-control/v1/policies/:pol_asso_id/update",
			post(report_observed_event_triggers_for_individual_ue_policy_association::<I, A>),
		)
		.with_state(api_impl)
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct CreateIndividualAmPolicyAssociationBodyValidator<'a> {
	body: &'a models::PolicyAssociationRequest,
}

#[tracing::instrument(skip_all)]
fn create_individual_am_policy_association_validation(
	body: models::PolicyAssociationRequest
) -> std::result::Result<(models::PolicyAssociationRequest,), ValidationErrors> {
	let b = CreateIndividualAmPolicyAssociationBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// CreateIndividualAmPolicyAssociation - POST
/// /npcf-am-policy-control/v1/policies
#[tracing::instrument(skip_all)]
async fn create_individual_am_policy_association<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::PolicyAssociationRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::am_policy_associations_collection::AmPolicyAssociationsCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		create_individual_am_policy_association_validation(body)
	})
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
		.create_individual_am_policy_association(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status201
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status400
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status401
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status403
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status404
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status411
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status413
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status415
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status429
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status500
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Status503
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
			apis::am_policy_associations_collection::CreateIndividualAmPolicyAssociationResponse::Statusdefault
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
fn delete_am_events_subsc_validation(
	path_params: models::DeleteAmEventsSubscPathParams
) -> std::result::Result<(models::DeleteAmEventsSubscPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeleteAmEventsSubsc - DELETE
/// /npcf-am-policyauthorization/v1/app-am-contexts/{appAmContextId}/
/// events-subscription
#[tracing::instrument(skip_all)]
async fn delete_am_events_subsc<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteAmEventsSubscPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::am_policy_events_subscription_document::AmPolicyEventsSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || delete_am_events_subsc_validation(path_params))
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
		.delete_am_events_subsc(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status307
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
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status308
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
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status400
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
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status401
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
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status403
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
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status404
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
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status429
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
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status500
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
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Status503
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
			apis::am_policy_events_subscription_document::DeleteAmEventsSubscResponse::Statusdefault
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
struct UpdateAmEventsSubscBodyValidator<'a> {
	body: &'a models::AmEventsSubscData,
}

#[tracing::instrument(skip_all)]
fn update_am_events_subsc_validation(
	path_params: models::UpdateAmEventsSubscPathParams,
	body: models::AmEventsSubscData,
) -> std::result::Result<
	(
		models::UpdateAmEventsSubscPathParams,
		models::AmEventsSubscData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = UpdateAmEventsSubscBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// UpdateAmEventsSubsc - PUT
/// /npcf-am-policyauthorization/v1/app-am-contexts/{appAmContextId}/
/// events-subscription
#[tracing::instrument(skip_all)]
async fn update_am_events_subsc<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateAmEventsSubscPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AmEventsSubscData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::am_policy_events_subscription_document::AmPolicyEventsSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || update_am_events_subsc_validation(path_params, body))
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
		.update_am_events_subsc(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status200
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status201
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status307
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status308
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status400
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status401
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status403
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status404
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status411
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status413
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status415
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status429
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status500
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Status503
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
			apis::am_policy_events_subscription_document::UpdateAmEventsSubscResponse::Statusdefault
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
struct PostAppAmContextsBodyValidator<'a> {
	body: &'a models::AppAmContextData,
}

#[tracing::instrument(skip_all)]
fn post_app_am_contexts_validation(
	body: models::AppAmContextData
) -> std::result::Result<(models::AppAmContextData,), ValidationErrors> {
	let b = PostAppAmContextsBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// PostAppAmContexts - POST /npcf-am-policyauthorization/v1/app-am-contexts
#[tracing::instrument(skip_all)]
async fn post_app_am_contexts<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::AppAmContextData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::application_am_contexts_collection::ApplicationAmContextsCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || post_app_am_contexts_validation(body))
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
		.post_app_am_contexts(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status201 {
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status400(
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status401(
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status403(
				body,
			) => {
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status404(
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status411(
				body,
			) => {
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status413(
				body,
			) => {
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status415(
				body,
			) => {
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status429(
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status500(
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Status503(
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
			apis::application_am_contexts_collection::PostAppAmContextsResponse::Statusdefault => {
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
struct PostAppSessionsBodyValidator<'a> {
	body: &'a models::AppSessionContext,
}

#[tracing::instrument(skip_all)]
fn post_app_sessions_validation(
	body: models::AppSessionContext
) -> std::result::Result<(models::AppSessionContext,), ValidationErrors> {
	let b = PostAppSessionsBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// PostAppSessions - POST /npcf-policyauthorization/v1/app-sessions
#[tracing::instrument(skip_all)]
async fn post_app_sessions<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::AppSessionContext>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::application_sessions_collection::ApplicationSessionsCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || post_app_sessions_validation(body))
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
		.post_app_sessions(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::application_sessions_collection::PostAppSessionsResponse::Status201 {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status303 {
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
				let mut response = response.status(303);
				response.body(Body::empty())
			}
			apis::application_sessions_collection::PostAppSessionsResponse::Status400(body) => {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status401(body) => {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status403 {
				body,
				retry_after,
			} => {
				if let Some(retry_after) = retry_after {
					let retry_after = match header::IntoHeaderValue(retry_after).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling retry_after \
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
						response_headers.insert(HeaderName::from_static(""), retry_after);
					}
				}
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status404(body) => {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status411(body) => {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status413(body) => {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status415(body) => {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status429(body) => {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status500(body) => {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Status503(body) => {
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
			apis::application_sessions_collection::PostAppSessionsResponse::Statusdefault => {
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
struct CreateBdtPolicyBodyValidator<'a> {
	body: &'a models::BdtReqData,
}

#[tracing::instrument(skip_all)]
fn create_bdt_policy_validation(
	body: models::BdtReqData
) -> std::result::Result<(models::BdtReqData,), ValidationErrors> {
	let b = CreateBdtPolicyBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// CreateBdtPolicy - POST /npcf-bdtpolicycontrol/v1/bdtpolicies
#[tracing::instrument(skip_all)]
async fn create_bdt_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::BdtReqData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::bdt_policies_collection::BdtPoliciesCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || create_bdt_policy_validation(body))
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
		.create_bdt_policy(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status201 {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status303 { location } => {
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
				let mut response = response.status(303);
				response.body(Body::empty())
			}
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status400(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status401(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status403(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status404(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status411(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status413(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status415(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status429(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status500(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Status503(body) => {
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
			apis::bdt_policies_collection::CreateBdtPolicyResponse::Statusdefault => {
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
fn delete_events_subsc_validation(
	path_params: models::DeleteEventsSubscPathParams
) -> std::result::Result<(models::DeleteEventsSubscPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeleteEventsSubsc - DELETE
/// /npcf-policyauthorization/v1/app-sessions/{appSessionId}/events-subscription
#[tracing::instrument(skip_all)]
async fn delete_events_subsc<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteEventsSubscPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::events_subscription_document::EventsSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || delete_events_subsc_validation(path_params))
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
		.delete_events_subsc(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::events_subscription_document::DeleteEventsSubscResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::events_subscription_document::DeleteEventsSubscResponse::Status307 {
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
			apis::events_subscription_document::DeleteEventsSubscResponse::Status308 {
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
			apis::events_subscription_document::DeleteEventsSubscResponse::Status400(body) => {
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
			apis::events_subscription_document::DeleteEventsSubscResponse::Status401(body) => {
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
			apis::events_subscription_document::DeleteEventsSubscResponse::Status403(body) => {
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
			apis::events_subscription_document::DeleteEventsSubscResponse::Status404(body) => {
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
			apis::events_subscription_document::DeleteEventsSubscResponse::Status429(body) => {
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
			apis::events_subscription_document::DeleteEventsSubscResponse::Status500(body) => {
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
			apis::events_subscription_document::DeleteEventsSubscResponse::Status503(body) => {
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
			apis::events_subscription_document::DeleteEventsSubscResponse::Statusdefault => {
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
struct UpdateEventsSubscBodyValidator<'a> {
	body: &'a models::EventsSubscReqData,
}

#[tracing::instrument(skip_all)]
fn update_events_subsc_validation(
	path_params: models::UpdateEventsSubscPathParams,
	body: models::EventsSubscReqData,
) -> std::result::Result<
	(
		models::UpdateEventsSubscPathParams,
		models::EventsSubscReqData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = UpdateEventsSubscBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// UpdateEventsSubsc - PUT
/// /npcf-policyauthorization/v1/app-sessions/{appSessionId}/events-subscription
#[tracing::instrument(skip_all)]
async fn update_events_subsc<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateEventsSubscPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::EventsSubscReqData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::events_subscription_document::EventsSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || update_events_subsc_validation(path_params, body))
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
		.update_events_subsc(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::events_subscription_document::UpdateEventsSubscResponse::Status200(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status201 {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::events_subscription_document::UpdateEventsSubscResponse::Status307 {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status308 {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status400(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status401(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status403(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status404(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status411(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status413(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status415(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status429(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status500(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Status503(body) => {
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
			apis::events_subscription_document::UpdateEventsSubscResponse::Statusdefault => {
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
fn delete_individual_am_policy_association_validation(
	path_params: models::DeleteIndividualAmPolicyAssociationPathParams
) -> std::result::Result<(models::DeleteIndividualAmPolicyAssociationPathParams,), ValidationErrors>
{
	path_params.validate()?;

	Ok((path_params,))
}

/// DeleteIndividualAmPolicyAssociation - DELETE
/// /npcf-am-policy-control/v1/policies/{polAssoId}
#[tracing::instrument(skip_all)]
async fn delete_individual_am_policy_association<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteIndividualAmPolicyAssociationPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_am_policy_association_document::IndividualAmPolicyAssociationDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		delete_individual_am_policy_association_validation(path_params)
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
		.delete_individual_am_policy_association(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status307
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
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status308
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
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status400
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
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status401
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
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status403
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
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status404
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
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status429
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
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status500
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
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Status503
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
			apis::individual_am_policy_association_document::DeleteIndividualAmPolicyAssociationResponse::Statusdefault
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
fn read_individual_am_policy_association_validation(
	path_params: models::ReadIndividualAmPolicyAssociationPathParams
) -> std::result::Result<(models::ReadIndividualAmPolicyAssociationPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// ReadIndividualAmPolicyAssociation - GET
/// /npcf-am-policy-control/v1/policies/{polAssoId}
#[tracing::instrument(skip_all)]
async fn read_individual_am_policy_association<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ReadIndividualAmPolicyAssociationPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_am_policy_association_document::IndividualAmPolicyAssociationDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		read_individual_am_policy_association_validation(path_params)
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
		.read_individual_am_policy_association(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status200
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status307
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status308
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status400
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status401
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status403
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status404
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status406
			=> {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status429
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status500
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Status503
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
			apis::individual_am_policy_association_document::ReadIndividualAmPolicyAssociationResponse::Statusdefault
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
struct ReportObservedEventTriggersForIndividualAmPolicyAssociationBodyValidator<'a> {
	body: &'a models::PolicyAssociationUpdateRequest,
}

#[tracing::instrument(skip_all)]
fn report_observed_event_triggers_for_individual_am_policy_association_validation(
	path_params: models::ReportObservedEventTriggersForIndividualAmPolicyAssociationPathParams,
	body: models::PolicyAssociationUpdateRequest,
) -> std::result::Result<
	(
		models::ReportObservedEventTriggersForIndividualAmPolicyAssociationPathParams,
		models::PolicyAssociationUpdateRequest,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b =
		ReportObservedEventTriggersForIndividualAmPolicyAssociationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ReportObservedEventTriggersForIndividualAmPolicyAssociation - POST
/// /npcf-am-policy-control/v1/policies/{polAssoId}/update
#[tracing::instrument(skip_all)]
async fn report_observed_event_triggers_for_individual_am_policy_association<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<
		models::ReportObservedEventTriggersForIndividualAmPolicyAssociationPathParams,
	>,
	State(api_impl): State<I>,
	Json(body): Json<models::PolicyAssociationUpdateRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_am_policy_association_document::IndividualAmPolicyAssociationDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		report_observed_event_triggers_for_individual_am_policy_association_validation(
			path_params,
			body,
		)
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
		.report_observed_event_triggers_for_individual_am_policy_association(
			method,
			host,
			cookies,
			path_params,
			body,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status200
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status307
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status308
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status400
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status401
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status403
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status404
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status411
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status413
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status415
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status429
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status500
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Status503
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
			apis::individual_am_policy_association_document::ReportObservedEventTriggersForIndividualAmPolicyAssociationResponse::Statusdefault
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
fn delete_app_am_context_validation(
	path_params: models::DeleteAppAmContextPathParams
) -> std::result::Result<(models::DeleteAppAmContextPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeleteAppAmContext - DELETE
/// /npcf-am-policyauthorization/v1/app-am-contexts/{appAmContextId}
#[tracing::instrument(skip_all)]
async fn delete_app_am_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteAppAmContextPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_application_am_context_document::IndividualApplicationAmContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || delete_app_am_context_validation(path_params))
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
		.delete_app_am_context(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status307
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
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status308
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
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status400
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
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status401
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
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status403
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
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status404
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
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status429
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
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status500
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
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Status503
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
			apis::individual_application_am_context_document::DeleteAppAmContextResponse::Statusdefault
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
fn get_app_am_context_validation(
	path_params: models::GetAppAmContextPathParams
) -> std::result::Result<(models::GetAppAmContextPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// GetAppAmContext - GET
/// /npcf-am-policyauthorization/v1/app-am-contexts/{appAmContextId}
#[tracing::instrument(skip_all)]
async fn get_app_am_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetAppAmContextPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_application_am_context_document::IndividualApplicationAmContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || get_app_am_context_validation(path_params))
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
		.get_app_am_context(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status200
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status307
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status308
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status400
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status401
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status403
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status404
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status406
			=> {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status429
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status500
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Status503
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
			apis::individual_application_am_context_document::GetAppAmContextResponse::Statusdefault
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
struct ModAppAmContextBodyValidator<'a> {
	body: &'a models::AppAmContextUpdateData,
}

#[tracing::instrument(skip_all)]
fn mod_app_am_context_validation(
	path_params: models::ModAppAmContextPathParams,
	body: models::AppAmContextUpdateData,
) -> std::result::Result<
	(
		models::ModAppAmContextPathParams,
		models::AppAmContextUpdateData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ModAppAmContextBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ModAppAmContext - PATCH
/// /npcf-am-policyauthorization/v1/app-am-contexts/{appAmContextId}
#[tracing::instrument(skip_all)]
async fn mod_app_am_context<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ModAppAmContextPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AppAmContextUpdateData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_application_am_context_document::IndividualApplicationAmContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || mod_app_am_context_validation(path_params, body))
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
		.mod_app_am_context(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status200
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status307
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status308
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status400
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status401
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status403
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status404
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status411
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status413
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status415
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status429
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status500
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Status503
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
			apis::individual_application_am_context_document::ModAppAmContextResponse::Statusdefault
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
struct DeleteAppSessionBodyValidator<'a> {
	body: &'a models::EventsSubscReqData,
}

#[tracing::instrument(skip_all)]
fn delete_app_session_validation(
	path_params: models::DeleteAppSessionPathParams,
	body: Option<models::EventsSubscReqData>,
) -> std::result::Result<
	(
		models::DeleteAppSessionPathParams,
		Option<models::EventsSubscReqData>,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	if let Some(body) = &body {
		let b = DeleteAppSessionBodyValidator { body };
		b.validate()?;
	}

	Ok((path_params, body))
}

/// DeleteAppSession - POST
/// /npcf-policyauthorization/v1/app-sessions/{appSessionId}/delete
#[tracing::instrument(skip_all)]
async fn delete_app_session<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteAppSessionPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::EventsSubscReqData>>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_application_session_context_document::IndividualApplicationSessionContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || delete_app_session_validation(path_params, body))
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
		.delete_app_session(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status200
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status307
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status308
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status400
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status401
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status403
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status404
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status411
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status413
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status415
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status429
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status500
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Status503
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
			apis::individual_application_session_context_document::DeleteAppSessionResponse::Statusdefault
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
fn get_app_session_validation(
	path_params: models::GetAppSessionPathParams
) -> std::result::Result<(models::GetAppSessionPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// GetAppSession - GET /npcf-policyauthorization/v1/app-sessions/{appSessionId}
#[tracing::instrument(skip_all)]
async fn get_app_session<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetAppSessionPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_application_session_context_document::IndividualApplicationSessionContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || get_app_session_validation(path_params))
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
		.get_app_session(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_application_session_context_document::GetAppSessionResponse::Status200
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Status307
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Status308
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Status400
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Status401
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Status403
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Status404
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Status406
			=> {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_application_session_context_document::GetAppSessionResponse::Status429
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Status500
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Status503
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
			apis::individual_application_session_context_document::GetAppSessionResponse::Statusdefault
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
struct ModAppSessionBodyValidator<'a> {
	body: &'a models::AppSessionContextUpdateDataPatch,
}

#[tracing::instrument(skip_all)]
fn mod_app_session_validation(
	path_params: models::ModAppSessionPathParams,
	body: models::AppSessionContextUpdateDataPatch,
) -> std::result::Result<
	(
		models::ModAppSessionPathParams,
		models::AppSessionContextUpdateDataPatch,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ModAppSessionBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ModAppSession - PATCH
/// /npcf-policyauthorization/v1/app-sessions/{appSessionId}
#[tracing::instrument(skip_all)]
async fn mod_app_session<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ModAppSessionPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AppSessionContextUpdateDataPatch>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_application_session_context_document::IndividualApplicationSessionContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || mod_app_session_validation(path_params, body))
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
		.mod_app_session(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_application_session_context_document::ModAppSessionResponse::Status200
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_application_session_context_document::ModAppSessionResponse::Status307
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status308
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status400
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status401
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status403
			{
				body,
				retry_after
			}
			=> {
				if let Some(retry_after) = retry_after {
					let retry_after = match header::IntoHeaderValue(retry_after).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling retry_after header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							retry_after,
						);
					}
				}
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status404
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status411
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status413
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status415
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status429
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status500
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Status503
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
			apis::individual_application_session_context_document::ModAppSessionResponse::Statusdefault
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
fn get_bdt_policy_validation(
	path_params: models::GetBdtPolicyPathParams
) -> std::result::Result<(models::GetBdtPolicyPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// GetBdtPolicy - GET /npcf-bdtpolicycontrol/v1/bdtpolicies/{bdtPolicyId}
#[tracing::instrument(skip_all)]
async fn get_bdt_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetBdtPolicyPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_bdt_policy_document::IndividualBdtPolicyDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || get_bdt_policy_validation(path_params))
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
		.get_bdt_policy(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status200(body) => {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status307 {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status308 {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status400(body) => {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status401(body) => {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status403(body) => {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status404(body) => {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status429(body) => {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status500(body) => {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Status503(body) => {
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
			apis::individual_bdt_policy_document::GetBdtPolicyResponse::Statusdefault => {
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
struct UpdateBdtPolicyBodyValidator<'a> {
	body: &'a models::PatchBdtPolicy,
}

#[tracing::instrument(skip_all)]
fn update_bdt_policy_validation(
	path_params: models::UpdateBdtPolicyPathParams,
	body: models::PatchBdtPolicy,
) -> std::result::Result<
	(models::UpdateBdtPolicyPathParams, models::PatchBdtPolicy),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = UpdateBdtPolicyBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// UpdateBdtPolicy - PATCH /npcf-bdtpolicycontrol/v1/bdtpolicies/{bdtPolicyId}
#[tracing::instrument(skip_all)]
async fn update_bdt_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateBdtPolicyPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::PatchBdtPolicy>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_bdt_policy_document::IndividualBdtPolicyDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || update_bdt_policy_validation(path_params, body))
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
		.update_bdt_policy(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status200(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status307 {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status308 {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status400(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status401(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status403(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status404(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status411(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status413(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status415(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status429(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status500(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Status503(body) => {
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
			apis::individual_bdt_policy_document::UpdateBdtPolicyResponse::Statusdefault => {
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
fn delete_mbs_app_session_ctxt_validation(
	path_params: models::DeleteMbsAppSessionCtxtPathParams
) -> std::result::Result<(models::DeleteMbsAppSessionCtxtPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeleteMbsAppSessionCtxt - DELETE /npcf-mbspolicyauth/v1/contexts/{contextId}
#[tracing::instrument(skip_all)]
async fn delete_mbs_app_session_ctxt<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteMbsAppSessionCtxtPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_mbs_application_session_context_document::IndividualMbsApplicationSessionContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || delete_mbs_app_session_ctxt_validation(path_params))
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
		.delete_mbs_app_session_ctxt(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status307
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
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status308
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
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status400
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
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status401
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
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status403
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
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status404
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
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status406
			=> {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status429
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
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status500
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
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Status503
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
			apis::individual_mbs_application_session_context_document::DeleteMbsAppSessionCtxtResponse::Statusdefault
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
fn get_mbs_app_session_ctxt_validation(
	path_params: models::GetMbsAppSessionCtxtPathParams
) -> std::result::Result<(models::GetMbsAppSessionCtxtPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// GetMbsAppSessionCtxt - GET /npcf-mbspolicyauth/v1/contexts/{contextId}
#[tracing::instrument(skip_all)]
async fn get_mbs_app_session_ctxt<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetMbsAppSessionCtxtPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_mbs_application_session_context_document::IndividualMbsApplicationSessionContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || get_mbs_app_session_ctxt_validation(path_params))
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
		.get_mbs_app_session_ctxt(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status200
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status307
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status308
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status400
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status401
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status403
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status404
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status406
			=> {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status429
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status500
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Status503
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
			apis::individual_mbs_application_session_context_document::GetMbsAppSessionCtxtResponse::Statusdefault
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
struct ModifyMbsAppSessionCtxtBodyValidator<'a> {
	body: &'a models::MbsAppSessionCtxtPatch,
}

#[tracing::instrument(skip_all)]
fn modify_mbs_app_session_ctxt_validation(
	path_params: models::ModifyMbsAppSessionCtxtPathParams,
	body: models::MbsAppSessionCtxtPatch,
) -> std::result::Result<
	(
		models::ModifyMbsAppSessionCtxtPathParams,
		models::MbsAppSessionCtxtPatch,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ModifyMbsAppSessionCtxtBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ModifyMbsAppSessionCtxt - PATCH /npcf-mbspolicyauth/v1/contexts/{contextId}
#[tracing::instrument(skip_all)]
async fn modify_mbs_app_session_ctxt<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ModifyMbsAppSessionCtxtPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::MbsAppSessionCtxtPatch>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_mbs_application_session_context_document::IndividualMbsApplicationSessionContextDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		modify_mbs_app_session_ctxt_validation(path_params, body)
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
		.modify_mbs_app_session_ctxt(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status200
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status307
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status308
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status400
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status401
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status403
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status404
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status406
			=> {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status429
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status500
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Status503
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
			apis::individual_mbs_application_session_context_document::ModifyMbsAppSessionCtxtResponse::Statusdefault
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
fn delete_ind_mbs_policy_validation(
	path_params: models::DeleteIndMbsPolicyPathParams
) -> std::result::Result<(models::DeleteIndMbsPolicyPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeleteIndMbsPolicy - DELETE
/// /npcf-mbspolicycontrol/v1/mbs-policies/{mbsPolicyId}
#[tracing::instrument(skip_all)]
async fn delete_ind_mbs_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteIndMbsPolicyPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_mbs_policy_document::IndividualMbsPolicyDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || delete_ind_mbs_policy_validation(path_params))
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
		.delete_ind_mbs_policy(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status307 {
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
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status308 {
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
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status400(body) => {
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
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status401(body) => {
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
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status403(body) => {
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
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status404(body) => {
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
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status429(body) => {
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
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status500(body) => {
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
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Status503(body) => {
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
			apis::individual_mbs_policy_document::DeleteIndMbsPolicyResponse::Statusdefault => {
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
fn get_ind_mbs_policy_validation(
	path_params: models::GetIndMbsPolicyPathParams
) -> std::result::Result<(models::GetIndMbsPolicyPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// GetIndMbsPolicy - GET /npcf-mbspolicycontrol/v1/mbs-policies/{mbsPolicyId}
#[tracing::instrument(skip_all)]
async fn get_ind_mbs_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetIndMbsPolicyPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_mbs_policy_document::IndividualMbsPolicyDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || get_ind_mbs_policy_validation(path_params))
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
		.get_ind_mbs_policy(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status200(body) => {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status307 {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status308 {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status400(body) => {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status401(body) => {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status403(body) => {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status404(body) => {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status429(body) => {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status500(body) => {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Status503(body) => {
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
			apis::individual_mbs_policy_document::GetIndMbsPolicyResponse::Statusdefault => {
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
struct UpdateIndMbsPolicyBodyValidator<'a> {
	body: &'a models::MbsPolicyCtxtDataUpdate,
}

#[tracing::instrument(skip_all)]
fn update_ind_mbs_policy_validation(
	path_params: models::UpdateIndMbsPolicyPathParams,
	body: models::MbsPolicyCtxtDataUpdate,
) -> std::result::Result<
	(
		models::UpdateIndMbsPolicyPathParams,
		models::MbsPolicyCtxtDataUpdate,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = UpdateIndMbsPolicyBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// UpdateIndMbsPolicy - POST
/// /npcf-mbspolicycontrol/v1/mbs-policies/{mbsPolicyId}/update
#[tracing::instrument(skip_all)]
async fn update_ind_mbs_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateIndMbsPolicyPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::MbsPolicyCtxtDataUpdate>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_mbs_policy_document::IndividualMbsPolicyDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || update_ind_mbs_policy_validation(path_params, body))
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
		.update_ind_mbs_policy(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status200(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status307 {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status308 {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status400(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status401(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status403(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status404(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status411(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status413(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status415(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status429(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status500(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Status503(body) => {
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
			apis::individual_mbs_policy_document::UpdateIndMbsPolicyResponse::Statusdefault => {
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
fn delete_pc_event_exposure_subsc_validation(
	path_params: models::DeletePcEventExposureSubscPathParams
) -> std::result::Result<(models::DeletePcEventExposureSubscPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeletePcEventExposureSubsc - DELETE
/// /npcf-eventexposure/v1/subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn delete_pc_event_exposure_subsc<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeletePcEventExposureSubscPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_policy_control_events_subscription_document::IndividualPolicyControlEventsSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || delete_pc_event_exposure_subsc_validation(path_params))
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
		.delete_pc_event_exposure_subsc(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status307
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
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status308
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
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status400
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
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status401
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
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status403
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
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status404
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
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status429
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
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status500
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
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Status503
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
			apis::individual_policy_control_events_subscription_document::DeletePcEventExposureSubscResponse::Statusdefault
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
fn get_pc_event_exposure_subsc_validation(
	path_params: models::GetPcEventExposureSubscPathParams
) -> std::result::Result<(models::GetPcEventExposureSubscPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// GetPcEventExposureSubsc - GET
/// /npcf-eventexposure/v1/subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn get_pc_event_exposure_subsc<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetPcEventExposureSubscPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_policy_control_events_subscription_document::IndividualPolicyControlEventsSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || get_pc_event_exposure_subsc_validation(path_params))
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
		.get_pc_event_exposure_subsc(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status200
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status307
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status308
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status400
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status401
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status403
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status404
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status406
			=> {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status429
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status500
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Status503
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
			apis::individual_policy_control_events_subscription_document::GetPcEventExposureSubscResponse::Statusdefault
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
struct PutPcEventExposureSubscBodyValidator<'a> {
	body: &'a models::PcEventExposureSubsc,
}

#[tracing::instrument(skip_all)]
fn put_pc_event_exposure_subsc_validation(
	path_params: models::PutPcEventExposureSubscPathParams,
	body: models::PcEventExposureSubsc,
) -> std::result::Result<
	(
		models::PutPcEventExposureSubscPathParams,
		models::PcEventExposureSubsc,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = PutPcEventExposureSubscBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// PutPcEventExposureSubsc - PUT
/// /npcf-eventexposure/v1/subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn put_pc_event_exposure_subsc<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::PutPcEventExposureSubscPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::PcEventExposureSubsc>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::individual_policy_control_events_subscription_document::IndividualPolicyControlEventsSubscriptionDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		put_pc_event_exposure_subsc_validation(path_params, body)
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
		.put_pc_event_exposure_subsc(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status200
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status307
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status308
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status400
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status401
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status403
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status404
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status411
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status413
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status415
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status429
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status500
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Status503
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
			apis::individual_policy_control_events_subscription_document::PutPcEventExposureSubscResponse::Statusdefault
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
struct DeleteSmPolicyBodyValidator<'a> {
	body: &'a models::SmPolicyDeleteData,
}

#[tracing::instrument(skip_all)]
fn delete_sm_policy_validation(
	path_params: models::DeleteSmPolicyPathParams,
	body: models::SmPolicyDeleteData,
) -> std::result::Result<
	(models::DeleteSmPolicyPathParams, models::SmPolicyDeleteData),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = DeleteSmPolicyBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// DeleteSmPolicy - POST
/// /npcf-smpolicycontrol/v1/sm-policies/{smPolicyId}/delete
#[tracing::instrument(skip_all)]
async fn delete_sm_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteSmPolicyPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SmPolicyDeleteData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_sm_policy_document::IndividualSmPolicyDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || delete_sm_policy_validation(path_params, body))
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
		.delete_sm_policy(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status307 {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status308 {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status400(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status401(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status403(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status404(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status411(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status413(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status415(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status429(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status500(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Status503(body) => {
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
			apis::individual_sm_policy_document::DeleteSmPolicyResponse::Statusdefault => {
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
fn get_sm_policy_validation(
	path_params: models::GetSmPolicyPathParams
) -> std::result::Result<(models::GetSmPolicyPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// GetSmPolicy - GET /npcf-smpolicycontrol/v1/sm-policies/{smPolicyId}
#[tracing::instrument(skip_all)]
async fn get_sm_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetSmPolicyPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_sm_policy_document::IndividualSmPolicyDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || get_sm_policy_validation(path_params))
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
		.get_sm_policy(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status200(body) => {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status307 {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status308 {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status400(body) => {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status401(body) => {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status403(body) => {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status404(body) => {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status429(body) => {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status500(body) => {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Status503(body) => {
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
			apis::individual_sm_policy_document::GetSmPolicyResponse::Statusdefault => {
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
struct UpdateSmPolicyBodyValidator<'a> {
	body: &'a models::SmPolicyUpdateContextData,
}

#[tracing::instrument(skip_all)]
fn update_sm_policy_validation(
	path_params: models::UpdateSmPolicyPathParams,
	body: models::SmPolicyUpdateContextData,
) -> std::result::Result<
	(
		models::UpdateSmPolicyPathParams,
		models::SmPolicyUpdateContextData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = UpdateSmPolicyBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// UpdateSmPolicy - POST
/// /npcf-smpolicycontrol/v1/sm-policies/{smPolicyId}/update
#[tracing::instrument(skip_all)]
async fn update_sm_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateSmPolicyPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SmPolicyUpdateContextData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_sm_policy_document::IndividualSmPolicyDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || update_sm_policy_validation(path_params, body))
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
		.update_sm_policy(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status200(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status307 {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status308 {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status400(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status401(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status403(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status404(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status411(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status413(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status415(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status429(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status500(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Status503(body) => {
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
			apis::individual_sm_policy_document::UpdateSmPolicyResponse::Statusdefault => {
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
fn delete_individual_ue_policy_association_validation(
	path_params: models::DeleteIndividualUePolicyAssociationPathParams
) -> std::result::Result<(models::DeleteIndividualUePolicyAssociationPathParams,), ValidationErrors>
{
	path_params.validate()?;

	Ok((path_params,))
}

/// DeleteIndividualUePolicyAssociation - DELETE
/// /npcf-ue-policy-control/v1/policies/{polAssoId}
#[tracing::instrument(skip_all)]
async fn delete_individual_ue_policy_association<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteIndividualUePolicyAssociationPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_policy_association_document::IndividualUePolicyAssociationDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		delete_individual_ue_policy_association_validation(path_params)
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
		.delete_individual_ue_policy_association(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status307
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
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status308
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
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status400
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
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status401
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
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status403
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
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status404
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
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status429
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
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status500
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
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Status503
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
			apis::individual_ue_policy_association_document::DeleteIndividualUePolicyAssociationResponse::Statusdefault
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
fn read_individual_ue_policy_association_validation(
	path_params: models::ReadIndividualUePolicyAssociationPathParams
) -> std::result::Result<(models::ReadIndividualUePolicyAssociationPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// ReadIndividualUePolicyAssociation - GET
/// /npcf-ue-policy-control/v1/policies/{polAssoId}
#[tracing::instrument(skip_all)]
async fn read_individual_ue_policy_association<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ReadIndividualUePolicyAssociationPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_policy_association_document::IndividualUePolicyAssociationDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		read_individual_ue_policy_association_validation(path_params)
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
		.read_individual_ue_policy_association(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status200
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status307
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status308
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status400
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status401
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status403
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status404
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status406
			=> {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status429
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status500
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Status503
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
			apis::individual_ue_policy_association_document::ReadIndividualUePolicyAssociationResponse::Statusdefault
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
struct ReportObservedEventTriggersForIndividualUePolicyAssociationBodyValidator<'a> {
	body: &'a models::PolicyAssociationUpdateRequest1,
}

#[tracing::instrument(skip_all)]
fn report_observed_event_triggers_for_individual_ue_policy_association_validation(
	path_params: models::ReportObservedEventTriggersForIndividualUePolicyAssociationPathParams,
	body: models::PolicyAssociationUpdateRequest1,
) -> std::result::Result<
	(
		models::ReportObservedEventTriggersForIndividualUePolicyAssociationPathParams,
		models::PolicyAssociationUpdateRequest1,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b =
		ReportObservedEventTriggersForIndividualUePolicyAssociationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ReportObservedEventTriggersForIndividualUePolicyAssociation - POST
/// /npcf-ue-policy-control/v1/policies/{polAssoId}/update
#[tracing::instrument(skip_all)]
async fn report_observed_event_triggers_for_individual_ue_policy_association<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<
		models::ReportObservedEventTriggersForIndividualUePolicyAssociationPathParams,
	>,
	State(api_impl): State<I>,
	Json(body): Json<models::PolicyAssociationUpdateRequest1>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::individual_ue_policy_association_document::IndividualUePolicyAssociationDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		report_observed_event_triggers_for_individual_ue_policy_association_validation(
			path_params,
			body,
		)
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
		.report_observed_event_triggers_for_individual_ue_policy_association(
			method,
			host,
			cookies,
			path_params,
			body,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status200
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status307
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status308
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status400
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status401
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status403
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status404
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status411
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status413
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status415
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status429
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status500
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Status503
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
			apis::individual_ue_policy_association_document::ReportObservedEventTriggersForIndividualUePolicyAssociationResponse::Statusdefault
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
struct CreateMbsAppSessionCtxtBodyValidator<'a> {
	body: &'a models::MbsAppSessionCtxt,
}

#[tracing::instrument(skip_all)]
fn create_mbs_app_session_ctxt_validation(
	body: models::MbsAppSessionCtxt
) -> std::result::Result<(models::MbsAppSessionCtxt,), ValidationErrors> {
	let b = CreateMbsAppSessionCtxtBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// CreateMbsAppSessionCtxt - POST /npcf-mbspolicyauth/v1/contexts
#[tracing::instrument(skip_all)]
async fn create_mbs_app_session_ctxt<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::MbsAppSessionCtxt>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::mbs_application_session_contexts_collection::MbsApplicationSessionContextsCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || create_mbs_app_session_ctxt_validation(body))
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
		.create_mbs_app_session_ctxt(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status201
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status400
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status401
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status403
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status404
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status411
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status413
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status415
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status429
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status500
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Status503
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
			apis::mbs_application_session_contexts_collection::CreateMbsAppSessionCtxtResponse::Statusdefault
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
struct CreateMbsPolicyBodyValidator<'a> {
	body: &'a models::MbsPolicyCtxtData,
}

#[tracing::instrument(skip_all)]
fn create_mbs_policy_validation(
	body: models::MbsPolicyCtxtData
) -> std::result::Result<(models::MbsPolicyCtxtData,), ValidationErrors> {
	let b = CreateMbsPolicyBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// CreateMbsPolicy - POST /npcf-mbspolicycontrol/v1/mbs-policies
#[tracing::instrument(skip_all)]
async fn create_mbs_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::MbsPolicyCtxtData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::mbs_policies_collection::MbsPoliciesCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || create_mbs_policy_validation(body))
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
		.create_mbs_policy(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status201 {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status308 {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status400(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status401(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status403(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status404(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status411(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status413(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status415(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status429(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status500(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Status503(body) => {
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
			apis::mbs_policies_collection::CreateMbsPolicyResponse::Statusdefault => {
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
struct PcscfRestorationBodyValidator<'a> {
	body: &'a models::PcscfRestorationRequestData,
}

#[tracing::instrument(skip_all)]
fn pcscf_restoration_validation(
	body: models::PcscfRestorationRequestData
) -> std::result::Result<(models::PcscfRestorationRequestData,), ValidationErrors> {
	let b = PcscfRestorationBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// PcscfRestoration - POST
/// /npcf-policyauthorization/v1/app-sessions/pcscf-restoration
#[tracing::instrument(skip_all)]
async fn pcscf_restoration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::PcscfRestorationRequestData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::pcscf_restoration_indication::PcscfRestorationIndication,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || pcscf_restoration_validation(body))
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
		.pcscf_restoration(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status307 {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status308 {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status400(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status401(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status403(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status404(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status411(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status413(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status415(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status429(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status500(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Status503(body) => {
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
			apis::pcscf_restoration_indication::PcscfRestorationResponse::Statusdefault => {
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
struct PostPcEventExposureSubscBodyValidator<'a> {
	body: &'a models::PcEventExposureSubsc,
}

#[tracing::instrument(skip_all)]
fn post_pc_event_exposure_subsc_validation(
	body: models::PcEventExposureSubsc
) -> std::result::Result<(models::PcEventExposureSubsc,), ValidationErrors> {
	let b = PostPcEventExposureSubscBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// PostPcEventExposureSubsc - POST /npcf-eventexposure/v1/subscriptions
#[tracing::instrument(skip_all)]
async fn post_pc_event_exposure_subsc<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::PcEventExposureSubsc>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::policy_control_events_subscription_collection::PolicyControlEventsSubscriptionCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || post_pc_event_exposure_subsc_validation(body))
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
		.post_pc_event_exposure_subsc(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status201
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status400
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status401
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status403
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status404
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status411
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status413
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status415
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status429
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status500
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Status503
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
			apis::policy_control_events_subscription_collection::PostPcEventExposureSubscResponse::Statusdefault
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
struct CreateSmPolicyBodyValidator<'a> {
	body: &'a models::SmPolicyContextData,
}

#[tracing::instrument(skip_all)]
fn create_sm_policy_validation(
	body: models::SmPolicyContextData
) -> std::result::Result<(models::SmPolicyContextData,), ValidationErrors> {
	let b = CreateSmPolicyBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// CreateSmPolicy - POST /npcf-smpolicycontrol/v1/sm-policies
#[tracing::instrument(skip_all)]
async fn create_sm_policy<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::SmPolicyContextData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::sm_policies_collection::SmPoliciesCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || create_sm_policy_validation(body))
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
		.create_sm_policy(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::sm_policies_collection::CreateSmPolicyResponse::Status201 { body, location } => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status308 { location } => {
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
				response.body(Body::empty())
			}
			apis::sm_policies_collection::CreateSmPolicyResponse::Status400(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status401(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status403(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status404(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status411(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status413(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status415(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status429(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status500(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Status503(body) => {
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
			apis::sm_policies_collection::CreateSmPolicyResponse::Statusdefault => {
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
struct CreateIndividualUePolicyAssociationBodyValidator<'a> {
	body: &'a models::PolicyAssociationRequest1,
}

#[tracing::instrument(skip_all)]
fn create_individual_ue_policy_association_validation(
	body: models::PolicyAssociationRequest1
) -> std::result::Result<(models::PolicyAssociationRequest1,), ValidationErrors> {
	let b = CreateIndividualUePolicyAssociationBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// CreateIndividualUePolicyAssociation - POST
/// /npcf-ue-policy-control/v1/policies
#[tracing::instrument(skip_all)]
async fn create_individual_ue_policy_association<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::PolicyAssociationRequest1>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ue_policy_associations_collection::UePolicyAssociationsCollection,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		create_individual_ue_policy_association_validation(body)
	})
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
		.create_individual_ue_policy_association(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status201
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status400
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status401
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status403
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status404
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status411
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status413
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status415
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status429
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status500
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Status503
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
			apis::ue_policy_associations_collection::CreateIndividualUePolicyAssociationResponse::Statusdefault
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
