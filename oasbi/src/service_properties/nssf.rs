pub enum NssfService {
	NSSAIAvailability(NssfNSSAIAvailabilityOperation),
	NSSelection(NssfNSSelectionOperation),
}

impl super::ServiceProperties for NssfService {
	fn get_path(&self) -> String {
		match self {
			NssfService::NSSAIAvailability(inner) => {
				format!("/nnssf-nssaiavailability/v1/{}", inner.get_path())
			}
			NssfService::NSSelection(inner) => {
				format!("/nnssf-nsselection/v2/{}", inner.get_path())
			}
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
	NSSAIAvailabilityPut,
	NSSAIAvailabilityPatch,
	NSSAIAvailabilityDelete,
	NSSAIAvailabilityPost,
	NSSAIAvailabilityUnsubscribe,
	NSSAIAvailabilitySubModifyPatch,
	NSSAIAvailabilityOptions,
}

impl super::ServiceProperties for NssfNSSAIAvailabilityOperation {
	fn get_path(&self) -> String {
		match self {
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityPut => {
				"nssai-availability/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityPatch => {
				"nssai-availability/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityDelete => {
				"nssai-availability/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityPost => {
				"nssai-availability/subscriptions".to_string()
			}
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityUnsubscribe => {
				"nssai-availability/subscriptions/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilitySubModifyPatch => {
				"nssai-availability/subscriptions/{}".to_string()
			}
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityOptions => {
				"nssai-availability".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityPut => reqwest::Method::PUT,
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityPatch => reqwest::Method::PATCH,
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityDelete => reqwest::Method::DELETE,
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityPost => reqwest::Method::POST,
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityUnsubscribe => reqwest::Method::DELETE,
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilitySubModifyPatch => {
				reqwest::Method::PATCH
			}
			NssfNSSAIAvailabilityOperation::NSSAIAvailabilityOptions => reqwest::Method::OPTIONS,
		}
	}
}

pub enum NssfNSSelectionOperation {
	NSSelectionGet,
}

impl super::ServiceProperties for NssfNSSelectionOperation {
	fn get_path(&self) -> String {
		match self {
			NssfNSSelectionOperation::NSSelectionGet => "network-slice-information".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NssfNSSelectionOperation::NSSelectionGet => reqwest::Method::GET,
		}
	}
}
