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
pub enum AmfServiceName {
    #[default]
    #[serde(rename = "namf_communication")]
    Communication,
    #[serde(rename = "namf_eventexposure")]
    EventExposure,
    #[serde(rename = "namf_location")]
    Location,
    #[serde(rename = "namf_mbsbroadcast")]
    MBSBroadcast,
    #[serde(rename = "namf_mbscommunication")]
    MBSCommunication,
    #[serde(rename = "namf_mt")]
    MT,
}

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
			AmfService::Communication(inner) => format!("/communication/{}", inner.get_path()),
			AmfService::EventExposure(inner) => format!("/eventexposure/{}", inner.get_path()),
			AmfService::Location(inner) => format!("/location/{}", inner.get_path()),
			AmfService::MBSBroadcast(inner) => format!("/mbsbroadcast/{}", inner.get_path()),
			AmfService::MBSCommunication(inner) => {
				format!("/mbscommunication/{}", inner.get_path())
			}
			AmfService::MT(inner) => format!("/mt/{}", inner.get_path()),
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
	Createuecontext,
	Releaseuecontext,
	Ebiassignment,
	Uecontexttransfer,
	Registrationstatusupdate,
	Relocateuecontext,
	Cancelrelocateuecontext,
	N1n2messagetransfer,
	N1n2messagesubscribe,
	N1n2messageunsubscribe,
	Nonuen2messagetransfer,
	Nonuen2infosubscribe,
	Nonuen2infounsubscribe,
	Amfstatuschangesubscribe,
	Amfstatuschangeunsubscribe,
	Amfstatuschangesubscribemodfy,
}

impl super::ServiceProperties for AmfCommunicationOperation {
	fn get_path(&self) -> String {
		match self {
			AmfCommunicationOperation::Createuecontext => "/ue-contexts/{}".to_string(),
			AmfCommunicationOperation::Releaseuecontext => "/ue-contexts/{}/release".to_string(),
			AmfCommunicationOperation::Ebiassignment => "/ue-contexts/{}/assign-ebi".to_string(),
			AmfCommunicationOperation::Uecontexttransfer => "/ue-contexts/{}/transfer".to_string(),
			AmfCommunicationOperation::Registrationstatusupdate => {
				"/ue-contexts/{}/transfer-update".to_string()
			}
			AmfCommunicationOperation::Relocateuecontext => "/ue-contexts/{}/relocate".to_string(),
			AmfCommunicationOperation::Cancelrelocateuecontext => {
				"/ue-contexts/{}/cancel-relocate".to_string()
			}
			AmfCommunicationOperation::N1n2messagetransfer => {
				"/ue-contexts/{}/n1-n2-messages".to_string()
			}
			AmfCommunicationOperation::N1n2messagesubscribe => {
				"/ue-contexts/{}/n1-n2-messages/subscriptions".to_string()
			}
			AmfCommunicationOperation::N1n2messageunsubscribe => {
				"/ue-contexts/{}/n1-n2-messages/subscriptions/{}".to_string()
			}
			AmfCommunicationOperation::Nonuen2messagetransfer => {
				"/non-ue-n2-messages/transfer".to_string()
			}
			AmfCommunicationOperation::Nonuen2infosubscribe => {
				"/non-ue-n2-messages/subscriptions".to_string()
			}
			AmfCommunicationOperation::Nonuen2infounsubscribe => {
				"/non-ue-n2-messages/subscriptions/{}".to_string()
			}
			AmfCommunicationOperation::Amfstatuschangesubscribe => "/subscriptions".to_string(),
			AmfCommunicationOperation::Amfstatuschangeunsubscribe => {
				"/subscriptions/{}".to_string()
			}
			AmfCommunicationOperation::Amfstatuschangesubscribemodfy => {
				"/subscriptions/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfCommunicationOperation::Createuecontext => reqwest::Method::PUT,
			AmfCommunicationOperation::Releaseuecontext => reqwest::Method::POST,
			AmfCommunicationOperation::Ebiassignment => reqwest::Method::POST,
			AmfCommunicationOperation::Uecontexttransfer => reqwest::Method::POST,
			AmfCommunicationOperation::Registrationstatusupdate => reqwest::Method::POST,
			AmfCommunicationOperation::Relocateuecontext => reqwest::Method::POST,
			AmfCommunicationOperation::Cancelrelocateuecontext => reqwest::Method::POST,
			AmfCommunicationOperation::N1n2messagetransfer => reqwest::Method::POST,
			AmfCommunicationOperation::N1n2messagesubscribe => reqwest::Method::POST,
			AmfCommunicationOperation::N1n2messageunsubscribe => reqwest::Method::DELETE,
			AmfCommunicationOperation::Nonuen2messagetransfer => reqwest::Method::POST,
			AmfCommunicationOperation::Nonuen2infosubscribe => reqwest::Method::POST,
			AmfCommunicationOperation::Nonuen2infounsubscribe => reqwest::Method::DELETE,
			AmfCommunicationOperation::Amfstatuschangesubscribe => reqwest::Method::POST,
			AmfCommunicationOperation::Amfstatuschangeunsubscribe => reqwest::Method::DELETE,
			AmfCommunicationOperation::Amfstatuschangesubscribemodfy => reqwest::Method::PUT,
		}
	}
}

pub enum AmfEventExposureOperation {
	Createsubscription,
	Modifysubscription,
	Deletesubscription,
}

impl super::ServiceProperties for AmfEventExposureOperation {
	fn get_path(&self) -> String {
		match self {
			AmfEventExposureOperation::Createsubscription => "/subscriptions".to_string(),
			AmfEventExposureOperation::Modifysubscription => "/subscriptions/{}".to_string(),
			AmfEventExposureOperation::Deletesubscription => "/subscriptions/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfEventExposureOperation::Createsubscription => reqwest::Method::POST,
			AmfEventExposureOperation::Modifysubscription => reqwest::Method::PATCH,
			AmfEventExposureOperation::Deletesubscription => reqwest::Method::DELETE,
		}
	}
}

pub enum AmfLocationOperation {
	Providepositioninginfo,
	Providelocationinfo,
	Cancellocation,
}

impl super::ServiceProperties for AmfLocationOperation {
	fn get_path(&self) -> String {
		match self {
			AmfLocationOperation::Providepositioninginfo => "/{}/provide-pos-info".to_string(),
			AmfLocationOperation::Providelocationinfo => "/{}/provide-loc-info".to_string(),
			AmfLocationOperation::Cancellocation => "/{}/cancel-pos-info".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfLocationOperation::Providepositioninginfo => reqwest::Method::POST,
			AmfLocationOperation::Providelocationinfo => reqwest::Method::POST,
			AmfLocationOperation::Cancellocation => reqwest::Method::POST,
		}
	}
}

pub enum AmfMBSBroadcastOperation {
	Contextcreate,
	Contextdelete,
	Contextupdate,
}

impl super::ServiceProperties for AmfMBSBroadcastOperation {
	fn get_path(&self) -> String {
		match self {
			AmfMBSBroadcastOperation::Contextcreate => "/mbs-contexts".to_string(),
			AmfMBSBroadcastOperation::Contextdelete => "/mbs-contexts/{}".to_string(),
			AmfMBSBroadcastOperation::Contextupdate => "/mbs-contexts/{}/update".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfMBSBroadcastOperation::Contextcreate => reqwest::Method::POST,
			AmfMBSBroadcastOperation::Contextdelete => reqwest::Method::DELETE,
			AmfMBSBroadcastOperation::Contextupdate => reqwest::Method::POST,
		}
	}
}

pub enum AmfMBSCommunicationOperation {
	N2messagetransfer,
}

impl super::ServiceProperties for AmfMBSCommunicationOperation {
	fn get_path(&self) -> String {
		match self {
			AmfMBSCommunicationOperation::N2messagetransfer => "/n2-messages/transfer".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfMBSCommunicationOperation::N2messagetransfer => reqwest::Method::POST,
		}
	}
}

pub enum AmfMTOperation {
	Providedomainselectioninfo,
	Enableuereachability,
	Enablegroupreachability,
}

impl super::ServiceProperties for AmfMTOperation {
	fn get_path(&self) -> String {
		match self {
			AmfMTOperation::Providedomainselectioninfo => "/ue-contexts/{}".to_string(),
			AmfMTOperation::Enableuereachability => "/ue-contexts/{}/ue-reachind".to_string(),
			AmfMTOperation::Enablegroupreachability => {
				"/ue-contexts/enable-group-reachability".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AmfMTOperation::Providedomainselectioninfo => reqwest::Method::GET,
			AmfMTOperation::Enableuereachability => reqwest::Method::PUT,
			AmfMTOperation::Enablegroupreachability => reqwest::Method::POST,
		}
	}
}
