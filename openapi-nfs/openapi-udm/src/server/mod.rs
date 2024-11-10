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
								  A: apis::amf3_gpp_access_registration_info_retrieval::Amf3GppAccessRegistrationInfoRetrieval + apis::amf_non3_gpp_access_registration_info_retrieval::AmfNon3GppAccessRegistrationInfoRetrieval + apis::amf_registration_for3_gpp_access::AmfRegistrationFor3GppAccess + apis::amf_registration_for_non3_gpp_access::AmfRegistrationForNon3GppAccess + apis::access_and_mobility_subscription_data_retrieval::AccessAndMobilitySubscriptionDataRetrieval + apis::authorize_the_nidd_configuration_request::AuthorizeTheNiddConfigurationRequest + apis::class5_gmbs_group_creation::Class5GmbsGroupCreation + apis::class5_gmbs_group_deletion::Class5GmbsGroupDeletion + apis::class5_gmbs_group_modification::Class5GmbsGroupModification + apis::class5_gvn_group_creation::Class5GvnGroupCreation + apis::class5_gvn_group_deletion::Class5GvnGroupDeletion + apis::class5_gvn_group_modification::Class5GvnGroupModification + apis::class5_mbs_subscription_data_retrieval::Class5MbsSubscriptionDataRetrieval + apis::confirm_auth::ConfirmAuth + apis::create_ee_subscription::CreateEeSubscription + apis::deconceal::Deconceal + apis::delete_auth::DeleteAuth + apis::delete_ee_subscription::DeleteEeSubscription + apis::enhanced_coverage_restriction_data_retrieval::EnhancedCoverageRestrictionDataRetrieval + apis::gpsito_supi_translation_or_supito_gpsi_translation::GpsitoSupiTranslationOrSupitoGpsiTranslation + apis::generate_auth_data::GenerateAuthData + apis::generate_gba_authentication_vectors::GenerateGbaAuthenticationVectors + apis::generate_hss_authentication_vectors::GenerateHssAuthenticationVectors + apis::generate_pro_se_authentication_vectors::GenerateProSeAuthenticationVectors + apis::get_auth_data_for_fnrg::GetAuthDataForFnrg + apis::group_identifiers::GroupIdentifiers + apis::ipsmgw_deregistration::IpsmgwDeregistration + apis::ipsmgw_registration::IpsmgwRegistration + apis::ipsmgw_registration_info_retrieval::IpsmgwRegistrationInfoRetrieval + apis::lcs_broadcast_assistance_data_types_retrieval::LcsBroadcastAssistanceDataTypesRetrieval + apis::lcs_mobile_originated_data_retrieval::LcsMobileOriginatedDataRetrieval + apis::lcs_privacy_data_retrieval::LcsPrivacyDataRetrieval + apis::multiple_identifiers::MultipleIdentifiers + apis::nwdaf_deregistration::NwdafDeregistration + apis::nwdaf_registration::NwdafRegistration + apis::nwdaf_registration_info_retrieval::NwdafRegistrationInfoRetrieval + apis::pei_update::PeiUpdate + apis::parameter_provisioning_data_entry_document::ParameterProvisioningDataEntryDocument + apis::parameter_update_in_the_amf_registration_for3_gpp_access::ParameterUpdateInTheAmfRegistrationFor3GppAccess + apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::ParameterUpdateInTheAmfRegistrationForNon3GppAccess + apis::parameter_update_in_the_nwdaf_registration::ParameterUpdateInTheNwdafRegistration + apis::parameter_update_in_the_smf_registration::ParameterUpdateInTheSmfRegistration + apis::prose_subscription_data_retrieval::ProseSubscriptionDataRetrieval + apis::provide_ue_location::ProvideUeLocation + apis::providing_acknowledgement_of_cag_update::ProvidingAcknowledgementOfCagUpdate + apis::providing_acknowledgement_of_snssais_update::ProvidingAcknowledgementOfSnssaisUpdate + apis::providing_acknowledgement_of_steering_of_roaming::ProvidingAcknowledgementOfSteeringOfRoaming + apis::providing_acknowledgement_of_ue_parameters_update::ProvidingAcknowledgementOfUeParametersUpdate + apis::query_ue_info::QueryUeInfo + apis::report_sm_delivery_status::ReportSmDeliveryStatus + apis::retrieval_of_multiple_data_sets::RetrievalOfMultipleDataSets + apis::retrieval_of_shared_data::RetrievalOfSharedData + apis::retrieval_of_the_individual_shared_data::RetrievalOfTheIndividualSharedData + apis::retrieve_smf_registration::RetrieveSmfRegistration + apis::roaming_information_update::RoamingInformationUpdate + apis::smf_deregistration::SmfDeregistration + apis::smf_selection_subscription_data_retrieval::SmfSelectionSubscriptionDataRetrieval + apis::smf_smf_registration::SmfSmfRegistration + apis::smsf3_gpp_access_registration_info_retrieval::Smsf3GppAccessRegistrationInfoRetrieval + apis::smsf_deregistration_for3_gpp_access::SmsfDeregistrationFor3GppAccess + apis::smsf_deregistration_for_non3_gpp_access::SmsfDeregistrationForNon3GppAccess + apis::smsf_non3_gpp_access_registration_info_retrieval::SmsfNon3GppAccessRegistrationInfoRetrieval + apis::smsf_registration_for3_gpp_access::SmsfRegistrationFor3GppAccess + apis::smsf_registration_for_non3_gpp_access::SmsfRegistrationForNon3GppAccess + apis::sms_management_subscription_data_retrieval::SmsManagementSubscriptionDataRetrieval + apis::sms_subscription_data_retrieval::SmsSubscriptionDataRetrieval + apis::send_routing_info_sm_custom_operation::SendRoutingInfoSmCustomOperation + apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemove + apis::service_specific_authorization_request::ServiceSpecificAuthorizationRequest + apis::session_management_subscription_data_retrieval::SessionManagementSubscriptionDataRetrieval + apis::slice_selection_subscription_data_retrieval::SliceSelectionSubscriptionDataRetrieval + apis::subscription_creation::SubscriptionCreation + apis::subscription_creation_for_shared_data::SubscriptionCreationForSharedData + apis::subscription_data_update::SubscriptionDataUpdate + apis::subscription_deletion::SubscriptionDeletion + apis::subscription_deletion_for_shared_data::SubscriptionDeletionForSharedData + apis::subscription_modification::SubscriptionModification + apis::trace_configuration_data_retrieval::TraceConfigurationDataRetrieval + apis::trigger_amf_for3_gpp_access_deregistration::TriggerAmfFor3GppAccessDeregistration + apis::trigger_pcscf_restoration::TriggerPcscfRestoration + apis::trigger_sor_info_update::TriggerSorInfoUpdate + apis::uecm_registration_info_retrieval::UecmRegistrationInfoRetrieval + apis::ue_context_in_amf_data_retrieval::UeContextInAmfDataRetrieval + apis::ue_context_in_smf_data_retrieval::UeContextInSmfDataRetrieval + apis::ue_context_in_smsf_data_retrieval::UeContextInSmsfDataRetrieval + apis::ue_location_information_retrieval::UeLocationInformationRetrieval + apis::update_ee_subscription::UpdateEeSubscription + apis::user_consent_subscription_data_retrieval::UserConsentSubscriptionDataRetrieval + apis::v2_x_subscription_data_retrieval::V2XSubscriptionDataRetrieval + 'static,
{
	// build our application with a route
	Router::new()
		.route(
			"/nudm-ee/v1/:ue_identity/ee-subscriptions",
			post(create_ee_subscription::<I, A>),
		)
		.route(
			"/nudm-ee/v1/:ue_identity/ee-subscriptions/:subscription_id",
			delete(delete_ee_subscription::<I, A>).patch(update_ee_subscription::<I, A>),
		)
		.route("/nudm-mt/v1/:supi", get(query_ue_info::<I, A>))
		.route(
			"/nudm-mt/v1/:supi/loc-info/provide-loc-info",
			post(provide_location_info::<I, A>),
		)
		.route(
			"/nudm-niddau/v1/:ue_identity/authorize",
			post(authorize_nidd_data::<I, A>),
		)
		.route(
			"/nudm-pp/v1/5g-vn-groups/:ext_group_id",
			delete(delete5_gvn_group::<I, A>)
				.get(get5_gvn_group::<I, A>)
				.patch(modify5_gvn_group::<I, A>)
				.put(create5_gvn_group::<I, A>),
		)
		.route("/nudm-pp/v1/:ue_id/pp-data", patch(update::<I, A>))
		.route(
			"/nudm-pp/v1/:ue_id/pp-data-store/:af_instance_id",
			delete(delete_pp_data_entry::<I, A>)
				.get(get_pp_data_entry::<I, A>)
				.put(create_pp_data_entry::<I, A>),
		)
		.route(
			"/nudm-pp/v1/mbs-group-membership/:ext_group_id",
			delete(delete5_g_mbs_group::<I, A>)
				.get(get5_g_mbs_group::<I, A>)
				.patch(modify5_g_mbs_group::<I, A>)
				.put(create5_g_mbs_group::<I, A>),
		)
		.route(
			"/nudm-rsds/v1/:ue_identity/sm-delivery-status",
			post(report_sm_delivery_status::<I, A>),
		)
		.route("/nudm-sdm/v2/:supi", get(get_data_sets::<I, A>))
		.route("/nudm-sdm/v2/:supi/5mbs-data", get(get_mbs_data::<I, A>))
		.route("/nudm-sdm/v2/:supi/am-data", get(get_am_data::<I, A>))
		.route("/nudm-sdm/v2/:supi/am-data/cag-ack", put(cag_ack::<I, A>))
		.route(
			"/nudm-sdm/v2/:supi/am-data/ecr-data",
			get(get_ecr_data::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:supi/am-data/sor-ack",
			put(sor_ack_info::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:supi/am-data/subscribed-snssais-ack",
			put(s_nssais_ack::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:supi/am-data/update-sor",
			post(update_sor_info::<I, A>),
		)
		.route("/nudm-sdm/v2/:supi/am-data/upu-ack", put(upu_ack::<I, A>))
		.route(
			"/nudm-sdm/v2/:supi/lcs-bca-data",
			get(get_lcs_bca_data::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:supi/lcs-mo-data",
			get(get_lcs_mo_data::<I, A>),
		)
		.route("/nudm-sdm/v2/:supi/nssai", get(get_nssai::<I, A>))
		.route("/nudm-sdm/v2/:supi/prose-data", get(get_prose_data::<I, A>))
		.route("/nudm-sdm/v2/:supi/sm-data", get(get_sm_data::<I, A>))
		.route(
			"/nudm-sdm/v2/:supi/smf-select-data",
			get(get_smf_sel_data::<I, A>),
		)
		.route("/nudm-sdm/v2/:supi/sms-data", get(get_sms_data::<I, A>))
		.route(
			"/nudm-sdm/v2/:supi/sms-mng-data",
			get(get_sms_mngt_data::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:supi/trace-data",
			get(get_trace_config_data::<I, A>),
		)
		.route("/nudm-sdm/v2/:supi/uc-data", get(get_uc_data::<I, A>))
		.route(
			"/nudm-sdm/v2/:supi/ue-context-in-amf-data",
			get(get_ue_ctx_in_amf_data::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:supi/ue-context-in-smf-data",
			get(get_ue_ctx_in_smf_data::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:supi/ue-context-in-smsf-data",
			get(get_ue_ctx_in_smsf_data::<I, A>),
		)
		.route("/nudm-sdm/v2/:supi/v2x-data", get(get_v2x_data::<I, A>))
		.route(
			"/nudm-sdm/v2/:ue_id/id-translation-result",
			get(get_supi_or_gpsi::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:ue_id/lcs-privacy-data",
			get(get_lcs_privacy_data::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:ue_id/sdm-subscriptions",
			post(subscribe::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/:ue_id/sdm-subscriptions/:subscription_id",
			delete(unsubscribe::<I, A>).patch(modify::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/group-data/group-identifiers",
			get(get_group_identifiers::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/multiple-identifiers",
			get(get_multiple_identifiers::<I, A>),
		)
		.route("/nudm-sdm/v2/shared-data", get(get_shared_data::<I, A>))
		.route(
			"/nudm-sdm/v2/shared-data-subscriptions",
			post(subscribe_to_shared_data::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/shared-data-subscriptions/:subscription_id",
			delete(unsubscribe_for_shared_data::<I, A>).patch(modify_shared_data_subs::<I, A>),
		)
		.route(
			"/nudm-sdm/v2/shared-data/:shared_data_id",
			get(get_individual_shared_data::<I, A>),
		)
		.route(
			"/nudm-ssau/v1/:ue_identity/:service_type/authorize",
			post(service_specific_authorization::<I, A>),
		)
		.route(
			"/nudm-ssau/v1/:ue_identity/:service_type/remove",
			post(service_specific_authorization_removal::<I, A>),
		)
		.route(
			"/nudm-ueau/v1/:supi/auth-events",
			post(confirm_auth::<I, A>),
		)
		.route(
			"/nudm-ueau/v1/:supi/auth-events/:auth_event_id",
			put(delete_auth::<I, A>),
		)
		.route(
			"/nudm-ueau/v1/:supi/gba-security-information/generate-av",
			post(generate_gba_av::<I, A>),
		)
		.route(
			"/nudm-ueau/v1/:supi/hss-security-information/:hss_auth_type/generate-av",
			post(generate_av::<I, A>),
		)
		.route(
			"/nudm-ueau/v1/:supi_or_suci/prose-security-information/generate-av",
			post(generate_prose_av::<I, A>),
		)
		.route(
			"/nudm-ueau/v1/:supi_or_suci/security-information-rg",
			get(get_rg_auth_data::<I, A>),
		)
		.route(
			"/nudm-ueau/v1/:supi_or_suci/security-information/generate-auth-data",
			post(generate_auth_data::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations",
			get(get_registrations::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/amf-3gpp-access",
			get(get3_gpp_registration::<I, A>)
				.patch(update3_gpp_registration::<I, A>)
				.put(call3_gpp_registration::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/amf-3gpp-access/dereg-amf",
			post(dereg_amf::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/amf-3gpp-access/pei-update",
			post(pei_update::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/amf-3gpp-access/roaming-info-update",
			post(update_roaming_information::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/amf-non-3gpp-access",
			get(get_non3_gpp_registration::<I, A>)
				.patch(update_non3_gpp_registration::<I, A>)
				.put(non3_gpp_registration::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/ip-sm-gw",
			delete(ip_sm_gw_deregistration::<I, A>)
				.get(get_ip_sm_gw_registration::<I, A>)
				.put(ip_sm_gw_registration::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/location",
			get(get_location_info::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/nwdaf-registrations",
			get(get_nwdaf_registration::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/nwdaf-registrations/:nwdaf_registration_id",
			delete(nwdaf_deregistration::<I, A>)
				.patch(update_nwdaf_registration::<I, A>)
				.put(nwdaf_registration::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/send-routing-info-sm",
			post(send_routing_info_sm::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/smf-registrations",
			get(get_smf_registration::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/smf-registrations/:pdu_session_id",
			delete(smf_deregistration::<I, A>)
				.get(retrieve_smf_registration::<I, A>)
				.patch(update_smf_registration::<I, A>)
				.put(registration::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/smsf-3gpp-access",
			delete(call3_gpp_smsf_deregistration::<I, A>)
				.get(get3_gpp_smsf_registration::<I, A>)
				.put(call3_gpp_smsf_registration::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/:ue_id/registrations/smsf-non-3gpp-access",
			delete(non3_gpp_smsf_deregistration::<I, A>)
				.get(get_non3_gpp_smsf_registration::<I, A>)
				.put(non3_gpp_smsf_registration::<I, A>),
		)
		.route(
			"/nudm-uecm/v1/restore-pcscf",
			post(trigger_pcscf_restoration::<I, A>),
		)
		.route("/nudm-ueid/v1/deconceal", post(deconceal::<I, A>))
		.with_state(api_impl)
}

#[tracing::instrument(skip_all)]
fn get3_gpp_registration_validation(
	path_params: models::Get3GppRegistrationPathParams,
	query_params: models::Get3GppRegistrationQueryParams,
) -> std::result::Result<
	(
		models::Get3GppRegistrationPathParams,
		models::Get3GppRegistrationQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// Get3GppRegistration - GET /nudm-uecm/v1/{ueId}/registrations/amf-3gpp-access
#[tracing::instrument(skip_all)]
async fn get3_gpp_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Get3GppRegistrationPathParams>,
	Query(query_params): Query<models::Get3GppRegistrationQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::amf3_gpp_access_registration_info_retrieval::Amf3GppAccessRegistrationInfoRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get3_gpp_registration_validation(path_params, query_params)
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
		.get3_gpp_registration(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::amf3_gpp_access_registration_info_retrieval::Get3GppRegistrationResponse::Status200
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
			apis::amf3_gpp_access_registration_info_retrieval::Get3GppRegistrationResponse::Status400
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
			apis::amf3_gpp_access_registration_info_retrieval::Get3GppRegistrationResponse::Status403
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
			apis::amf3_gpp_access_registration_info_retrieval::Get3GppRegistrationResponse::Status404
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
			apis::amf3_gpp_access_registration_info_retrieval::Get3GppRegistrationResponse::Status500
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
			apis::amf3_gpp_access_registration_info_retrieval::Get3GppRegistrationResponse::Status503
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
			apis::amf3_gpp_access_registration_info_retrieval::Get3GppRegistrationResponse::Statusdefault
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
fn get_non3_gpp_registration_validation(
	path_params: models::GetNon3GppRegistrationPathParams,
	query_params: models::GetNon3GppRegistrationQueryParams,
) -> std::result::Result<
	(
		models::GetNon3GppRegistrationPathParams,
		models::GetNon3GppRegistrationQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetNon3GppRegistration - GET
/// /nudm-uecm/v1/{ueId}/registrations/amf-non-3gpp-access
#[tracing::instrument(skip_all)]
async fn get_non3_gpp_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetNon3GppRegistrationPathParams>,
	Query(query_params): Query<models::GetNon3GppRegistrationQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::amf_non3_gpp_access_registration_info_retrieval::AmfNon3GppAccessRegistrationInfoRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_non3_gpp_registration_validation(path_params, query_params)
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
		.get_non3_gpp_registration(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::amf_non3_gpp_access_registration_info_retrieval::GetNon3GppRegistrationResponse::Status200
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
			apis::amf_non3_gpp_access_registration_info_retrieval::GetNon3GppRegistrationResponse::Status400
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
			apis::amf_non3_gpp_access_registration_info_retrieval::GetNon3GppRegistrationResponse::Status403
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
			apis::amf_non3_gpp_access_registration_info_retrieval::GetNon3GppRegistrationResponse::Status404
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
			apis::amf_non3_gpp_access_registration_info_retrieval::GetNon3GppRegistrationResponse::Status500
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
			apis::amf_non3_gpp_access_registration_info_retrieval::GetNon3GppRegistrationResponse::Status503
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
			apis::amf_non3_gpp_access_registration_info_retrieval::GetNon3GppRegistrationResponse::Statusdefault
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
struct Call3GppRegistrationBodyValidator<'a> {
	body: &'a models::Amf3GppAccessRegistration,
}

#[tracing::instrument(skip_all)]
fn call3_gpp_registration_validation(
	path_params: models::Call3GppRegistrationPathParams,
	body: models::Amf3GppAccessRegistration,
) -> std::result::Result<
	(
		models::Call3GppRegistrationPathParams,
		models::Amf3GppAccessRegistration,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = Call3GppRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// Call3GppRegistration - PUT
/// /nudm-uecm/v1/{ueId}/registrations/amf-3gpp-access
#[tracing::instrument(skip_all)]
async fn call3_gpp_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Call3GppRegistrationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::Amf3GppAccessRegistration>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::amf_registration_for3_gpp_access::AmfRegistrationFor3GppAccess,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || call3_gpp_registration_validation(path_params, body))
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
		.call3_gpp_registration(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::amf_registration_for3_gpp_access::Call3GppRegistrationResponse::Status200(
				body,
			) => {
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
			apis::amf_registration_for3_gpp_access::Call3GppRegistrationResponse::Status201 {
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
			apis::amf_registration_for3_gpp_access::Call3GppRegistrationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::amf_registration_for3_gpp_access::Call3GppRegistrationResponse::Status400(
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
			apis::amf_registration_for3_gpp_access::Call3GppRegistrationResponse::Status403(
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
			apis::amf_registration_for3_gpp_access::Call3GppRegistrationResponse::Status404(
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
			apis::amf_registration_for3_gpp_access::Call3GppRegistrationResponse::Status500(
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
			apis::amf_registration_for3_gpp_access::Call3GppRegistrationResponse::Status503(
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
			apis::amf_registration_for3_gpp_access::Call3GppRegistrationResponse::Statusdefault => {
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
struct Non3GppRegistrationBodyValidator<'a> {
	body: &'a models::AmfNon3GppAccessRegistration,
}

#[tracing::instrument(skip_all)]
fn non3_gpp_registration_validation(
	path_params: models::Non3GppRegistrationPathParams,
	body: models::AmfNon3GppAccessRegistration,
) -> std::result::Result<
	(
		models::Non3GppRegistrationPathParams,
		models::AmfNon3GppAccessRegistration,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = Non3GppRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// Non3GppRegistration - PUT
/// /nudm-uecm/v1/{ueId}/registrations/amf-non-3gpp-access
#[tracing::instrument(skip_all)]
async fn non3_gpp_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Non3GppRegistrationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AmfNon3GppAccessRegistration>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::amf_registration_for_non3_gpp_access::AmfRegistrationForNon3GppAccess,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || non3_gpp_registration_validation(path_params, body))
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
		.non3_gpp_registration(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::amf_registration_for_non3_gpp_access::Non3GppRegistrationResponse::Status200
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
			apis::amf_registration_for_non3_gpp_access::Non3GppRegistrationResponse::Status201
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
			apis::amf_registration_for_non3_gpp_access::Non3GppRegistrationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::amf_registration_for_non3_gpp_access::Non3GppRegistrationResponse::Status400
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
			apis::amf_registration_for_non3_gpp_access::Non3GppRegistrationResponse::Status403
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
			apis::amf_registration_for_non3_gpp_access::Non3GppRegistrationResponse::Status404
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
			apis::amf_registration_for_non3_gpp_access::Non3GppRegistrationResponse::Status500
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
			apis::amf_registration_for_non3_gpp_access::Non3GppRegistrationResponse::Status503
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
			apis::amf_registration_for_non3_gpp_access::Non3GppRegistrationResponse::Statusdefault
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
fn get_am_data_validation(
	header_params: models::GetAmDataHeaderParams,
	path_params: models::GetAmDataPathParams,
	query_params: models::GetAmDataQueryParams,
) -> std::result::Result<
	(
		models::GetAmDataHeaderParams,
		models::GetAmDataPathParams,
		models::GetAmDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetAmData - GET /nudm-sdm/v2/{supi}/am-data
#[tracing::instrument(skip_all)]
async fn get_am_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetAmDataPathParams>,
	Query(query_params): Query<models::GetAmDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::access_and_mobility_subscription_data_retrieval::AccessAndMobilitySubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetAmDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_am_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_am_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::access_and_mobility_subscription_data_retrieval::GetAmDataResponse::Status200
			{
				body,
				cache_control,
				etag,
				last_modified
			}
			=> {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling cache_control header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							cache_control,
						);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling last_modified header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							last_modified,
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
			apis::access_and_mobility_subscription_data_retrieval::GetAmDataResponse::Status400
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
			apis::access_and_mobility_subscription_data_retrieval::GetAmDataResponse::Status404
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
			apis::access_and_mobility_subscription_data_retrieval::GetAmDataResponse::Status500
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
			apis::access_and_mobility_subscription_data_retrieval::GetAmDataResponse::Status503
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
			apis::access_and_mobility_subscription_data_retrieval::GetAmDataResponse::Statusdefault
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
struct AuthorizeNiddDataBodyValidator<'a> {
	body: &'a models::AuthorizationInfo,
}

#[tracing::instrument(skip_all)]
fn authorize_nidd_data_validation(
	path_params: models::AuthorizeNiddDataPathParams,
	body: models::AuthorizationInfo,
) -> std::result::Result<
	(
		models::AuthorizeNiddDataPathParams,
		models::AuthorizationInfo,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = AuthorizeNiddDataBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// AuthorizeNiddData - POST /nudm-niddau/v1/{ueIdentity}/authorize
#[tracing::instrument(skip_all)]
async fn authorize_nidd_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::AuthorizeNiddDataPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AuthorizationInfo>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::authorize_the_nidd_configuration_request::AuthorizeTheNiddConfigurationRequest,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || authorize_nidd_data_validation(path_params, body))
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
		.authorize_nidd_data(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::authorize_the_nidd_configuration_request::AuthorizeNiddDataResponse::Status200
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
			apis::authorize_the_nidd_configuration_request::AuthorizeNiddDataResponse::Status400
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
			apis::authorize_the_nidd_configuration_request::AuthorizeNiddDataResponse::Status403
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
			apis::authorize_the_nidd_configuration_request::AuthorizeNiddDataResponse::Status404
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
			apis::authorize_the_nidd_configuration_request::AuthorizeNiddDataResponse::Status500
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
			apis::authorize_the_nidd_configuration_request::AuthorizeNiddDataResponse::Status501
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
			apis::authorize_the_nidd_configuration_request::AuthorizeNiddDataResponse::Status503
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
			apis::authorize_the_nidd_configuration_request::AuthorizeNiddDataResponse::Statusdefault
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
struct Create5GMbsGroupBodyValidator<'a> {
	body: &'a models::MulticastMbsGroupMemb,
}

#[tracing::instrument(skip_all)]
fn create5_g_mbs_group_validation(
	path_params: models::Create5GMbsGroupPathParams,
	body: models::MulticastMbsGroupMemb,
) -> std::result::Result<
	(
		models::Create5GMbsGroupPathParams,
		models::MulticastMbsGroupMemb,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = Create5GMbsGroupBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// Create5GMbsGroup - PUT /nudm-pp/v1/mbs-group-membership/{extGroupId}
#[tracing::instrument(skip_all)]
async fn create5_g_mbs_group<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Create5GMbsGroupPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::MulticastMbsGroupMemb>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::class5_gmbs_group_creation::Class5GmbsGroupCreation,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || create5_g_mbs_group_validation(path_params, body))
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
		.create5_g_mbs_group(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status201 => {
				let mut response = response.status(201);
				response.body(Body::empty())
			}
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status400(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status401(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status403(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status404(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status411(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status413(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status415(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status429(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status500(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status502(body) => {
				let mut response = response.status(502);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Status503(body) => {
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
			apis::class5_gmbs_group_creation::Create5GMbsGroupResponse::Statusdefault => {
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
fn delete5_g_mbs_group_validation(
	path_params: models::Delete5GMbsGroupPathParams
) -> std::result::Result<(models::Delete5GMbsGroupPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// Delete5GMbsGroup - DELETE /nudm-pp/v1/mbs-group-membership/{extGroupId}
#[tracing::instrument(skip_all)]
async fn delete5_g_mbs_group<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Delete5GMbsGroupPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::class5_gmbs_group_deletion::Class5GmbsGroupDeletion,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || delete5_g_mbs_group_validation(path_params))
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
		.delete5_g_mbs_group(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Status400(body) => {
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
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Status401(body) => {
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
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Status403(body) => {
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
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Status404(body) => {
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
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Status429(body) => {
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
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Status500(body) => {
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
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Status502(body) => {
				let mut response = response.status(502);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Status503(body) => {
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
			apis::class5_gmbs_group_deletion::Delete5GMbsGroupResponse::Statusdefault => {
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
fn get5_g_mbs_group_validation(
	path_params: models::Get5GMbsGroupPathParams
) -> std::result::Result<(models::Get5GMbsGroupPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// Get5GMbsGroup - GET /nudm-pp/v1/mbs-group-membership/{extGroupId}
#[tracing::instrument(skip_all)]
async fn get5_g_mbs_group<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Get5GMbsGroupPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::class5_gmbs_group_modification::Class5GmbsGroupModification,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || get5_g_mbs_group_validation(path_params))
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
		.get5_g_mbs_group(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status200(body) => {
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
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status400(body) => {
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
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status401(body) => {
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
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status403(body) => {
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
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status404(body) => {
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
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status429(body) => {
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
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status500(body) => {
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
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status502(body) => {
				let mut response = response.status(502);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Status503(body) => {
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
			apis::class5_gmbs_group_modification::Get5GMbsGroupResponse::Statusdefault => {
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
struct Modify5GMbsGroupBodyValidator<'a> {
	body: &'a models::MulticastMbsGroupMemb,
}

#[tracing::instrument(skip_all)]
fn modify5_g_mbs_group_validation(
	path_params: models::Modify5GMbsGroupPathParams,
	query_params: models::Modify5GMbsGroupQueryParams,
	body: models::MulticastMbsGroupMemb,
) -> std::result::Result<
	(
		models::Modify5GMbsGroupPathParams,
		models::Modify5GMbsGroupQueryParams,
		models::MulticastMbsGroupMemb,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = Modify5GMbsGroupBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// Modify5GMbsGroup - PATCH /nudm-pp/v1/mbs-group-membership/{extGroupId}
#[tracing::instrument(skip_all)]
async fn modify5_g_mbs_group<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Modify5GMbsGroupPathParams>,
	Query(query_params): Query<models::Modify5GMbsGroupQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::MulticastMbsGroupMemb>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::class5_gmbs_group_modification::Class5GmbsGroupModification,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		modify5_g_mbs_group_validation(path_params, query_params, body)
	})
	.await
	.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.modify5_g_mbs_group(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status200(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status400(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status401(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status403(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status404(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status411(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status413(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status415(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status429(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status500(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status502(body) => {
				let mut response = response.status(502);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Status503(body) => {
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
			apis::class5_gmbs_group_modification::Modify5GMbsGroupResponse::Statusdefault => {
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
struct Create5GVnGroupBodyValidator<'a> {
	body: &'a models::Model5GvnGroupConfiguration,
}

#[tracing::instrument(skip_all)]
fn create5_gvn_group_validation(
	path_params: models::Create5GVnGroupPathParams,
	body: models::Model5GvnGroupConfiguration,
) -> std::result::Result<
	(
		models::Create5GVnGroupPathParams,
		models::Model5GvnGroupConfiguration,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = Create5GVnGroupBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// Create5GVnGroup - PUT /nudm-pp/v1/5g-vn-groups/{extGroupId}
#[tracing::instrument(skip_all)]
async fn create5_gvn_group<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Create5GVnGroupPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::Model5GvnGroupConfiguration>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::class5_gvn_group_creation::Class5GvnGroupCreation,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || create5_gvn_group_validation(path_params, body))
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
		.create5_gvn_group(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::class5_gvn_group_creation::Create5GVnGroupResponse::Status201 => {
				let mut response = response.status(201);
				response.body(Body::empty())
			}
			apis::class5_gvn_group_creation::Create5GVnGroupResponse::Status400(body) => {
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
			apis::class5_gvn_group_creation::Create5GVnGroupResponse::Status403(body) => {
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
			apis::class5_gvn_group_creation::Create5GVnGroupResponse::Status404(body) => {
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
			apis::class5_gvn_group_creation::Create5GVnGroupResponse::Status500(body) => {
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
			apis::class5_gvn_group_creation::Create5GVnGroupResponse::Status503(body) => {
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
			apis::class5_gvn_group_creation::Create5GVnGroupResponse::Statusdefault => {
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
fn delete5_gvn_group_validation(
	path_params: models::Delete5GVnGroupPathParams,
	query_params: models::Delete5GVnGroupQueryParams,
) -> std::result::Result<
	(
		models::Delete5GVnGroupPathParams,
		models::Delete5GVnGroupQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// Delete5GVnGroup - DELETE /nudm-pp/v1/5g-vn-groups/{extGroupId}
#[tracing::instrument(skip_all)]
async fn delete5_gvn_group<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Delete5GVnGroupPathParams>,
	Query(query_params): Query<models::Delete5GVnGroupQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::class5_gvn_group_deletion::Class5GvnGroupDeletion,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		delete5_gvn_group_validation(path_params, query_params)
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
		.delete5_gvn_group(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::class5_gvn_group_deletion::Delete5GVnGroupResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::class5_gvn_group_deletion::Delete5GVnGroupResponse::Status400(body) => {
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
			apis::class5_gvn_group_deletion::Delete5GVnGroupResponse::Status403(body) => {
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
			apis::class5_gvn_group_deletion::Delete5GVnGroupResponse::Status404(body) => {
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
			apis::class5_gvn_group_deletion::Delete5GVnGroupResponse::Status500(body) => {
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
			apis::class5_gvn_group_deletion::Delete5GVnGroupResponse::Status503(body) => {
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
			apis::class5_gvn_group_deletion::Delete5GVnGroupResponse::Statusdefault => {
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
fn get5_gvn_group_validation(
	path_params: models::Get5GVnGroupPathParams
) -> std::result::Result<(models::Get5GVnGroupPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// Get5GVnGroup - GET /nudm-pp/v1/5g-vn-groups/{extGroupId}
#[tracing::instrument(skip_all)]
async fn get5_gvn_group<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Get5GVnGroupPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::class5_gvn_group_modification::Class5GvnGroupModification,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || get5_gvn_group_validation(path_params))
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
		.get5_gvn_group(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::class5_gvn_group_modification::Get5GVnGroupResponse::Status200(body) => {
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
			apis::class5_gvn_group_modification::Get5GVnGroupResponse::Status400(body) => {
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
			apis::class5_gvn_group_modification::Get5GVnGroupResponse::Status403(body) => {
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
			apis::class5_gvn_group_modification::Get5GVnGroupResponse::Status404(body) => {
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
			apis::class5_gvn_group_modification::Get5GVnGroupResponse::Status500(body) => {
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
			apis::class5_gvn_group_modification::Get5GVnGroupResponse::Status503(body) => {
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
			apis::class5_gvn_group_modification::Get5GVnGroupResponse::Statusdefault => {
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
struct Modify5GVnGroupBodyValidator<'a> {
	body: &'a models::Model5GvnGroupConfiguration,
}

#[tracing::instrument(skip_all)]
fn modify5_gvn_group_validation(
	path_params: models::Modify5GVnGroupPathParams,
	query_params: models::Modify5GVnGroupQueryParams,
	body: models::Model5GvnGroupConfiguration,
) -> std::result::Result<
	(
		models::Modify5GVnGroupPathParams,
		models::Modify5GVnGroupQueryParams,
		models::Model5GvnGroupConfiguration,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = Modify5GVnGroupBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// Modify5GVnGroup - PATCH /nudm-pp/v1/5g-vn-groups/{extGroupId}
#[tracing::instrument(skip_all)]
async fn modify5_gvn_group<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Modify5GVnGroupPathParams>,
	Query(query_params): Query<models::Modify5GVnGroupQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::Model5GvnGroupConfiguration>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::class5_gvn_group_modification::Class5GvnGroupModification,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		modify5_gvn_group_validation(path_params, query_params, body)
	})
	.await
	.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.modify5_gvn_group(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::class5_gvn_group_modification::Modify5GVnGroupResponse::Status200(body) => {
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
			apis::class5_gvn_group_modification::Modify5GVnGroupResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::class5_gvn_group_modification::Modify5GVnGroupResponse::Status400(body) => {
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
			apis::class5_gvn_group_modification::Modify5GVnGroupResponse::Status403(body) => {
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
			apis::class5_gvn_group_modification::Modify5GVnGroupResponse::Status404(body) => {
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
			apis::class5_gvn_group_modification::Modify5GVnGroupResponse::Status500(body) => {
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
			apis::class5_gvn_group_modification::Modify5GVnGroupResponse::Status503(body) => {
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
			apis::class5_gvn_group_modification::Modify5GVnGroupResponse::Statusdefault => {
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
fn get_mbs_data_validation(
	header_params: models::GetMbsDataHeaderParams,
	path_params: models::GetMbsDataPathParams,
	query_params: models::GetMbsDataQueryParams,
) -> std::result::Result<
	(
		models::GetMbsDataHeaderParams,
		models::GetMbsDataPathParams,
		models::GetMbsDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetMbsData - GET /nudm-sdm/v2/{supi}/5mbs-data
#[tracing::instrument(skip_all)]
async fn get_mbs_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetMbsDataPathParams>,
	Query(query_params): Query<models::GetMbsDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::class5_mbs_subscription_data_retrieval::Class5MbsSubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetMbsDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_mbs_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_mbs_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::class5_mbs_subscription_data_retrieval::GetMbsDataResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::class5_mbs_subscription_data_retrieval::GetMbsDataResponse::Status400(body) => {
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
			apis::class5_mbs_subscription_data_retrieval::GetMbsDataResponse::Status404(body) => {
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
			apis::class5_mbs_subscription_data_retrieval::GetMbsDataResponse::Status500(body) => {
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
			apis::class5_mbs_subscription_data_retrieval::GetMbsDataResponse::Status503(body) => {
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
			apis::class5_mbs_subscription_data_retrieval::GetMbsDataResponse::Statusdefault => {
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
struct ConfirmAuthBodyValidator<'a> {
	body: &'a models::AuthEvent,
}

#[tracing::instrument(skip_all)]
fn confirm_auth_validation(
	path_params: models::ConfirmAuthPathParams,
	body: models::AuthEvent,
) -> std::result::Result<(models::ConfirmAuthPathParams, models::AuthEvent), ValidationErrors> {
	path_params.validate()?;
	let b = ConfirmAuthBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ConfirmAuth - POST /nudm-ueau/v1/{supi}/auth-events
#[tracing::instrument(skip_all)]
async fn confirm_auth<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ConfirmAuthPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AuthEvent>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::confirm_auth::ConfirmAuth,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || confirm_auth_validation(path_params, body))
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
		.confirm_auth(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::confirm_auth::ConfirmAuthResponse::Status201 { body, location } => {
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
			apis::confirm_auth::ConfirmAuthResponse::Status400(body) => {
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
			apis::confirm_auth::ConfirmAuthResponse::Status404(body) => {
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
			apis::confirm_auth::ConfirmAuthResponse::Status500(body) => {
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
			apis::confirm_auth::ConfirmAuthResponse::Status503(body) => {
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
			apis::confirm_auth::ConfirmAuthResponse::Statusdefault => {
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
struct CreateEeSubscriptionBodyValidator<'a> {
	body: &'a models::EeSubscription,
}

#[tracing::instrument(skip_all)]
fn create_ee_subscription_validation(
	path_params: models::CreateEeSubscriptionPathParams,
	body: models::EeSubscription,
) -> std::result::Result<
	(
		models::CreateEeSubscriptionPathParams,
		models::EeSubscription,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = CreateEeSubscriptionBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// CreateEeSubscription - POST /nudm-ee/v1/{ueIdentity}/ee-subscriptions
#[tracing::instrument(skip_all)]
async fn create_ee_subscription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::CreateEeSubscriptionPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::EeSubscription>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::create_ee_subscription::CreateEeSubscription,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || create_ee_subscription_validation(path_params, body))
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
		.create_ee_subscription(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::create_ee_subscription::CreateEeSubscriptionResponse::Status201 {
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
			apis::create_ee_subscription::CreateEeSubscriptionResponse::Status400(body) => {
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
			apis::create_ee_subscription::CreateEeSubscriptionResponse::Status403(body) => {
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
			apis::create_ee_subscription::CreateEeSubscriptionResponse::Status404(body) => {
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
			apis::create_ee_subscription::CreateEeSubscriptionResponse::Status500(body) => {
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
			apis::create_ee_subscription::CreateEeSubscriptionResponse::Status501(body) => {
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
			apis::create_ee_subscription::CreateEeSubscriptionResponse::Status503(body) => {
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
			apis::create_ee_subscription::CreateEeSubscriptionResponse::Statusdefault => {
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
struct DeconcealBodyValidator<'a> {
	body: &'a models::DeconcealReqData,
}

#[tracing::instrument(skip_all)]
fn deconceal_validation(
	body: models::DeconcealReqData
) -> std::result::Result<(models::DeconcealReqData,), ValidationErrors> {
	let b = DeconcealBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// Deconceal - POST /nudm-ueid/v1/deconceal
#[tracing::instrument(skip_all)]
async fn deconceal<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::DeconcealReqData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::deconceal::Deconceal,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || deconceal_validation(body))
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
		.deconceal(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::deconceal::DeconcealResponse::Status200(body) => {
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
			apis::deconceal::DeconcealResponse::Status400(body) => {
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
			apis::deconceal::DeconcealResponse::Status403(body) => {
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
			apis::deconceal::DeconcealResponse::Status404(body) => {
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
			apis::deconceal::DeconcealResponse::Status500(body) => {
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
			apis::deconceal::DeconcealResponse::Status501(body) => {
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
			apis::deconceal::DeconcealResponse::Status503(body) => {
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
			apis::deconceal::DeconcealResponse::Statusdefault => {
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
struct DeleteAuthBodyValidator<'a> {
	body: &'a models::AuthEvent,
}

#[tracing::instrument(skip_all)]
fn delete_auth_validation(
	path_params: models::DeleteAuthPathParams,
	body: models::AuthEvent,
) -> std::result::Result<(models::DeleteAuthPathParams, models::AuthEvent), ValidationErrors> {
	path_params.validate()?;
	let b = DeleteAuthBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// DeleteAuth - PUT /nudm-ueau/v1/{supi}/auth-events/{authEventId}
#[tracing::instrument(skip_all)]
async fn delete_auth<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteAuthPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AuthEvent>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::delete_auth::DeleteAuth,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || delete_auth_validation(path_params, body))
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
		.delete_auth(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::delete_auth::DeleteAuthResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::delete_auth::DeleteAuthResponse::Status400(body) => {
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
			apis::delete_auth::DeleteAuthResponse::Status404(body) => {
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
			apis::delete_auth::DeleteAuthResponse::Status500(body) => {
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
			apis::delete_auth::DeleteAuthResponse::Status503(body) => {
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
			apis::delete_auth::DeleteAuthResponse::Statusdefault => {
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
fn delete_ee_subscription_validation(
	path_params: models::DeleteEeSubscriptionPathParams
) -> std::result::Result<(models::DeleteEeSubscriptionPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeleteEeSubscription - DELETE
/// /nudm-ee/v1/{ueIdentity}/ee-subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn delete_ee_subscription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeleteEeSubscriptionPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::delete_ee_subscription::DeleteEeSubscription,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || delete_ee_subscription_validation(path_params))
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
		.delete_ee_subscription(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::delete_ee_subscription::DeleteEeSubscriptionResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::delete_ee_subscription::DeleteEeSubscriptionResponse::Status400(body) => {
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
			apis::delete_ee_subscription::DeleteEeSubscriptionResponse::Status404(body) => {
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
			apis::delete_ee_subscription::DeleteEeSubscriptionResponse::Status500(body) => {
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
			apis::delete_ee_subscription::DeleteEeSubscriptionResponse::Status503(body) => {
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
			apis::delete_ee_subscription::DeleteEeSubscriptionResponse::Statusdefault => {
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
fn get_ecr_data_validation(
	header_params: models::GetEcrDataHeaderParams,
	path_params: models::GetEcrDataPathParams,
	query_params: models::GetEcrDataQueryParams,
) -> std::result::Result<
	(
		models::GetEcrDataHeaderParams,
		models::GetEcrDataPathParams,
		models::GetEcrDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetEcrData - GET /nudm-sdm/v2/{supi}/am-data/ecr-data
#[tracing::instrument(skip_all)]
async fn get_ecr_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetEcrDataPathParams>,
	Query(query_params): Query<models::GetEcrDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::enhanced_coverage_restriction_data_retrieval::EnhancedCoverageRestrictionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetEcrDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_ecr_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_ecr_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::enhanced_coverage_restriction_data_retrieval::GetEcrDataResponse::Status200
			{
				body,
				cache_control,
				etag,
				last_modified
			}
			=> {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling cache_control header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							cache_control,
						);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling last_modified header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							last_modified,
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
			apis::enhanced_coverage_restriction_data_retrieval::GetEcrDataResponse::Status400
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
			apis::enhanced_coverage_restriction_data_retrieval::GetEcrDataResponse::Status404
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
			apis::enhanced_coverage_restriction_data_retrieval::GetEcrDataResponse::Status500
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
			apis::enhanced_coverage_restriction_data_retrieval::GetEcrDataResponse::Status503
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
			apis::enhanced_coverage_restriction_data_retrieval::GetEcrDataResponse::Statusdefault
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
fn get_supi_or_gpsi_validation(
	header_params: models::GetSupiOrGpsiHeaderParams,
	path_params: models::GetSupiOrGpsiPathParams,
	query_params: models::GetSupiOrGpsiQueryParams,
) -> std::result::Result<
	(
		models::GetSupiOrGpsiHeaderParams,
		models::GetSupiOrGpsiPathParams,
		models::GetSupiOrGpsiQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetSupiOrGpsi - GET /nudm-sdm/v2/{ueId}/id-translation-result
#[tracing::instrument(skip_all)]
async fn get_supi_or_gpsi<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetSupiOrGpsiPathParams>,
	Query(query_params): Query<models::GetSupiOrGpsiQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::gpsito_supi_translation_or_supito_gpsi_translation::GpsitoSupiTranslationOrSupitoGpsiTranslation,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetSupiOrGpsiHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_supi_or_gpsi_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_supi_or_gpsi(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::gpsito_supi_translation_or_supito_gpsi_translation::GetSupiOrGpsiResponse::Status200
			{
				body,
				cache_control,
				etag,
				last_modified
			}
			=> {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling cache_control header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							cache_control,
						);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling last_modified header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							last_modified,
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
			apis::gpsito_supi_translation_or_supito_gpsi_translation::GetSupiOrGpsiResponse::Status400
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
			apis::gpsito_supi_translation_or_supito_gpsi_translation::GetSupiOrGpsiResponse::Status403
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
			apis::gpsito_supi_translation_or_supito_gpsi_translation::GetSupiOrGpsiResponse::Status404
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
			apis::gpsito_supi_translation_or_supito_gpsi_translation::GetSupiOrGpsiResponse::Status500
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
			apis::gpsito_supi_translation_or_supito_gpsi_translation::GetSupiOrGpsiResponse::Status503
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
			apis::gpsito_supi_translation_or_supito_gpsi_translation::GetSupiOrGpsiResponse::Statusdefault
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
struct GenerateAuthDataBodyValidator<'a> {
	body: &'a models::AuthenticationInfoRequest,
}

#[tracing::instrument(skip_all)]
fn generate_auth_data_validation(
	path_params: models::GenerateAuthDataPathParams,
	body: models::AuthenticationInfoRequest,
) -> std::result::Result<
	(
		models::GenerateAuthDataPathParams,
		models::AuthenticationInfoRequest,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = GenerateAuthDataBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// GenerateAuthData - POST
/// /nudm-ueau/v1/{supiOrSuci}/security-information/generate-auth-data
#[tracing::instrument(skip_all)]
async fn generate_auth_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GenerateAuthDataPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AuthenticationInfoRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::generate_auth_data::GenerateAuthData,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || generate_auth_data_validation(path_params, body))
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
		.generate_auth_data(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::generate_auth_data::GenerateAuthDataResponse::Status200(body) => {
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
			apis::generate_auth_data::GenerateAuthDataResponse::Status400(body) => {
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
			apis::generate_auth_data::GenerateAuthDataResponse::Status403(body) => {
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
			apis::generate_auth_data::GenerateAuthDataResponse::Status404(body) => {
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
			apis::generate_auth_data::GenerateAuthDataResponse::Status500(body) => {
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
			apis::generate_auth_data::GenerateAuthDataResponse::Status501(body) => {
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
			apis::generate_auth_data::GenerateAuthDataResponse::Status503(body) => {
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
			apis::generate_auth_data::GenerateAuthDataResponse::Statusdefault => {
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
struct GenerateGbaAvBodyValidator<'a> {
	body: &'a models::GbaAuthenticationInfoRequest,
}

#[tracing::instrument(skip_all)]
fn generate_gba_av_validation(
	path_params: models::GenerateGbaAvPathParams,
	body: models::GbaAuthenticationInfoRequest,
) -> std::result::Result<
	(
		models::GenerateGbaAvPathParams,
		models::GbaAuthenticationInfoRequest,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = GenerateGbaAvBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// GenerateGbaAv - POST
/// /nudm-ueau/v1/{supi}/gba-security-information/generate-av
#[tracing::instrument(skip_all)]
async fn generate_gba_av<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GenerateGbaAvPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::GbaAuthenticationInfoRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::generate_gba_authentication_vectors::GenerateGbaAuthenticationVectors,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || generate_gba_av_validation(path_params, body))
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
		.generate_gba_av(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::generate_gba_authentication_vectors::GenerateGbaAvResponse::Status200(body) => {
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
			apis::generate_gba_authentication_vectors::GenerateGbaAvResponse::Status400(body) => {
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
			apis::generate_gba_authentication_vectors::GenerateGbaAvResponse::Status403(body) => {
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
			apis::generate_gba_authentication_vectors::GenerateGbaAvResponse::Status404(body) => {
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
			apis::generate_gba_authentication_vectors::GenerateGbaAvResponse::Status500(body) => {
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
			apis::generate_gba_authentication_vectors::GenerateGbaAvResponse::Status501(body) => {
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
			apis::generate_gba_authentication_vectors::GenerateGbaAvResponse::Status503(body) => {
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
			apis::generate_gba_authentication_vectors::GenerateGbaAvResponse::Statusdefault => {
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
struct GenerateAvBodyValidator<'a> {
	body: &'a models::HssAuthenticationInfoRequest,
}

#[tracing::instrument(skip_all)]
fn generate_av_validation(
	path_params: models::GenerateAvPathParams,
	body: models::HssAuthenticationInfoRequest,
) -> std::result::Result<
	(
		models::GenerateAvPathParams,
		models::HssAuthenticationInfoRequest,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = GenerateAvBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// GenerateAv - POST
/// /nudm-ueau/v1/{supi}/hss-security-information/{hssAuthType}/generate-av
#[tracing::instrument(skip_all)]
async fn generate_av<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GenerateAvPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::HssAuthenticationInfoRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::generate_hss_authentication_vectors::GenerateHssAuthenticationVectors,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || generate_av_validation(path_params, body))
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
		.generate_av(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::generate_hss_authentication_vectors::GenerateAvResponse::Status200(body) => {
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
			apis::generate_hss_authentication_vectors::GenerateAvResponse::Status400(body) => {
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
			apis::generate_hss_authentication_vectors::GenerateAvResponse::Status403(body) => {
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
			apis::generate_hss_authentication_vectors::GenerateAvResponse::Status404(body) => {
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
			apis::generate_hss_authentication_vectors::GenerateAvResponse::Status500(body) => {
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
			apis::generate_hss_authentication_vectors::GenerateAvResponse::Status501(body) => {
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
			apis::generate_hss_authentication_vectors::GenerateAvResponse::Status503(body) => {
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
			apis::generate_hss_authentication_vectors::GenerateAvResponse::Statusdefault => {
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
struct GenerateProseAvBodyValidator<'a> {
	body: &'a models::ProSeAuthenticationInfoRequest,
}

#[tracing::instrument(skip_all)]
fn generate_prose_av_validation(
	path_params: models::GenerateProseAvPathParams,
	body: models::ProSeAuthenticationInfoRequest,
) -> std::result::Result<
	(
		models::GenerateProseAvPathParams,
		models::ProSeAuthenticationInfoRequest,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = GenerateProseAvBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// GenerateProseAv - POST
/// /nudm-ueau/v1/{supiOrSuci}/prose-security-information/generate-av
#[tracing::instrument(skip_all)]
async fn generate_prose_av<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GenerateProseAvPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::ProSeAuthenticationInfoRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::generate_pro_se_authentication_vectors::GenerateProSeAuthenticationVectors,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || generate_prose_av_validation(path_params, body))
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
		.generate_prose_av(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::generate_pro_se_authentication_vectors::GenerateProseAvResponse::Status200
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
			apis::generate_pro_se_authentication_vectors::GenerateProseAvResponse::Status400
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
			apis::generate_pro_se_authentication_vectors::GenerateProseAvResponse::Status403
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
			apis::generate_pro_se_authentication_vectors::GenerateProseAvResponse::Status404
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
			apis::generate_pro_se_authentication_vectors::GenerateProseAvResponse::Status500
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
			apis::generate_pro_se_authentication_vectors::GenerateProseAvResponse::Status501
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
			apis::generate_pro_se_authentication_vectors::GenerateProseAvResponse::Status503
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
			apis::generate_pro_se_authentication_vectors::GenerateProseAvResponse::Statusdefault
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
fn get_rg_auth_data_validation(
	header_params: models::GetRgAuthDataHeaderParams,
	path_params: models::GetRgAuthDataPathParams,
	query_params: models::GetRgAuthDataQueryParams,
) -> std::result::Result<
	(
		models::GetRgAuthDataHeaderParams,
		models::GetRgAuthDataPathParams,
		models::GetRgAuthDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetRgAuthData - GET /nudm-ueau/v1/{supiOrSuci}/security-information-rg
#[tracing::instrument(skip_all)]
async fn get_rg_auth_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetRgAuthDataPathParams>,
	Query(query_params): Query<models::GetRgAuthDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::get_auth_data_for_fnrg::GetAuthDataForFnrg,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetRgAuthDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_rg_auth_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_rg_auth_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::get_auth_data_for_fnrg::GetRgAuthDataResponse::Status200(body) => {
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
			apis::get_auth_data_for_fnrg::GetRgAuthDataResponse::Status400(body) => {
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
			apis::get_auth_data_for_fnrg::GetRgAuthDataResponse::Status403(body) => {
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
			apis::get_auth_data_for_fnrg::GetRgAuthDataResponse::Status404(body) => {
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
			apis::get_auth_data_for_fnrg::GetRgAuthDataResponse::Status500(body) => {
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
			apis::get_auth_data_for_fnrg::GetRgAuthDataResponse::Status503(body) => {
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
			apis::get_auth_data_for_fnrg::GetRgAuthDataResponse::Statusdefault => {
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
fn get_group_identifiers_validation(
	header_params: models::GetGroupIdentifiersHeaderParams,
	query_params: models::GetGroupIdentifiersQueryParams,
) -> std::result::Result<
	(
		models::GetGroupIdentifiersHeaderParams,
		models::GetGroupIdentifiersQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	query_params.validate()?;

	Ok((header_params, query_params))
}

/// GetGroupIdentifiers - GET /nudm-sdm/v2/group-data/group-identifiers
#[tracing::instrument(skip_all)]
async fn get_group_identifiers<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Query(query_params): Query<models::GetGroupIdentifiersQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::group_identifiers::GroupIdentifiers,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetGroupIdentifiersHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_group_identifiers_validation(header_params, query_params)
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
		.get_group_identifiers(method, host, cookies, header_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::group_identifiers::GetGroupIdentifiersResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::group_identifiers::GetGroupIdentifiersResponse::Status400(body) => {
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
			apis::group_identifiers::GetGroupIdentifiersResponse::Status403(body) => {
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
			apis::group_identifiers::GetGroupIdentifiersResponse::Status404(body) => {
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
			apis::group_identifiers::GetGroupIdentifiersResponse::Status500(body) => {
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
			apis::group_identifiers::GetGroupIdentifiersResponse::Status503(body) => {
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
			apis::group_identifiers::GetGroupIdentifiersResponse::Statusdefault => {
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
fn ip_sm_gw_deregistration_validation(
	path_params: models::IpSmGwDeregistrationPathParams
) -> std::result::Result<(models::IpSmGwDeregistrationPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// IpSmGwDeregistration - DELETE /nudm-uecm/v1/{ueId}/registrations/ip-sm-gw
#[tracing::instrument(skip_all)]
async fn ip_sm_gw_deregistration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::IpSmGwDeregistrationPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ipsmgw_deregistration::IpsmgwDeregistration,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || ip_sm_gw_deregistration_validation(path_params))
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
		.ip_sm_gw_deregistration(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ipsmgw_deregistration::IpSmGwDeregistrationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::ipsmgw_deregistration::IpSmGwDeregistrationResponse::Status400(body) => {
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
			apis::ipsmgw_deregistration::IpSmGwDeregistrationResponse::Status404(body) => {
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
			apis::ipsmgw_deregistration::IpSmGwDeregistrationResponse::Status500(body) => {
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
			apis::ipsmgw_deregistration::IpSmGwDeregistrationResponse::Status503(body) => {
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
			apis::ipsmgw_deregistration::IpSmGwDeregistrationResponse::Statusdefault => {
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
struct IpSmGwRegistrationBodyValidator<'a> {
	body: &'a models::IpSmGwRegistration,
}

#[tracing::instrument(skip_all)]
fn ip_sm_gw_registration_validation(
	path_params: models::IpSmGwRegistrationPathParams,
	body: models::IpSmGwRegistration,
) -> std::result::Result<
	(
		models::IpSmGwRegistrationPathParams,
		models::IpSmGwRegistration,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = IpSmGwRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// IpSmGwRegistration - PUT /nudm-uecm/v1/{ueId}/registrations/ip-sm-gw
#[tracing::instrument(skip_all)]
async fn ip_sm_gw_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::IpSmGwRegistrationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::IpSmGwRegistration>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ipsmgw_registration::IpsmgwRegistration,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || ip_sm_gw_registration_validation(path_params, body))
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
		.ip_sm_gw_registration(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ipsmgw_registration::IpSmGwRegistrationResponse::Status200(body) => {
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
			apis::ipsmgw_registration::IpSmGwRegistrationResponse::Status201 { body, location } => {
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
			apis::ipsmgw_registration::IpSmGwRegistrationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::ipsmgw_registration::IpSmGwRegistrationResponse::Status400(body) => {
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
			apis::ipsmgw_registration::IpSmGwRegistrationResponse::Status403(body) => {
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
			apis::ipsmgw_registration::IpSmGwRegistrationResponse::Status404(body) => {
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
			apis::ipsmgw_registration::IpSmGwRegistrationResponse::Status500(body) => {
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
			apis::ipsmgw_registration::IpSmGwRegistrationResponse::Status503(body) => {
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
			apis::ipsmgw_registration::IpSmGwRegistrationResponse::Statusdefault => {
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
fn get_ip_sm_gw_registration_validation(
	path_params: models::GetIpSmGwRegistrationPathParams
) -> std::result::Result<(models::GetIpSmGwRegistrationPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// GetIpSmGwRegistration - GET /nudm-uecm/v1/{ueId}/registrations/ip-sm-gw
#[tracing::instrument(skip_all)]
async fn get_ip_sm_gw_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetIpSmGwRegistrationPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ipsmgw_registration_info_retrieval::IpsmgwRegistrationInfoRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || get_ip_sm_gw_registration_validation(path_params))
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
		.get_ip_sm_gw_registration(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ipsmgw_registration_info_retrieval::GetIpSmGwRegistrationResponse::Status200
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
			apis::ipsmgw_registration_info_retrieval::GetIpSmGwRegistrationResponse::Status400
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
			apis::ipsmgw_registration_info_retrieval::GetIpSmGwRegistrationResponse::Status403
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
			apis::ipsmgw_registration_info_retrieval::GetIpSmGwRegistrationResponse::Status404
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
			apis::ipsmgw_registration_info_retrieval::GetIpSmGwRegistrationResponse::Status500
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
			apis::ipsmgw_registration_info_retrieval::GetIpSmGwRegistrationResponse::Status503
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
			apis::ipsmgw_registration_info_retrieval::GetIpSmGwRegistrationResponse::Statusdefault
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
fn get_lcs_bca_data_validation(
	header_params: models::GetLcsBcaDataHeaderParams,
	path_params: models::GetLcsBcaDataPathParams,
	query_params: models::GetLcsBcaDataQueryParams,
) -> std::result::Result<
	(
		models::GetLcsBcaDataHeaderParams,
		models::GetLcsBcaDataPathParams,
		models::GetLcsBcaDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetLcsBcaData - GET /nudm-sdm/v2/{supi}/lcs-bca-data
#[tracing::instrument(skip_all)]
async fn get_lcs_bca_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetLcsBcaDataPathParams>,
	Query(query_params): Query<models::GetLcsBcaDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::lcs_broadcast_assistance_data_types_retrieval::LcsBroadcastAssistanceDataTypesRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetLcsBcaDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_lcs_bca_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_lcs_bca_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::lcs_broadcast_assistance_data_types_retrieval::GetLcsBcaDataResponse::Status200
			{
				body,
				cache_control,
				etag,
				last_modified
			}
			=> {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling cache_control header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							cache_control,
						);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling last_modified header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							last_modified,
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
			apis::lcs_broadcast_assistance_data_types_retrieval::GetLcsBcaDataResponse::Status400
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
			apis::lcs_broadcast_assistance_data_types_retrieval::GetLcsBcaDataResponse::Status404
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
			apis::lcs_broadcast_assistance_data_types_retrieval::GetLcsBcaDataResponse::Status500
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
			apis::lcs_broadcast_assistance_data_types_retrieval::GetLcsBcaDataResponse::Status503
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
			apis::lcs_broadcast_assistance_data_types_retrieval::GetLcsBcaDataResponse::Statusdefault
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
fn get_lcs_mo_data_validation(
	header_params: models::GetLcsMoDataHeaderParams,
	path_params: models::GetLcsMoDataPathParams,
	query_params: models::GetLcsMoDataQueryParams,
) -> std::result::Result<
	(
		models::GetLcsMoDataHeaderParams,
		models::GetLcsMoDataPathParams,
		models::GetLcsMoDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetLcsMoData - GET /nudm-sdm/v2/{supi}/lcs-mo-data
#[tracing::instrument(skip_all)]
async fn get_lcs_mo_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetLcsMoDataPathParams>,
	Query(query_params): Query<models::GetLcsMoDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::lcs_mobile_originated_data_retrieval::LcsMobileOriginatedDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetLcsMoDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_lcs_mo_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_lcs_mo_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::lcs_mobile_originated_data_retrieval::GetLcsMoDataResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::lcs_mobile_originated_data_retrieval::GetLcsMoDataResponse::Status400(body) => {
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
			apis::lcs_mobile_originated_data_retrieval::GetLcsMoDataResponse::Status404(body) => {
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
			apis::lcs_mobile_originated_data_retrieval::GetLcsMoDataResponse::Status500(body) => {
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
			apis::lcs_mobile_originated_data_retrieval::GetLcsMoDataResponse::Status503(body) => {
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
			apis::lcs_mobile_originated_data_retrieval::GetLcsMoDataResponse::Statusdefault => {
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
fn get_lcs_privacy_data_validation(
	header_params: models::GetLcsPrivacyDataHeaderParams,
	path_params: models::GetLcsPrivacyDataPathParams,
	query_params: models::GetLcsPrivacyDataQueryParams,
) -> std::result::Result<
	(
		models::GetLcsPrivacyDataHeaderParams,
		models::GetLcsPrivacyDataPathParams,
		models::GetLcsPrivacyDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetLcsPrivacyData - GET /nudm-sdm/v2/{ueId}/lcs-privacy-data
#[tracing::instrument(skip_all)]
async fn get_lcs_privacy_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetLcsPrivacyDataPathParams>,
	Query(query_params): Query<models::GetLcsPrivacyDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::lcs_privacy_data_retrieval::LcsPrivacyDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetLcsPrivacyDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_lcs_privacy_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_lcs_privacy_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::lcs_privacy_data_retrieval::GetLcsPrivacyDataResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::lcs_privacy_data_retrieval::GetLcsPrivacyDataResponse::Status400(body) => {
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
			apis::lcs_privacy_data_retrieval::GetLcsPrivacyDataResponse::Status404(body) => {
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
			apis::lcs_privacy_data_retrieval::GetLcsPrivacyDataResponse::Status500(body) => {
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
			apis::lcs_privacy_data_retrieval::GetLcsPrivacyDataResponse::Status503(body) => {
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
			apis::lcs_privacy_data_retrieval::GetLcsPrivacyDataResponse::Statusdefault => {
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
fn get_multiple_identifiers_validation(
	query_params: models::GetMultipleIdentifiersQueryParams
) -> std::result::Result<(models::GetMultipleIdentifiersQueryParams,), ValidationErrors> {
	query_params.validate()?;

	Ok((query_params,))
}

/// GetMultipleIdentifiers - GET /nudm-sdm/v2/multiple-identifiers
#[tracing::instrument(skip_all)]
async fn get_multiple_identifiers<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Query(query_params): Query<models::GetMultipleIdentifiersQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::multiple_identifiers::MultipleIdentifiers,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || get_multiple_identifiers_validation(query_params))
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
		.get_multiple_identifiers(method, host, cookies, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status400(body) => {
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
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status401(body) => {
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
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status403(body) => {
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
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status404(body) => {
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
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status406 => {
				let mut response = response.status(406);
				response.body(Body::empty())
			}
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status429(body) => {
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
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status500(body) => {
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
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status502(body) => {
				let mut response = response.status(502);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Status503(body) => {
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
			apis::multiple_identifiers::GetMultipleIdentifiersResponse::Statusdefault => {
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
fn nwdaf_deregistration_validation(
	path_params: models::NwdafDeregistrationPathParams
) -> std::result::Result<(models::NwdafDeregistrationPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// NwdafDeregistration - DELETE
/// /nudm-uecm/v1/{ueId}/registrations/nwdaf-registrations/{nwdafRegistrationId}
#[tracing::instrument(skip_all)]
async fn nwdaf_deregistration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::NwdafDeregistrationPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nwdaf_deregistration::NwdafDeregistration,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || nwdaf_deregistration_validation(path_params))
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
		.nwdaf_deregistration(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nwdaf_deregistration::NwdafDeregistrationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::nwdaf_deregistration::NwdafDeregistrationResponse::Status400(body) => {
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
			apis::nwdaf_deregistration::NwdafDeregistrationResponse::Status404(body) => {
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
			apis::nwdaf_deregistration::NwdafDeregistrationResponse::Status500(body) => {
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
			apis::nwdaf_deregistration::NwdafDeregistrationResponse::Status503(body) => {
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
			apis::nwdaf_deregistration::NwdafDeregistrationResponse::Statusdefault => {
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
struct NwdafRegistrationBodyValidator<'a> {
	body: &'a models::NwdafRegistration,
}

#[tracing::instrument(skip_all)]
fn nwdaf_registration_validation(
	path_params: models::NwdafRegistrationPathParams,
	body: models::NwdafRegistration,
) -> std::result::Result<
	(
		models::NwdafRegistrationPathParams,
		models::NwdafRegistration,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = NwdafRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// NwdafRegistration - PUT
/// /nudm-uecm/v1/{ueId}/registrations/nwdaf-registrations/{nwdafRegistrationId}
#[tracing::instrument(skip_all)]
async fn nwdaf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::NwdafRegistrationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::NwdafRegistration>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nwdaf_registration::NwdafRegistration,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || nwdaf_registration_validation(path_params, body))
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
		.nwdaf_registration(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nwdaf_registration::NwdafRegistrationResponse::Status200(body) => {
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
			apis::nwdaf_registration::NwdafRegistrationResponse::Status201(body) => {
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
			apis::nwdaf_registration::NwdafRegistrationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::nwdaf_registration::NwdafRegistrationResponse::Status400(body) => {
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
			apis::nwdaf_registration::NwdafRegistrationResponse::Status403(body) => {
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
			apis::nwdaf_registration::NwdafRegistrationResponse::Status404(body) => {
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
			apis::nwdaf_registration::NwdafRegistrationResponse::Status500(body) => {
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
			apis::nwdaf_registration::NwdafRegistrationResponse::Status503(body) => {
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
			apis::nwdaf_registration::NwdafRegistrationResponse::Statusdefault => {
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
fn get_nwdaf_registration_validation(
	path_params: models::GetNwdafRegistrationPathParams,
	query_params: models::GetNwdafRegistrationQueryParams,
) -> std::result::Result<
	(
		models::GetNwdafRegistrationPathParams,
		models::GetNwdafRegistrationQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetNwdafRegistration - GET
/// /nudm-uecm/v1/{ueId}/registrations/nwdaf-registrations
#[tracing::instrument(skip_all)]
async fn get_nwdaf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetNwdafRegistrationPathParams>,
	Query(query_params): Query<models::GetNwdafRegistrationQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::nwdaf_registration_info_retrieval::NwdafRegistrationInfoRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_nwdaf_registration_validation(path_params, query_params)
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
		.get_nwdaf_registration(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::nwdaf_registration_info_retrieval::GetNwdafRegistrationResponse::Status200
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
			apis::nwdaf_registration_info_retrieval::GetNwdafRegistrationResponse::Status400
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
			apis::nwdaf_registration_info_retrieval::GetNwdafRegistrationResponse::Status403
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
			apis::nwdaf_registration_info_retrieval::GetNwdafRegistrationResponse::Status404
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
			apis::nwdaf_registration_info_retrieval::GetNwdafRegistrationResponse::Status500
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
			apis::nwdaf_registration_info_retrieval::GetNwdafRegistrationResponse::Status503
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
			apis::nwdaf_registration_info_retrieval::GetNwdafRegistrationResponse::Statusdefault
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
struct PeiUpdateBodyValidator<'a> {
	body: &'a models::PeiUpdateInfo,
}

#[tracing::instrument(skip_all)]
fn pei_update_validation(
	path_params: models::PeiUpdatePathParams,
	body: models::PeiUpdateInfo,
) -> std::result::Result<(models::PeiUpdatePathParams, models::PeiUpdateInfo), ValidationErrors> {
	path_params.validate()?;
	let b = PeiUpdateBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// PeiUpdate - POST
/// /nudm-uecm/v1/{ueId}/registrations/amf-3gpp-access/pei-update
#[tracing::instrument(skip_all)]
async fn pei_update<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::PeiUpdatePathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::PeiUpdateInfo>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::pei_update::PeiUpdate,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || pei_update_validation(path_params, body))
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
		.pei_update(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::pei_update::PeiUpdateResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::pei_update::PeiUpdateResponse::Status400(body) => {
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
			apis::pei_update::PeiUpdateResponse::Status403(body) => {
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
			apis::pei_update::PeiUpdateResponse::Status404(body) => {
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
			apis::pei_update::PeiUpdateResponse::Status500(body) => {
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
			apis::pei_update::PeiUpdateResponse::Status503(body) => {
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
			apis::pei_update::PeiUpdateResponse::Statusdefault => {
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
struct CreatePpDataEntryBodyValidator<'a> {
	body: &'a models::PpDataEntry,
}

#[tracing::instrument(skip_all)]
fn create_pp_data_entry_validation(
	path_params: models::CreatePpDataEntryPathParams,
	body: models::PpDataEntry,
) -> std::result::Result<(models::CreatePpDataEntryPathParams, models::PpDataEntry), ValidationErrors>
{
	path_params.validate()?;
	let b = CreatePpDataEntryBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// CreatePpDataEntry - PUT /nudm-pp/v1/{ueId}/pp-data-store/{afInstanceId}
#[tracing::instrument(skip_all)]
async fn create_pp_data_entry<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::CreatePpDataEntryPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::PpDataEntry>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::parameter_provisioning_data_entry_document::ParameterProvisioningDataEntryDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || create_pp_data_entry_validation(path_params, body))
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
		.create_pp_data_entry(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::parameter_provisioning_data_entry_document::CreatePpDataEntryResponse::Status201
			(body)
			=> {
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
			apis::parameter_provisioning_data_entry_document::CreatePpDataEntryResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::parameter_provisioning_data_entry_document::CreatePpDataEntryResponse::Status400
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
			apis::parameter_provisioning_data_entry_document::CreatePpDataEntryResponse::Status403
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
			apis::parameter_provisioning_data_entry_document::CreatePpDataEntryResponse::Status404
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
			apis::parameter_provisioning_data_entry_document::CreatePpDataEntryResponse::Status500
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
			apis::parameter_provisioning_data_entry_document::CreatePpDataEntryResponse::Status503
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
			apis::parameter_provisioning_data_entry_document::CreatePpDataEntryResponse::Statusdefault
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
fn delete_pp_data_entry_validation(
	path_params: models::DeletePpDataEntryPathParams
) -> std::result::Result<(models::DeletePpDataEntryPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// DeletePpDataEntry - DELETE /nudm-pp/v1/{ueId}/pp-data-store/{afInstanceId}
#[tracing::instrument(skip_all)]
async fn delete_pp_data_entry<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeletePpDataEntryPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::parameter_provisioning_data_entry_document::ParameterProvisioningDataEntryDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || delete_pp_data_entry_validation(path_params))
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
		.delete_pp_data_entry(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::parameter_provisioning_data_entry_document::DeletePpDataEntryResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::parameter_provisioning_data_entry_document::DeletePpDataEntryResponse::Status400
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
			apis::parameter_provisioning_data_entry_document::DeletePpDataEntryResponse::Status403
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
			apis::parameter_provisioning_data_entry_document::DeletePpDataEntryResponse::Status404
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
			apis::parameter_provisioning_data_entry_document::DeletePpDataEntryResponse::Status500
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
			apis::parameter_provisioning_data_entry_document::DeletePpDataEntryResponse::Status503
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
			apis::parameter_provisioning_data_entry_document::DeletePpDataEntryResponse::Statusdefault
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
fn get_pp_data_entry_validation(
	path_params: models::GetPpDataEntryPathParams,
	query_params: models::GetPpDataEntryQueryParams,
) -> std::result::Result<
	(
		models::GetPpDataEntryPathParams,
		models::GetPpDataEntryQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetPpDataEntry - GET /nudm-pp/v1/{ueId}/pp-data-store/{afInstanceId}
#[tracing::instrument(skip_all)]
async fn get_pp_data_entry<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetPpDataEntryPathParams>,
	Query(query_params): Query<models::GetPpDataEntryQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::parameter_provisioning_data_entry_document::ParameterProvisioningDataEntryDocument,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_pp_data_entry_validation(path_params, query_params)
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
		.get_pp_data_entry(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::parameter_provisioning_data_entry_document::GetPpDataEntryResponse::Status200
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
			apis::parameter_provisioning_data_entry_document::GetPpDataEntryResponse::Status400
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
			apis::parameter_provisioning_data_entry_document::GetPpDataEntryResponse::Status403
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
			apis::parameter_provisioning_data_entry_document::GetPpDataEntryResponse::Status404
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
			apis::parameter_provisioning_data_entry_document::GetPpDataEntryResponse::Status500
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
			apis::parameter_provisioning_data_entry_document::GetPpDataEntryResponse::Status503
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
			apis::parameter_provisioning_data_entry_document::GetPpDataEntryResponse::Statusdefault
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
struct Update3GppRegistrationBodyValidator<'a> {
	body: &'a models::Amf3GppAccessRegistrationModification,
}

#[tracing::instrument(skip_all)]
fn update3_gpp_registration_validation(
	path_params: models::Update3GppRegistrationPathParams,
	query_params: models::Update3GppRegistrationQueryParams,
	body: models::Amf3GppAccessRegistrationModification,
) -> std::result::Result<
	(
		models::Update3GppRegistrationPathParams,
		models::Update3GppRegistrationQueryParams,
		models::Amf3GppAccessRegistrationModification,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = Update3GppRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// Update3GppRegistration - PATCH
/// /nudm-uecm/v1/{ueId}/registrations/amf-3gpp-access
#[tracing::instrument(skip_all)]
async fn update3_gpp_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Update3GppRegistrationPathParams>,
	Query(query_params): Query<models::Update3GppRegistrationQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::Amf3GppAccessRegistrationModification>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::parameter_update_in_the_amf_registration_for3_gpp_access::ParameterUpdateInTheAmfRegistrationFor3GppAccess,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		update3_gpp_registration_validation(path_params, query_params, body)
	})
	.await
	.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.update3_gpp_registration(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::parameter_update_in_the_amf_registration_for3_gpp_access::Update3GppRegistrationResponse::Status200
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
			apis::parameter_update_in_the_amf_registration_for3_gpp_access::Update3GppRegistrationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::parameter_update_in_the_amf_registration_for3_gpp_access::Update3GppRegistrationResponse::Status400
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
			apis::parameter_update_in_the_amf_registration_for3_gpp_access::Update3GppRegistrationResponse::Status403
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
			apis::parameter_update_in_the_amf_registration_for3_gpp_access::Update3GppRegistrationResponse::Status404
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
			apis::parameter_update_in_the_amf_registration_for3_gpp_access::Update3GppRegistrationResponse::Status422
			(body)
			=> {
				let mut response = response.status(422);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::parameter_update_in_the_amf_registration_for3_gpp_access::Update3GppRegistrationResponse::Status500
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
			apis::parameter_update_in_the_amf_registration_for3_gpp_access::Update3GppRegistrationResponse::Status503
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
			apis::parameter_update_in_the_amf_registration_for3_gpp_access::Update3GppRegistrationResponse::Statusdefault
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
struct UpdateNon3GppRegistrationBodyValidator<'a> {
	body: &'a models::AmfNon3GppAccessRegistrationModification,
}

#[tracing::instrument(skip_all)]
fn update_non3_gpp_registration_validation(
	path_params: models::UpdateNon3GppRegistrationPathParams,
	query_params: models::UpdateNon3GppRegistrationQueryParams,
	body: models::AmfNon3GppAccessRegistrationModification,
) -> std::result::Result<
	(
		models::UpdateNon3GppRegistrationPathParams,
		models::UpdateNon3GppRegistrationQueryParams,
		models::AmfNon3GppAccessRegistrationModification,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = UpdateNon3GppRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// UpdateNon3GppRegistration - PATCH
/// /nudm-uecm/v1/{ueId}/registrations/amf-non-3gpp-access
#[tracing::instrument(skip_all)]
async fn update_non3_gpp_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateNon3GppRegistrationPathParams>,
	Query(query_params): Query<models::UpdateNon3GppRegistrationQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AmfNon3GppAccessRegistrationModification>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::ParameterUpdateInTheAmfRegistrationForNon3GppAccess,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		update_non3_gpp_registration_validation(path_params, query_params, body)
	})
	.await
	.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.update_non3_gpp_registration(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::UpdateNon3GppRegistrationResponse::Status200
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
			apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::UpdateNon3GppRegistrationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::UpdateNon3GppRegistrationResponse::Status400
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
			apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::UpdateNon3GppRegistrationResponse::Status403
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
			apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::UpdateNon3GppRegistrationResponse::Status404
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
			apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::UpdateNon3GppRegistrationResponse::Status422
			(body)
			=> {
				let mut response = response.status(422);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::UpdateNon3GppRegistrationResponse::Status500
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
			apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::UpdateNon3GppRegistrationResponse::Status503
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
			apis::parameter_update_in_the_amf_registration_for_non3_gpp_access::UpdateNon3GppRegistrationResponse::Statusdefault
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
struct UpdateNwdafRegistrationBodyValidator<'a> {
	body: &'a models::NwdafRegistrationModification,
}

#[tracing::instrument(skip_all)]
fn update_nwdaf_registration_validation(
	path_params: models::UpdateNwdafRegistrationPathParams,
	query_params: models::UpdateNwdafRegistrationQueryParams,
	body: models::NwdafRegistrationModification,
) -> std::result::Result<
	(
		models::UpdateNwdafRegistrationPathParams,
		models::UpdateNwdafRegistrationQueryParams,
		models::NwdafRegistrationModification,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = UpdateNwdafRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// UpdateNwdafRegistration - PATCH
/// /nudm-uecm/v1/{ueId}/registrations/nwdaf-registrations/{nwdafRegistrationId}
#[tracing::instrument(skip_all)]
async fn update_nwdaf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateNwdafRegistrationPathParams>,
	Query(query_params): Query<models::UpdateNwdafRegistrationQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::NwdafRegistrationModification>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::parameter_update_in_the_nwdaf_registration::ParameterUpdateInTheNwdafRegistration,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		update_nwdaf_registration_validation(path_params, query_params, body)
	})
	.await
	.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.update_nwdaf_registration(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::parameter_update_in_the_nwdaf_registration::UpdateNwdafRegistrationResponse::Status200
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
			apis::parameter_update_in_the_nwdaf_registration::UpdateNwdafRegistrationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::parameter_update_in_the_nwdaf_registration::UpdateNwdafRegistrationResponse::Status400
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
			apis::parameter_update_in_the_nwdaf_registration::UpdateNwdafRegistrationResponse::Status403
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
			apis::parameter_update_in_the_nwdaf_registration::UpdateNwdafRegistrationResponse::Status404
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
			apis::parameter_update_in_the_nwdaf_registration::UpdateNwdafRegistrationResponse::Status422
			(body)
			=> {
				let mut response = response.status(422);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::parameter_update_in_the_nwdaf_registration::UpdateNwdafRegistrationResponse::Status500
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
			apis::parameter_update_in_the_nwdaf_registration::UpdateNwdafRegistrationResponse::Status503
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
			apis::parameter_update_in_the_nwdaf_registration::UpdateNwdafRegistrationResponse::Statusdefault
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
struct UpdateSmfRegistrationBodyValidator<'a> {
	body: &'a models::SmfRegistrationModification,
}

#[tracing::instrument(skip_all)]
fn update_smf_registration_validation(
	path_params: models::UpdateSmfRegistrationPathParams,
	query_params: models::UpdateSmfRegistrationQueryParams,
	body: models::SmfRegistrationModification,
) -> std::result::Result<
	(
		models::UpdateSmfRegistrationPathParams,
		models::UpdateSmfRegistrationQueryParams,
		models::SmfRegistrationModification,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = UpdateSmfRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// UpdateSmfRegistration - PATCH
/// /nudm-uecm/v1/{ueId}/registrations/smf-registrations/{pduSessionId}
#[tracing::instrument(skip_all)]
async fn update_smf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateSmfRegistrationPathParams>,
	Query(query_params): Query<models::UpdateSmfRegistrationQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SmfRegistrationModification>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::parameter_update_in_the_smf_registration::ParameterUpdateInTheSmfRegistration,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		update_smf_registration_validation(path_params, query_params, body)
	})
	.await
	.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.update_smf_registration(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::parameter_update_in_the_smf_registration::UpdateSmfRegistrationResponse::Status200
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
			apis::parameter_update_in_the_smf_registration::UpdateSmfRegistrationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::parameter_update_in_the_smf_registration::UpdateSmfRegistrationResponse::Status400
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
			apis::parameter_update_in_the_smf_registration::UpdateSmfRegistrationResponse::Status404
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
			apis::parameter_update_in_the_smf_registration::UpdateSmfRegistrationResponse::Status422
			(body)
			=> {
				let mut response = response.status(422);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::parameter_update_in_the_smf_registration::UpdateSmfRegistrationResponse::Status500
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
			apis::parameter_update_in_the_smf_registration::UpdateSmfRegistrationResponse::Status503
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
			apis::parameter_update_in_the_smf_registration::UpdateSmfRegistrationResponse::Statusdefault
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
fn get_prose_data_validation(
	header_params: models::GetProseDataHeaderParams,
	path_params: models::GetProseDataPathParams,
	query_params: models::GetProseDataQueryParams,
) -> std::result::Result<
	(
		models::GetProseDataHeaderParams,
		models::GetProseDataPathParams,
		models::GetProseDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetProseData - GET /nudm-sdm/v2/{supi}/prose-data
#[tracing::instrument(skip_all)]
async fn get_prose_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetProseDataPathParams>,
	Query(query_params): Query<models::GetProseDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::prose_subscription_data_retrieval::ProseSubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetProseDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_prose_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_prose_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::prose_subscription_data_retrieval::GetProseDataResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::prose_subscription_data_retrieval::GetProseDataResponse::Status400(body) => {
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
			apis::prose_subscription_data_retrieval::GetProseDataResponse::Status404(body) => {
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
			apis::prose_subscription_data_retrieval::GetProseDataResponse::Status500(body) => {
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
			apis::prose_subscription_data_retrieval::GetProseDataResponse::Status503(body) => {
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
			apis::prose_subscription_data_retrieval::GetProseDataResponse::Statusdefault => {
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
struct ProvideLocationInfoBodyValidator<'a> {
	body: &'a models::LocationInfoRequest,
}

#[tracing::instrument(skip_all)]
fn provide_location_info_validation(
	path_params: models::ProvideLocationInfoPathParams,
	body: models::LocationInfoRequest,
) -> std::result::Result<
	(
		models::ProvideLocationInfoPathParams,
		models::LocationInfoRequest,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ProvideLocationInfoBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ProvideLocationInfo - POST /nudm-mt/v1/{supi}/loc-info/provide-loc-info
#[tracing::instrument(skip_all)]
async fn provide_location_info<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ProvideLocationInfoPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::LocationInfoRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::provide_ue_location::ProvideUeLocation,
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
			apis::provide_ue_location::ProvideLocationInfoResponse::Status200(body) => {
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
			apis::provide_ue_location::ProvideLocationInfoResponse::Status400(body) => {
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
			apis::provide_ue_location::ProvideLocationInfoResponse::Status404(body) => {
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
			apis::provide_ue_location::ProvideLocationInfoResponse::Status500(body) => {
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
			apis::provide_ue_location::ProvideLocationInfoResponse::Status501(body) => {
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
			apis::provide_ue_location::ProvideLocationInfoResponse::Status503(body) => {
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
			apis::provide_ue_location::ProvideLocationInfoResponse::Statusdefault => {
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
struct CAgAckBodyValidator<'a> {
	body: &'a models::AcknowledgeInfo,
}

#[tracing::instrument(skip_all)]
fn cag_ack_validation(
	path_params: models::CAgAckPathParams,
	body: Option<models::AcknowledgeInfo>,
) -> std::result::Result<
	(models::CAgAckPathParams, Option<models::AcknowledgeInfo>),
	ValidationErrors,
> {
	path_params.validate()?;
	if let Some(body) = &body {
		let b = CAgAckBodyValidator { body };
		b.validate()?;
	}

	Ok((path_params, body))
}

/// CAgAck - PUT /nudm-sdm/v2/{supi}/am-data/cag-ack
#[tracing::instrument(skip_all)]
async fn cag_ack<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::CAgAckPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::AcknowledgeInfo>>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::providing_acknowledgement_of_cag_update::ProvidingAcknowledgementOfCagUpdate,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || cag_ack_validation(path_params, body))
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
		.cag_ack(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::providing_acknowledgement_of_cag_update::CAgAckResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::providing_acknowledgement_of_cag_update::CAgAckResponse::Status400(body) => {
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
			apis::providing_acknowledgement_of_cag_update::CAgAckResponse::Status500(body) => {
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
			apis::providing_acknowledgement_of_cag_update::CAgAckResponse::Status503(body) => {
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
			apis::providing_acknowledgement_of_cag_update::CAgAckResponse::Statusdefault => {
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
struct SNssaisAckBodyValidator<'a> {
	body: &'a models::AcknowledgeInfo,
}

#[tracing::instrument(skip_all)]
fn s_nssais_ack_validation(
	path_params: models::SNssaisAckPathParams,
	body: Option<models::AcknowledgeInfo>,
) -> std::result::Result<
	(
		models::SNssaisAckPathParams,
		Option<models::AcknowledgeInfo>,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	if let Some(body) = &body {
		let b = SNssaisAckBodyValidator { body };
		b.validate()?;
	}

	Ok((path_params, body))
}

/// SNssaisAck - PUT /nudm-sdm/v2/{supi}/am-data/subscribed-snssais-ack
#[tracing::instrument(skip_all)]
async fn s_nssais_ack<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::SNssaisAckPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::AcknowledgeInfo>>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::providing_acknowledgement_of_snssais_update::ProvidingAcknowledgementOfSnssaisUpdate,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || s_nssais_ack_validation(path_params, body))
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
		.s_nssais_ack(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::providing_acknowledgement_of_snssais_update::SNssaisAckResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::providing_acknowledgement_of_snssais_update::SNssaisAckResponse::Status400
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
			apis::providing_acknowledgement_of_snssais_update::SNssaisAckResponse::Status500
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
			apis::providing_acknowledgement_of_snssais_update::SNssaisAckResponse::Status503
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
			apis::providing_acknowledgement_of_snssais_update::SNssaisAckResponse::Statusdefault
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
struct SorAckInfoBodyValidator<'a> {
	body: &'a models::AcknowledgeInfo,
}

#[tracing::instrument(skip_all)]
fn sor_ack_info_validation(
	path_params: models::SorAckInfoPathParams,
	body: Option<models::AcknowledgeInfo>,
) -> std::result::Result<
	(
		models::SorAckInfoPathParams,
		Option<models::AcknowledgeInfo>,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	if let Some(body) = &body {
		let b = SorAckInfoBodyValidator { body };
		b.validate()?;
	}

	Ok((path_params, body))
}

/// SorAckInfo - PUT /nudm-sdm/v2/{supi}/am-data/sor-ack
#[tracing::instrument(skip_all)]
async fn sor_ack_info<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::SorAckInfoPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::AcknowledgeInfo>>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::providing_acknowledgement_of_steering_of_roaming::ProvidingAcknowledgementOfSteeringOfRoaming,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || sor_ack_info_validation(path_params, body))
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
		.sor_ack_info(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::providing_acknowledgement_of_steering_of_roaming::SorAckInfoResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::providing_acknowledgement_of_steering_of_roaming::SorAckInfoResponse::Status400
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
			apis::providing_acknowledgement_of_steering_of_roaming::SorAckInfoResponse::Status500
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
			apis::providing_acknowledgement_of_steering_of_roaming::SorAckInfoResponse::Status503
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
			apis::providing_acknowledgement_of_steering_of_roaming::SorAckInfoResponse::Statusdefault
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
struct UpuAckBodyValidator<'a> {
	body: &'a models::AcknowledgeInfo,
}

#[tracing::instrument(skip_all)]
fn upu_ack_validation(
	path_params: models::UpuAckPathParams,
	body: Option<models::AcknowledgeInfo>,
) -> std::result::Result<
	(models::UpuAckPathParams, Option<models::AcknowledgeInfo>),
	ValidationErrors,
> {
	path_params.validate()?;
	if let Some(body) = &body {
		let b = UpuAckBodyValidator { body };
		b.validate()?;
	}

	Ok((path_params, body))
}

/// UpuAck - PUT /nudm-sdm/v2/{supi}/am-data/upu-ack
#[tracing::instrument(skip_all)]
async fn upu_ack<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpuAckPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::AcknowledgeInfo>>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::providing_acknowledgement_of_ue_parameters_update::ProvidingAcknowledgementOfUeParametersUpdate,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || upu_ack_validation(path_params, body))
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
		.upu_ack(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::providing_acknowledgement_of_ue_parameters_update::UpuAckResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::providing_acknowledgement_of_ue_parameters_update::UpuAckResponse::Status400
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
			apis::providing_acknowledgement_of_ue_parameters_update::UpuAckResponse::Status500
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
			apis::providing_acknowledgement_of_ue_parameters_update::UpuAckResponse::Status503
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
			apis::providing_acknowledgement_of_ue_parameters_update::UpuAckResponse::Statusdefault
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
fn query_ue_info_validation(
	path_params: models::QueryUeInfoPathParams,
	query_params: models::QueryUeInfoQueryParams,
) -> std::result::Result<
	(
		models::QueryUeInfoPathParams,
		models::QueryUeInfoQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// QueryUeInfo - GET /nudm-mt/v1/{supi}
#[tracing::instrument(skip_all)]
async fn query_ue_info<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::QueryUeInfoPathParams>,
	Query(query_params): Query<models::QueryUeInfoQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::query_ue_info::QueryUeInfo,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || query_ue_info_validation(path_params, query_params))
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
		.query_ue_info(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::query_ue_info::QueryUeInfoResponse::Status200(body) => {
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
			apis::query_ue_info::QueryUeInfoResponse::Status400(body) => {
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
			apis::query_ue_info::QueryUeInfoResponse::Status404(body) => {
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
			apis::query_ue_info::QueryUeInfoResponse::Status500(body) => {
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
			apis::query_ue_info::QueryUeInfoResponse::Status501(body) => {
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
			apis::query_ue_info::QueryUeInfoResponse::Status503(body) => {
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
			apis::query_ue_info::QueryUeInfoResponse::Statusdefault => {
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
struct ReportSmDeliveryStatusBodyValidator<'a> {
	body: &'a models::SmDeliveryStatus,
}

#[tracing::instrument(skip_all)]
fn report_sm_delivery_status_validation(
	path_params: models::ReportSmDeliveryStatusPathParams,
	body: models::SmDeliveryStatus,
) -> std::result::Result<
	(
		models::ReportSmDeliveryStatusPathParams,
		models::SmDeliveryStatus,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ReportSmDeliveryStatusBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ReportSmDeliveryStatus - POST /nudm-rsds/v1/{ueIdentity}/sm-delivery-status
#[tracing::instrument(skip_all)]
async fn report_sm_delivery_status<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ReportSmDeliveryStatusPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SmDeliveryStatus>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::report_sm_delivery_status::ReportSmDeliveryStatus,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		report_sm_delivery_status_validation(path_params, body)
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
		.report_sm_delivery_status(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::report_sm_delivery_status::ReportSmDeliveryStatusResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::report_sm_delivery_status::ReportSmDeliveryStatusResponse::Status400(body) => {
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
			apis::report_sm_delivery_status::ReportSmDeliveryStatusResponse::Status404(body) => {
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
			apis::report_sm_delivery_status::ReportSmDeliveryStatusResponse::Status500(body) => {
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
			apis::report_sm_delivery_status::ReportSmDeliveryStatusResponse::Status503(body) => {
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
			apis::report_sm_delivery_status::ReportSmDeliveryStatusResponse::Statusdefault => {
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
fn get_data_sets_validation(
	header_params: models::GetDataSetsHeaderParams,
	path_params: models::GetDataSetsPathParams,
	query_params: models::GetDataSetsQueryParams,
) -> std::result::Result<
	(
		models::GetDataSetsHeaderParams,
		models::GetDataSetsPathParams,
		models::GetDataSetsQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetDataSets - GET /nudm-sdm/v2/{supi}
#[tracing::instrument(skip_all)]
async fn get_data_sets<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetDataSetsPathParams>,
	Query(query_params): Query<models::GetDataSetsQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::retrieval_of_multiple_data_sets::RetrievalOfMultipleDataSets,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetDataSetsHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_data_sets_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_data_sets(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::retrieval_of_multiple_data_sets::GetDataSetsResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::retrieval_of_multiple_data_sets::GetDataSetsResponse::Status400(body) => {
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
			apis::retrieval_of_multiple_data_sets::GetDataSetsResponse::Status404(body) => {
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
			apis::retrieval_of_multiple_data_sets::GetDataSetsResponse::Status500(body) => {
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
			apis::retrieval_of_multiple_data_sets::GetDataSetsResponse::Status503(body) => {
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
			apis::retrieval_of_multiple_data_sets::GetDataSetsResponse::Statusdefault => {
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
fn get_shared_data_validation(
	header_params: models::GetSharedDataHeaderParams,
	query_params: models::GetSharedDataQueryParams,
) -> std::result::Result<
	(
		models::GetSharedDataHeaderParams,
		models::GetSharedDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	query_params.validate()?;

	Ok((header_params, query_params))
}

/// GetSharedData - GET /nudm-sdm/v2/shared-data
#[tracing::instrument(skip_all)]
async fn get_shared_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Query(query_params): Query<models::GetSharedDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::retrieval_of_shared_data::RetrievalOfSharedData,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetSharedDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_shared_data_validation(header_params, query_params)
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
		.get_shared_data(method, host, cookies, header_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::retrieval_of_shared_data::GetSharedDataResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::retrieval_of_shared_data::GetSharedDataResponse::Status400(body) => {
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
			apis::retrieval_of_shared_data::GetSharedDataResponse::Status404(body) => {
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
			apis::retrieval_of_shared_data::GetSharedDataResponse::Status500(body) => {
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
			apis::retrieval_of_shared_data::GetSharedDataResponse::Status503(body) => {
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
			apis::retrieval_of_shared_data::GetSharedDataResponse::Statusdefault => {
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
fn get_individual_shared_data_validation(
	header_params: models::GetIndividualSharedDataHeaderParams,
	path_params: models::GetIndividualSharedDataPathParams,
	query_params: models::GetIndividualSharedDataQueryParams,
) -> std::result::Result<
	(
		models::GetIndividualSharedDataHeaderParams,
		models::GetIndividualSharedDataPathParams,
		models::GetIndividualSharedDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetIndividualSharedData - GET /nudm-sdm/v2/shared-data/{sharedDataId}
#[tracing::instrument(skip_all)]
async fn get_individual_shared_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetIndividualSharedDataPathParams>,
	Query(query_params): Query<models::GetIndividualSharedDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::retrieval_of_the_individual_shared_data::RetrievalOfTheIndividualSharedData,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetIndividualSharedDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_individual_shared_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_individual_shared_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::retrieval_of_the_individual_shared_data::GetIndividualSharedDataResponse::Status200
			{
				body,
				cache_control,
				etag,
				last_modified
			}
			=> {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling cache_control header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							cache_control,
						);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling last_modified header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							last_modified,
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
			apis::retrieval_of_the_individual_shared_data::GetIndividualSharedDataResponse::Status400
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
			apis::retrieval_of_the_individual_shared_data::GetIndividualSharedDataResponse::Status404
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
			apis::retrieval_of_the_individual_shared_data::GetIndividualSharedDataResponse::Status500
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
			apis::retrieval_of_the_individual_shared_data::GetIndividualSharedDataResponse::Status503
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
			apis::retrieval_of_the_individual_shared_data::GetIndividualSharedDataResponse::Statusdefault
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
fn retrieve_smf_registration_validation(
	path_params: models::RetrieveSmfRegistrationPathParams
) -> std::result::Result<(models::RetrieveSmfRegistrationPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// RetrieveSmfRegistration - GET
/// /nudm-uecm/v1/{ueId}/registrations/smf-registrations/{pduSessionId}
#[tracing::instrument(skip_all)]
async fn retrieve_smf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::RetrieveSmfRegistrationPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::retrieve_smf_registration::RetrieveSmfRegistration,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || retrieve_smf_registration_validation(path_params))
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
		.retrieve_smf_registration(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::retrieve_smf_registration::RetrieveSmfRegistrationResponse::Status200(body) => {
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
			apis::retrieve_smf_registration::RetrieveSmfRegistrationResponse::Status400(body) => {
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
			apis::retrieve_smf_registration::RetrieveSmfRegistrationResponse::Status403(body) => {
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
			apis::retrieve_smf_registration::RetrieveSmfRegistrationResponse::Status404(body) => {
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
			apis::retrieve_smf_registration::RetrieveSmfRegistrationResponse::Status500(body) => {
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
			apis::retrieve_smf_registration::RetrieveSmfRegistrationResponse::Status503(body) => {
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
			apis::retrieve_smf_registration::RetrieveSmfRegistrationResponse::Statusdefault => {
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
struct UpdateRoamingInformationBodyValidator<'a> {
	body: &'a models::RoamingInfoUpdate,
}

#[tracing::instrument(skip_all)]
fn update_roaming_information_validation(
	path_params: models::UpdateRoamingInformationPathParams,
	body: models::RoamingInfoUpdate,
) -> std::result::Result<
	(
		models::UpdateRoamingInformationPathParams,
		models::RoamingInfoUpdate,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = UpdateRoamingInformationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// UpdateRoamingInformation - POST
/// /nudm-uecm/v1/{ueId}/registrations/amf-3gpp-access/roaming-info-update
#[tracing::instrument(skip_all)]
async fn update_roaming_information<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateRoamingInformationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::RoamingInfoUpdate>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::roaming_information_update::RoamingInformationUpdate,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		update_roaming_information_validation(path_params, body)
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
		.update_roaming_information(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::roaming_information_update::UpdateRoamingInformationResponse::Status201 {
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
			apis::roaming_information_update::UpdateRoamingInformationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::roaming_information_update::UpdateRoamingInformationResponse::Status400(body) => {
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
			apis::roaming_information_update::UpdateRoamingInformationResponse::Status403(body) => {
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
			apis::roaming_information_update::UpdateRoamingInformationResponse::Status404(body) => {
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
			apis::roaming_information_update::UpdateRoamingInformationResponse::Status500(body) => {
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
			apis::roaming_information_update::UpdateRoamingInformationResponse::Status503(body) => {
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
			apis::roaming_information_update::UpdateRoamingInformationResponse::Statusdefault => {
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
fn smf_deregistration_validation(
	path_params: models::SmfDeregistrationPathParams,
	query_params: models::SmfDeregistrationQueryParams,
) -> std::result::Result<
	(
		models::SmfDeregistrationPathParams,
		models::SmfDeregistrationQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// SmfDeregistration - DELETE
/// /nudm-uecm/v1/{ueId}/registrations/smf-registrations/{pduSessionId}
#[tracing::instrument(skip_all)]
async fn smf_deregistration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::SmfDeregistrationPathParams>,
	Query(query_params): Query<models::SmfDeregistrationQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::smf_deregistration::SmfDeregistration,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		smf_deregistration_validation(path_params, query_params)
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
		.smf_deregistration(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smf_deregistration::SmfDeregistrationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::smf_deregistration::SmfDeregistrationResponse::Status400(body) => {
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
			apis::smf_deregistration::SmfDeregistrationResponse::Status404(body) => {
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
			apis::smf_deregistration::SmfDeregistrationResponse::Status422(body) => {
				let mut response = response.status(422);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?,
					);
				}

				let body_content = tokio::task::spawn_blocking(move || {
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})
				})
				.await
				.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::smf_deregistration::SmfDeregistrationResponse::Status500(body) => {
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
			apis::smf_deregistration::SmfDeregistrationResponse::Status503(body) => {
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
			apis::smf_deregistration::SmfDeregistrationResponse::Statusdefault => {
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
fn get_smf_sel_data_validation(
	header_params: models::GetSmfSelDataHeaderParams,
	path_params: models::GetSmfSelDataPathParams,
	query_params: models::GetSmfSelDataQueryParams,
) -> std::result::Result<
	(
		models::GetSmfSelDataHeaderParams,
		models::GetSmfSelDataPathParams,
		models::GetSmfSelDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetSmfSelData - GET /nudm-sdm/v2/{supi}/smf-select-data
#[tracing::instrument(skip_all)]
async fn get_smf_sel_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetSmfSelDataPathParams>,
	Query(query_params): Query<models::GetSmfSelDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::smf_selection_subscription_data_retrieval::SmfSelectionSubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetSmfSelDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_smf_sel_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_smf_sel_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smf_selection_subscription_data_retrieval::GetSmfSelDataResponse::Status200
			{
				body,
				cache_control,
				etag,
				last_modified
			}
			=> {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling cache_control header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							cache_control,
						);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling last_modified header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							last_modified,
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
			apis::smf_selection_subscription_data_retrieval::GetSmfSelDataResponse::Status400
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
			apis::smf_selection_subscription_data_retrieval::GetSmfSelDataResponse::Status404
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
			apis::smf_selection_subscription_data_retrieval::GetSmfSelDataResponse::Status500
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
			apis::smf_selection_subscription_data_retrieval::GetSmfSelDataResponse::Status503
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
			apis::smf_selection_subscription_data_retrieval::GetSmfSelDataResponse::Statusdefault
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
fn get_smf_registration_validation(
	path_params: models::GetSmfRegistrationPathParams,
	query_params: models::GetSmfRegistrationQueryParams,
) -> std::result::Result<
	(
		models::GetSmfRegistrationPathParams,
		models::GetSmfRegistrationQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetSmfRegistration - GET
/// /nudm-uecm/v1/{ueId}/registrations/smf-registrations
#[tracing::instrument(skip_all)]
async fn get_smf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetSmfRegistrationPathParams>,
	Query(query_params): Query<models::GetSmfRegistrationQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::smf_smf_registration::SmfSmfRegistration,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_smf_registration_validation(path_params, query_params)
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
		.get_smf_registration(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smf_smf_registration::GetSmfRegistrationResponse::Status200(body) => {
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
			apis::smf_smf_registration::GetSmfRegistrationResponse::Status400(body) => {
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
			apis::smf_smf_registration::GetSmfRegistrationResponse::Status403(body) => {
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
			apis::smf_smf_registration::GetSmfRegistrationResponse::Status404(body) => {
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
			apis::smf_smf_registration::GetSmfRegistrationResponse::Status500(body) => {
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
			apis::smf_smf_registration::GetSmfRegistrationResponse::Status503(body) => {
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
			apis::smf_smf_registration::GetSmfRegistrationResponse::Statusdefault => {
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
struct RegistrationBodyValidator<'a> {
	body: &'a models::SmfRegistration,
}

#[tracing::instrument(skip_all)]
fn registration_validation(
	path_params: models::RegistrationPathParams,
	body: models::SmfRegistration,
) -> std::result::Result<(models::RegistrationPathParams, models::SmfRegistration), ValidationErrors>
{
	path_params.validate()?;
	let b = RegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// Registration - PUT
/// /nudm-uecm/v1/{ueId}/registrations/smf-registrations/{pduSessionId}
#[tracing::instrument(skip_all)]
async fn registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::RegistrationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SmfRegistration>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::smf_smf_registration::SmfSmfRegistration,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || registration_validation(path_params, body))
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
		.registration(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smf_smf_registration::RegistrationResponse::Status200(body) => {
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
			apis::smf_smf_registration::RegistrationResponse::Status201 { body, location } => {
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
			apis::smf_smf_registration::RegistrationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::smf_smf_registration::RegistrationResponse::Status400(body) => {
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
			apis::smf_smf_registration::RegistrationResponse::Status403(body) => {
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
			apis::smf_smf_registration::RegistrationResponse::Status404(body) => {
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
			apis::smf_smf_registration::RegistrationResponse::Status500(body) => {
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
			apis::smf_smf_registration::RegistrationResponse::Status503(body) => {
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
			apis::smf_smf_registration::RegistrationResponse::Statusdefault => {
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
fn get3_gpp_smsf_registration_validation(
	path_params: models::Get3GppSmsfRegistrationPathParams,
	query_params: models::Get3GppSmsfRegistrationQueryParams,
) -> std::result::Result<
	(
		models::Get3GppSmsfRegistrationPathParams,
		models::Get3GppSmsfRegistrationQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// Get3GppSmsfRegistration - GET
/// /nudm-uecm/v1/{ueId}/registrations/smsf-3gpp-access
#[tracing::instrument(skip_all)]
async fn get3_gpp_smsf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Get3GppSmsfRegistrationPathParams>,
	Query(query_params): Query<models::Get3GppSmsfRegistrationQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::smsf3_gpp_access_registration_info_retrieval::Smsf3GppAccessRegistrationInfoRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get3_gpp_smsf_registration_validation(path_params, query_params)
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
		.get3_gpp_smsf_registration(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smsf3_gpp_access_registration_info_retrieval::Get3GppSmsfRegistrationResponse::Status200
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
			apis::smsf3_gpp_access_registration_info_retrieval::Get3GppSmsfRegistrationResponse::Status400
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
			apis::smsf3_gpp_access_registration_info_retrieval::Get3GppSmsfRegistrationResponse::Status403
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
			apis::smsf3_gpp_access_registration_info_retrieval::Get3GppSmsfRegistrationResponse::Status404
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
			apis::smsf3_gpp_access_registration_info_retrieval::Get3GppSmsfRegistrationResponse::Status500
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
			apis::smsf3_gpp_access_registration_info_retrieval::Get3GppSmsfRegistrationResponse::Status503
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
			apis::smsf3_gpp_access_registration_info_retrieval::Get3GppSmsfRegistrationResponse::Statusdefault
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
fn call3_gpp_smsf_deregistration_validation(
	header_params: models::Call3GppSmsfDeregistrationHeaderParams,
	path_params: models::Call3GppSmsfDeregistrationPathParams,
	query_params: models::Call3GppSmsfDeregistrationQueryParams,
) -> std::result::Result<
	(
		models::Call3GppSmsfDeregistrationHeaderParams,
		models::Call3GppSmsfDeregistrationPathParams,
		models::Call3GppSmsfDeregistrationQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// Call3GppSmsfDeregistration - DELETE
/// /nudm-uecm/v1/{ueId}/registrations/smsf-3gpp-access
#[tracing::instrument(skip_all)]
async fn call3_gpp_smsf_deregistration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::Call3GppSmsfDeregistrationPathParams>,
	Query(query_params): Query<models::Call3GppSmsfDeregistrationQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::smsf_deregistration_for3_gpp_access::SmsfDeregistrationFor3GppAccess,
{
	// Header parameters
	let header_params = {
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

		models::Call3GppSmsfDeregistrationHeaderParams {
			if_match: header_if_match,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		call3_gpp_smsf_deregistration_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.call3_gpp_smsf_deregistration(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smsf_deregistration_for3_gpp_access::Call3GppSmsfDeregistrationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::smsf_deregistration_for3_gpp_access::Call3GppSmsfDeregistrationResponse::Status400
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
			apis::smsf_deregistration_for3_gpp_access::Call3GppSmsfDeregistrationResponse::Status404
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
			apis::smsf_deregistration_for3_gpp_access::Call3GppSmsfDeregistrationResponse::Status422
			(body)
			=> {
				let mut response = response.status(422);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::smsf_deregistration_for3_gpp_access::Call3GppSmsfDeregistrationResponse::Status500
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
			apis::smsf_deregistration_for3_gpp_access::Call3GppSmsfDeregistrationResponse::Status503
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
			apis::smsf_deregistration_for3_gpp_access::Call3GppSmsfDeregistrationResponse::Statusdefault
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
fn non3_gpp_smsf_deregistration_validation(
	header_params: models::Non3GppSmsfDeregistrationHeaderParams,
	path_params: models::Non3GppSmsfDeregistrationPathParams,
	query_params: models::Non3GppSmsfDeregistrationQueryParams,
) -> std::result::Result<
	(
		models::Non3GppSmsfDeregistrationHeaderParams,
		models::Non3GppSmsfDeregistrationPathParams,
		models::Non3GppSmsfDeregistrationQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// Non3GppSmsfDeregistration - DELETE
/// /nudm-uecm/v1/{ueId}/registrations/smsf-non-3gpp-access
#[tracing::instrument(skip_all)]
async fn non3_gpp_smsf_deregistration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::Non3GppSmsfDeregistrationPathParams>,
	Query(query_params): Query<models::Non3GppSmsfDeregistrationQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::smsf_deregistration_for_non3_gpp_access::SmsfDeregistrationForNon3GppAccess,
{
	// Header parameters
	let header_params = {
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

		models::Non3GppSmsfDeregistrationHeaderParams {
			if_match: header_if_match,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		non3_gpp_smsf_deregistration_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.non3_gpp_smsf_deregistration(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smsf_deregistration_for_non3_gpp_access::Non3GppSmsfDeregistrationResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::smsf_deregistration_for_non3_gpp_access::Non3GppSmsfDeregistrationResponse::Status400
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
			apis::smsf_deregistration_for_non3_gpp_access::Non3GppSmsfDeregistrationResponse::Status404
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
			apis::smsf_deregistration_for_non3_gpp_access::Non3GppSmsfDeregistrationResponse::Status422
			(body)
			=> {
				let mut response = response.status(422);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::smsf_deregistration_for_non3_gpp_access::Non3GppSmsfDeregistrationResponse::Status500
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
			apis::smsf_deregistration_for_non3_gpp_access::Non3GppSmsfDeregistrationResponse::Status503
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
			apis::smsf_deregistration_for_non3_gpp_access::Non3GppSmsfDeregistrationResponse::Statusdefault
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
fn get_non3_gpp_smsf_registration_validation(
	path_params: models::GetNon3GppSmsfRegistrationPathParams,
	query_params: models::GetNon3GppSmsfRegistrationQueryParams,
) -> std::result::Result<
	(
		models::GetNon3GppSmsfRegistrationPathParams,
		models::GetNon3GppSmsfRegistrationQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetNon3GppSmsfRegistration - GET
/// /nudm-uecm/v1/{ueId}/registrations/smsf-non-3gpp-access
#[tracing::instrument(skip_all)]
async fn get_non3_gpp_smsf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetNon3GppSmsfRegistrationPathParams>,
	Query(query_params): Query<models::GetNon3GppSmsfRegistrationQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::smsf_non3_gpp_access_registration_info_retrieval::SmsfNon3GppAccessRegistrationInfoRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_non3_gpp_smsf_registration_validation(path_params, query_params)
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
		.get_non3_gpp_smsf_registration(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smsf_non3_gpp_access_registration_info_retrieval::GetNon3GppSmsfRegistrationResponse::Status200
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
			apis::smsf_non3_gpp_access_registration_info_retrieval::GetNon3GppSmsfRegistrationResponse::Status400
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
			apis::smsf_non3_gpp_access_registration_info_retrieval::GetNon3GppSmsfRegistrationResponse::Status403
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
			apis::smsf_non3_gpp_access_registration_info_retrieval::GetNon3GppSmsfRegistrationResponse::Status404
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
			apis::smsf_non3_gpp_access_registration_info_retrieval::GetNon3GppSmsfRegistrationResponse::Status500
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
			apis::smsf_non3_gpp_access_registration_info_retrieval::GetNon3GppSmsfRegistrationResponse::Status503
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
			apis::smsf_non3_gpp_access_registration_info_retrieval::GetNon3GppSmsfRegistrationResponse::Statusdefault
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
struct Call3GppSmsfRegistrationBodyValidator<'a> {
	body: &'a models::SmsfRegistration,
}

#[tracing::instrument(skip_all)]
fn call3_gpp_smsf_registration_validation(
	path_params: models::Call3GppSmsfRegistrationPathParams,
	body: models::SmsfRegistration,
) -> std::result::Result<
	(
		models::Call3GppSmsfRegistrationPathParams,
		models::SmsfRegistration,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = Call3GppSmsfRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// Call3GppSmsfRegistration - PUT
/// /nudm-uecm/v1/{ueId}/registrations/smsf-3gpp-access
#[tracing::instrument(skip_all)]
async fn call3_gpp_smsf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Call3GppSmsfRegistrationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SmsfRegistration>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::smsf_registration_for3_gpp_access::SmsfRegistrationFor3GppAccess,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		call3_gpp_smsf_registration_validation(path_params, body)
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
		.call3_gpp_smsf_registration(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smsf_registration_for3_gpp_access::Call3GppSmsfRegistrationResponse::Status200
			{
				body,
				etag
			}
			=> {
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
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
			apis::smsf_registration_for3_gpp_access::Call3GppSmsfRegistrationResponse::Status201
			{
				body,
				location,
				etag
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
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
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
			apis::smsf_registration_for3_gpp_access::Call3GppSmsfRegistrationResponse::Status204
			{
				etag
			}
			=> {
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::smsf_registration_for3_gpp_access::Call3GppSmsfRegistrationResponse::Status400
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
			apis::smsf_registration_for3_gpp_access::Call3GppSmsfRegistrationResponse::Status403
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
			apis::smsf_registration_for3_gpp_access::Call3GppSmsfRegistrationResponse::Status404
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
			apis::smsf_registration_for3_gpp_access::Call3GppSmsfRegistrationResponse::Status500
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
			apis::smsf_registration_for3_gpp_access::Call3GppSmsfRegistrationResponse::Status503
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
			apis::smsf_registration_for3_gpp_access::Call3GppSmsfRegistrationResponse::Statusdefault
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
struct Non3GppSmsfRegistrationBodyValidator<'a> {
	body: &'a models::SmsfRegistration,
}

#[tracing::instrument(skip_all)]
fn non3_gpp_smsf_registration_validation(
	path_params: models::Non3GppSmsfRegistrationPathParams,
	body: models::SmsfRegistration,
) -> std::result::Result<
	(
		models::Non3GppSmsfRegistrationPathParams,
		models::SmsfRegistration,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = Non3GppSmsfRegistrationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// Non3GppSmsfRegistration - PUT
/// /nudm-uecm/v1/{ueId}/registrations/smsf-non-3gpp-access
#[tracing::instrument(skip_all)]
async fn non3_gpp_smsf_registration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::Non3GppSmsfRegistrationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SmsfRegistration>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::smsf_registration_for_non3_gpp_access::SmsfRegistrationForNon3GppAccess,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		non3_gpp_smsf_registration_validation(path_params, body)
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
		.non3_gpp_smsf_registration(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::smsf_registration_for_non3_gpp_access::Non3GppSmsfRegistrationResponse::Status200
			{
				body,
				etag
			}
			=> {
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
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
			apis::smsf_registration_for_non3_gpp_access::Non3GppSmsfRegistrationResponse::Status201
			{
				body,
				location,
				etag
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
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
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
			apis::smsf_registration_for_non3_gpp_access::Non3GppSmsfRegistrationResponse::Status204
			{
				etag
			}
			=> {
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::smsf_registration_for_non3_gpp_access::Non3GppSmsfRegistrationResponse::Status400
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
			apis::smsf_registration_for_non3_gpp_access::Non3GppSmsfRegistrationResponse::Status403
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
			apis::smsf_registration_for_non3_gpp_access::Non3GppSmsfRegistrationResponse::Status404
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
			apis::smsf_registration_for_non3_gpp_access::Non3GppSmsfRegistrationResponse::Status500
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
			apis::smsf_registration_for_non3_gpp_access::Non3GppSmsfRegistrationResponse::Status503
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
			apis::smsf_registration_for_non3_gpp_access::Non3GppSmsfRegistrationResponse::Statusdefault
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
fn get_sms_mngt_data_validation(
	header_params: models::GetSmsMngtDataHeaderParams,
	path_params: models::GetSmsMngtDataPathParams,
	query_params: models::GetSmsMngtDataQueryParams,
) -> std::result::Result<
	(
		models::GetSmsMngtDataHeaderParams,
		models::GetSmsMngtDataPathParams,
		models::GetSmsMngtDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetSmsMngtData - GET /nudm-sdm/v2/{supi}/sms-mng-data
#[tracing::instrument(skip_all)]
async fn get_sms_mngt_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetSmsMngtDataPathParams>,
	Query(query_params): Query<models::GetSmsMngtDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::sms_management_subscription_data_retrieval::SmsManagementSubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetSmsMngtDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_sms_mngt_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_sms_mngt_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::sms_management_subscription_data_retrieval::GetSmsMngtDataResponse::Status200
			{
				body,
				cache_control,
				etag,
				last_modified
			}
			=> {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling cache_control header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							cache_control,
						);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling last_modified header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							last_modified,
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
			apis::sms_management_subscription_data_retrieval::GetSmsMngtDataResponse::Status400
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
			apis::sms_management_subscription_data_retrieval::GetSmsMngtDataResponse::Status404
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
			apis::sms_management_subscription_data_retrieval::GetSmsMngtDataResponse::Status500
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
			apis::sms_management_subscription_data_retrieval::GetSmsMngtDataResponse::Status503
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
			apis::sms_management_subscription_data_retrieval::GetSmsMngtDataResponse::Statusdefault
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
fn get_sms_data_validation(
	header_params: models::GetSmsDataHeaderParams,
	path_params: models::GetSmsDataPathParams,
	query_params: models::GetSmsDataQueryParams,
) -> std::result::Result<
	(
		models::GetSmsDataHeaderParams,
		models::GetSmsDataPathParams,
		models::GetSmsDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetSmsData - GET /nudm-sdm/v2/{supi}/sms-data
#[tracing::instrument(skip_all)]
async fn get_sms_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetSmsDataPathParams>,
	Query(query_params): Query<models::GetSmsDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::sms_subscription_data_retrieval::SmsSubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetSmsDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_sms_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_sms_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::sms_subscription_data_retrieval::GetSmsDataResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::sms_subscription_data_retrieval::GetSmsDataResponse::Status400(body) => {
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
			apis::sms_subscription_data_retrieval::GetSmsDataResponse::Status404(body) => {
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
			apis::sms_subscription_data_retrieval::GetSmsDataResponse::Status500(body) => {
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
			apis::sms_subscription_data_retrieval::GetSmsDataResponse::Status503(body) => {
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
			apis::sms_subscription_data_retrieval::GetSmsDataResponse::Statusdefault => {
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
struct SendRoutingInfoSmBodyValidator<'a> {
	body: &'a models::RoutingInfoSmRequest,
}

#[tracing::instrument(skip_all)]
fn send_routing_info_sm_validation(
	path_params: models::SendRoutingInfoSmPathParams,
	body: models::RoutingInfoSmRequest,
) -> std::result::Result<
	(
		models::SendRoutingInfoSmPathParams,
		models::RoutingInfoSmRequest,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = SendRoutingInfoSmBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// SendRoutingInfoSm - POST
/// /nudm-uecm/v1/{ueId}/registrations/send-routing-info-sm
#[tracing::instrument(skip_all)]
async fn send_routing_info_sm<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::SendRoutingInfoSmPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::RoutingInfoSmRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::send_routing_info_sm_custom_operation::SendRoutingInfoSmCustomOperation,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || send_routing_info_sm_validation(path_params, body))
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
		.send_routing_info_sm(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::send_routing_info_sm_custom_operation::SendRoutingInfoSmResponse::Status200
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
			apis::send_routing_info_sm_custom_operation::SendRoutingInfoSmResponse::Status400
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
			apis::send_routing_info_sm_custom_operation::SendRoutingInfoSmResponse::Status403
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
			apis::send_routing_info_sm_custom_operation::SendRoutingInfoSmResponse::Status404
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
			apis::send_routing_info_sm_custom_operation::SendRoutingInfoSmResponse::Status500
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
			apis::send_routing_info_sm_custom_operation::SendRoutingInfoSmResponse::Status503
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
			apis::send_routing_info_sm_custom_operation::SendRoutingInfoSmResponse::Statusdefault
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
struct ServiceSpecificAuthorizationRemovalBodyValidator<'a> {
	body: &'a models::ServiceSpecificAuthorizationRemoveData,
}

#[tracing::instrument(skip_all)]
fn service_specific_authorization_removal_validation(
	path_params: models::ServiceSpecificAuthorizationRemovalPathParams,
	body: models::ServiceSpecificAuthorizationRemoveData,
) -> std::result::Result<
	(
		models::ServiceSpecificAuthorizationRemovalPathParams,
		models::ServiceSpecificAuthorizationRemoveData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ServiceSpecificAuthorizationRemovalBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ServiceSpecificAuthorizationRemoval - POST
/// /nudm-ssau/v1/{ueIdentity}/{serviceType}/remove
#[tracing::instrument(skip_all)]
async fn service_specific_authorization_removal<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ServiceSpecificAuthorizationRemovalPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::ServiceSpecificAuthorizationRemoveData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemove,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		service_specific_authorization_removal_validation(path_params, body)
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
		.service_specific_authorization_removal(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status400
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status401
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status403
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status404
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status411
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status413
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status415
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status429
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status500
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status501
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status502
			(body)
			=> {
				let mut response = response.status(502);
				{
					let mut response_headers = response.headers_mut().unwrap();
					response_headers.insert(
						CONTENT_TYPE,
						HeaderValue::from_str("application/problem+json").map_err(|e| {
							error!(error = ?e);
							StatusCode::INTERNAL_SERVER_ERROR
						})?);
				}

				let body_content = tokio::task::spawn_blocking(move ||
					serde_json::to_vec(&body).map_err(|e| {
						error!(error = ?e);
						StatusCode::INTERNAL_SERVER_ERROR
					})).await.unwrap()?;
				response.body(Body::from(body_content))
			}
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Status503
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
			apis::service_specific_authorization_remove::ServiceSpecificAuthorizationRemovalResponse::Statusdefault
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
struct ServiceSpecificAuthorizationBodyValidator<'a> {
	body: &'a models::ServiceSpecificAuthorizationInfo,
}

#[tracing::instrument(skip_all)]
fn service_specific_authorization_validation(
	path_params: models::ServiceSpecificAuthorizationPathParams,
	body: models::ServiceSpecificAuthorizationInfo,
) -> std::result::Result<
	(
		models::ServiceSpecificAuthorizationPathParams,
		models::ServiceSpecificAuthorizationInfo,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	let b = ServiceSpecificAuthorizationBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// ServiceSpecificAuthorization - POST
/// /nudm-ssau/v1/{ueIdentity}/{serviceType}/authorize
#[tracing::instrument(skip_all)]
async fn service_specific_authorization<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ServiceSpecificAuthorizationPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::ServiceSpecificAuthorizationInfo>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::service_specific_authorization_request::ServiceSpecificAuthorizationRequest,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		service_specific_authorization_validation(path_params, body)
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
		.service_specific_authorization(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::service_specific_authorization_request::ServiceSpecificAuthorizationResponse::Status200
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
			apis::service_specific_authorization_request::ServiceSpecificAuthorizationResponse::Status400
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
			apis::service_specific_authorization_request::ServiceSpecificAuthorizationResponse::Status403
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
			apis::service_specific_authorization_request::ServiceSpecificAuthorizationResponse::Status404
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
			apis::service_specific_authorization_request::ServiceSpecificAuthorizationResponse::Status500
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
			apis::service_specific_authorization_request::ServiceSpecificAuthorizationResponse::Status501
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
			apis::service_specific_authorization_request::ServiceSpecificAuthorizationResponse::Status503
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
			apis::service_specific_authorization_request::ServiceSpecificAuthorizationResponse::Statusdefault
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
fn get_sm_data_validation(
	header_params: models::GetSmDataHeaderParams,
	path_params: models::GetSmDataPathParams,
	query_params: models::GetSmDataQueryParams,
) -> std::result::Result<
	(
		models::GetSmDataHeaderParams,
		models::GetSmDataPathParams,
		models::GetSmDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetSmData - GET /nudm-sdm/v2/{supi}/sm-data
#[tracing::instrument(skip_all)]
async fn get_sm_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetSmDataPathParams>,
	Query(query_params): Query<models::GetSmDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
  where
	  I: AsRef<A> + Send + Sync,
	  A: apis::session_management_subscription_data_retrieval::SessionManagementSubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetSmDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_sm_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_sm_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::session_management_subscription_data_retrieval::GetSmDataResponse::Status200
			{
				body,
				cache_control,
				etag,
				last_modified
			}
			=> {
				if let Some(cache_control) = cache_control {
					let cache_control = match header::IntoHeaderValue(cache_control).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling cache_control header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							cache_control,
						);
					}
				}
				if let Some(etag) = etag {
					let etag = match header::IntoHeaderValue(etag).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling etag header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							etag,
						);
					}
				}
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!("An internal server error occurred handling last_modified header - {}", e))).map_err(|e| {
								error!(error = ?e);
								StatusCode::INTERNAL_SERVER_ERROR
							});
						}
					};

					{
						let mut response_headers = response.headers_mut().unwrap();
						response_headers.insert(
							HeaderName::from_static(""),
							last_modified,
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
			apis::session_management_subscription_data_retrieval::GetSmDataResponse::Status400
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
			apis::session_management_subscription_data_retrieval::GetSmDataResponse::Status404
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
			apis::session_management_subscription_data_retrieval::GetSmDataResponse::Status500
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
			apis::session_management_subscription_data_retrieval::GetSmDataResponse::Status503
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
			apis::session_management_subscription_data_retrieval::GetSmDataResponse::Statusdefault
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
fn get_nssai_validation(
	header_params: models::GetNssaiHeaderParams,
	path_params: models::GetNssaiPathParams,
	query_params: models::GetNssaiQueryParams,
) -> std::result::Result<
	(
		models::GetNssaiHeaderParams,
		models::GetNssaiPathParams,
		models::GetNssaiQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetNssai - GET /nudm-sdm/v2/{supi}/nssai
#[tracing::instrument(skip_all)]
async fn get_nssai<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetNssaiPathParams>,
	Query(query_params): Query<models::GetNssaiQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::slice_selection_subscription_data_retrieval::SliceSelectionSubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetNssaiHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_nssai_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_nssai(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::slice_selection_subscription_data_retrieval::GetNssaiResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::slice_selection_subscription_data_retrieval::GetNssaiResponse::Status400(
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
			apis::slice_selection_subscription_data_retrieval::GetNssaiResponse::Status404(
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
			apis::slice_selection_subscription_data_retrieval::GetNssaiResponse::Status500(
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
			apis::slice_selection_subscription_data_retrieval::GetNssaiResponse::Status503(
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
			apis::slice_selection_subscription_data_retrieval::GetNssaiResponse::Statusdefault => {
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
struct SubscribeBodyValidator<'a> {
	body: &'a models::SdmSubscription,
}

#[tracing::instrument(skip_all)]
fn subscribe_validation(
	path_params: models::SubscribePathParams,
	body: models::SdmSubscription,
) -> std::result::Result<(models::SubscribePathParams, models::SdmSubscription), ValidationErrors> {
	path_params.validate()?;
	let b = SubscribeBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// Subscribe - POST /nudm-sdm/v2/{ueId}/sdm-subscriptions
#[tracing::instrument(skip_all)]
async fn subscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::SubscribePathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SdmSubscription>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_creation::SubscriptionCreation,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || subscribe_validation(path_params, body))
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
		.subscribe(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_creation::SubscribeResponse::Status201 { body, location } => {
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
			apis::subscription_creation::SubscribeResponse::Status400(body) => {
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
			apis::subscription_creation::SubscribeResponse::Status404(body) => {
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
			apis::subscription_creation::SubscribeResponse::Status500(body) => {
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
			apis::subscription_creation::SubscribeResponse::Status501(body) => {
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
			apis::subscription_creation::SubscribeResponse::Status503(body) => {
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
			apis::subscription_creation::SubscribeResponse::Statusdefault => {
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
struct SubscribeToSharedDataBodyValidator<'a> {
	body: &'a models::SdmSubscription,
}

#[tracing::instrument(skip_all)]
fn subscribe_to_shared_data_validation(
	body: models::SdmSubscription
) -> std::result::Result<(models::SdmSubscription,), ValidationErrors> {
	let b = SubscribeToSharedDataBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// SubscribeToSharedData - POST /nudm-sdm/v2/shared-data-subscriptions
#[tracing::instrument(skip_all)]
async fn subscribe_to_shared_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::SdmSubscription>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_creation_for_shared_data::SubscriptionCreationForSharedData,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || subscribe_to_shared_data_validation(body))
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
		.subscribe_to_shared_data(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_creation_for_shared_data::SubscribeToSharedDataResponse::Status201
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
			apis::subscription_creation_for_shared_data::SubscribeToSharedDataResponse::Status400
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
			apis::subscription_creation_for_shared_data::SubscribeToSharedDataResponse::Status404
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
			apis::subscription_creation_for_shared_data::SubscribeToSharedDataResponse::Statusdefault
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
struct UpdateBodyValidator<'a> {
	body: &'a models::PpData,
}

#[tracing::instrument(skip_all)]
fn update_validation(
	path_params: models::UpdatePathParams,
	query_params: models::UpdateQueryParams,
	body: models::PpData,
) -> std::result::Result<
	(
		models::UpdatePathParams,
		models::UpdateQueryParams,
		models::PpData,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = UpdateBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// Update - PATCH /nudm-pp/v1/{ueId}/pp-data
#[tracing::instrument(skip_all)]
async fn update<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdatePathParams>,
	Query(query_params): Query<models::UpdateQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::PpData>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_data_update::SubscriptionDataUpdate,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || update_validation(path_params, query_params, body))
			.await
			.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.update(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_data_update::UpdateResponse::Status200(body) => {
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
			apis::subscription_data_update::UpdateResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::subscription_data_update::UpdateResponse::Status400(body) => {
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
			apis::subscription_data_update::UpdateResponse::Status403(body) => {
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
			apis::subscription_data_update::UpdateResponse::Status404(body) => {
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
			apis::subscription_data_update::UpdateResponse::Status500(body) => {
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
			apis::subscription_data_update::UpdateResponse::Status503(body) => {
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
			apis::subscription_data_update::UpdateResponse::Statusdefault => {
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
fn unsubscribe_validation(
	path_params: models::UnsubscribePathParams
) -> std::result::Result<(models::UnsubscribePathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// Unsubscribe - DELETE /nudm-sdm/v2/{ueId}/sdm-subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn unsubscribe<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UnsubscribePathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_deletion::SubscriptionDeletion,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || unsubscribe_validation(path_params))
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
		.unsubscribe(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_deletion::UnsubscribeResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::subscription_deletion::UnsubscribeResponse::Status400(body) => {
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
			apis::subscription_deletion::UnsubscribeResponse::Status404(body) => {
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
			apis::subscription_deletion::UnsubscribeResponse::Status500(body) => {
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
			apis::subscription_deletion::UnsubscribeResponse::Status503(body) => {
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
			apis::subscription_deletion::UnsubscribeResponse::Statusdefault => {
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
fn unsubscribe_for_shared_data_validation(
	path_params: models::UnsubscribeForSharedDataPathParams
) -> std::result::Result<(models::UnsubscribeForSharedDataPathParams,), ValidationErrors> {
	path_params.validate()?;

	Ok((path_params,))
}

/// UnsubscribeForSharedData - DELETE
/// /nudm-sdm/v2/shared-data-subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn unsubscribe_for_shared_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UnsubscribeForSharedDataPathParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_deletion_for_shared_data::SubscriptionDeletionForSharedData,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || unsubscribe_for_shared_data_validation(path_params))
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
		.unsubscribe_for_shared_data(method, host, cookies, path_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_deletion_for_shared_data::UnsubscribeForSharedDataResponse::Status204
			=> {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::subscription_deletion_for_shared_data::UnsubscribeForSharedDataResponse::Status400
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
			apis::subscription_deletion_for_shared_data::UnsubscribeForSharedDataResponse::Status404
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
			apis::subscription_deletion_for_shared_data::UnsubscribeForSharedDataResponse::Status500
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
			apis::subscription_deletion_for_shared_data::UnsubscribeForSharedDataResponse::Status503
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
			apis::subscription_deletion_for_shared_data::UnsubscribeForSharedDataResponse::Statusdefault
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
struct ModifyBodyValidator<'a> {
	body: &'a models::SdmSubsModification,
}

#[tracing::instrument(skip_all)]
fn modify_validation(
	path_params: models::ModifyPathParams,
	query_params: models::ModifyQueryParams,
	body: models::SdmSubsModification,
) -> std::result::Result<
	(
		models::ModifyPathParams,
		models::ModifyQueryParams,
		models::SdmSubsModification,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = ModifyBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// Modify - PATCH /nudm-sdm/v2/{ueId}/sdm-subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn modify<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ModifyPathParams>,
	Query(query_params): Query<models::ModifyQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SdmSubsModification>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_modification::SubscriptionModification,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || modify_validation(path_params, query_params, body))
			.await
			.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.modify(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_modification::ModifyResponse::Status200(body) => {
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
			apis::subscription_modification::ModifyResponse::Status400(body) => {
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
			apis::subscription_modification::ModifyResponse::Status403(body) => {
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
			apis::subscription_modification::ModifyResponse::Status404(body) => {
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
			apis::subscription_modification::ModifyResponse::Status500(body) => {
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
			apis::subscription_modification::ModifyResponse::Status503(body) => {
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
			apis::subscription_modification::ModifyResponse::Statusdefault => {
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
struct ModifySharedDataSubsBodyValidator<'a> {
	body: &'a models::SdmSubsModification,
}

#[tracing::instrument(skip_all)]
fn modify_shared_data_subs_validation(
	path_params: models::ModifySharedDataSubsPathParams,
	query_params: models::ModifySharedDataSubsQueryParams,
	body: models::SdmSubsModification,
) -> std::result::Result<
	(
		models::ModifySharedDataSubsPathParams,
		models::ModifySharedDataSubsQueryParams,
		models::SdmSubsModification,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = ModifySharedDataSubsBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// ModifySharedDataSubs - PATCH
/// /nudm-sdm/v2/shared-data-subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn modify_shared_data_subs<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::ModifySharedDataSubsPathParams>,
	Query(query_params): Query<models::ModifySharedDataSubsQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::SdmSubsModification>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::subscription_modification::SubscriptionModification,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		modify_shared_data_subs_validation(path_params, query_params, body)
	})
	.await
	.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.modify_shared_data_subs(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::subscription_modification::ModifySharedDataSubsResponse::Status200(body) => {
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
			apis::subscription_modification::ModifySharedDataSubsResponse::Status400(body) => {
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
			apis::subscription_modification::ModifySharedDataSubsResponse::Status403(body) => {
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
			apis::subscription_modification::ModifySharedDataSubsResponse::Status404(body) => {
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
			apis::subscription_modification::ModifySharedDataSubsResponse::Status500(body) => {
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
			apis::subscription_modification::ModifySharedDataSubsResponse::Status503(body) => {
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
			apis::subscription_modification::ModifySharedDataSubsResponse::Statusdefault => {
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
fn get_trace_config_data_validation(
	header_params: models::GetTraceConfigDataHeaderParams,
	path_params: models::GetTraceConfigDataPathParams,
	query_params: models::GetTraceConfigDataQueryParams,
) -> std::result::Result<
	(
		models::GetTraceConfigDataHeaderParams,
		models::GetTraceConfigDataPathParams,
		models::GetTraceConfigDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetTraceConfigData - GET /nudm-sdm/v2/{supi}/trace-data
#[tracing::instrument(skip_all)]
async fn get_trace_config_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetTraceConfigDataPathParams>,
	Query(query_params): Query<models::GetTraceConfigDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::trace_configuration_data_retrieval::TraceConfigurationDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetTraceConfigDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_trace_config_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_trace_config_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::trace_configuration_data_retrieval::GetTraceConfigDataResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::trace_configuration_data_retrieval::GetTraceConfigDataResponse::Status400(
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
			apis::trace_configuration_data_retrieval::GetTraceConfigDataResponse::Status404(
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
			apis::trace_configuration_data_retrieval::GetTraceConfigDataResponse::Status500(
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
			apis::trace_configuration_data_retrieval::GetTraceConfigDataResponse::Status503(
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
			apis::trace_configuration_data_retrieval::GetTraceConfigDataResponse::Statusdefault => {
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
struct DeregAmfBodyValidator<'a> {
	body: &'a models::AmfDeregInfo,
}

#[tracing::instrument(skip_all)]
fn dereg_amf_validation(
	path_params: models::DeregAmfPathParams,
	body: models::AmfDeregInfo,
) -> std::result::Result<(models::DeregAmfPathParams, models::AmfDeregInfo), ValidationErrors> {
	path_params.validate()?;
	let b = DeregAmfBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, body))
}

/// DeregAmf - POST /nudm-uecm/v1/{ueId}/registrations/amf-3gpp-access/dereg-amf
#[tracing::instrument(skip_all)]
async fn dereg_amf<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::DeregAmfPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<models::AmfDeregInfo>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::trigger_amf_for3_gpp_access_deregistration::TriggerAmfFor3GppAccessDeregistration,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || dereg_amf_validation(path_params, body))
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
		.dereg_amf(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::trigger_amf_for3_gpp_access_deregistration::DeregAmfResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::trigger_amf_for3_gpp_access_deregistration::DeregAmfResponse::Status400(body) => {
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
			apis::trigger_amf_for3_gpp_access_deregistration::DeregAmfResponse::Status403(body) => {
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
			apis::trigger_amf_for3_gpp_access_deregistration::DeregAmfResponse::Status404(body) => {
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
			apis::trigger_amf_for3_gpp_access_deregistration::DeregAmfResponse::Status500(body) => {
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
			apis::trigger_amf_for3_gpp_access_deregistration::DeregAmfResponse::Status503(body) => {
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
			apis::trigger_amf_for3_gpp_access_deregistration::DeregAmfResponse::Statusdefault => {
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
struct TriggerPcscfRestorationBodyValidator<'a> {
	body: &'a models::TriggerRequest,
}

#[tracing::instrument(skip_all)]
fn trigger_pcscf_restoration_validation(
	body: models::TriggerRequest
) -> std::result::Result<(models::TriggerRequest,), ValidationErrors> {
	let b = TriggerPcscfRestorationBodyValidator { body: &body };
	b.validate()?;

	Ok((body,))
}

/// TriggerPcscfRestoration - POST /nudm-uecm/v1/restore-pcscf
#[tracing::instrument(skip_all)]
async fn trigger_pcscf_restoration<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	State(api_impl): State<I>,
	Json(body): Json<models::TriggerRequest>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::trigger_pcscf_restoration::TriggerPcscfRestoration,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || trigger_pcscf_restoration_validation(body))
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
		.trigger_pcscf_restoration(method, host, cookies, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::trigger_pcscf_restoration::TriggerPcscfRestorationResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::trigger_pcscf_restoration::TriggerPcscfRestorationResponse::Status400(body) => {
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
			apis::trigger_pcscf_restoration::TriggerPcscfRestorationResponse::Status403(body) => {
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
			apis::trigger_pcscf_restoration::TriggerPcscfRestorationResponse::Status404(body) => {
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
			apis::trigger_pcscf_restoration::TriggerPcscfRestorationResponse::Status500(body) => {
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
			apis::trigger_pcscf_restoration::TriggerPcscfRestorationResponse::Status501(body) => {
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
			apis::trigger_pcscf_restoration::TriggerPcscfRestorationResponse::Status503(body) => {
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
			apis::trigger_pcscf_restoration::TriggerPcscfRestorationResponse::Statusdefault => {
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
struct UpdateSorInfoBodyValidator<'a> {
	body: &'a models::SorUpdateInfo,
}

#[tracing::instrument(skip_all)]
fn update_sor_info_validation(
	path_params: models::UpdateSorInfoPathParams,
	body: Option<models::SorUpdateInfo>,
) -> std::result::Result<
	(
		models::UpdateSorInfoPathParams,
		Option<models::SorUpdateInfo>,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	if let Some(body) = &body {
		let b = UpdateSorInfoBodyValidator { body };
		b.validate()?;
	}

	Ok((path_params, body))
}

/// UpdateSorInfo - POST /nudm-sdm/v2/{supi}/am-data/update-sor
#[tracing::instrument(skip_all)]
async fn update_sor_info<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateSorInfoPathParams>,
	State(api_impl): State<I>,
	Json(body): Json<Option<models::SorUpdateInfo>>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::trigger_sor_info_update::TriggerSorInfoUpdate,
{
	#[allow(clippy::redundant_closure)]
	let validation =
		tokio::task::spawn_blocking(move || update_sor_info_validation(path_params, body))
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
		.update_sor_info(method, host, cookies, path_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::trigger_sor_info_update::UpdateSorInfoResponse::Status200(body) => {
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
			apis::trigger_sor_info_update::UpdateSorInfoResponse::Status400(body) => {
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
			apis::trigger_sor_info_update::UpdateSorInfoResponse::Status404(body) => {
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
			apis::trigger_sor_info_update::UpdateSorInfoResponse::Status500(body) => {
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
			apis::trigger_sor_info_update::UpdateSorInfoResponse::Status503(body) => {
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
			apis::trigger_sor_info_update::UpdateSorInfoResponse::Statusdefault => {
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
fn get_registrations_validation(
	path_params: models::GetRegistrationsPathParams,
	query_params: models::GetRegistrationsQueryParams,
) -> std::result::Result<
	(
		models::GetRegistrationsPathParams,
		models::GetRegistrationsQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetRegistrations - GET /nudm-uecm/v1/{ueId}/registrations
#[tracing::instrument(skip_all)]
async fn get_registrations<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetRegistrationsPathParams>,
	Query(query_params): Query<models::GetRegistrationsQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::uecm_registration_info_retrieval::UecmRegistrationInfoRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_registrations_validation(path_params, query_params)
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
		.get_registrations(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::uecm_registration_info_retrieval::GetRegistrationsResponse::Status200(body) => {
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
			apis::uecm_registration_info_retrieval::GetRegistrationsResponse::Status400(body) => {
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
			apis::uecm_registration_info_retrieval::GetRegistrationsResponse::Status403(body) => {
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
			apis::uecm_registration_info_retrieval::GetRegistrationsResponse::Status404(body) => {
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
			apis::uecm_registration_info_retrieval::GetRegistrationsResponse::Status500(body) => {
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
			apis::uecm_registration_info_retrieval::GetRegistrationsResponse::Status503(body) => {
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
			apis::uecm_registration_info_retrieval::GetRegistrationsResponse::Statusdefault => {
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
fn get_ue_ctx_in_amf_data_validation(
	path_params: models::GetUeCtxInAmfDataPathParams,
	query_params: models::GetUeCtxInAmfDataQueryParams,
) -> std::result::Result<
	(
		models::GetUeCtxInAmfDataPathParams,
		models::GetUeCtxInAmfDataQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetUeCtxInAmfData - GET /nudm-sdm/v2/{supi}/ue-context-in-amf-data
#[tracing::instrument(skip_all)]
async fn get_ue_ctx_in_amf_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetUeCtxInAmfDataPathParams>,
	Query(query_params): Query<models::GetUeCtxInAmfDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ue_context_in_amf_data_retrieval::UeContextInAmfDataRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_ue_ctx_in_amf_data_validation(path_params, query_params)
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
		.get_ue_ctx_in_amf_data(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_context_in_amf_data_retrieval::GetUeCtxInAmfDataResponse::Status200(body) => {
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
			apis::ue_context_in_amf_data_retrieval::GetUeCtxInAmfDataResponse::Status400(body) => {
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
			apis::ue_context_in_amf_data_retrieval::GetUeCtxInAmfDataResponse::Status404(body) => {
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
			apis::ue_context_in_amf_data_retrieval::GetUeCtxInAmfDataResponse::Status500(body) => {
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
			apis::ue_context_in_amf_data_retrieval::GetUeCtxInAmfDataResponse::Status503(body) => {
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
			apis::ue_context_in_amf_data_retrieval::GetUeCtxInAmfDataResponse::Statusdefault => {
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
fn get_ue_ctx_in_smf_data_validation(
	path_params: models::GetUeCtxInSmfDataPathParams,
	query_params: models::GetUeCtxInSmfDataQueryParams,
) -> std::result::Result<
	(
		models::GetUeCtxInSmfDataPathParams,
		models::GetUeCtxInSmfDataQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetUeCtxInSmfData - GET /nudm-sdm/v2/{supi}/ue-context-in-smf-data
#[tracing::instrument(skip_all)]
async fn get_ue_ctx_in_smf_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetUeCtxInSmfDataPathParams>,
	Query(query_params): Query<models::GetUeCtxInSmfDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ue_context_in_smf_data_retrieval::UeContextInSmfDataRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_ue_ctx_in_smf_data_validation(path_params, query_params)
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
		.get_ue_ctx_in_smf_data(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_context_in_smf_data_retrieval::GetUeCtxInSmfDataResponse::Status200(body) => {
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
			apis::ue_context_in_smf_data_retrieval::GetUeCtxInSmfDataResponse::Status400(body) => {
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
			apis::ue_context_in_smf_data_retrieval::GetUeCtxInSmfDataResponse::Status404(body) => {
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
			apis::ue_context_in_smf_data_retrieval::GetUeCtxInSmfDataResponse::Status500(body) => {
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
			apis::ue_context_in_smf_data_retrieval::GetUeCtxInSmfDataResponse::Status503(body) => {
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
			apis::ue_context_in_smf_data_retrieval::GetUeCtxInSmfDataResponse::Statusdefault => {
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
fn get_ue_ctx_in_smsf_data_validation(
	path_params: models::GetUeCtxInSmsfDataPathParams,
	query_params: models::GetUeCtxInSmsfDataQueryParams,
) -> std::result::Result<
	(
		models::GetUeCtxInSmsfDataPathParams,
		models::GetUeCtxInSmsfDataQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetUeCtxInSmsfData - GET /nudm-sdm/v2/{supi}/ue-context-in-smsf-data
#[tracing::instrument(skip_all)]
async fn get_ue_ctx_in_smsf_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetUeCtxInSmsfDataPathParams>,
	Query(query_params): Query<models::GetUeCtxInSmsfDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ue_context_in_smsf_data_retrieval::UeContextInSmsfDataRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_ue_ctx_in_smsf_data_validation(path_params, query_params)
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
		.get_ue_ctx_in_smsf_data(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_context_in_smsf_data_retrieval::GetUeCtxInSmsfDataResponse::Status200(
				body,
			) => {
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
			apis::ue_context_in_smsf_data_retrieval::GetUeCtxInSmsfDataResponse::Status400(
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
			apis::ue_context_in_smsf_data_retrieval::GetUeCtxInSmsfDataResponse::Status404(
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
			apis::ue_context_in_smsf_data_retrieval::GetUeCtxInSmsfDataResponse::Status500(
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
			apis::ue_context_in_smsf_data_retrieval::GetUeCtxInSmsfDataResponse::Status503(
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
			apis::ue_context_in_smsf_data_retrieval::GetUeCtxInSmsfDataResponse::Statusdefault => {
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
fn get_location_info_validation(
	path_params: models::GetLocationInfoPathParams,
	query_params: models::GetLocationInfoQueryParams,
) -> std::result::Result<
	(
		models::GetLocationInfoPathParams,
		models::GetLocationInfoQueryParams,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;

	Ok((path_params, query_params))
}

/// GetLocationInfo - GET /nudm-uecm/v1/{ueId}/registrations/location
#[tracing::instrument(skip_all)]
async fn get_location_info<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::GetLocationInfoPathParams>,
	Query(query_params): Query<models::GetLocationInfoQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::ue_location_information_retrieval::UeLocationInformationRetrieval,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_location_info_validation(path_params, query_params)
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
		.get_location_info(method, host, cookies, path_params, query_params)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::ue_location_information_retrieval::GetLocationInfoResponse::Status200(body) => {
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
			apis::ue_location_information_retrieval::GetLocationInfoResponse::Status400(body) => {
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
			apis::ue_location_information_retrieval::GetLocationInfoResponse::Status403(body) => {
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
			apis::ue_location_information_retrieval::GetLocationInfoResponse::Status404(body) => {
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
			apis::ue_location_information_retrieval::GetLocationInfoResponse::Status500(body) => {
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
			apis::ue_location_information_retrieval::GetLocationInfoResponse::Status503(body) => {
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
			apis::ue_location_information_retrieval::GetLocationInfoResponse::Statusdefault => {
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
struct UpdateEeSubscriptionBodyValidator<'a> {
	#[validate(length(min = 1))]
	body: &'a Vec<models::PatchItem>,
}

#[tracing::instrument(skip_all)]
fn update_ee_subscription_validation(
	path_params: models::UpdateEeSubscriptionPathParams,
	query_params: models::UpdateEeSubscriptionQueryParams,
	body: Vec<models::PatchItem>,
) -> std::result::Result<
	(
		models::UpdateEeSubscriptionPathParams,
		models::UpdateEeSubscriptionQueryParams,
		Vec<models::PatchItem>,
	),
	ValidationErrors,
> {
	path_params.validate()?;
	query_params.validate()?;
	let b = UpdateEeSubscriptionBodyValidator { body: &body };
	b.validate()?;

	Ok((path_params, query_params, body))
}

/// UpdateEeSubscription - PATCH
/// /nudm-ee/v1/{ueIdentity}/ee-subscriptions/{subscriptionId}
#[tracing::instrument(skip_all)]
async fn update_ee_subscription<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	Path(path_params): Path<models::UpdateEeSubscriptionPathParams>,
	Query(query_params): Query<models::UpdateEeSubscriptionQueryParams>,
	State(api_impl): State<I>,
	Json(body): Json<Vec<models::PatchItem>>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::update_ee_subscription::UpdateEeSubscription,
{
	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		update_ee_subscription_validation(path_params, query_params, body)
	})
	.await
	.unwrap();

	let Ok((path_params, query_params, body)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.update_ee_subscription(method, host, cookies, path_params, query_params, body)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::update_ee_subscription::UpdateEeSubscriptionResponse::Status200(body) => {
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
			apis::update_ee_subscription::UpdateEeSubscriptionResponse::Status204 => {
				let mut response = response.status(204);
				response.body(Body::empty())
			}
			apis::update_ee_subscription::UpdateEeSubscriptionResponse::Status403(body) => {
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
			apis::update_ee_subscription::UpdateEeSubscriptionResponse::Status404(body) => {
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
			apis::update_ee_subscription::UpdateEeSubscriptionResponse::Statusdefault => {
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
fn get_uc_data_validation(
	header_params: models::GetUcDataHeaderParams,
	path_params: models::GetUcDataPathParams,
	query_params: models::GetUcDataQueryParams,
) -> std::result::Result<
	(
		models::GetUcDataHeaderParams,
		models::GetUcDataPathParams,
		models::GetUcDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetUcData - GET /nudm-sdm/v2/{supi}/uc-data
#[tracing::instrument(skip_all)]
async fn get_uc_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetUcDataPathParams>,
	Query(query_params): Query<models::GetUcDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::user_consent_subscription_data_retrieval::UserConsentSubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetUcDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_uc_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_uc_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::user_consent_subscription_data_retrieval::GetUcDataResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::user_consent_subscription_data_retrieval::GetUcDataResponse::Status400(body) => {
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
			apis::user_consent_subscription_data_retrieval::GetUcDataResponse::Status404(body) => {
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
			apis::user_consent_subscription_data_retrieval::GetUcDataResponse::Status500(body) => {
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
			apis::user_consent_subscription_data_retrieval::GetUcDataResponse::Status503(body) => {
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
			apis::user_consent_subscription_data_retrieval::GetUcDataResponse::Statusdefault => {
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
fn get_v2x_data_validation(
	header_params: models::GetV2xDataHeaderParams,
	path_params: models::GetV2xDataPathParams,
	query_params: models::GetV2xDataQueryParams,
) -> std::result::Result<
	(
		models::GetV2xDataHeaderParams,
		models::GetV2xDataPathParams,
		models::GetV2xDataQueryParams,
	),
	ValidationErrors,
> {
	header_params.validate()?;
	path_params.validate()?;
	query_params.validate()?;

	Ok((header_params, path_params, query_params))
}

/// GetV2xData - GET /nudm-sdm/v2/{supi}/v2x-data
#[tracing::instrument(skip_all)]
async fn get_v2x_data<I, A>(
	method: Method,
	host: Host,
	cookies: CookieJar,
	headers: HeaderMap,
	Path(path_params): Path<models::GetV2xDataPathParams>,
	Query(query_params): Query<models::GetV2xDataQueryParams>,
	State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
	I: AsRef<A> + Send + Sync,
	A: apis::v2_x_subscription_data_retrieval::V2XSubscriptionDataRetrieval,
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
		let header_if_modified_since = headers.get(HeaderName::from_static("if_modified_since"));

		let header_if_modified_since = match header_if_modified_since {
			Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
				Ok(result) => Some(result.0),
				Err(err) => {
					return Response::builder()
						.status(StatusCode::BAD_REQUEST)
						.body(Body::from(format!(
							"Invalid header If-Modified-Since - {}",
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

		models::GetV2xDataHeaderParams {
			if_none_match: header_if_none_match,
			if_modified_since: header_if_modified_since,
		}
	};

	#[allow(clippy::redundant_closure)]
	let validation = tokio::task::spawn_blocking(move || {
		get_v2x_data_validation(header_params, path_params, query_params)
	})
	.await
	.unwrap();

	let Ok((header_params, path_params, query_params)) = validation else {
		return Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from(validation.unwrap_err().to_string()))
			.map_err(|_| StatusCode::BAD_REQUEST);
	};

	let result = api_impl
		.as_ref()
		.get_v2x_data(
			method,
			host,
			cookies,
			header_params,
			path_params,
			query_params,
		)
		.await;

	let mut response = Response::builder();

	let resp = match result {
		Ok(rsp) => match rsp {
			apis::v2_x_subscription_data_retrieval::GetV2xDataResponse::Status200 {
				body,
				cache_control,
				etag,
				last_modified,
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
				if let Some(last_modified) = last_modified {
					let last_modified = match header::IntoHeaderValue(last_modified).try_into() {
						Ok(val) => val,
						Err(e) => {
							return Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(format!(
									"An internal server error occurred handling last_modified \
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
						response_headers.insert(HeaderName::from_static(""), last_modified);
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
			apis::v2_x_subscription_data_retrieval::GetV2xDataResponse::Status400(body) => {
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
			apis::v2_x_subscription_data_retrieval::GetV2xDataResponse::Status404(body) => {
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
			apis::v2_x_subscription_data_retrieval::GetV2xDataResponse::Status500(body) => {
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
			apis::v2_x_subscription_data_retrieval::GetV2xDataResponse::Status503(body) => {
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
			apis::v2_x_subscription_data_retrieval::GetV2xDataResponse::Statusdefault => {
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
