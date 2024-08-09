#[allow(unused_imports)]
use crate::progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use crate::progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///Error returned in the access token response message
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Error returned in the access token response message",
    ///  "type": "object",
    ///  "required": [
    ///    "error"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "type": "string",
    ///      "enum": [
    ///        "invalid_request",
    ///        "invalid_client",
    ///        "invalid_grant",
    ///        "unauthorized_client",
    ///        "unsupported_grant_type",
    ///        "invalid_scope"
    ///      ]
    ///    },
    ///    "error_description": {
    ///      "type": "string"
    ///    },
    ///    "error_uri": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct AccessTokenErr {
        pub error: AccessTokenErrError,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error_description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error_uri: Option<String>,
    }

    impl From<&AccessTokenErr> for AccessTokenErr {
        fn from(value: &AccessTokenErr) -> Self {
            value.clone()
        }
    }

    ///AccessTokenErrError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "invalid_request",
    ///    "invalid_client",
    ///    "invalid_grant",
    ///    "unauthorized_client",
    ///    "unsupported_grant_type",
    ///    "invalid_scope"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum AccessTokenErrError {
        #[serde(rename = "invalid_request")]
        InvalidRequest,
        #[serde(rename = "invalid_client")]
        InvalidClient,
        #[serde(rename = "invalid_grant")]
        InvalidGrant,
        #[serde(rename = "unauthorized_client")]
        UnauthorizedClient,
        #[serde(rename = "unsupported_grant_type")]
        UnsupportedGrantType,
        #[serde(rename = "invalid_scope")]
        InvalidScope,
    }

    impl From<&AccessTokenErrError> for AccessTokenErrError {
        fn from(value: &AccessTokenErrError) -> Self {
            value.clone()
        }
    }

    impl ToString for AccessTokenErrError {
        fn to_string(&self) -> String {
            match *self {
                Self::InvalidRequest => "invalid_request".to_string(),
                Self::InvalidClient => "invalid_client".to_string(),
                Self::InvalidGrant => "invalid_grant".to_string(),
                Self::UnauthorizedClient => "unauthorized_client".to_string(),
                Self::UnsupportedGrantType => "unsupported_grant_type".to_string(),
                Self::InvalidScope => "invalid_scope".to_string(),
            }
        }
    }

    impl std::str::FromStr for AccessTokenErrError {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "invalid_request" => Ok(Self::InvalidRequest),
                "invalid_client" => Ok(Self::InvalidClient),
                "invalid_grant" => Ok(Self::InvalidGrant),
                "unauthorized_client" => Ok(Self::UnauthorizedClient),
                "unsupported_grant_type" => Ok(Self::UnsupportedGrantType),
                "invalid_scope" => Ok(Self::InvalidScope),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for AccessTokenErrError {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for AccessTokenErrError {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for AccessTokenErrError {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Contains information related to the access token request
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct AccessTokenReq {
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
        #[serde(
            rename = "requesterPlmn",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub requester_plmn: Option<PlmnId>,
        #[serde(
            rename = "requesterPlmnList",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub requester_plmn_list: Vec<PlmnId>,
        #[serde(
            rename = "requesterSnpnList",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub requester_snpn_list: Vec<PlmnIdNid>,
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
        pub target_nsi_list: Vec<String>,
        #[serde(
            rename = "targetPlmn",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub target_plmn: Option<PlmnId>,
        #[serde(
            rename = "targetSnpn",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub target_snpn: Option<PlmnIdNid>,
        #[serde(
            rename = "targetSnssaiList",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub target_snssai_list: Vec<Snssai>,
    }

    impl From<&AccessTokenReq> for AccessTokenReq {
        fn from(value: &AccessTokenReq) -> Self {
            value.clone()
        }
    }

    ///AccessTokenReqGrantType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "client_credentials"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum AccessTokenReqGrantType {
        #[serde(rename = "client_credentials")]
        ClientCredentials,
    }

    impl From<&AccessTokenReqGrantType> for AccessTokenReqGrantType {
        fn from(value: &AccessTokenReqGrantType) -> Self {
            value.clone()
        }
    }

    impl ToString for AccessTokenReqGrantType {
        fn to_string(&self) -> String {
            match *self {
                Self::ClientCredentials => "client_credentials".to_string(),
            }
        }
    }

    impl std::str::FromStr for AccessTokenReqGrantType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "client_credentials" => Ok(Self::ClientCredentials),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for AccessTokenReqGrantType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for AccessTokenReqGrantType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for AccessTokenReqGrantType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///AccessTokenReqScope
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^([a-zA-Z0-9_:-]+)( [a-zA-Z0-9_:-]+)*$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct AccessTokenReqScope(String);
    impl std::ops::Deref for AccessTokenReqScope {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<AccessTokenReqScope> for String {
        fn from(value: AccessTokenReqScope) -> Self {
            value.0
        }
    }

    impl From<&AccessTokenReqScope> for AccessTokenReqScope {
        fn from(value: &AccessTokenReqScope) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for AccessTokenReqScope {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^([a-zA-Z0-9_:-]+)( [a-zA-Z0-9_:-]+)*$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err(
                    "doesn't match pattern \"^([a-zA-Z0-9_:-]+)( [a-zA-Z0-9_:-]+)*$\"".into(),
                );
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for AccessTokenReqScope {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for AccessTokenReqScope {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for AccessTokenReqScope {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for AccessTokenReqScope {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Indicates whether the access is  via 3GPP or via non-3GPP.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Indicates whether the access is  via 3GPP or via
    /// non-3GPP.",
    ///  "type": "string",
    ///  "enum": [
    ///    "3GPP_ACCESS",
    ///    "NON_3GPP_ACCESS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum AccessType {
        #[serde(rename = "3GPP_ACCESS")]
        _3gppAccess,
        #[serde(rename = "NON_3GPP_ACCESS")]
        Non3gppAccess,
    }

    impl From<&AccessType> for AccessType {
        fn from(value: &AccessType) -> Self {
            value.clone()
        }
    }

    impl ToString for AccessType {
        fn to_string(&self) -> String {
            match *self {
                Self::_3gppAccess => "3GPP_ACCESS".to_string(),
                Self::Non3gppAccess => "NON_3GPP_ACCESS".to_string(),
            }
        }
    }

    impl std::str::FromStr for AccessType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "3GPP_ACCESS" => Ok(Self::_3gppAccess),
                "NON_3GPP_ACCESS" => Ok(Self::Non3gppAccess),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for AccessType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for AccessType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for AccessType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Contains an array of allowed S-NSSAI that constitute the allowed NSSAI
    /// information for the authorized network slice information
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///Contains the authorized S-NSSAI and optional mapped home S-NSSAI and
    /// network slice instance information
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///Contains the authorized network slice information
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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
        ///Map indicating whether the NRF requires Oauth2-based authorization
        /// for accessing its services. The key of the map shall be the name of
        /// an NRF service, e.g. "nnrf-nfm" or "nnrf-disc"
        #[serde(
            rename = "nrfOauth2Required",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub nrf_oauth2_required: std::collections::HashMap<String, bool>,
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

    ///AuthorizedNetworkSliceInfoTargetAmfSet
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct AuthorizedNetworkSliceInfoTargetAmfSet(String);
    impl std::ops::Deref for AuthorizedNetworkSliceInfoTargetAmfSet {
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

    impl std::str::FromStr for AuthorizedNetworkSliceInfoTargetAmfSet {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err ("doesn't match pattern \"^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for AuthorizedNetworkSliceInfoTargetAmfSet {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for AuthorizedNetworkSliceInfoTargetAmfSet {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for AuthorizedNetworkSliceInfoTargetAmfSet {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for AuthorizedNetworkSliceInfoTargetAmfSet {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///This contains the Nssai availability data information per TA authorized
    /// by the NSSF
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///This contains the Nssai availability data information authorized by the
    /// NSSF
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///Contains the configured S-NSSAI(s) authorized by the NSSF in the serving
    /// PLMN and optional mapped home S-NSSAI
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///string with format 'date-time' as defined in OpenAPI.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "string with format 'date-time' as defined in OpenAPI.",
    ///  "type": "string",
    ///  "format": "date-time"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DateTime(pub chrono::DateTime<chrono::offset::Utc>);
    impl std::ops::Deref for DateTime {
        type Target = chrono::DateTime<chrono::offset::Utc>;
        fn deref(&self) -> &chrono::DateTime<chrono::offset::Utc> {
            &self.0
        }
    }

    impl From<DateTime> for chrono::DateTime<chrono::offset::Utc> {
        fn from(value: DateTime) -> Self {
            value.0
        }
    }

    impl From<&DateTime> for DateTime {
        fn from(value: &DateTime) -> Self {
            value.clone()
        }
    }

    impl From<chrono::DateTime<chrono::offset::Utc>> for DateTime {
        fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for DateTime {
        type Err = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for DateTime {
        type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for DateTime {
        type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for DateTime {
        type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for DateTime {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///The sdRanges and wildcardSd attributes shall be exclusive from each
    /// other. If one of these attributes is present,  the sd attribute shall
    /// also be present and it shall contain one Slice Differentiator value
    /// within the range of SD  (if the sdRanges attribute is present) or with
    /// any value (if the wildcardSd attribute is present).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The sdRanges and wildcardSd attributes shall be
    /// exclusive from each other. If one of these attributes is present,  the
    /// sd attribute shall also be present and it shall contain one Slice
    /// Differentiator value within the range of SD  (if the sdRanges attribute
    /// is present) or with any value (if the wildcardSd attribute is
    /// present).\n",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Snssai"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/SnssaiExtension"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ExtSnssai {
        ///3-octet string, representing the Slice Differentiator, in
        /// hexadecimal representation. Each character in the string shall take
        /// a value of "0" to "9", "a" to "f" or "A" to "F" and shall represent
        /// 4 bits. The most significant character representing the 4 most
        /// significant bits of the SD shall appear first in the string, and the
        /// character representing the 4 least significant bit of the SD shall
        /// appear last in the string. This is an optional parameter that
        /// complements the Slice/Service type(s) to allow to  differentiate
        /// amongst multiple Network Slices of the same Slice/Service type. This
        /// IE shall be absent if no SD value is associated with the SST.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sd: Option<ExtSnssaiSd>,
        ///Unsigned integer, within the range 0 to 255, representing the
        /// Slice/Service Type.  It indicates the expected Network Slice
        /// behaviour in terms of features and services. Values 0 to 127
        /// correspond to the standardized SST range. Values 128 to 255
        /// correspond  to the Operator-specific range. See clause 28.4.2 of
        /// 3GPP TS 23.003. Standardized values are defined in clause 5.15.2.2
        /// of 3GPP TS 23.501.
        pub sst: u8,
    }

    impl From<&ExtSnssai> for ExtSnssai {
        fn from(value: &ExtSnssai) -> Self {
            value.clone()
        }
    }

    ///3-octet string, representing the Slice Differentiator, in hexadecimal
    /// representation. Each character in the string shall take a value of "0"
    /// to "9", "a" to "f" or "A" to "F" and shall represent 4 bits. The most
    /// significant character representing the 4 most significant bits of the SD
    /// shall appear first in the string, and the character representing the 4
    /// least significant bit of the SD shall appear last in the string. This is
    /// an optional parameter that complements the Slice/Service type(s) to
    /// allow to  differentiate amongst multiple Network Slices of the same
    /// Slice/Service type. This IE shall be absent if no SD value is associated
    /// with the SST.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "3-octet string, representing the Slice Differentiator,
    /// in hexadecimal representation. Each character in the string shall take a
    /// value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall
    /// represent 4 bits. The most significant character representing the 4 most
    /// significant bits of the SD shall appear first in the string, and the
    /// character representing the 4 least significant bit of the SD shall
    /// appear last in the string. This is an optional parameter that
    /// complements the Slice/Service type(s) to allow to  differentiate amongst
    /// multiple Network Slices of the same Slice/Service type. This IE shall be
    /// absent if no SD value is associated with the SST.\n",
    ///  "type": "string",
    ///  "pattern": "^[A-Fa-f0-9]{6}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct ExtSnssaiSd(String);
    impl std::ops::Deref for ExtSnssaiSd {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<ExtSnssaiSd> for String {
        fn from(value: ExtSnssaiSd) -> Self {
            value.0
        }
    }

    impl From<&ExtSnssaiSd> for ExtSnssaiSd {
        fn from(value: &ExtSnssaiSd) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for ExtSnssaiSd {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[A-Fa-f0-9]{6}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[A-Fa-f0-9]{6}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for ExtSnssaiSd {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ExtSnssaiSd {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ExtSnssaiSd {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for ExtSnssaiSd {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Fully Qualified Domain Name
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Fully Qualified Domain Name",
    ///  "type": "string",
    ///  "maxLength": 253,
    ///  "minLength": 4,
    ///  "pattern":
    /// "^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct Fqdn(String);
    impl std::ops::Deref for Fqdn {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Fqdn> for String {
        fn from(value: Fqdn) -> Self {
            value.0
        }
    }

    impl From<&Fqdn> for Fqdn {
        fn from(value: &Fqdn) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for Fqdn {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if value.len() > 253usize {
                return Err("longer than 253 characters".into());
            }
            if value.len() < 4usize {
                return Err("shorter than 4 characters".into());
            }
            if regress::Regex::new(
                "^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$",
            )
            .unwrap()
            .find(value)
            .is_none()
            {
                return Err ("doesn't match pattern \"^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for Fqdn {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Fqdn {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Fqdn {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for Fqdn {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///It contains an invalid parameter and a related description.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "It contains an invalid parameter and a related
    /// description.",
    ///  "type": "object",
    ///  "required": [
    ///    "param"
    ///  ],
    ///  "properties": {
    ///    "param": {
    ///      "description": "If the invalid parameter is an attribute in a JSON
    /// body, this IE shall contain the  attribute's name and shall be encoded
    /// as a JSON Pointer. If the invalid parameter is  an HTTP header, this IE
    /// shall be formatted as the concatenation of the string \"header \"  plus
    /// the name of such header. If the invalid parameter is a query parameter,
    /// this IE  shall be formatted as the concatenation of the string \"query
    /// \" plus the name of such  query parameter. If the invalid parameter is a
    /// variable part in the path of a resource  URI, this IE shall contain the
    /// name of the variable, including the symbols \"{\" and \"}\"  used in
    /// OpenAPI specification as the notation to represent variable path
    /// segments.\n",
    ///      "type": "string"
    ///    },
    ///    "reason": {
    ///      "description": "A human-readable reason, e.g. \"must be a positive
    /// integer\". In cases involving failed  operations in a PATCH request, the
    /// reason string should identify the operation that  failed using the
    /// operation's array index to assist in correlation of the invalid
    /// parameter with the failed operation, e.g.\" Replacement value invalid
    /// for attribute  (failed operation index= 4)\"\n",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct InvalidParam {
        ///If the invalid parameter is an attribute in a JSON body, this IE
        /// shall contain the  attribute's name and shall be encoded as a JSON
        /// Pointer. If the invalid parameter is  an HTTP header, this IE shall
        /// be formatted as the concatenation of the string "header "  plus the
        /// name of such header. If the invalid parameter is a query parameter,
        /// this IE  shall be formatted as the concatenation of the string
        /// "query " plus the name of such  query parameter. If the invalid
        /// parameter is a variable part in the path of a resource  URI, this IE
        /// shall contain the name of the variable, including the symbols "{"
        /// and "}"  used in OpenAPI specification as the notation to represent
        /// variable path segments.
        pub param: String,
        ///A human-readable reason, e.g. "must be a positive integer". In cases
        /// involving failed  operations in a PATCH request, the reason string
        /// should identify the operation that  failed using the operation's
        /// array index to assist in correlation of the invalid  parameter with
        /// the failed operation, e.g." Replacement value invalid for attribute
        /// (failed operation index= 4)"
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
    }

    impl From<&InvalidParam> for InvalidParam {
        fn from(value: &InvalidParam) -> Self {
            value.clone()
        }
    }

    ///Contains the mapping of S-NSSAI in the serving network and the value of
    /// the home network
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///Mobile Country Code part of the PLMN, comprising 3 digits, as defined in
    /// clause 9.3.3.5 of 3GPP TS 38.413.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Mobile Country Code part of the PLMN, comprising 3
    /// digits, as defined in clause 9.3.3.5 of 3GPP TS 38.413. \n",
    ///  "type": "string",
    ///  "pattern": "^\\d{3}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct Mcc(String);
    impl std::ops::Deref for Mcc {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Mcc> for String {
        fn from(value: Mcc) -> Self {
            value.0
        }
    }

    impl From<&Mcc> for Mcc {
        fn from(value: &Mcc) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for Mcc {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^\\d{3}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^\\d{3}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for Mcc {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Mcc {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Mcc {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for Mcc {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Mobile Network Code part of the PLMN, comprising 2 or 3 digits, as
    /// defined in clause 9.3.3.5 of 3GPP TS 38.413.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Mobile Network Code part of the PLMN, comprising 2 or 3
    /// digits, as defined in clause 9.3.3.5 of 3GPP TS 38.413.",
    ///  "type": "string",
    ///  "pattern": "^\\d{2,3}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct Mnc(String);
    impl std::ops::Deref for Mnc {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Mnc> for String {
        fn from(value: Mnc) -> Self {
            value.0
        }
    }

    impl From<&Mnc> for Mnc {
        fn from(value: &Mnc) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for Mnc {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^\\d{2,3}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^\\d{2,3}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for Mnc {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Mnc {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Mnc {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for Mnc {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///String uniquely identifying a NF instance. The format of the NF Instance
    /// ID shall be a  Universally Unique Identifier (UUID) version 4, as
    /// described in IETF RFC 4122.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "String uniquely identifying a NF instance. The format
    /// of the NF Instance ID shall be a  Universally Unique Identifier (UUID)
    /// version 4, as described in IETF RFC 4122. \n",
    ///  "type": "string",
    ///  "format": "uuid"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct NfInstanceId(pub uuid::Uuid);
    impl std::ops::Deref for NfInstanceId {
        type Target = uuid::Uuid;
        fn deref(&self) -> &uuid::Uuid {
            &self.0
        }
    }

    impl From<NfInstanceId> for uuid::Uuid {
        fn from(value: NfInstanceId) -> Self {
            value.0
        }
    }

    impl From<&NfInstanceId> for NfInstanceId {
        fn from(value: &NfInstanceId) -> Self {
            value.clone()
        }
    }

    impl From<uuid::Uuid> for NfInstanceId {
        fn from(value: uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for NfInstanceId {
        type Err = <uuid::Uuid as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for NfInstanceId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NfInstanceId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NfInstanceId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for NfInstanceId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///NF Service Set Identifier (see clause 28.12 of 3GPP TS 23.003) formatted
    /// as the following  string "set<Set ID>.sn<Service Name>.nfi<NF Instance
    /// ID>.5gc.mnc<MNC>.mcc<MCC>", or
    /// "set<SetID>.sn<ServiceName>.nfi<NFInstanceID>.5gc.nid<NID>.mnc<MNC>.
    /// mcc<MCC>" with  <MCC> encoded as defined in clause 5.4.2 ("Mcc" data
    /// type definition)   <MNC> encoding the Mobile Network Code part of the
    /// PLMN, comprising 3 digits.  If there are only 2 significant digits
    /// in the MNC, one "0" digit shall be inserted  at the left side to
    /// fill the 3 digits coding of MNC.  Pattern: '^[0-9]{3}$'
    /// <NID> encoded as defined in clause 5.4.2 ("Nid" data type definition)
    /// <NFInstanceId> encoded as defined in clause 5.3.2  <ServiceName> encoded
    /// as defined in 3GPP TS 29.510  <Set ID> encoded as a string of characters
    /// consisting of alphabetic  characters (A-Z and a-z), digits (0-9)
    /// and/or the hyphen (-) and that shall end  with either an alphabetic
    /// character or a digit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NF Service Set Identifier (see clause 28.12 of 3GPP TS
    /// 23.003) formatted as the following  string \"set<Set ID>.sn<Service
    /// Name>.nfi<NF Instance ID>.5gc.mnc<MNC>.mcc<MCC>\", or
    /// \"set<SetID>.sn<ServiceName>.nfi<NFInstanceID>.5gc.nid<NID>.mnc<MNC>.
    /// mcc<MCC>\" with  <MCC> encoded as defined in clause 5.4.2 (\"Mcc\" data
    /// type definition)   <MNC> encoding the Mobile Network Code part of the
    /// PLMN, comprising 3 digits. \n  If there are only 2 significant digits in
    /// the MNC, one \"0\" digit shall be inserted \n  at the left side to fill
    /// the 3 digits coding of MNC.  Pattern: '^[0-9]{3}$'\n<NID> encoded as
    /// defined in clause 5.4.2 (\"Nid\" data type definition)  <NFInstanceId>
    /// encoded as defined in clause 5.3.2  <ServiceName> encoded as defined in
    /// 3GPP TS 29.510  <Set ID> encoded as a string of characters consisting of
    /// alphabetic \n  characters (A-Z and a-z), digits (0-9) and/or the hyphen
    /// (-) and that shall end \n  with either an alphabetic character or a
    /// digit.\n",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
    )]
    pub struct NfServiceSetId(pub String);
    impl std::ops::Deref for NfServiceSetId {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<NfServiceSetId> for String {
        fn from(value: NfServiceSetId) -> Self {
            value.0
        }
    }

    impl From<&NfServiceSetId> for NfServiceSetId {
        fn from(value: &NfServiceSetId) -> Self {
            value.clone()
        }
    }

    impl From<String> for NfServiceSetId {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for NfServiceSetId {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for NfServiceSetId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///NF Set Identifier (see clause 28.12 of 3GPP TS 23.003), formatted as the
    /// following string "set<Set ID>.<nftype>set.5gc.mnc<MNC>.mcc<MCC>", or
    /// "set<SetID>.<NFType>set.5gc.nid<NID>.mnc<MNC>.mcc<MCC>" with  <MCC>
    /// encoded as defined in clause 5.4.2 ("Mcc" data type definition)  <MNC>
    /// encoding the Mobile Network Code part of the PLMN, comprising 3 digits.
    ///  If there are only 2 significant digits in the MNC, one "0" digit shall
    /// be inserted  at the left side to fill the 3 digits coding of MNC.
    /// Pattern: '^[0-9]{3}$' <NFType> encoded as a value defined in Table
    /// 6.1.6.3.3-1 of 3GPP TS 29.510 but  with lower case characters <Set
    /// ID> encoded as a string of characters consisting of  alphabetic
    /// characters (A-Z and a-z), digits (0-9) and/or the hyphen (-) and that
    ///  shall end with either an alphabetic character or a digit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NF Set Identifier (see clause 28.12 of 3GPP TS 23.003),
    /// formatted as the following string \"set<Set
    /// ID>.<nftype>set.5gc.mnc<MNC>.mcc<MCC>\", or
    /// \"set<SetID>.<NFType>set.5gc.nid<NID>.mnc<MNC>.mcc<MCC>\" with  <MCC>
    /// encoded as defined in clause 5.4.2 (\"Mcc\" data type definition)  <MNC>
    /// encoding the Mobile Network Code part of the PLMN, comprising 3 digits.
    /// \n  If there are only 2 significant digits in the MNC, one \"0\" digit
    /// shall be inserted \n  at the left side to fill the 3 digits coding of
    /// MNC.  Pattern: '^[0-9]{3}$'\n<NFType> encoded as a value defined in
    /// Table 6.1.6.3.3-1 of 3GPP TS 29.510 but \n  with lower case characters
    /// <Set ID> encoded as a string of characters consisting of \n  alphabetic
    /// characters (A-Z and a-z), digits (0-9) and/or the hyphen (-) and that \n
    /// shall end with either an alphabetic character or a digit. \n",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
    )]
    pub struct NfSetId(pub String);
    impl std::ops::Deref for NfSetId {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<NfSetId> for String {
        fn from(value: NfSetId) -> Self {
            value.0
        }
    }

    impl From<&NfSetId> for NfSetId {
        fn from(value: &NfSetId) -> Self {
            value.clone()
        }
    }

    impl From<String> for NfSetId {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for NfSetId {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for NfSetId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///NF types known to NRF
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NF types known to NRF",
    ///  "anyOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "NRF",
    ///        "UDM",
    ///        "AMF",
    ///        "SMF",
    ///        "AUSF",
    ///        "NEF",
    ///        "PCF",
    ///        "SMSF",
    ///        "NSSF",
    ///        "UDR",
    ///        "LMF",
    ///        "GMLC",
    ///        "5G_EIR",
    ///        "SEPP",
    ///        "UPF",
    ///        "N3IWF",
    ///        "AF",
    ///        "UDSF",
    ///        "BSF",
    ///        "CHF",
    ///        "NWDAF",
    ///        "PCSCF",
    ///        "CBCF",
    ///        "HSS",
    ///        "UCMF",
    ///        "SOR_AF",
    ///        "SPAF",
    ///        "MME",
    ///        "SCSAS",
    ///        "SCEF",
    ///        "SCP",
    ///        "NSSAAF",
    ///        "ICSCF",
    ///        "SCSCF",
    ///        "DRA",
    ///        "IMS_AS",
    ///        "AANF",
    ///        "5G_DDNMF",
    ///        "NSACF",
    ///        "MFAF",
    ///        "EASDF",
    ///        "DCCF",
    ///        "MB_SMF",
    ///        "TSCTSF",
    ///        "ADRF",
    ///        "GBA_BSF",
    ///        "CEF",
    ///        "MB_UPF",
    ///        "NSWOF",
    ///        "PKMF",
    ///        "MNPF",
    ///        "SMS_GMSC",
    ///        "SMS_IWMSC",
    ///        "MBSF",
    ///        "MBSTF",
    ///        "PANF",
    ///        "IP_SM_GW",
    ///        "SMS_ROUTER"
    ///      ]
    ///    },
    ///    {
    ///      "type": "string"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct NfType {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<NfTypeSubtype0>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<String>,
    }

    impl From<&NfType> for NfType {
        fn from(value: &NfType) -> Self {
            value.clone()
        }
    }

    ///NfTypeSubtype0
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NRF",
    ///    "UDM",
    ///    "AMF",
    ///    "SMF",
    ///    "AUSF",
    ///    "NEF",
    ///    "PCF",
    ///    "SMSF",
    ///    "NSSF",
    ///    "UDR",
    ///    "LMF",
    ///    "GMLC",
    ///    "5G_EIR",
    ///    "SEPP",
    ///    "UPF",
    ///    "N3IWF",
    ///    "AF",
    ///    "UDSF",
    ///    "BSF",
    ///    "CHF",
    ///    "NWDAF",
    ///    "PCSCF",
    ///    "CBCF",
    ///    "HSS",
    ///    "UCMF",
    ///    "SOR_AF",
    ///    "SPAF",
    ///    "MME",
    ///    "SCSAS",
    ///    "SCEF",
    ///    "SCP",
    ///    "NSSAAF",
    ///    "ICSCF",
    ///    "SCSCF",
    ///    "DRA",
    ///    "IMS_AS",
    ///    "AANF",
    ///    "5G_DDNMF",
    ///    "NSACF",
    ///    "MFAF",
    ///    "EASDF",
    ///    "DCCF",
    ///    "MB_SMF",
    ///    "TSCTSF",
    ///    "ADRF",
    ///    "GBA_BSF",
    ///    "CEF",
    ///    "MB_UPF",
    ///    "NSWOF",
    ///    "PKMF",
    ///    "MNPF",
    ///    "SMS_GMSC",
    ///    "SMS_IWMSC",
    ///    "MBSF",
    ///    "MBSTF",
    ///    "PANF",
    ///    "IP_SM_GW",
    ///    "SMS_ROUTER"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum NfTypeSubtype0 {
        #[serde(rename = "NRF")]
        Nrf,
        #[serde(rename = "UDM")]
        Udm,
        #[serde(rename = "AMF")]
        Amf,
        #[serde(rename = "SMF")]
        Smf,
        #[serde(rename = "AUSF")]
        Ausf,
        #[serde(rename = "NEF")]
        Nef,
        #[serde(rename = "PCF")]
        Pcf,
        #[serde(rename = "SMSF")]
        Smsf,
        #[serde(rename = "NSSF")]
        Nssf,
        #[serde(rename = "UDR")]
        Udr,
        #[serde(rename = "LMF")]
        Lmf,
        #[serde(rename = "GMLC")]
        Gmlc,
        #[serde(rename = "5G_EIR")]
        _5gEir,
        #[serde(rename = "SEPP")]
        Sepp,
        #[serde(rename = "UPF")]
        Upf,
        #[serde(rename = "N3IWF")]
        N3iwf,
        #[serde(rename = "AF")]
        Af,
        #[serde(rename = "UDSF")]
        Udsf,
        #[serde(rename = "BSF")]
        Bsf,
        #[serde(rename = "CHF")]
        Chf,
        #[serde(rename = "NWDAF")]
        Nwdaf,
        #[serde(rename = "PCSCF")]
        Pcscf,
        #[serde(rename = "CBCF")]
        Cbcf,
        #[serde(rename = "HSS")]
        Hss,
        #[serde(rename = "UCMF")]
        Ucmf,
        #[serde(rename = "SOR_AF")]
        SorAf,
        #[serde(rename = "SPAF")]
        Spaf,
        #[serde(rename = "MME")]
        Mme,
        #[serde(rename = "SCSAS")]
        Scsas,
        #[serde(rename = "SCEF")]
        Scef,
        #[serde(rename = "SCP")]
        Scp,
        #[serde(rename = "NSSAAF")]
        Nssaaf,
        #[serde(rename = "ICSCF")]
        Icscf,
        #[serde(rename = "SCSCF")]
        Scscf,
        #[serde(rename = "DRA")]
        Dra,
        #[serde(rename = "IMS_AS")]
        ImsAs,
        #[serde(rename = "AANF")]
        Aanf,
        #[serde(rename = "5G_DDNMF")]
        _5gDdnmf,
        #[serde(rename = "NSACF")]
        Nsacf,
        #[serde(rename = "MFAF")]
        Mfaf,
        #[serde(rename = "EASDF")]
        Easdf,
        #[serde(rename = "DCCF")]
        Dccf,
        #[serde(rename = "MB_SMF")]
        MbSmf,
        #[serde(rename = "TSCTSF")]
        Tsctsf,
        #[serde(rename = "ADRF")]
        Adrf,
        #[serde(rename = "GBA_BSF")]
        GbaBsf,
        #[serde(rename = "CEF")]
        Cef,
        #[serde(rename = "MB_UPF")]
        MbUpf,
        #[serde(rename = "NSWOF")]
        Nswof,
        #[serde(rename = "PKMF")]
        Pkmf,
        #[serde(rename = "MNPF")]
        Mnpf,
        #[serde(rename = "SMS_GMSC")]
        SmsGmsc,
        #[serde(rename = "SMS_IWMSC")]
        SmsIwmsc,
        #[serde(rename = "MBSF")]
        Mbsf,
        #[serde(rename = "MBSTF")]
        Mbstf,
        #[serde(rename = "PANF")]
        Panf,
        #[serde(rename = "IP_SM_GW")]
        IpSmGw,
        #[serde(rename = "SMS_ROUTER")]
        SmsRouter,
    }

    impl From<&NfTypeSubtype0> for NfTypeSubtype0 {
        fn from(value: &NfTypeSubtype0) -> Self {
            value.clone()
        }
    }

    impl ToString for NfTypeSubtype0 {
        fn to_string(&self) -> String {
            match *self {
                Self::Nrf => "NRF".to_string(),
                Self::Udm => "UDM".to_string(),
                Self::Amf => "AMF".to_string(),
                Self::Smf => "SMF".to_string(),
                Self::Ausf => "AUSF".to_string(),
                Self::Nef => "NEF".to_string(),
                Self::Pcf => "PCF".to_string(),
                Self::Smsf => "SMSF".to_string(),
                Self::Nssf => "NSSF".to_string(),
                Self::Udr => "UDR".to_string(),
                Self::Lmf => "LMF".to_string(),
                Self::Gmlc => "GMLC".to_string(),
                Self::_5gEir => "5G_EIR".to_string(),
                Self::Sepp => "SEPP".to_string(),
                Self::Upf => "UPF".to_string(),
                Self::N3iwf => "N3IWF".to_string(),
                Self::Af => "AF".to_string(),
                Self::Udsf => "UDSF".to_string(),
                Self::Bsf => "BSF".to_string(),
                Self::Chf => "CHF".to_string(),
                Self::Nwdaf => "NWDAF".to_string(),
                Self::Pcscf => "PCSCF".to_string(),
                Self::Cbcf => "CBCF".to_string(),
                Self::Hss => "HSS".to_string(),
                Self::Ucmf => "UCMF".to_string(),
                Self::SorAf => "SOR_AF".to_string(),
                Self::Spaf => "SPAF".to_string(),
                Self::Mme => "MME".to_string(),
                Self::Scsas => "SCSAS".to_string(),
                Self::Scef => "SCEF".to_string(),
                Self::Scp => "SCP".to_string(),
                Self::Nssaaf => "NSSAAF".to_string(),
                Self::Icscf => "ICSCF".to_string(),
                Self::Scscf => "SCSCF".to_string(),
                Self::Dra => "DRA".to_string(),
                Self::ImsAs => "IMS_AS".to_string(),
                Self::Aanf => "AANF".to_string(),
                Self::_5gDdnmf => "5G_DDNMF".to_string(),
                Self::Nsacf => "NSACF".to_string(),
                Self::Mfaf => "MFAF".to_string(),
                Self::Easdf => "EASDF".to_string(),
                Self::Dccf => "DCCF".to_string(),
                Self::MbSmf => "MB_SMF".to_string(),
                Self::Tsctsf => "TSCTSF".to_string(),
                Self::Adrf => "ADRF".to_string(),
                Self::GbaBsf => "GBA_BSF".to_string(),
                Self::Cef => "CEF".to_string(),
                Self::MbUpf => "MB_UPF".to_string(),
                Self::Nswof => "NSWOF".to_string(),
                Self::Pkmf => "PKMF".to_string(),
                Self::Mnpf => "MNPF".to_string(),
                Self::SmsGmsc => "SMS_GMSC".to_string(),
                Self::SmsIwmsc => "SMS_IWMSC".to_string(),
                Self::Mbsf => "MBSF".to_string(),
                Self::Mbstf => "MBSTF".to_string(),
                Self::Panf => "PANF".to_string(),
                Self::IpSmGw => "IP_SM_GW".to_string(),
                Self::SmsRouter => "SMS_ROUTER".to_string(),
            }
        }
    }

    impl std::str::FromStr for NfTypeSubtype0 {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "NRF" => Ok(Self::Nrf),
                "UDM" => Ok(Self::Udm),
                "AMF" => Ok(Self::Amf),
                "SMF" => Ok(Self::Smf),
                "AUSF" => Ok(Self::Ausf),
                "NEF" => Ok(Self::Nef),
                "PCF" => Ok(Self::Pcf),
                "SMSF" => Ok(Self::Smsf),
                "NSSF" => Ok(Self::Nssf),
                "UDR" => Ok(Self::Udr),
                "LMF" => Ok(Self::Lmf),
                "GMLC" => Ok(Self::Gmlc),
                "5G_EIR" => Ok(Self::_5gEir),
                "SEPP" => Ok(Self::Sepp),
                "UPF" => Ok(Self::Upf),
                "N3IWF" => Ok(Self::N3iwf),
                "AF" => Ok(Self::Af),
                "UDSF" => Ok(Self::Udsf),
                "BSF" => Ok(Self::Bsf),
                "CHF" => Ok(Self::Chf),
                "NWDAF" => Ok(Self::Nwdaf),
                "PCSCF" => Ok(Self::Pcscf),
                "CBCF" => Ok(Self::Cbcf),
                "HSS" => Ok(Self::Hss),
                "UCMF" => Ok(Self::Ucmf),
                "SOR_AF" => Ok(Self::SorAf),
                "SPAF" => Ok(Self::Spaf),
                "MME" => Ok(Self::Mme),
                "SCSAS" => Ok(Self::Scsas),
                "SCEF" => Ok(Self::Scef),
                "SCP" => Ok(Self::Scp),
                "NSSAAF" => Ok(Self::Nssaaf),
                "ICSCF" => Ok(Self::Icscf),
                "SCSCF" => Ok(Self::Scscf),
                "DRA" => Ok(Self::Dra),
                "IMS_AS" => Ok(Self::ImsAs),
                "AANF" => Ok(Self::Aanf),
                "5G_DDNMF" => Ok(Self::_5gDdnmf),
                "NSACF" => Ok(Self::Nsacf),
                "MFAF" => Ok(Self::Mfaf),
                "EASDF" => Ok(Self::Easdf),
                "DCCF" => Ok(Self::Dccf),
                "MB_SMF" => Ok(Self::MbSmf),
                "TSCTSF" => Ok(Self::Tsctsf),
                "ADRF" => Ok(Self::Adrf),
                "GBA_BSF" => Ok(Self::GbaBsf),
                "CEF" => Ok(Self::Cef),
                "MB_UPF" => Ok(Self::MbUpf),
                "NSWOF" => Ok(Self::Nswof),
                "PKMF" => Ok(Self::Pkmf),
                "MNPF" => Ok(Self::Mnpf),
                "SMS_GMSC" => Ok(Self::SmsGmsc),
                "SMS_IWMSC" => Ok(Self::SmsIwmsc),
                "MBSF" => Ok(Self::Mbsf),
                "MBSTF" => Ok(Self::Mbstf),
                "PANF" => Ok(Self::Panf),
                "IP_SM_GW" => Ok(Self::IpSmGw),
                "SMS_ROUTER" => Ok(Self::SmsRouter),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for NfTypeSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NfTypeSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NfTypeSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///This represents the Network Identifier, which together with a PLMN ID is
    /// used to identify an SNPN (see 3GPP TS 23.003 and 3GPP TS 23.501 clause
    /// 5.30.2.1).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "This represents the Network Identifier, which together
    /// with a PLMN ID is used to identify an SNPN (see 3GPP TS 23.003 and 3GPP
    /// TS 23.501 clause 5.30.2.1). \n",
    ///  "type": "string",
    ///  "pattern": "^[A-Fa-f0-9]{11}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct Nid(String);
    impl std::ops::Deref for Nid {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Nid> for String {
        fn from(value: Nid) -> Self {
            value.0
        }
    }

    impl From<&Nid> for Nid {
        fn from(value: &Nid) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for Nid {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[A-Fa-f0-9]{11}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[A-Fa-f0-9]{11}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for Nid {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Nid {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Nid {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for Nid {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///String providing a Network Slice Simultaneous Registration Group. See
    /// clause 5.15.12 of  3GPP TS 23.501
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "String providing a Network Slice Simultaneous
    /// Registration Group. See clause 5.15.12 of  3GPP TS 23.501\n",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
    )]
    pub struct NsSrg(pub String);
    impl std::ops::Deref for NsSrg {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<NsSrg> for String {
        fn from(value: NsSrg) -> Self {
            value.0
        }
    }

    impl From<&NsSrg> for NsSrg {
        fn from(value: &NsSrg) -> Self {
            value.clone()
        }
    }

    impl From<String> for NsSrg {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for NsSrg {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for NsSrg {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///The Network Slice AS Group ID, see 3GPP TS 38.413
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The Network Slice AS Group ID, see 3GPP TS 38.413\n",
    ///  "type": "integer"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct NsagId(pub i64);
    impl std::ops::Deref for NsagId {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<NsagId> for i64 {
        fn from(value: NsagId) -> Self {
            value.0
        }
    }

    impl From<&NsagId> for NsagId {
        fn from(value: &NsagId) -> Self {
            value.clone()
        }
    }

    impl From<i64> for NsagId {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for NsagId {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for NsagId {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NsagId {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NsagId {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for NsagId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///Contains the association of NSAGs and S-NSSAI(s) along with the TA(s)
    /// within which the association is valid.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///Contains the Identifier of the selected Network Slice instance
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contains the Identifier of the selected Network Slice
    /// instance",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
    )]
    pub struct NsiId(pub String);
    impl std::ops::Deref for NsiId {
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

    ///Contains the API URIs of NRF services to be used to discover
    /// NFs/services, subscribe to NF status changes and/or request access
    /// tokens within the selected Network Slice instance and optional the
    /// Identifier of the selected Network Slice instance
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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
        ///Map indicating whether the NRF requires Oauth2-based authorization
        /// for accessing its services. The key of the map shall be the name of
        /// an NRF service, e.g. "nnrf-nfm" or "nnrf-disc"
        #[serde(
            rename = "nrfOauth2Required",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub nrf_oauth2_required: std::collections::HashMap<String, bool>,
        #[serde(rename = "nsiId", default, skip_serializing_if = "Option::is_none")]
        pub nsi_id: Option<NsiId>,
    }

    impl From<&NsiInformation> for NsiInformation {
        fn from(value: &NsiInformation) -> Self {
            value.clone()
        }
    }

    ///This contains the Nssai availability information requested by the AMF
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///NssaiAvailabilityInfoAmfSetId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct NssaiAvailabilityInfoAmfSetId(String);
    impl std::ops::Deref for NssaiAvailabilityInfoAmfSetId {
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

    impl std::str::FromStr for NssaiAvailabilityInfoAmfSetId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err ("doesn't match pattern \"^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for NssaiAvailabilityInfoAmfSetId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NssaiAvailabilityInfoAmfSetId {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NssaiAvailabilityInfoAmfSetId {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for NssaiAvailabilityInfoAmfSetId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///This contains the notification for created event subscription
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///This contains the information for event subscription
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///NssfEventSubscriptionCreateDataAmfSetId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct NssfEventSubscriptionCreateDataAmfSetId(String);
    impl std::ops::Deref for NssfEventSubscriptionCreateDataAmfSetId {
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

    impl std::str::FromStr for NssfEventSubscriptionCreateDataAmfSetId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err ("doesn't match pattern \"^[0-9]{3}-[0-9]{2,3}-[A-Fa-f0-9]{2}-[0-3][A-Fa-f0-9]{2}$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for NssfEventSubscriptionCreateDataAmfSetId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NssfEventSubscriptionCreateDataAmfSetId {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NssfEventSubscriptionCreateDataAmfSetId {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for NssfEventSubscriptionCreateDataAmfSetId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///This contains the information for created event subscription
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///This contains the event for the subscription
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "This contains the event for the subscription",
    ///  "anyOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "SNSSAI_STATUS_CHANGE_REPORT"
    ///      ]
    ///    },
    ///    {
    ///      "type": "string"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct NssfEventType {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<NssfEventTypeSubtype0>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<String>,
    }

    impl From<&NssfEventType> for NssfEventType {
        fn from(value: &NssfEventType) -> Self {
            value.clone()
        }
    }

    ///NssfEventTypeSubtype0
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "SNSSAI_STATUS_CHANGE_REPORT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum NssfEventTypeSubtype0 {
        #[serde(rename = "SNSSAI_STATUS_CHANGE_REPORT")]
        SnssaiStatusChangeReport,
    }

    impl From<&NssfEventTypeSubtype0> for NssfEventTypeSubtype0 {
        fn from(value: &NssfEventTypeSubtype0) -> Self {
            value.clone()
        }
    }

    impl ToString for NssfEventTypeSubtype0 {
        fn to_string(&self) -> String {
            match *self {
                Self::SnssaiStatusChangeReport => "SNSSAI_STATUS_CHANGE_REPORT".to_string(),
            }
        }
    }

    impl std::str::FromStr for NssfEventTypeSubtype0 {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "SNSSAI_STATUS_CHANGE_REPORT" => Ok(Self::SnssaiStatusChangeReport),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for NssfEventTypeSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NssfEventTypeSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NssfEventTypeSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///This contains the JSON Patch instructions for updating the Nssai
    /// availability data information at the NSSF
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "This contains the JSON Patch instructions for updating
    /// the Nssai availability data information at the NSSF",
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/PatchItem"
    ///  },
    ///  "minItems": 1
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct PatchDocument(pub Vec<PatchItem>);
    impl std::ops::Deref for PatchDocument {
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

    ///it contains information on data to be changed.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "it contains information on data to be changed.",
    ///  "type": "object",
    ///  "required": [
    ///    "op",
    ///    "path"
    ///  ],
    ///  "properties": {
    ///    "from": {
    ///      "description": "indicates the path of the source JSON element
    /// (according to JSON Pointer syntax) being moved or copied to the location
    /// indicated by the \"path\" attribute.\n",
    ///      "type": "string"
    ///    },
    ///    "op": {
    ///      "$ref": "#/components/schemas/PatchOperation"
    ///    },
    ///    "path": {
    ///      "description": "contains a JSON pointer value (as defined in IETF
    /// RFC 6901) that references a location of a resource on which the patch
    /// operation shall be performed.\n",
    ///      "type": "string"
    ///    },
    ///    "value": {}
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct PatchItem {
        ///indicates the path of the source JSON element (according to JSON
        /// Pointer syntax) being moved or copied to the location indicated by
        /// the "path" attribute.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,
        pub op: PatchOperation,
        ///contains a JSON pointer value (as defined in IETF RFC 6901) that
        /// references a location of a resource on which the patch operation
        /// shall be performed.
        pub path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<serde_json::Value>,
    }

    impl From<&PatchItem> for PatchItem {
        fn from(value: &PatchItem) -> Self {
            value.clone()
        }
    }

    ///Operations as defined in IETF RFC 6902.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Operations as defined in IETF RFC 6902.",
    ///  "anyOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "add",
    ///        "copy",
    ///        "move",
    ///        "remove",
    ///        "replace",
    ///        "test"
    ///      ]
    ///    },
    ///    {
    ///      "type": "string"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct PatchOperation {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<PatchOperationSubtype0>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<String>,
    }

    impl From<&PatchOperation> for PatchOperation {
        fn from(value: &PatchOperation) -> Self {
            value.clone()
        }
    }

    ///PatchOperationSubtype0
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "add",
    ///    "copy",
    ///    "move",
    ///    "remove",
    ///    "replace",
    ///    "test"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum PatchOperationSubtype0 {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "copy")]
        Copy,
        #[serde(rename = "move")]
        Move,
        #[serde(rename = "remove")]
        Remove,
        #[serde(rename = "replace")]
        Replace,
        #[serde(rename = "test")]
        Test,
    }

    impl From<&PatchOperationSubtype0> for PatchOperationSubtype0 {
        fn from(value: &PatchOperationSubtype0) -> Self {
            value.clone()
        }
    }

    impl ToString for PatchOperationSubtype0 {
        fn to_string(&self) -> String {
            match *self {
                Self::Add => "add".to_string(),
                Self::Copy => "copy".to_string(),
                Self::Move => "move".to_string(),
                Self::Remove => "remove".to_string(),
                Self::Replace => "replace".to_string(),
                Self::Test => "test".to_string(),
            }
        }
    }

    impl std::str::FromStr for PatchOperationSubtype0 {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "add" => Ok(Self::Add),
                "copy" => Ok(Self::Copy),
                "move" => Ok(Self::Move),
                "remove" => Ok(Self::Remove),
                "replace" => Ok(Self::Replace),
                "test" => Ok(Self::Test),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for PatchOperationSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for PatchOperationSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for PatchOperationSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///When PlmnId needs to be converted to string (e.g. when used in maps as
    /// key), the string  shall be composed of three digits "mcc" followed by
    /// "-" and two or three digits "mnc".
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "When PlmnId needs to be converted to string (e.g. when
    /// used in maps as key), the string  shall be composed of three digits
    /// \"mcc\" followed by \"-\" and two or three digits \"mnc\".\n",
    ///  "type": "object",
    ///  "required": [
    ///    "mcc",
    ///    "mnc"
    ///  ],
    ///  "properties": {
    ///    "mcc": {
    ///      "$ref": "#/components/schemas/Mcc"
    ///    },
    ///    "mnc": {
    ///      "$ref": "#/components/schemas/Mnc"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct PlmnId {
        pub mcc: Mcc,
        pub mnc: Mnc,
    }

    impl From<&PlmnId> for PlmnId {
        fn from(value: &PlmnId) -> Self {
            value.clone()
        }
    }

    ///Contains the serving core network operator PLMN ID and, for an SNPN, the
    /// NID that together with the PLMN ID identifies the SNPN.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contains the serving core network operator PLMN ID and,
    /// for an SNPN, the NID that together with the PLMN ID identifies the
    /// SNPN.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "mcc",
    ///    "mnc"
    ///  ],
    ///  "properties": {
    ///    "mcc": {
    ///      "$ref": "#/components/schemas/Mcc"
    ///    },
    ///    "mnc": {
    ///      "$ref": "#/components/schemas/Mnc"
    ///    },
    ///    "nid": {
    ///      "$ref": "#/components/schemas/Nid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct PlmnIdNid {
        pub mcc: Mcc,
        pub mnc: Mnc,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nid: Option<Nid>,
    }

    impl From<&PlmnIdNid> for PlmnIdNid {
        fn from(value: &PlmnIdNid) -> Self {
            value.clone()
        }
    }

    ///Provides additional information in an error response.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Provides additional information in an error response.",
    ///  "type": "object",
    ///  "properties": {
    ///    "accessTokenError": {
    ///      "$ref": "#/components/schemas/AccessTokenErr"
    ///    },
    ///    "accessTokenRequest": {
    ///      "$ref": "#/components/schemas/AccessTokenReq"
    ///    },
    ///    "cause": {
    ///      "description": "A machine-readable application error cause specific
    /// to this occurrence of the problem.  This IE should be present and
    /// provide application-related error information, if available.\n",
    ///      "type": "string"
    ///    },
    ///    "detail": {
    ///      "description": "A human-readable explanation specific to this
    /// occurrence of the problem.",
    ///      "type": "string"
    ///    },
    ///    "instance": {
    ///      "$ref": "#/components/schemas/Uri"
    ///    },
    ///    "invalidParams": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InvalidParam"
    ///      },
    ///      "minItems": 1
    ///    },
    ///    "nrfId": {
    ///      "$ref": "#/components/schemas/Fqdn"
    ///    },
    ///    "status": {
    ///      "type": "integer"
    ///    },
    ///    "supportedFeatures": {
    ///      "$ref": "#/components/schemas/SupportedFeatures"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "$ref": "#/components/schemas/Uri"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ProblemDetails {
        #[serde(
            rename = "accessTokenError",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_token_error: Option<AccessTokenErr>,
        #[serde(
            rename = "accessTokenRequest",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_token_request: Option<AccessTokenReq>,
        ///A machine-readable application error cause specific to this
        /// occurrence of the problem.  This IE should be present and provide
        /// application-related error information, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cause: Option<String>,
        ///A human-readable explanation specific to this occurrence of the
        /// problem.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub detail: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub instance: Option<Uri>,
        #[serde(
            rename = "invalidParams",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub invalid_params: Vec<InvalidParam>,
        #[serde(rename = "nrfId", default, skip_serializing_if = "Option::is_none")]
        pub nrf_id: Option<Fqdn>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        #[serde(
            rename = "supportedFeatures",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supported_features: Option<SupportedFeatures>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<Uri>,
    }

    impl From<&ProblemDetails> for ProblemDetails {
        fn from(value: &ProblemDetails) -> Self {
            value.clone()
        }
    }

    ///The response shall include a Location header field containing a
    /// different URI  (pointing to a different URI of an other service
    /// instance), or the same URI if a request  is redirected to the same
    /// target resource via a different SCP.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The response shall include a Location header field
    /// containing a different URI  (pointing to a different URI of an other
    /// service instance), or the same URI if a request  is redirected to the
    /// same target resource via a different SCP.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "cause": {
    ///      "type": "string"
    ///    },
    ///    "targetScp": {
    ///      "$ref": "#/components/schemas/Uri"
    ///    },
    ///    "targetSepp": {
    ///      "$ref": "#/components/schemas/Uri"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct RedirectResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cause: Option<String>,
        #[serde(rename = "targetScp", default, skip_serializing_if = "Option::is_none")]
        pub target_scp: Option<Uri>,
        #[serde(
            rename = "targetSepp",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub target_sepp: Option<Uri>,
    }

    impl From<&RedirectResponse> for RedirectResponse {
        fn from(value: &RedirectResponse) -> Self {
            value.clone()
        }
    }

    ///This contains the restricted SNssai information per PLMN
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///Contains the indication on roaming
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contains the indication on roaming",
    ///  "anyOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "NON_ROAMING",
    ///        "LOCAL_BREAKOUT",
    ///        "HOME_ROUTED_ROAMING"
    ///      ]
    ///    },
    ///    {
    ///      "type": "string"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct RoamingIndication {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<RoamingIndicationSubtype0>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<String>,
    }

    impl From<&RoamingIndication> for RoamingIndication {
        fn from(value: &RoamingIndication) -> Self {
            value.clone()
        }
    }

    ///RoamingIndicationSubtype0
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NON_ROAMING",
    ///    "LOCAL_BREAKOUT",
    ///    "HOME_ROUTED_ROAMING"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum RoamingIndicationSubtype0 {
        #[serde(rename = "NON_ROAMING")]
        NonRoaming,
        #[serde(rename = "LOCAL_BREAKOUT")]
        LocalBreakout,
        #[serde(rename = "HOME_ROUTED_ROAMING")]
        HomeRoutedRoaming,
    }

    impl From<&RoamingIndicationSubtype0> for RoamingIndicationSubtype0 {
        fn from(value: &RoamingIndicationSubtype0) -> Self {
            value.clone()
        }
    }

    impl ToString for RoamingIndicationSubtype0 {
        fn to_string(&self) -> String {
            match *self {
                Self::NonRoaming => "NON_ROAMING".to_string(),
                Self::LocalBreakout => "LOCAL_BREAKOUT".to_string(),
                Self::HomeRoutedRoaming => "HOME_ROUTED_ROAMING".to_string(),
            }
        }
    }

    impl std::str::FromStr for RoamingIndicationSubtype0 {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "NON_ROAMING" => Ok(Self::NonRoaming),
                "LOCAL_BREAKOUT" => Ok(Self::LocalBreakout),
                "HOME_ROUTED_ROAMING" => Ok(Self::HomeRoutedRoaming),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for RoamingIndicationSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for RoamingIndicationSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for RoamingIndicationSubtype0 {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Contains the association of NSAGs and S-NSSAI(s) along with the TA(s)
    /// within which the association is valid.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///A range of SDs (Slice Differentiators)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A range of SDs (Slice Differentiators)",
    ///  "type": "object",
    ///  "properties": {
    ///    "end": {
    ///      "description": "Last value identifying the end of an SD range. This
    /// string shall be formatted as specified for the sd attribute of the
    /// Snssai data type in clause 5.4.4.2.\n",
    ///      "type": "string",
    ///      "pattern": "^[A-Fa-f0-9]{6}$"
    ///    },
    ///    "start": {
    ///      "description": "First value identifying the start of an SD range.
    /// This string shall be formatted as specified for the sd attribute of the
    /// Snssai data type in clause 5.4.4.2.\n",
    ///      "type": "string",
    ///      "pattern": "^[A-Fa-f0-9]{6}$"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SdRange {
        ///Last value identifying the end of an SD range. This string shall be
        /// formatted as specified for the sd attribute of the Snssai data type
        /// in clause 5.4.4.2.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end: Option<SdRangeEnd>,
        ///First value identifying the start of an SD range. This string shall
        /// be formatted as specified for the sd attribute of the Snssai data
        /// type in clause 5.4.4.2.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub start: Option<SdRangeStart>,
    }

    impl From<&SdRange> for SdRange {
        fn from(value: &SdRange) -> Self {
            value.clone()
        }
    }

    ///Last value identifying the end of an SD range. This string shall be
    /// formatted as specified for the sd attribute of the Snssai data type in
    /// clause 5.4.4.2.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Last value identifying the end of an SD range. This
    /// string shall be formatted as specified for the sd attribute of the
    /// Snssai data type in clause 5.4.4.2.\n",
    ///  "type": "string",
    ///  "pattern": "^[A-Fa-f0-9]{6}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct SdRangeEnd(String);
    impl std::ops::Deref for SdRangeEnd {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<SdRangeEnd> for String {
        fn from(value: SdRangeEnd) -> Self {
            value.0
        }
    }

    impl From<&SdRangeEnd> for SdRangeEnd {
        fn from(value: &SdRangeEnd) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for SdRangeEnd {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[A-Fa-f0-9]{6}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[A-Fa-f0-9]{6}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for SdRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SdRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SdRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for SdRangeEnd {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///First value identifying the start of an SD range. This string shall be
    /// formatted as specified for the sd attribute of the Snssai data type in
    /// clause 5.4.4.2.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "First value identifying the start of an SD range. This
    /// string shall be formatted as specified for the sd attribute of the
    /// Snssai data type in clause 5.4.4.2.\n",
    ///  "type": "string",
    ///  "pattern": "^[A-Fa-f0-9]{6}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct SdRangeStart(String);
    impl std::ops::Deref for SdRangeStart {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<SdRangeStart> for String {
        fn from(value: SdRangeStart) -> Self {
            value.0
        }
    }

    impl From<&SdRangeStart> for SdRangeStart {
        fn from(value: &SdRangeStart) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for SdRangeStart {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[A-Fa-f0-9]{6}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[A-Fa-f0-9]{6}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for SdRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SdRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SdRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for SdRangeStart {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Contains the slice information requested during PDU Session
    /// establishment procedure
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///Contains the slice information requested during a Registration procedure
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///Contains the slice information requested during UE configuration update
    /// procedure
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///When Snssai needs to be converted to string (e.g. when used in maps as
    /// key), the string shall be composed of one to three digits "sst"
    /// optionally followed by "-" and 6 hexadecimal digits "sd".
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "When Snssai needs to be converted to string (e.g. when
    /// used in maps as key), the string shall be composed of one to three
    /// digits \"sst\" optionally followed by \"-\" and 6 hexadecimal digits
    /// \"sd\".\n",
    ///  "type": "object",
    ///  "required": [
    ///    "sst"
    ///  ],
    ///  "properties": {
    ///    "sd": {
    ///      "description": "3-octet string, representing the Slice
    /// Differentiator, in hexadecimal representation. Each character in the
    /// string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to
    /// \"F\" and shall represent 4 bits. The most significant character
    /// representing the 4 most significant bits of the SD shall appear first in
    /// the string, and the character representing the 4 least significant bit
    /// of the SD shall appear last in the string. This is an optional parameter
    /// that complements the Slice/Service type(s) to allow to  differentiate
    /// amongst multiple Network Slices of the same Slice/Service type. This IE
    /// shall be absent if no SD value is associated with the SST.\n",
    ///      "type": "string",
    ///      "pattern": "^[A-Fa-f0-9]{6}$"
    ///    },
    ///    "sst": {
    ///      "description": "Unsigned integer, within the range 0 to 255,
    /// representing the Slice/Service Type.  It indicates the expected Network
    /// Slice behaviour in terms of features and services. Values 0 to 127
    /// correspond to the standardized SST range. Values 128 to 255 correspond
    /// to the Operator-specific range. See clause 28.4.2 of 3GPP TS 23.003.
    /// Standardized values are defined in clause 5.15.2.2 of 3GPP TS 23.501.
    /// \n",
    ///      "type": "integer",
    ///      "maximum": 255.0,
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Snssai {
        ///3-octet string, representing the Slice Differentiator, in
        /// hexadecimal representation. Each character in the string shall take
        /// a value of "0" to "9", "a" to "f" or "A" to "F" and shall represent
        /// 4 bits. The most significant character representing the 4 most
        /// significant bits of the SD shall appear first in the string, and the
        /// character representing the 4 least significant bit of the SD shall
        /// appear last in the string. This is an optional parameter that
        /// complements the Slice/Service type(s) to allow to  differentiate
        /// amongst multiple Network Slices of the same Slice/Service type. This
        /// IE shall be absent if no SD value is associated with the SST.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sd: Option<SnssaiSd>,
        ///Unsigned integer, within the range 0 to 255, representing the
        /// Slice/Service Type.  It indicates the expected Network Slice
        /// behaviour in terms of features and services. Values 0 to 127
        /// correspond to the standardized SST range. Values 128 to 255
        /// correspond  to the Operator-specific range. See clause 28.4.2 of
        /// 3GPP TS 23.003. Standardized values are defined in clause 5.15.2.2
        /// of 3GPP TS 23.501.
        pub sst: u8,
    }

    impl From<&Snssai> for Snssai {
        fn from(value: &Snssai) -> Self {
            value.clone()
        }
    }

    ///Extensions to the Snssai data type, sdRanges and wildcardSd shall not be
    /// present simultaneously
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Extensions to the Snssai data type, sdRanges and
    /// wildcardSd shall not be present simultaneously\n",
    ///  "type": "object",
    ///  "not": {
    ///    "required": [
    ///      "sdRanges",
    ///      "wildcardSd"
    ///    ]
    ///  },
    ///  "properties": {
    ///    "sdRanges": {
    ///      "description": "When present, it shall contain the range(s) of
    /// Slice Differentiator values supported for the Slice/Service Type value
    /// indicated in the sst attribute of the Snssai data type\n",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SdRange"
    ///      },
    ///      "minItems": 1
    ///    },
    ///    "wildcardSd": {
    ///      "description": "When present, it shall be set to true, to indicate
    /// that all SD values are supported for the Slice/Service Type value
    /// indicated in the sst attribute of the Snssai data type.\n",
    ///      "type": "boolean",
    ///      "enum": [
    ///        true
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SnssaiExtension {}
    impl From<&SnssaiExtension> for SnssaiExtension {
        fn from(value: &SnssaiExtension) -> Self {
            value.clone()
        }
    }

    ///3-octet string, representing the Slice Differentiator, in hexadecimal
    /// representation. Each character in the string shall take a value of "0"
    /// to "9", "a" to "f" or "A" to "F" and shall represent 4 bits. The most
    /// significant character representing the 4 most significant bits of the SD
    /// shall appear first in the string, and the character representing the 4
    /// least significant bit of the SD shall appear last in the string. This is
    /// an optional parameter that complements the Slice/Service type(s) to
    /// allow to  differentiate amongst multiple Network Slices of the same
    /// Slice/Service type. This IE shall be absent if no SD value is associated
    /// with the SST.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "3-octet string, representing the Slice Differentiator,
    /// in hexadecimal representation. Each character in the string shall take a
    /// value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall
    /// represent 4 bits. The most significant character representing the 4 most
    /// significant bits of the SD shall appear first in the string, and the
    /// character representing the 4 least significant bit of the SD shall
    /// appear last in the string. This is an optional parameter that
    /// complements the Slice/Service type(s) to allow to  differentiate amongst
    /// multiple Network Slices of the same Slice/Service type. This IE shall be
    /// absent if no SD value is associated with the SST.\n",
    ///  "type": "string",
    ///  "pattern": "^[A-Fa-f0-9]{6}$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct SnssaiSd(String);
    impl std::ops::Deref for SnssaiSd {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<SnssaiSd> for String {
        fn from(value: SnssaiSd) -> Self {
            value.0
        }
    }

    impl From<&SnssaiSd> for SnssaiSd {
        fn from(value: &SnssaiSd) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for SnssaiSd {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[A-Fa-f0-9]{6}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[A-Fa-f0-9]{6}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for SnssaiSd {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SnssaiSd {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SnssaiSd {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for SnssaiSd {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Contains the subscribed S-NSSAI
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///A string used to indicate the features supported by an API that is used
    /// as defined in clause  6.6 in 3GPP TS 29.500. The string shall contain a
    /// bitmask indicating supported features in  hexadecimal representation
    /// Each character in the string shall take a value of "0" to "9",  "a" to
    /// "f" or "A" to "F" and shall represent the support of 4 features as
    /// described in  table 5.2.2-3. The most significant character representing
    /// the highest-numbered features shall  appear first in the string, and the
    /// character representing features 1 to 4 shall appear last  in the string.
    /// The list of features and their numbering (starting with 1) are defined
    /// separately for each API. If the string contains a lower number of
    /// characters than there are  defined features for an API, all features
    /// that would be represented by characters that are not  present in the
    /// string are not supported.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A string used to indicate the features supported by an API that is used as defined in clause  6.6 in 3GPP TS 29.500. The string shall contain a bitmask indicating supported features in  hexadecimal representation Each character in the string shall take a value of \"0\" to \"9\",  \"a\" to \"f\" or \"A\" to \"F\" and shall represent the support of 4 features as described in  table 5.2.2-3. The most significant character representing the highest-numbered features shall  appear first in the string, and the character representing features 1 to 4 shall appear last  in the string. The list of features and their numbering (starting with 1) are defined  separately for each API. If the string contains a lower number of characters than there are  defined features for an API, all features that would be represented by characters that are not  present in the string are not supported.\n",
    ///  "type": "string",
    ///  "pattern": "^[A-Fa-f0-9]*$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct SupportedFeatures(String);
    impl std::ops::Deref for SupportedFeatures {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<SupportedFeatures> for String {
        fn from(value: SupportedFeatures) -> Self {
            value.0
        }
    }

    impl From<&SupportedFeatures> for SupportedFeatures {
        fn from(value: &SupportedFeatures) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for SupportedFeatures {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[A-Fa-f0-9]*$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[A-Fa-f0-9]*$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for SupportedFeatures {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SupportedFeatures {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SupportedFeatures {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for SupportedFeatures {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///This contains the Nssai availability data information per TA supported
    /// by the AMF
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///2 or 3-octet string identifying a tracking area code as specified in
    /// clause 9.3.3.10  of 3GPP TS 38.413, in hexadecimal representation. Each
    /// character in the string shall  take a value of "0" to "9", "a" to "f" or
    /// "A" to "F" and shall represent 4 bits. The most significant character
    /// representing the 4 most significant bits of the TAC shall  appear first
    /// in the string, and the character representing the 4 least significant
    /// bit  of the TAC shall appear last in the string.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "2 or 3-octet string identifying a tracking area code as
    /// specified in clause 9.3.3.10  of 3GPP TS 38.413, in hexadecimal
    /// representation. Each character in the string shall  take a value of
    /// \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4
    /// bits. The most significant character representing the 4 most significant
    /// bits of the TAC shall  appear first in the string, and the character
    /// representing the 4 least significant bit  of the TAC shall appear last
    /// in the string. \n",
    ///  "type": "string",
    ///  "pattern": "(^[A-Fa-f0-9]{4}$)|(^[A-Fa-f0-9]{6}$)"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct Tac(String);
    impl std::ops::Deref for Tac {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Tac> for String {
        fn from(value: Tac) -> Self {
            value.0
        }
    }

    impl From<&Tac> for Tac {
        fn from(value: &Tac) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for Tac {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("(^[A-Fa-f0-9]{4}$)|(^[A-Fa-f0-9]{6}$)")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err(
                    "doesn't match pattern \"(^[A-Fa-f0-9]{4}$)|(^[A-Fa-f0-9]{6}$)\"".into(),
                );
            }
            Ok(Self(value.to_string()))
        }
    }

    impl std::convert::TryFrom<&str> for Tac {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Tac {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Tac {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for Tac {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Range of TACs (Tracking Area Codes)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///TacRangeEnd
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct TacRangeEnd(String);
    impl std::ops::Deref for TacRangeEnd {
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

    impl std::str::FromStr for TacRangeEnd {
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

    impl std::convert::TryFrom<&str> for TacRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for TacRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for TacRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for TacRangeEnd {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///TacRangeStart
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Serialize)]
    pub struct TacRangeStart(String);
    impl std::ops::Deref for TacRangeStart {
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

    impl std::str::FromStr for TacRangeStart {
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

    impl std::convert::TryFrom<&str> for TacRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for TacRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for TacRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> serde::Deserialize<'de> for TacRangeStart {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Contains the tracking area identity as described in 3GPP 23.003
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contains the tracking area identity as described in
    /// 3GPP 23.003",
    ///  "type": "object",
    ///  "required": [
    ///    "plmnId",
    ///    "tac"
    ///  ],
    ///  "properties": {
    ///    "nid": {
    ///      "$ref": "#/components/schemas/Nid"
    ///    },
    ///    "plmnId": {
    ///      "$ref": "#/components/schemas/PlmnId"
    ///    },
    ///    "tac": {
    ///      "$ref": "#/components/schemas/Tac"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Tai {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nid: Option<Nid>,
        #[serde(rename = "plmnId")]
        pub plmn_id: PlmnId,
        pub tac: Tac,
    }

    impl From<&Tai> for Tai {
        fn from(value: &Tai) -> Self {
            value.clone()
        }
    }

    ///Range of TAIs (Tracking Area Identities)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
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
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
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

    ///String providing an URI formatted according to RFC 3986.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "String providing an URI formatted according to RFC
    /// 3986.",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
    )]
    pub struct Uri(pub String);
    impl std::ops::Deref for Uri {
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

#[derive(Clone, Debug)]
///Client for Openapi-5GC
///
///Merged Apis. © 2024, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI,
/// TSDSI, TTA, TTC). All rights reserved.
///
///Version: 1.2.6
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.2.6"
    }
}

#[allow(clippy::all)]
impl Client {}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
