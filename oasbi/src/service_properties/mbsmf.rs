pub enum MbsmfService {
	MBSSession(MbsmfMBSSessionOperation),
	TMGI(MbsmfTMGIOperation),
}

impl super::ServiceProperties for MbsmfService {
	fn get_path(&self) -> String {
		match self {
			MbsmfService::MBSSession(inner) => {
				format!("/nmbsmf-mbssession/v1/{}", inner.get_path())
			}
			MbsmfService::TMGI(inner) => format!("/nmbsmf-tmgi/v1/{}", inner.get_path()),
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
	ContextUpdate,
	StatusSubscribe,
	StatusSubscribeMod,
	StatusUnSubscribe,
	ContextStatusSubscribe,
	ContextStatusSubscribeMod,
	ContextStatusUnSubscribe,
}

impl super::ServiceProperties for MbsmfMBSSessionOperation {
	fn get_path(&self) -> String {
		match self {
			MbsmfMBSSessionOperation::Create => "mbs-sessions".to_string(),
			MbsmfMBSSessionOperation::Update => "mbs-sessions/{}".to_string(),
			MbsmfMBSSessionOperation::Release => "mbs-sessions/{}".to_string(),
			MbsmfMBSSessionOperation::ContextUpdate => "mbs-sessions/contexts/update".to_string(),
			MbsmfMBSSessionOperation::StatusSubscribe => "mbs-sessions/subscriptions".to_string(),
			MbsmfMBSSessionOperation::StatusSubscribeMod => {
				"mbs-sessions/subscriptions/{}".to_string()
			}
			MbsmfMBSSessionOperation::StatusUnSubscribe => {
				"mbs-sessions/subscriptions/{}".to_string()
			}
			MbsmfMBSSessionOperation::ContextStatusSubscribe => {
				"mbs-sessions/contexts/subscriptions".to_string()
			}
			MbsmfMBSSessionOperation::ContextStatusSubscribeMod => {
				"mbs-sessions/contexts/subscriptions/{}".to_string()
			}
			MbsmfMBSSessionOperation::ContextStatusUnSubscribe => {
				"mbs-sessions/contexts/subscriptions/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			MbsmfMBSSessionOperation::Create => reqwest::Method::POST,
			MbsmfMBSSessionOperation::Update => reqwest::Method::PATCH,
			MbsmfMBSSessionOperation::Release => reqwest::Method::DELETE,
			MbsmfMBSSessionOperation::ContextUpdate => reqwest::Method::POST,
			MbsmfMBSSessionOperation::StatusSubscribe => reqwest::Method::POST,
			MbsmfMBSSessionOperation::StatusSubscribeMod => reqwest::Method::PATCH,
			MbsmfMBSSessionOperation::StatusUnSubscribe => reqwest::Method::DELETE,
			MbsmfMBSSessionOperation::ContextStatusSubscribe => reqwest::Method::POST,
			MbsmfMBSSessionOperation::ContextStatusSubscribeMod => reqwest::Method::PATCH,
			MbsmfMBSSessionOperation::ContextStatusUnSubscribe => reqwest::Method::DELETE,
		}
	}
}

pub enum MbsmfTMGIOperation {
	AllocateTmgi,
	TMGIDeallocate,
}

impl super::ServiceProperties for MbsmfTMGIOperation {
	fn get_path(&self) -> String {
		match self {
			MbsmfTMGIOperation::AllocateTmgi => "tmgi".to_string(),
			MbsmfTMGIOperation::TMGIDeallocate => "tmgi".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			MbsmfTMGIOperation::AllocateTmgi => reqwest::Method::POST,
			MbsmfTMGIOperation::TMGIDeallocate => reqwest::Method::DELETE,
		}
	}
}
