use serde_with::{json::JsonString, serde_as};

use crate::common::{
	AccessTokenReqGrantType,
	AccessTokenReqScope,
	Fqdn,
	NfInstanceId,
	NfServiceSetId,
	NfSetId,
	NfType,
	PlmnId,
	PlmnIdNid,
	Snssai,
	Uri,
};

/// Contains information related to the access token request
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "Contains information related to the access token
/// request",
///  "type": "object",
///  "required": [
///    "grant_type",
///    "nfInstanceId",
///    "scope"
///  ],
///  "properties": {
///    "grant_type": {
///      "type": "string",
///      "enum": [
///        "client_credentials"
///      ]
///    },
///    "hnrfAccessTokenUri": {
///      "$ref": "#/components/schemas/Uri"
///    },
///    "nfInstanceId": {
///      "$ref": "#/components/schemas/NfInstanceId"
///    },
///    "nfType": {
///      "$ref": "#/components/schemas/NFType"
///    },
///    "requesterFqdn": {
///      "$ref": "#/components/schemas/Fqdn"
///    },
///    "requesterPlmn": {
///      "$ref": "#/components/schemas/PlmnId"
///    },
///    "requesterPlmnList": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/PlmnId"
///      },
///      "minItems": 2
///    },
///    "requesterSnpnList": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/PlmnIdNid"
///      },
///      "minItems": 1
///    },
///    "requesterSnssaiList": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/Snssai"
///      },
///      "minItems": 1
///    },
///    "scope": {
///      "type": "string",
///      "pattern": "^([a-zA-Z0-9_:-]+)( [a-zA-Z0-9_:-]+)*$"
///    },
///    "sourceNfInstanceId": {
///      "$ref": "#/components/schemas/NfInstanceId"
///    },
///    "targetNfInstanceId": {
///      "$ref": "#/components/schemas/NfInstanceId"
///    },
///    "targetNfServiceSetId": {
///      "$ref": "#/components/schemas/NfServiceSetId"
///    },
///    "targetNfSetId": {
///      "$ref": "#/components/schemas/NfSetId"
///    },
///    "targetNfType": {
///      "$ref": "#/components/schemas/NFType"
///    },
///    "targetNsiList": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "minItems": 1
///    },
///    "targetPlmn": {
///      "$ref": "#/components/schemas/PlmnId"
///    },
///    "targetSnpn": {
///      "$ref": "#/components/schemas/PlmnIdNid"
///    },
///    "targetSnssaiList": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/Snssai"
///      },
///      "minItems": 1
///    }
///  }
/// }
/// ```
/// </details>
/// Request Body Format for /oauth2/token api
/// <details><summary>JSON schema</summary>
/// ```yaml
/// requestBody:
///  content:
///    application/x-www-form-urlencoded:
///      schema:
///        $ref: '#/components/schemas/AccessTokenReq'
///      encoding:
///        requesterPlmn:
///          contentType: application/json
///        requesterPlmnList:
///          contentType: application/json
///        requesterSnssaiList:
///          contentType: application/json
///        requesterSnpnList:
///          contentType: application/json
///        targetPlmn:
///          contentType: application/json
///        targetSnpn:
///          contentType: application/json
///        targetSnssaiList:
///          contentType: application/json
///        targetNsiList:
///          style: form
///          explode: true
/// ```
/// </details>
#[serde_as]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault)]
pub struct OauthAccessTokenReq {
	pub grant_type: AccessTokenReqGrantType,
	#[serde(
		rename = "hnrfAccessTokenUri",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub hnrf_access_token_uri: Option<Uri>,
	#[serde(rename = "nfInstanceId")]
	pub nf_instance_id: NfInstanceId,
	#[serde(rename = "nfType", default, skip_serializing_if = "Option::is_none")]
	pub nf_type: Option<NfType>,
	#[serde(
		rename = "requesterFqdn",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub requester_fqdn: Option<Fqdn>,
	#[serde_as(as = "JsonString")]
	#[serde(
		rename = "requesterPlmn",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub requester_plmn: Option<PlmnId>,
	#[serde_as(as = "JsonString")]
	#[serde(
		rename = "requesterPlmnList",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	pub requester_plmn_list: Vec<PlmnId>,
	#[serde_as(as = "JsonString")]
	#[serde(
		rename = "requesterSnpnList",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	pub requester_snpn_list: Vec<PlmnIdNid>,
	#[serde_as(as = "JsonString")]
	#[serde(
		rename = "requesterSnssaiList",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	pub requester_snssai_list: Vec<Snssai>,
	pub scope: AccessTokenReqScope,
	#[serde(
		rename = "sourceNfInstanceId",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub source_nf_instance_id: Option<NfInstanceId>,
	#[serde(
		rename = "targetNfInstanceId",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub target_nf_instance_id: Option<NfInstanceId>,
	#[serde(
		rename = "targetNfServiceSetId",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub target_nf_service_set_id: Option<NfServiceSetId>,
	#[serde(
		rename = "targetNfSetId",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub target_nf_set_id: Option<NfSetId>,
	#[serde(
		rename = "targetNfType",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub target_nf_type: Option<NfType>,
	#[serde(
		rename = "targetNsiList",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	#[serde_as(as = "JsonString")]
	pub target_nsi_list: Vec<String>,
	#[serde(
		rename = "targetPlmn",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub target_plmn: Option<PlmnId>,
	#[serde_as(as = "JsonString")]
	#[serde(
		rename = "targetSnpn",
		default,
		skip_serializing_if = "Option::is_none"
	)]
	pub target_snpn: Option<PlmnIdNid>,
	#[serde_as(as = "JsonString")]
	#[serde(
		rename = "targetSnssaiList",
		default,
		skip_serializing_if = "Vec::is_empty"
	)]
	pub target_snssai_list: Vec<Snssai>,
}

impl From<&OauthAccessTokenReq> for OauthAccessTokenReq {
	fn from(value: &OauthAccessTokenReq) -> Self {
		value.clone()
	}
}
