#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};

#[allow(unused_imports)]
pub use crate::progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use crate::progenitor_client::{RequestBuilderExt, encode_path};

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {

	/// Error types.
	pub use crate::common::*;

	/// Contains an array of allowed S-NSSAI that constitute the allowed NSSAI
	/// information for the authorized network slice information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains an array of allowed S-NSSAI that constitute
	/// the allowed NSSAI information for the authorized network slice
	/// information\n",
	///  "type": "object",
	///  "required": [
	///    "accessType",
	///    "allowedSnssaiList"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "allowedSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AllowedSnssai"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AllowedNssai {
		#[serde(rename = "accessType")]
		pub access_type: AccessType,
		#[serde(rename = "allowedSnssaiList")]
		pub allowed_snssai_list: Vec<AllowedSnssai>,
	}

	impl From<&AllowedNssai> for AllowedNssai {
		fn from(value: &AllowedNssai) -> Self {
			value.clone()
		}
	}

	/// Contains the authorized S-NSSAI and optional mapped home S-NSSAI and
	/// network slice instance information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the authorized S-NSSAI and optional mapped
	/// home S-NSSAI and network slice instance information\n",
	///  "type": "object",
	///  "required": [
	///    "allowedSnssai"
	///  ],
	///  "properties": {
	///    "allowedSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "mappedHomeSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "nsiInformationList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsiInformation"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AllowedSnssai {
		#[serde(rename = "allowedSnssai")]
		pub allowed_snssai: Snssai,
		#[serde(
			rename = "mappedHomeSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mapped_home_snssai: Option<Snssai>,
		#[serde(
			rename = "nsiInformationList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nsi_information_list: Vec<NsiInformation>,
	}

	impl From<&AllowedSnssai> for AllowedSnssai {
		fn from(value: &AllowedSnssai) -> Self {
			value.clone()
		}
	}

	/// Contains the authorized network slice information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the authorized network slice information",
	///  "type": "object",
	///  "properties": {
	///    "allowedNssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AllowedNssai"
	///      },
	///      "minItems": 1
	///    },
	///    "candidateAmfList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfInstanceId"
	///      },
	///      "minItems": 1
	///    },
	///    "configuredNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ConfiguredSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "nrfAmfSet": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfAmfSetAccessTokenUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfAmfSetNfMgtUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfOauth2Required": {
	///      "description": "Map indicating whether the NRF requires
	/// Oauth2-based authorization for accessing its services. The key of the
	/// map shall be the name of an NRF service, e.g. \"nnrf-nfm\" or
	/// \"nnrf-disc\"\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "boolean"
	///      }
	///    },
	///    "nsagInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsagInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "nsiInformation": {
	///      "$ref": "#/components/schemas/NsiInformation"
	///    },
	///    "rejectedNssaiInPlmn": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "rejectedNssaiInTa": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetAmfServiceSet": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "targetAmfSet": {
	///      "type": "string",
	///      "pattern":
	/// "^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$"
	///    },
	///    "targetNssai": {
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
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AuthorizedNetworkSliceInfo {
		#[serde(
			rename = "allowedNssaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_nssai_list: Vec<AllowedNssai>,
		#[serde(
			rename = "candidateAmfList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub candidate_amf_list: Vec<NfInstanceId>,
		#[serde(
			rename = "configuredNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub configured_nssai: Vec<ConfiguredSnssai>,
		#[serde(rename = "nrfAmfSet", default, skip_serializing_if = "Option::is_none")]
		pub nrf_amf_set: Option<Uri>,
		#[serde(
			rename = "nrfAmfSetAccessTokenUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_amf_set_access_token_uri: Option<Uri>,
		#[serde(
			rename = "nrfAmfSetNfMgtUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_amf_set_nf_mgt_uri: Option<Uri>,
		/// Map indicating whether the NRF requires Oauth2-based authorization
		/// for accessing its services. The key of the map shall be the name of
		/// an NRF service, e.g. "nnrf-nfm" or "nnrf-disc"
		#[serde(
			rename = "nrfOauth2Required",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nrf_oauth2_required: ::std::collections::HashMap<String, bool>,
		#[serde(rename = "nsagInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub nsag_infos: Vec<NsagInfo>,
		#[serde(
			rename = "nsiInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nsi_information: Option<NsiInformation>,
		#[serde(
			rename = "rejectedNssaiInPlmn",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub rejected_nssai_in_plmn: Vec<Snssai>,
		#[serde(
			rename = "rejectedNssaiInTa",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub rejected_nssai_in_ta: Vec<Snssai>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "targetAmfServiceSet",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_amf_service_set: Option<NfServiceSetId>,
		#[serde(
			rename = "targetAmfSet",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_amf_set: Option<AuthorizedNetworkSliceInfoTargetAmfSet>,
		#[serde(rename = "targetNssai", default, skip_serializing_if = "Vec::is_empty")]
		pub target_nssai: Vec<Snssai>,
	}

	impl From<&AuthorizedNetworkSliceInfo> for AuthorizedNetworkSliceInfo {
		fn from(value: &AuthorizedNetworkSliceInfo) -> Self {
			value.clone()
		}
	}

	/// AuthorizedNetworkSliceInfoTargetAmfSet
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct AuthorizedNetworkSliceInfoTargetAmfSet(String);

	impl ::std::ops::Deref for AuthorizedNetworkSliceInfoTargetAmfSet {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AuthorizedNetworkSliceInfoTargetAmfSet> for String {
		fn from(value: AuthorizedNetworkSliceInfoTargetAmfSet) -> Self {
			value.0
		}
	}

	impl From<&AuthorizedNetworkSliceInfoTargetAmfSet> for AuthorizedNetworkSliceInfoTargetAmfSet {
		fn from(value: &AuthorizedNetworkSliceInfoTargetAmfSet) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AuthorizedNetworkSliceInfoTargetAmfSet {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \
				            \"^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for AuthorizedNetworkSliceInfoTargetAmfSet {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AuthorizedNetworkSliceInfoTargetAmfSet {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AuthorizedNetworkSliceInfoTargetAmfSet {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AuthorizedNetworkSliceInfoTargetAmfSet {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// This contains the Nssai availability data information per TA authorized
	/// by the NSSF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the Nssai availability data information
	/// per TA authorized by the NSSF",
	///  "type": "object",
	///  "required": [
	///    "supportedSnssaiList",
	///    "tai"
	///  ],
	///  "properties": {
	///    "nsagInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-NsagInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "restrictedSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RestrictedSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "tai": {
	///      "$ref": "#/components/schemas/Tai"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AuthorizedNssaiAvailabilityData {
		#[serde(rename = "nsagInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub nsag_infos: Vec<SchemasNsagInfo>,
		#[serde(
			rename = "restrictedSnssaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub restricted_snssai_list: Vec<RestrictedSnssai>,
		#[serde(rename = "supportedSnssaiList")]
		pub supported_snssai_list: Vec<ExtSnssai>,
		pub tai: Tai,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&AuthorizedNssaiAvailabilityData> for AuthorizedNssaiAvailabilityData {
		fn from(value: &AuthorizedNssaiAvailabilityData) -> Self {
			value.clone()
		}
	}

	/// This contains the Nssai availability data information authorized by the
	/// NSSF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the Nssai availability data information
	/// authorized by the NSSF",
	///  "type": "object",
	///  "required": [
	///    "authorizedNssaiAvailabilityData"
	///  ],
	///  "properties": {
	///    "authorizedNssaiAvailabilityData": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AuthorizedNssaiAvailabilityData"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AuthorizedNssaiAvailabilityInfo {
		#[serde(rename = "authorizedNssaiAvailabilityData")]
		pub authorized_nssai_availability_data: Vec<AuthorizedNssaiAvailabilityData>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&AuthorizedNssaiAvailabilityInfo> for AuthorizedNssaiAvailabilityInfo {
		fn from(value: &AuthorizedNssaiAvailabilityInfo) -> Self {
			value.clone()
		}
	}

	/// Contains the configured S-NSSAI(s) authorized by the NSSF in the serving
	/// PLMN and optional mapped home S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the configured S-NSSAI(s) authorized by the
	/// NSSF in the serving PLMN and optional mapped home S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "configuredSnssai"
	///  ],
	///  "properties": {
	///    "configuredSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "mappedHomeSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ConfiguredSnssai {
		#[serde(rename = "configuredSnssai")]
		pub configured_snssai: Snssai,
		#[serde(
			rename = "mappedHomeSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mapped_home_snssai: Option<Snssai>,
	}

	impl From<&ConfiguredSnssai> for ConfiguredSnssai {
		fn from(value: &ConfiguredSnssai) -> Self {
			value.clone()
		}
	}

	/// Contains the mapping of S-NSSAI in the serving network and the value of
	/// the home network
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the mapping of S-NSSAI in the serving network
	/// and the value of the home network",
	///  "type": "object",
	///  "required": [
	///    "homeSnssai",
	///    "servingSnssai"
	///  ],
	///  "properties": {
	///    "homeSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "servingSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MappingOfSnssai {
		#[serde(rename = "homeSnssai")]
		pub home_snssai: Snssai,
		#[serde(rename = "servingSnssai")]
		pub serving_snssai: Snssai,
	}

	impl From<&MappingOfSnssai> for MappingOfSnssai {
		fn from(value: &MappingOfSnssai) -> Self {
			value.clone()
		}
	}

	/// Contains the association of NSAGs and S-NSSAI(s) along with the TA(s)
	/// within which the association is valid.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the association of NSAGs and S-NSSAI(s) along
	/// with the TA(s) within which the association is valid.",
	///  "type": "object",
	///  "required": [
	///    "nsagIds",
	///    "snssaiList"
	///  ],
	///  "properties": {
	///    "nsagIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsagId"
	///      },
	///      "minItems": 1
	///    },
	///    "snssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsagInfo {
		#[serde(rename = "nsagIds")]
		pub nsag_ids: Vec<NsagId>,
		#[serde(rename = "snssaiList")]
		pub snssai_list: Vec<Snssai>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&NsagInfo> for NsagInfo {
		fn from(value: &NsagInfo) -> Self {
			value.clone()
		}
	}

	/// Contains the Identifier of the selected Network Slice instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Identifier of the selected Network Slice
	/// instance",
	///  "type": "string"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub struct NsiId(pub String);

	impl ::std::ops::Deref for NsiId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NsiId> for String {
		fn from(value: NsiId) -> Self {
			value.0
		}
	}

	impl From<&NsiId> for NsiId {
		fn from(value: &NsiId) -> Self {
			value.clone()
		}
	}

	impl From<String> for NsiId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NsiId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for NsiId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains the API URIs of NRF services to be used to discover
	/// NFs/services, subscribe to NF status changes and/or request access
	/// tokens within the selected Network Slice instance and optional the
	/// Identifier of the selected Network Slice instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the API URIs of NRF services to be used to
	/// discover NFs/services, subscribe to NF status changes and/or request
	/// access tokens within the selected Network Slice instance and optional
	/// the Identifier of the selected Network Slice instance\n",
	///  "type": "object",
	///  "required": [
	///    "nrfId"
	///  ],
	///  "properties": {
	///    "nrfAccessTokenUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfId": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfNfMgtUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfOauth2Required": {
	///      "description": "Map indicating whether the NRF requires
	/// Oauth2-based authorization for accessing its services. The key of the
	/// map shall be the name of an NRF service, e.g. \"nnrf-nfm\" or
	/// \"nnrf-disc\"\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "boolean"
	///      }
	///    },
	///    "nsiId": {
	///      "$ref": "#/components/schemas/NsiId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsiInformation {
		#[serde(
			rename = "nrfAccessTokenUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_access_token_uri: Option<Uri>,
		#[serde(rename = "nrfId")]
		pub nrf_id: Uri,
		#[serde(
			rename = "nrfNfMgtUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_nf_mgt_uri: Option<Uri>,
		/// Map indicating whether the NRF requires Oauth2-based authorization
		/// for accessing its services. The key of the map shall be the name of
		/// an NRF service, e.g. "nnrf-nfm" or "nnrf-disc"
		#[serde(
			rename = "nrfOauth2Required",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nrf_oauth2_required: ::std::collections::HashMap<String, bool>,
		#[serde(rename = "nsiId", default, skip_serializing_if = "Option::is_none")]
		pub nsi_id: Option<NsiId>,
	}

	impl From<&NsiInformation> for NsiInformation {
		fn from(value: &NsiInformation) -> Self {
			value.clone()
		}
	}

	/// This contains the Nssai availability information requested by the AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the Nssai availability information
	/// requested by the AMF",
	///  "type": "object",
	///  "required": [
	///    "supportedNssaiAvailabilityData"
	///  ],
	///  "properties": {
	///    "amfSetId": {
	///      "type": "string",
	///      "pattern":
	/// "^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "supportedNssaiAvailabilityData": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupportedNssaiAvailabilityData"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NssaiAvailabilityInfo {
		#[serde(rename = "amfSetId", default, skip_serializing_if = "Option::is_none")]
		pub amf_set_id: Option<NssaiAvailabilityInfoAmfSetId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "supportedNssaiAvailabilityData")]
		pub supported_nssai_availability_data: Vec<SupportedNssaiAvailabilityData>,
	}

	impl From<&NssaiAvailabilityInfo> for NssaiAvailabilityInfo {
		fn from(value: &NssaiAvailabilityInfo) -> Self {
			value.clone()
		}
	}

	/// NssaiAvailabilityInfoAmfSetId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct NssaiAvailabilityInfoAmfSetId(String);

	impl ::std::ops::Deref for NssaiAvailabilityInfoAmfSetId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NssaiAvailabilityInfoAmfSetId> for String {
		fn from(value: NssaiAvailabilityInfoAmfSetId) -> Self {
			value.0
		}
	}

	impl From<&NssaiAvailabilityInfoAmfSetId> for NssaiAvailabilityInfoAmfSetId {
		fn from(value: &NssaiAvailabilityInfoAmfSetId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for NssaiAvailabilityInfoAmfSetId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \
				            \"^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for NssaiAvailabilityInfoAmfSetId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for NssaiAvailabilityInfoAmfSetId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for NssaiAvailabilityInfoAmfSetId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for NssaiAvailabilityInfoAmfSetId {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// This contains the notification for created event subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the notification for created event
	/// subscription",
	///  "type": "object",
	///  "required": [
	///    "authorizedNssaiAvailabilityData",
	///    "subscriptionId"
	///  ],
	///  "properties": {
	///    "authorizedNssaiAvailabilityData": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AuthorizedNssaiAvailabilityData"
	///      }
	///    },
	///    "subscriptionId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NssfEventNotification {
		#[serde(rename = "authorizedNssaiAvailabilityData")]
		pub authorized_nssai_availability_data: Vec<AuthorizedNssaiAvailabilityData>,
		#[serde(rename = "subscriptionId")]
		pub subscription_id: String,
	}

	impl From<&NssfEventNotification> for NssfEventNotification {
		fn from(value: &NssfEventNotification) -> Self {
			value.clone()
		}
	}

	/// This contains the information for event subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the information for event subscription",
	///  "type": "object",
	///  "required": [
	///    "event",
	///    "nfNssaiAvailabilityUri",
	///    "taiList"
	///  ],
	///  "properties": {
	///    "amfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "amfSetId": {
	///      "type": "string",
	///      "pattern":
	/// "^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$"
	///    },
	///    "event": {
	///      "$ref": "#/components/schemas/NssfEventType"
	///    },
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "nfNssaiAvailabilityUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      }
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NssfEventSubscriptionCreateData {
		#[serde(rename = "amfId", default, skip_serializing_if = "Option::is_none")]
		pub amf_id: Option<NfInstanceId>,
		#[serde(rename = "amfSetId", default, skip_serializing_if = "Option::is_none")]
		pub amf_set_id: Option<NssfEventSubscriptionCreateDataAmfSetId>,
		pub event: NssfEventType,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
		#[serde(rename = "nfNssaiAvailabilityUri")]
		pub nf_nssai_availability_uri: Uri,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "taiList")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&NssfEventSubscriptionCreateData> for NssfEventSubscriptionCreateData {
		fn from(value: &NssfEventSubscriptionCreateData) -> Self {
			value.clone()
		}
	}

	/// NssfEventSubscriptionCreateDataAmfSetId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct NssfEventSubscriptionCreateDataAmfSetId(String);

	impl ::std::ops::Deref for NssfEventSubscriptionCreateDataAmfSetId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NssfEventSubscriptionCreateDataAmfSetId> for String {
		fn from(value: NssfEventSubscriptionCreateDataAmfSetId) -> Self {
			value.0
		}
	}

	impl From<&NssfEventSubscriptionCreateDataAmfSetId> for NssfEventSubscriptionCreateDataAmfSetId {
		fn from(value: &NssfEventSubscriptionCreateDataAmfSetId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for NssfEventSubscriptionCreateDataAmfSetId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \
				            \"^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for NssfEventSubscriptionCreateDataAmfSetId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for NssfEventSubscriptionCreateDataAmfSetId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for NssfEventSubscriptionCreateDataAmfSetId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for NssfEventSubscriptionCreateDataAmfSetId {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// This contains the information for created event subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the information for created event
	/// subscription",
	///  "type": "object",
	///  "required": [
	///    "subscriptionId"
	///  ],
	///  "properties": {
	///    "authorizedNssaiAvailabilityData": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AuthorizedNssaiAvailabilityData"
	///      },
	///      "minItems": 1
	///    },
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "subscriptionId": {
	///      "type": "string"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NssfEventSubscriptionCreatedData {
		#[serde(
			rename = "authorizedNssaiAvailabilityData",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub authorized_nssai_availability_data: Vec<AuthorizedNssaiAvailabilityData>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
		#[serde(rename = "subscriptionId")]
		pub subscription_id: String,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&NssfEventSubscriptionCreatedData> for NssfEventSubscriptionCreatedData {
		fn from(value: &NssfEventSubscriptionCreatedData) -> Self {
			value.clone()
		}
	}

	/// This contains the event for the subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the event for the subscription",
	///  "type": "string",
	///  "enum": [
	///    "SNSSAI_STATUS_CHANGE_REPORT"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum NssfEventType {
		#[default]
		#[serde(rename = "SNSSAI_STATUS_CHANGE_REPORT")]
		SnssaiStatusChangeReport,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NssfEventType> for NssfEventType {
		fn from(value: &NssfEventType) -> Self {
			value.clone()
		}
	}

	impl ToString for NssfEventType {
		fn to_string(&self) -> String {
			match *self {
				Self::SnssaiStatusChangeReport => "SNSSAI_STATUS_CHANGE_REPORT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NssfEventType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SNSSAI_STATUS_CHANGE_REPORT" => Ok(Self::SnssaiStatusChangeReport),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NssfEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NssfEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NssfEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// This contains the JSON Patch instructions for updating the Nssai
	/// availability data information at the NSSF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the JSON Patch instructions for updating
	/// the Nssai availability data information at the NSSF",
	///  "type": "array",
	///  "items": {
	///    "$ref": "#/components/schemas/PatchItem"
	///  },
	///  "minItems": 1
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PatchDocument(pub Vec<PatchItem>);

	impl ::std::ops::Deref for PatchDocument {
		type Target = Vec<PatchItem>;
		fn deref(&self) -> &Vec<PatchItem> {
			&self.0
		}
	}

	impl From<PatchDocument> for Vec<PatchItem> {
		fn from(value: PatchDocument) -> Self {
			value.0
		}
	}

	impl From<&PatchDocument> for PatchDocument {
		fn from(value: &PatchDocument) -> Self {
			value.clone()
		}
	}

	impl From<Vec<PatchItem>> for PatchDocument {
		fn from(value: Vec<PatchItem>) -> Self {
			Self(value)
		}
	}

	/// This contains the restricted SNssai information per PLMN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the restricted SNssai information per
	/// PLMN",
	///  "type": "object",
	///  "required": [
	///    "homePlmnId",
	///    "sNssaiList"
	///  ],
	///  "properties": {
	///    "homePlmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "homePlmnIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 1
	///    },
	///    "roamingRestriction": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "sNssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RestrictedSnssai {
		#[serde(rename = "homePlmnId")]
		pub home_plmn_id: PlmnId,
		#[serde(
			rename = "homePlmnIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub home_plmn_id_list: Vec<PlmnId>,
		#[serde(rename = "roamingRestriction", default)]
		pub roaming_restriction: bool,
		#[serde(rename = "sNssaiList")]
		pub s_nssai_list: Vec<ExtSnssai>,
	}

	impl From<&RestrictedSnssai> for RestrictedSnssai {
		fn from(value: &RestrictedSnssai) -> Self {
			value.clone()
		}
	}

	/// Contains the indication on roaming
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the indication on roaming",
	///  "type": "string",
	///  "enum": [
	///    "NON_ROAMING",
	///    "LOCAL_BREAKOUT",
	///    "HOME_ROUTED_ROAMING"
	///  ],
	///  "x-allow-unknown": true
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum RoamingIndication {
		#[default]
		#[serde(rename = "NON_ROAMING")]
		NonRoaming,
		#[serde(rename = "LOCAL_BREAKOUT")]
		LocalBreakout,
		#[serde(rename = "HOME_ROUTED_ROAMING")]
		HomeRoutedRoaming,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RoamingIndication> for RoamingIndication {
		fn from(value: &RoamingIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for RoamingIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::NonRoaming => "NON_ROAMING".to_string(),
				Self::LocalBreakout => "LOCAL_BREAKOUT".to_string(),
				Self::HomeRoutedRoaming => "HOME_ROUTED_ROAMING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RoamingIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NON_ROAMING" => Ok(Self::NonRoaming),
				"LOCAL_BREAKOUT" => Ok(Self::LocalBreakout),
				"HOME_ROUTED_ROAMING" => Ok(Self::HomeRoutedRoaming),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RoamingIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RoamingIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RoamingIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the association of NSAGs and S-NSSAI(s) along with the TA(s)
	/// within which the association is valid.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the association of NSAGs and S-NSSAI(s) along
	/// with the TA(s) within which the association is valid.",
	///  "type": "object",
	///  "required": [
	///    "nsagIds",
	///    "snssaiList"
	///  ],
	///  "properties": {
	///    "nsagIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsagId"
	///      },
	///      "minItems": 1
	///    },
	///    "snssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasNsagInfo {
		#[serde(rename = "nsagIds")]
		pub nsag_ids: Vec<NsagId>,
		#[serde(rename = "snssaiList")]
		pub snssai_list: Vec<Snssai>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&SchemasNsagInfo> for SchemasNsagInfo {
		fn from(value: &SchemasNsagInfo) -> Self {
			value.clone()
		}
	}

	/// Contains the slice information requested during PDU Session
	/// establishment procedure
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the slice information requested during PDU
	/// Session establishment procedure",
	///  "type": "object",
	///  "required": [
	///    "roamingIndication",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "homeSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "roamingIndication": {
	///      "$ref": "#/components/schemas/RoamingIndication"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SliceInfoForPduSession {
		#[serde(
			rename = "homeSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub home_snssai: Option<Snssai>,
		#[serde(rename = "roamingIndication")]
		pub roaming_indication: RoamingIndication,
		#[serde(rename = "sNssai")]
		pub s_nssai: Snssai,
	}

	impl From<&SliceInfoForPduSession> for SliceInfoForPduSession {
		fn from(value: &SliceInfoForPduSession) -> Self {
			value.clone()
		}
	}

	/// Contains the slice information requested during a Registration procedure
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the slice information requested during a
	/// Registration procedure",
	///  "type": "object",
	///  "properties": {
	///    "allowedNssaiCurrentAccess": {
	///      "$ref": "#/components/schemas/AllowedNssai"
	///    },
	///    "allowedNssaiOtherAccess": {
	///      "$ref": "#/components/schemas/AllowedNssai"
	///    },
	///    "defaultConfiguredSnssaiInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "mappingOfNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MappingOfSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "nsagSupported": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "requestMapping": {
	///      "type": "boolean"
	///    },
	///    "requestedNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssaiForMapping": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "subscribedNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SubscribedSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "suppressNssrgInd": {
	///      "type": "boolean"
	///    },
	///    "ueSupNssrgInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SliceInfoForRegistration {
		#[serde(
			rename = "allowedNssaiCurrentAccess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub allowed_nssai_current_access: Option<AllowedNssai>,
		#[serde(
			rename = "allowedNssaiOtherAccess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub allowed_nssai_other_access: Option<AllowedNssai>,
		#[serde(rename = "defaultConfiguredSnssaiInd", default)]
		pub default_configured_snssai_ind: bool,
		#[serde(
			rename = "mappingOfNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mapping_of_nssai: Vec<MappingOfSnssai>,
		#[serde(rename = "nsagSupported", default)]
		pub nsag_supported: bool,
		#[serde(
			rename = "requestMapping",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub request_mapping: Option<bool>,
		#[serde(
			rename = "requestedNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub requested_nssai: Vec<Snssai>,
		#[serde(
			rename = "sNssaiForMapping",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub s_nssai_for_mapping: Vec<Snssai>,
		#[serde(
			rename = "subscribedNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub subscribed_nssai: Vec<SubscribedSnssai>,
		#[serde(
			rename = "suppressNssrgInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub suppress_nssrg_ind: Option<bool>,
		#[serde(
			rename = "ueSupNssrgInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_sup_nssrg_ind: Option<bool>,
	}

	impl From<&SliceInfoForRegistration> for SliceInfoForRegistration {
		fn from(value: &SliceInfoForRegistration) -> Self {
			value.clone()
		}
	}

	/// Contains the slice information requested during UE configuration update
	/// procedure
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the slice information requested during UE
	/// configuration update procedure",
	///  "type": "object",
	///  "properties": {
	///    "allowedNssaiCurrentAccess": {
	///      "$ref": "#/components/schemas/AllowedNssai"
	///    },
	///    "allowedNssaiOtherAccess": {
	///      "$ref": "#/components/schemas/AllowedNssai"
	///    },
	///    "defaultConfiguredSnssaiInd": {
	///      "type": "boolean"
	///    },
	///    "mappingOfNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MappingOfSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "nsagSupported": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "rejectedNssaiRa": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "requestedNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "subscribedNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SubscribedSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "suppressNssrgInd": {
	///      "type": "boolean"
	///    },
	///    "ueSupNssrgInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SliceInfoForUeConfigurationUpdate {
		#[serde(
			rename = "allowedNssaiCurrentAccess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub allowed_nssai_current_access: Option<AllowedNssai>,
		#[serde(
			rename = "allowedNssaiOtherAccess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub allowed_nssai_other_access: Option<AllowedNssai>,
		#[serde(
			rename = "defaultConfiguredSnssaiInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub default_configured_snssai_ind: Option<bool>,
		#[serde(
			rename = "mappingOfNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mapping_of_nssai: Vec<MappingOfSnssai>,
		#[serde(rename = "nsagSupported", default)]
		pub nsag_supported: bool,
		#[serde(
			rename = "rejectedNssaiRa",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub rejected_nssai_ra: Vec<Snssai>,
		#[serde(
			rename = "requestedNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub requested_nssai: Vec<Snssai>,
		#[serde(
			rename = "subscribedNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub subscribed_nssai: Vec<SubscribedSnssai>,
		#[serde(
			rename = "suppressNssrgInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub suppress_nssrg_ind: Option<bool>,
		#[serde(
			rename = "ueSupNssrgInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_sup_nssrg_ind: Option<bool>,
	}

	impl From<&SliceInfoForUeConfigurationUpdate> for SliceInfoForUeConfigurationUpdate {
		fn from(value: &SliceInfoForUeConfigurationUpdate) -> Self {
			value.clone()
		}
	}

	/// Contains the subscribed S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the subscribed S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "subscribedSnssai"
	///  ],
	///  "properties": {
	///    "defaultIndication": {
	///      "type": "boolean"
	///    },
	///    "subscribedNsSrgList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsSrg"
	///      },
	///      "minItems": 1
	///    },
	///    "subscribedSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SubscribedSnssai {
		#[serde(
			rename = "defaultIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub default_indication: Option<bool>,
		#[serde(
			rename = "subscribedNsSrgList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub subscribed_ns_srg_list: Vec<NsSrg>,
		#[serde(rename = "subscribedSnssai")]
		pub subscribed_snssai: Snssai,
	}

	impl From<&SubscribedSnssai> for SubscribedSnssai {
		fn from(value: &SubscribedSnssai) -> Self {
			value.clone()
		}
	}

	/// This contains the Nssai availability data information per TA supported
	/// by the AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This contains the Nssai availability data information
	/// per TA supported by the AMF",
	///  "type": "object",
	///  "required": [
	///    "supportedSnssaiList",
	///    "tai"
	///  ],
	///  "properties": {
	///    "nsagInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-NsagInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "tai": {
	///      "$ref": "#/components/schemas/Tai"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SupportedNssaiAvailabilityData {
		#[serde(rename = "nsagInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub nsag_infos: Vec<SchemasNsagInfo>,
		#[serde(rename = "supportedSnssaiList")]
		pub supported_snssai_list: Vec<ExtSnssai>,
		pub tai: Tai,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&SupportedNssaiAvailabilityData> for SupportedNssaiAvailabilityData {
		fn from(value: &SupportedNssaiAvailabilityData) -> Self {
			value.clone()
		}
	}

	/// Range of TACs (Tracking Area Codes)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of TACs (Tracking Area Codes)",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "type": "string",
	///      "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
	///    },
	///    "pattern": {
	///      "type": "string"
	///    },
	///    "start": {
	///      "type": "string",
	///      "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TacRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<TacRangeEnd>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pattern: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<TacRangeStart>,
	}

	impl From<&TacRange> for TacRange {
		fn from(value: &TacRange) -> Self {
			value.clone()
		}
	}

	/// TacRangeEnd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct TacRangeEnd(String);

	impl ::std::ops::Deref for TacRangeEnd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TacRangeEnd> for String {
		fn from(value: TacRangeEnd) -> Self {
			value.0
		}
	}

	impl From<&TacRangeEnd> for TacRangeEnd {
		fn from(value: &TacRangeEnd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TacRangeEnd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TacRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TacRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TacRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TacRangeEnd {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// TacRangeStart
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct TacRangeStart(String);

	impl ::std::ops::Deref for TacRangeStart {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TacRangeStart> for String {
		fn from(value: TacRangeStart) -> Self {
			value.0
		}
	}

	impl From<&TacRangeStart> for TacRangeStart {
		fn from(value: &TacRangeStart) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TacRangeStart {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TacRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TacRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TacRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TacRangeStart {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// Range of TAIs (Tracking Area Identities)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of TAIs (Tracking Area Identities)",
	///  "type": "object",
	///  "required": [
	///    "plmnId",
	///    "tacRangeList"
	///  ],
	///  "properties": {
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "tacRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TacRange"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TaiRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		#[serde(rename = "tacRangeList")]
		pub tac_range_list: Vec<TacRange>,
	}

	impl From<&TaiRange> for TaiRange {
		fn from(value: &TaiRange) -> Self {
			value.clone()
		}
	}

	/// String providing an URI formatted according to RFC 3986.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String providing an URI formatted according to RFC
	/// 3986.",
	///  "type": "string"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub struct Uri(pub String);

	impl ::std::ops::Deref for Uri {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Uri> for String {
		fn from(value: Uri) -> Self {
			value.0
		}
	}

	impl From<&Uri> for Uri {
		fn from(value: &Uri) -> Self {
			value.clone()
		}
	}

	impl From<String> for Uri {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Uri {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for Uri {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}
}
