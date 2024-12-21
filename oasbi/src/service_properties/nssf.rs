
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
    Copy,
)]
pub enum NssfServiceName {
    #[default]
    #[serde(rename = "nnssf-nssaiavailability")]
    NSSAIAvailability,
    #[serde(rename = "nnssf-nsselection")]
    NSSelection,
}

pub enum NssfService {
	NSSAIAvailability(NssfNSSAIAvailabilityOperation),
	NSSelection(NssfNSSelectionOperation),
}

impl super::ServiceProperties for NssfService {
	fn get_path(&self) -> String {
		match self {
			NssfService::NSSAIAvailability(inner) => {
				format!("/nssaiavailability/{}", inner.get_path())
			}
			NssfService::NSSelection(inner) => format!("/nsselection/{}", inner.get_path()),
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NssfService::NSSAIAvailability(inner) => inner.get_http_method(),
			NssfService::NSSelection(inner) => inner.get_http_method(),
		}
	}
}

pub enum NssfNSSAIAvailabilityOperation {
	Nssaiavailabilityput,
	Nssaiavailabilitypatch,
	Nssaiavailabilitydelete,
	Nssaiavailabilitypost,
	Nssaiavailabilityunsubscribe,
	Nssaiavailabilitysubmodifypatch,
	Nssaiavailabilityoptions,
}

impl super::ServiceProperties for NssfNSSAIAvailabilityOperation {
	fn get_path(&self) -> String {
		match self {
			NssfNSSAIAvailabilityOperation::Nssaiavailabilityput => {
				"/nssai-availability/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::Nssaiavailabilitypatch => {
				"/nssai-availability/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::Nssaiavailabilitydelete => {
				"/nssai-availability/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::Nssaiavailabilitypost => {
				"/nssai-availability/subscriptions".to_string()
			}
			NssfNSSAIAvailabilityOperation::Nssaiavailabilityunsubscribe => {
				"/nssai-availability/subscriptions/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::Nssaiavailabilitysubmodifypatch => {
				"/nssai-availability/subscriptions/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::Nssaiavailabilityoptions => {
				"/nssai-availability".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NssfNSSAIAvailabilityOperation::Nssaiavailabilityput => reqwest::Method::PUT,
			NssfNSSAIAvailabilityOperation::Nssaiavailabilitypatch => reqwest::Method::PATCH,
			NssfNSSAIAvailabilityOperation::Nssaiavailabilitydelete => reqwest::Method::DELETE,
			NssfNSSAIAvailabilityOperation::Nssaiavailabilitypost => reqwest::Method::POST,
			NssfNSSAIAvailabilityOperation::Nssaiavailabilityunsubscribe => reqwest::Method::DELETE,
			NssfNSSAIAvailabilityOperation::Nssaiavailabilitysubmodifypatch => {
				reqwest::Method::PATCH
			}
			NssfNSSAIAvailabilityOperation::Nssaiavailabilityoptions => reqwest::Method::OPTIONS,
		}
	}
}

pub enum NssfNSSelectionOperation {
	Nsselectionget,
}

impl super::ServiceProperties for NssfNSSelectionOperation {
	fn get_path(&self) -> String {
		match self {
			NssfNSSelectionOperation::Nsselectionget => "/network-slice-information".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NssfNSSelectionOperation::Nsselectionget => reqwest::Method::GET,
		}
	}
}
