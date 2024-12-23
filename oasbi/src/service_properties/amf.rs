pub enum AmfService {
	Communication(AmfCommunicationOperation),
	EventExposure(AmfEventExposureOperation),
	Location(AmfLocationOperation),
	MBSBroadcast(AmfMBSBroadcastOperation),
	MBSCommunication(AmfMBSCommunicationOperation),
	MT(AmfMTOperation),
}

impl super::ServiceProperties for AmfService {
	fn get_path(&self) -> String {
		match self {
			AmfService::Communication(inner) => format!("/namf-comm/v1/{}", inner.get_path()),
			AmfService::EventExposure(inner) => format!("/namf-evts/v1/{}", inner.get_path()),
			AmfService::Location(inner) => format!("/namf-loc/v1/{}", inner.get_path()),
			AmfService::MBSBroadcast(inner) => format!("/namf-mbs-bc/v1/{}", inner.get_path()),
			AmfService::MBSCommunication(inner) => {
				format!("/namf-mbs-comm/v1/{}", inner.get_path())
			}
			AmfService::MT(inner) => format!("/namf-mt/v1/{}", inner.get_path()),
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfService::Communication(inner) => inner.get_http_method(),
			AmfService::EventExposure(inner) => inner.get_http_method(),
			AmfService::Location(inner) => inner.get_http_method(),
			AmfService::MBSBroadcast(inner) => inner.get_http_method(),
			AmfService::MBSCommunication(inner) => inner.get_http_method(),
			AmfService::MT(inner) => inner.get_http_method(),
		}
	}
}

pub enum AmfCommunicationOperation {
	CreateUEContext,
	ReleaseUEContext,
	EBIAssignment,
	UEContextTransfer,
	RegistrationStatusUpdate,
	RelocateUEContext,
	CancelRelocateUEContext,
	N1N2MessageTransfer,
	N1N2MessageSubscribe,
	N1N2MessageUnSubscribe,
	NonUeN2MessageTransfer,
	NonUeN2InfoSubscribe,
	NonUeN2InfoUnSubscribe,
	AMFStatusChangeSubscribe,
	AMFStatusChangeUnSubscribe,
	AMFStatusChangeSubscribeModfy,
}

impl super::ServiceProperties for AmfCommunicationOperation {
	fn get_path(&self) -> String {
		match self {
			AmfCommunicationOperation::CreateUEContext => "ue-contexts/{}".to_string(),
			AmfCommunicationOperation::ReleaseUEContext => "ue-contexts/{}/release".to_string(),
			AmfCommunicationOperation::EBIAssignment => "ue-contexts/{}/assign-ebi".to_string(),
			AmfCommunicationOperation::UEContextTransfer => "ue-contexts/{}/transfer".to_string(),
			AmfCommunicationOperation::RegistrationStatusUpdate => {
				"ue-contexts/{}/transfer-update".to_string()
			}
			AmfCommunicationOperation::RelocateUEContext => "ue-contexts/{}/relocate".to_string(),
			AmfCommunicationOperation::CancelRelocateUEContext => {
				"ue-contexts/{}/cancel-relocate".to_string()
			}
			AmfCommunicationOperation::N1N2MessageTransfer => {
				"ue-contexts/{}/n1-n2-messages".to_string()
			}
			AmfCommunicationOperation::N1N2MessageSubscribe => {
				"ue-contexts/{}/n1-n2-messages/subscriptions".to_string()
			}
			AmfCommunicationOperation::N1N2MessageUnSubscribe => {
				"ue-contexts/{}/n1-n2-messages/subscriptions/{}".to_string()
			}
			AmfCommunicationOperation::NonUeN2MessageTransfer => {
				"non-ue-n2-messages/transfer".to_string()
			}
			AmfCommunicationOperation::NonUeN2InfoSubscribe => {
				"non-ue-n2-messages/subscriptions".to_string()
			}
			AmfCommunicationOperation::NonUeN2InfoUnSubscribe => {
				"non-ue-n2-messages/subscriptions/{}".to_string()
			}
			AmfCommunicationOperation::AMFStatusChangeSubscribe => "subscriptions".to_string(),
			AmfCommunicationOperation::AMFStatusChangeUnSubscribe => "subscriptions/{}".to_string(),
			AmfCommunicationOperation::AMFStatusChangeSubscribeModfy => {
				"subscriptions/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfCommunicationOperation::CreateUEContext => reqwest::Method::PUT,
			AmfCommunicationOperation::ReleaseUEContext => reqwest::Method::POST,
			AmfCommunicationOperation::EBIAssignment => reqwest::Method::POST,
			AmfCommunicationOperation::UEContextTransfer => reqwest::Method::POST,
			AmfCommunicationOperation::RegistrationStatusUpdate => reqwest::Method::POST,
			AmfCommunicationOperation::RelocateUEContext => reqwest::Method::POST,
			AmfCommunicationOperation::CancelRelocateUEContext => reqwest::Method::POST,
			AmfCommunicationOperation::N1N2MessageTransfer => reqwest::Method::POST,
			AmfCommunicationOperation::N1N2MessageSubscribe => reqwest::Method::POST,
			AmfCommunicationOperation::N1N2MessageUnSubscribe => reqwest::Method::DELETE,
			AmfCommunicationOperation::NonUeN2MessageTransfer => reqwest::Method::POST,
			AmfCommunicationOperation::NonUeN2InfoSubscribe => reqwest::Method::POST,
			AmfCommunicationOperation::NonUeN2InfoUnSubscribe => reqwest::Method::DELETE,
			AmfCommunicationOperation::AMFStatusChangeSubscribe => reqwest::Method::POST,
			AmfCommunicationOperation::AMFStatusChangeUnSubscribe => reqwest::Method::DELETE,
			AmfCommunicationOperation::AMFStatusChangeSubscribeModfy => reqwest::Method::PUT,
		}
	}
}

pub enum AmfEventExposureOperation {
	CreateSubscription,
	ModifySubscription,
	DeleteSubscription,
}

impl super::ServiceProperties for AmfEventExposureOperation {
	fn get_path(&self) -> String {
		match self {
			AmfEventExposureOperation::CreateSubscription => "subscriptions".to_string(),
			AmfEventExposureOperation::ModifySubscription => "subscriptions/{}".to_string(),
			AmfEventExposureOperation::DeleteSubscription => "subscriptions/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfEventExposureOperation::CreateSubscription => reqwest::Method::POST,
			AmfEventExposureOperation::ModifySubscription => reqwest::Method::PATCH,
			AmfEventExposureOperation::DeleteSubscription => reqwest::Method::DELETE,
		}
	}
}

pub enum AmfLocationOperation {
	ProvidePositioningInfo,
	ProvideLocationInfo,
	CancelLocation,
}

impl super::ServiceProperties for AmfLocationOperation {
	fn get_path(&self) -> String {
		match self {
			AmfLocationOperation::ProvidePositioningInfo => "{}/provide-pos-info".to_string(),
			AmfLocationOperation::ProvideLocationInfo => "{}/provide-loc-info".to_string(),
			AmfLocationOperation::CancelLocation => "{}/cancel-pos-info".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfLocationOperation::ProvidePositioningInfo => reqwest::Method::POST,
			AmfLocationOperation::ProvideLocationInfo => reqwest::Method::POST,
			AmfLocationOperation::CancelLocation => reqwest::Method::POST,
		}
	}
}

pub enum AmfMBSBroadcastOperation {
	ContextCreate,
	ContextDelete,
	ContextUpdate,
}

impl super::ServiceProperties for AmfMBSBroadcastOperation {
	fn get_path(&self) -> String {
		match self {
			AmfMBSBroadcastOperation::ContextCreate => "mbs-contexts".to_string(),
			AmfMBSBroadcastOperation::ContextDelete => "mbs-contexts/{}".to_string(),
			AmfMBSBroadcastOperation::ContextUpdate => "mbs-contexts/{}/update".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfMBSBroadcastOperation::ContextCreate => reqwest::Method::POST,
			AmfMBSBroadcastOperation::ContextDelete => reqwest::Method::DELETE,
			AmfMBSBroadcastOperation::ContextUpdate => reqwest::Method::POST,
		}
	}
}

pub enum AmfMBSCommunicationOperation {
	N2MessageTransfer,
}

impl super::ServiceProperties for AmfMBSCommunicationOperation {
	fn get_path(&self) -> String {
		match self {
			AmfMBSCommunicationOperation::N2MessageTransfer => "n2-messages/transfer".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfMBSCommunicationOperation::N2MessageTransfer => reqwest::Method::POST,
		}
	}
}

pub enum AmfMTOperation {
	ProvideDomainSelectionInfo,
	EnableUeReachability,
	EnableGroupReachability,
}

impl super::ServiceProperties for AmfMTOperation {
	fn get_path(&self) -> String {
		match self {
			AmfMTOperation::ProvideDomainSelectionInfo => "ue-contexts/{}".to_string(),
			AmfMTOperation::EnableUeReachability => "ue-contexts/{}/ue-reachind".to_string(),
			AmfMTOperation::EnableGroupReachability => {
				"ue-contexts/enable-group-reachability".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfMTOperation::ProvideDomainSelectionInfo => reqwest::Method::GET,
			AmfMTOperation::EnableUeReachability => reqwest::Method::PUT,
			AmfMTOperation::EnableGroupReachability => reqwest::Method::POST,
		}
	}
}
