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
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
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
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
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
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
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
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct AccessTokenReqScope(String);
    impl ::std::ops::Deref for AccessTokenReqScope {
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

    impl ::std::str::FromStr for AccessTokenReqScope {
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

    impl ::std::convert::TryFrom<&str> for AccessTokenReqScope {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for AccessTokenReqScope {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for AccessTokenReqScope {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for AccessTokenReqScope {
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

    ///Contains identities representing those UEs potentially affected by a
    /// data-loss event at the UDR
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contains identities representing those UEs potentially
    /// affected by a data-loss event at the UDR",
    ///  "type": "object",
    ///  "properties": {
    ///    "dnnList": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Dnn"
    ///      },
    ///      "minItems": 1
    ///    },
    ///    "gpsiRanges": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IdentityRange"
    ///      },
    ///      "minItems": 1
    ///    },
    ///    "resetIds": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "minItems": 1
    ///    },
    ///    "sNssaiList": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Snssai"
    ///      },
    ///      "minItems": 1
    ///    },
    ///    "supiRanges": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SupiRange"
    ///      },
    ///      "minItems": 1
    ///    },
    ///    "udrGroupId": {
    ///      "$ref": "#/components/schemas/NfGroupId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DataRestorationNotification {
        #[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
        pub dnn_list: Vec<Dnn>,
        #[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
        pub gpsi_ranges: Vec<IdentityRange>,
        #[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
        pub reset_ids: Vec<String>,
        #[serde(rename = "sNssaiList", default, skip_serializing_if = "Vec::is_empty")]
        pub s_nssai_list: Vec<Snssai>,
        #[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
        pub supi_ranges: Vec<SupiRange>,
        #[serde(
            rename = "udrGroupId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub udr_group_id: Option<NfGroupId>,
    }

    impl From<&DataRestorationNotification> for DataRestorationNotification {
        fn from(value: &DataRestorationNotification) -> Self {
            value.clone()
        }
    }

    ///String representing a Data Network as defined in clause 9A of 3GPP TS
    /// 23.003;  it shall contain either a DNN Network Identifier, or a full DNN
    /// with both the Network  Identifier and Operator Identifier, as specified
    /// in 3GPP TS 23.003 clause 9.1.1 and 9.1.2. It shall be coded as string in
    /// which the labels are separated by dots  (e.g. "Label1.Label2.Label3").
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "String representing a Data Network as defined in clause
    /// 9A of 3GPP TS 23.003;  it shall contain either a DNN Network Identifier,
    /// or a full DNN with both the Network  Identifier and Operator Identifier,
    /// as specified in 3GPP TS 23.003 clause 9.1.1 and 9.1.2. It shall be coded
    /// as string in which the labels are separated by dots  (e.g.
    /// \"Label1.Label2.Label3\").\n",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct Dnn(pub String);
    impl ::std::ops::Deref for Dnn {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Dnn> for String {
        fn from(value: Dnn) -> Self {
            value.0
        }
    }

    impl From<&Dnn> for Dnn {
        fn from(value: &Dnn) -> Self {
            value.clone()
        }
    }

    impl From<String> for Dnn {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Dnn {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for Dnn {
        fn to_string(&self) -> String {
            self.0.to_string()
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
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Fqdn(String);
    impl ::std::ops::Deref for Fqdn {
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

    impl ::std::str::FromStr for Fqdn {
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

    impl ::std::convert::TryFrom<&str> for Fqdn {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Fqdn {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Fqdn {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Fqdn {
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

    ///A range of GPSIs (subscriber identities), either based on a numeric
    /// range, or based on regular-expression matching
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A range of GPSIs (subscriber identities), either based
    /// on a numeric range, or based on regular-expression matching\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "end": {
    ///      "type": "string",
    ///      "pattern": "^[0-9]+$"
    ///    },
    ///    "pattern": {
    ///      "type": "string"
    ///    },
    ///    "start": {
    ///      "type": "string",
    ///      "pattern": "^[0-9]+$"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IdentityRange {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end: Option<IdentityRangeEnd>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pattern: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub start: Option<IdentityRangeStart>,
    }

    impl From<&IdentityRange> for IdentityRange {
        fn from(value: &IdentityRange) -> Self {
            value.clone()
        }
    }

    ///IdentityRangeEnd
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^[0-9]+$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct IdentityRangeEnd(String);
    impl ::std::ops::Deref for IdentityRangeEnd {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<IdentityRangeEnd> for String {
        fn from(value: IdentityRangeEnd) -> Self {
            value.0
        }
    }

    impl From<&IdentityRangeEnd> for IdentityRangeEnd {
        fn from(value: &IdentityRangeEnd) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for IdentityRangeEnd {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[0-9]+$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[0-9]+$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for IdentityRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for IdentityRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for IdentityRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for IdentityRangeEnd {
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

    ///IdentityRangeStart
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^[0-9]+$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct IdentityRangeStart(String);
    impl ::std::ops::Deref for IdentityRangeStart {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<IdentityRangeStart> for String {
        fn from(value: IdentityRangeStart) -> Self {
            value.0
        }
    }

    impl From<&IdentityRangeStart> for IdentityRangeStart {
        fn from(value: &IdentityRangeStart) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for IdentityRangeStart {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[0-9]+$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[0-9]+$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for IdentityRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for IdentityRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for IdentityRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for IdentityRangeStart {
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
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
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Mcc(String);
    impl ::std::ops::Deref for Mcc {
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

    impl ::std::str::FromStr for Mcc {
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

    impl ::std::convert::TryFrom<&str> for Mcc {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Mcc {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Mcc {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Mcc {
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
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Mnc(String);
    impl ::std::ops::Deref for Mnc {
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

    impl ::std::str::FromStr for Mnc {
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

    impl ::std::convert::TryFrom<&str> for Mnc {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Mnc {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Mnc {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Mnc {
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

    ///Identifier of a group of NFs.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Identifier of a group of NFs.",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct NfGroupId(pub String);
    impl ::std::ops::Deref for NfGroupId {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<NfGroupId> for String {
        fn from(value: NfGroupId) -> Self {
            value.0
        }
    }

    impl From<&NfGroupId> for NfGroupId {
        fn from(value: &NfGroupId) -> Self {
            value.clone()
        }
    }

    impl From<String> for NfGroupId {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for NfGroupId {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for NfGroupId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///Contains the NFGroupIds for the requested NF types. The NFType is the
    /// key of the map.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contains the NFGroupIds for the requested NF types. The
    /// NFType is the key of the map.",
    ///  "type": "object",
    ///  "minProperties": 1,
    ///  "additionalProperties": {
    ///    "$ref": "#/components/schemas/NfGroupId"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NfGroupIdMapResult(pub ::std::collections::HashMap<String, NfGroupId>);
    impl ::std::ops::Deref for NfGroupIdMapResult {
        type Target = ::std::collections::HashMap<String, NfGroupId>;
        fn deref(&self) -> &::std::collections::HashMap<String, NfGroupId> {
            &self.0
        }
    }

    impl From<NfGroupIdMapResult> for ::std::collections::HashMap<String, NfGroupId> {
        fn from(value: NfGroupIdMapResult) -> Self {
            value.0
        }
    }

    impl From<&NfGroupIdMapResult> for NfGroupIdMapResult {
        fn from(value: &NfGroupIdMapResult) -> Self {
            value.clone()
        }
    }

    impl From<::std::collections::HashMap<String, NfGroupId>> for NfGroupIdMapResult {
        fn from(value: ::std::collections::HashMap<String, NfGroupId>) -> Self {
            Self(value)
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NfInstanceId(pub uuid::Uuid);
    impl ::std::ops::Deref for NfInstanceId {
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
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct NfServiceSetId(pub String);
    impl ::std::ops::Deref for NfServiceSetId {
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
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct NfSetId(pub String);
    impl ::std::ops::Deref for NfSetId {
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
    ///  ],
    ///  "x-allow-unknown": true
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum NfType {
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
        FiveGEir,
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
        FiveGDdnmf,
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
        #[serde(untagged)]
        UnknownOther(String),
    }

    impl From<&NfType> for NfType {
        fn from(value: &NfType) -> Self {
            value.clone()
        }
    }

    impl ToString for NfType {
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
                Self::FiveGEir => "5G_EIR".to_string(),
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
                Self::FiveGDdnmf => "5G_DDNMF".to_string(),
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
                Self::UnknownOther(ref value) => value.clone(),
            }
        }
    }

    impl std::str::FromStr for NfType {
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
                "5G_EIR" => Ok(Self::FiveGEir),
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
                "5G_DDNMF" => Ok(Self::FiveGDdnmf),
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
                _ => Ok(Self::UnknownOther(value.to_string())),
            }
        }
    }

    impl std::convert::TryFrom<&str> for NfType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NfType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NfType {
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
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Nid(String);
    impl ::std::ops::Deref for Nid {
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

    impl ::std::str::FromStr for Nid {
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

    impl ::std::convert::TryFrom<&str> for Nid {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Nid {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Nid {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Nid {
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
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
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SnssaiSd(String);
    impl ::std::ops::Deref for SnssaiSd {
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

    impl ::std::str::FromStr for SnssaiSd {
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

    impl ::std::convert::TryFrom<&str> for SnssaiSd {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for SnssaiSd {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for SnssaiSd {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SnssaiSd {
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

    ///Represents the Subscription Identifier SUPI or GPSI or IMPI or IMPU.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Represents the Subscription Identifier SUPI or GPSI or
    /// IMPI or IMPU.",
    ///  "type": "string",
    ///  "pattern":
    /// "^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|impi-.
    /// +|impu-.+|.+)$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SubscriberId(String);
    impl ::std::ops::Deref for SubscriberId {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<SubscriberId> for String {
        fn from(value: SubscriberId) -> Self {
            value.0
        }
    }

    impl From<&SubscriberId> for SubscriberId {
        fn from(value: &SubscriberId) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for SubscriberId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress :: Regex :: new ("^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|impi-.+|impu-.+|.+)$") . unwrap () . find (value) . is_none () { return Err ("doesn't match pattern \"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|impi-.+|impu-.+|.+)$\"" . into ()) ; }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for SubscriberId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for SubscriberId {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for SubscriberId {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SubscriberId {
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

    ///A range of SUPIs (subscriber identities), either based on a numeric
    /// range, or based on regular-expression matching
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A range of SUPIs (subscriber identities), either based
    /// on a numeric range, or based on regular-expression matching\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "end": {
    ///      "type": "string",
    ///      "pattern": "^[0-9]+$"
    ///    },
    ///    "pattern": {
    ///      "type": "string"
    ///    },
    ///    "start": {
    ///      "type": "string",
    ///      "pattern": "^[0-9]+$"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SupiRange {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end: Option<SupiRangeEnd>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pattern: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub start: Option<SupiRangeStart>,
    }

    impl From<&SupiRange> for SupiRange {
        fn from(value: &SupiRange) -> Self {
            value.clone()
        }
    }

    ///SupiRangeEnd
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^[0-9]+$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SupiRangeEnd(String);
    impl ::std::ops::Deref for SupiRangeEnd {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<SupiRangeEnd> for String {
        fn from(value: SupiRangeEnd) -> Self {
            value.0
        }
    }

    impl From<&SupiRangeEnd> for SupiRangeEnd {
        fn from(value: &SupiRangeEnd) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for SupiRangeEnd {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[0-9]+$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[0-9]+$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for SupiRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for SupiRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for SupiRangeEnd {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SupiRangeEnd {
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

    ///SupiRangeStart
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^[0-9]+$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SupiRangeStart(String);
    impl ::std::ops::Deref for SupiRangeStart {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<SupiRangeStart> for String {
        fn from(value: SupiRangeStart) -> Self {
            value.0
        }
    }

    impl From<&SupiRangeStart> for SupiRangeStart {
        fn from(value: &SupiRangeStart) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for SupiRangeStart {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^[0-9]+$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[0-9]+$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for SupiRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for SupiRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for SupiRangeStart {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SupiRangeStart {
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
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SupportedFeatures(String);
    impl ::std::ops::Deref for SupportedFeatures {
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

    impl ::std::str::FromStr for SupportedFeatures {
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

    impl ::std::convert::TryFrom<&str> for SupportedFeatures {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for SupportedFeatures {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for SupportedFeatures {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SupportedFeatures {
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
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
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
