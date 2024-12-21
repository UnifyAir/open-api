
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
pub enum MbsmfServiceName {
    #[default]
    #[serde(rename = "nmbsmf-mbssession")]
    MBSSession,
    #[serde(rename = "nmbsmf-tmgi")]
    TMGI,
}

pub enum MbsmfService {
	MBSSession(MbsmfMBSSessionOperation),
	TMGI(MbsmfTMGIOperation),
}

impl super::ServiceProperties for MbsmfService {
	fn get_path(&self) -> String {
		match self {
			MbsmfService::MBSSession(inner) => format!("/mbssession/{}", inner.get_path()),
			MbsmfService::TMGI(inner) => format!("/tmgi/{}", inner.get_path()),
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			MbsmfService::MBSSession(inner) => inner.get_http_method(),
			MbsmfService::TMGI(inner) => inner.get_http_method(),
		}
	}
}

pub enum MbsmfMBSSessionOperation {
	Create,
	Update,
	Release,
	Contextupdate,
	Statussubscribe,
	Statussubscribemod,
	Statusunsubscribe,
	Contextstatussubscribe,
	Contextstatussubscribemod,
	Contextstatusunsubscribe,
}

impl super::ServiceProperties for MbsmfMBSSessionOperation {
	fn get_path(&self) -> String {
		match self {
			MbsmfMBSSessionOperation::Create => "/mbs-sessions".to_string(),
			MbsmfMBSSessionOperation::Update => "/mbs-sessions/{}".to_string(),
			MbsmfMBSSessionOperation::Release => "/mbs-sessions/{}".to_string(),
			MbsmfMBSSessionOperation::Contextupdate => "/mbs-sessions/contexts/update".to_string(),
			MbsmfMBSSessionOperation::Statussubscribe => "/mbs-sessions/subscriptions".to_string(),
			MbsmfMBSSessionOperation::Statussubscribemod => {
				"/mbs-sessions/subscriptions/{}".to_string()
			}
			MbsmfMBSSessionOperation::Statusunsubscribe => {
				"/mbs-sessions/subscriptions/{}".to_string()
			}
			MbsmfMBSSessionOperation::Contextstatussubscribe => {
				"/mbs-sessions/contexts/subscriptions".to_string()
			}
			MbsmfMBSSessionOperation::Contextstatussubscribemod => {
				"/mbs-sessions/contexts/subscriptions/{}".to_string()
			}
			MbsmfMBSSessionOperation::Contextstatusunsubscribe => {
				"/mbs-sessions/contexts/subscriptions/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			MbsmfMBSSessionOperation::Create => reqwest::Method::POST,
			MbsmfMBSSessionOperation::Update => reqwest::Method::PATCH,
			MbsmfMBSSessionOperation::Release => reqwest::Method::DELETE,
			MbsmfMBSSessionOperation::Contextupdate => reqwest::Method::POST,
			MbsmfMBSSessionOperation::Statussubscribe => reqwest::Method::POST,
			MbsmfMBSSessionOperation::Statussubscribemod => reqwest::Method::PATCH,
			MbsmfMBSSessionOperation::Statusunsubscribe => reqwest::Method::DELETE,
			MbsmfMBSSessionOperation::Contextstatussubscribe => reqwest::Method::POST,
			MbsmfMBSSessionOperation::Contextstatussubscribemod => reqwest::Method::PATCH,
			MbsmfMBSSessionOperation::Contextstatusunsubscribe => reqwest::Method::DELETE,
		}
	}
}

pub enum MbsmfTMGIOperation {
	Allocatetmgi,
	Tmgideallocate,
}

impl super::ServiceProperties for MbsmfTMGIOperation {
	fn get_path(&self) -> String {
		match self {
			MbsmfTMGIOperation::Allocatetmgi => "/tmgi".to_string(),
			MbsmfTMGIOperation::Tmgideallocate => "/tmgi".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			MbsmfTMGIOperation::Allocatetmgi => reqwest::Method::POST,
			MbsmfTMGIOperation::Tmgideallocate => reqwest::Method::DELETE,
		}
	}
}
