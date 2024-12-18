#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};

#[allow(unused_imports)]
use crate::progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use crate::progenitor_client::{ByteStream, Error, ResponseValue};

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
	
	/// Error types.
	pub use crate::common::*;

	/// Event Type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Event Type",
	///  "type": "string",
	///  "enum": [
	///    "QOS_MONITORING"
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
	pub enum EventType {
		#[default]
		#[serde(rename = "QOS_MONITORING")]
		QosMonitoring,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&EventType> for EventType {
		fn from(value: &EventType) -> Self {
			value.clone()
		}
	}

	impl ToString for EventType {
		fn to_string(&self) -> String {
			match *self {
				Self::QosMonitoring => "QOS_MONITORING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for EventType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"QOS_MONITORING" => Ok(Self::QosMonitoring),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for EventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for EventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for EventType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// the list of NotificationItems
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "the list of NotificationItems",
	///  "type": "object",
	///  "required": [
	///    "notificationItems"
	///  ],
	///  "properties": {
	///    "correlationId": {
	///      "type": "string"
	///    },
	///    "notificationItems": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NotificationItem"
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
	pub struct NotificationData {
		#[serde(
			rename = "correlationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub correlation_id: Option<String>,
		#[serde(rename = "notificationItems")]
		pub notification_items: Vec<NotificationItem>,
	}

	impl From<&NotificationData> for NotificationData {
		fn from(value: &NotificationData) -> Self {
			value.clone()
		}
	}

	/// represents a report on one subscribed event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "represents a report on one subscribed event",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "ueIpv4Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ueIpv6Prefix"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ueMacAddr"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "eventType",
	///    "timeStamp"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "eventType": {
	///      "$ref": "#/components/schemas/EventType"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "qosMonitoringMeasurement": {
	///      "$ref": "#/components/schemas/QosMonitoringMeasurement"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "startTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "ueIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ueIpv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "ueMacAddr": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum NotificationItem {
		#[default]
		Variant0 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "eventType")]
			event_type: EventType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(
				rename = "qosMonitoringMeasurement",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			qos_monitoring_measurement: Option<QosMonitoringMeasurement>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
			#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
			start_time: Option<DateTime>,
			#[serde(rename = "timeStamp")]
			time_stamp: DateTime,
			#[serde(rename = "ueIpv4Addr")]
			ue_ipv4_addr: Ipv4Addr,
		},
		Variant1 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "eventType")]
			event_type: EventType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(
				rename = "qosMonitoringMeasurement",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			qos_monitoring_measurement: Option<QosMonitoringMeasurement>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
			#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
			start_time: Option<DateTime>,
			#[serde(rename = "timeStamp")]
			time_stamp: DateTime,
			#[serde(rename = "ueIpv6Prefix")]
			ue_ipv6_prefix: Ipv6Prefix,
		},
		Variant2 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "eventType")]
			event_type: EventType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(
				rename = "qosMonitoringMeasurement",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			qos_monitoring_measurement: Option<QosMonitoringMeasurement>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
			#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
			start_time: Option<DateTime>,
			#[serde(rename = "timeStamp")]
			time_stamp: DateTime,
			#[serde(rename = "ueMacAddr")]
			ue_mac_addr: MacAddr48,
		},
	}

	impl From<&NotificationItem> for NotificationItem {
		fn from(value: &NotificationItem) -> Self {
			value.clone()
		}
	}

	/// QoS Monitoring Measurement information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "QoS Monitoring Measurement information",
	///  "type": "object",
	///  "properties": {
	///    "dlPacketDelay": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "measureFailure": {
	///      "type": "boolean",
	///      "enum": [
	///        true
	///      ]
	///    },
	///    "rtrPacketDelay": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "ulPacketDelay": {
	///      "$ref": "#/components/schemas/Uint32"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosMonitoringMeasurement {
		#[serde(
			rename = "dlPacketDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dl_packet_delay: Option<Uint32>,
		#[serde(
			rename = "measureFailure",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub measure_failure: Option<bool>,
		#[serde(
			rename = "rtrPacketDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rtr_packet_delay: Option<Uint32>,
		#[serde(
			rename = "ulPacketDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ul_packet_delay: Option<Uint32>,
	}

	impl From<&QosMonitoringMeasurement> for QosMonitoringMeasurement {
		fn from(value: &QosMonitoringMeasurement) -> Self {
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
