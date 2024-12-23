pub enum SmfService {
	PDUSession(SmfPDUSessionOperation),
	EventExposure(SmfEventExposureOperation),
	NIDD(SmfNIDDOperation),
}

impl super::ServiceProperties for SmfService {
	fn get_path(&self) -> String {
		match self {
			SmfService::PDUSession(inner) => format!("/nsmf-pdusession/v1/{}", inner.get_path()),
			SmfService::EventExposure(inner) => {
				format!("/nsmf-event-exposure/v1/{}", inner.get_path())
			}
			SmfService::NIDD(inner) => format!("/nsmf-nidd/v1/{}", inner.get_path()),
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			SmfService::PDUSession(inner) => inner.get_http_method(),
			SmfService::EventExposure(inner) => inner.get_http_method(),
			SmfService::NIDD(inner) => inner.get_http_method(),
		}
	}
}

pub enum SmfPDUSessionOperation {
	PostSmContexts,
	RetrieveSmContext,
	UpdateSmContext,
	ReleaseSmContext,
	SendMoData,
	PostPduSessions,
	UpdatePduSession,
	ReleasePduSession,
	RetrievePduSession,
	TransferMoData,
}

impl super::ServiceProperties for SmfPDUSessionOperation {
	fn get_path(&self) -> String {
		match self {
			SmfPDUSessionOperation::PostSmContexts => "sm-contexts".to_string(),
			SmfPDUSessionOperation::RetrieveSmContext => "sm-contexts/{}/retrieve".to_string(),
			SmfPDUSessionOperation::UpdateSmContext => "sm-contexts/{}/modify".to_string(),
			SmfPDUSessionOperation::ReleaseSmContext => "sm-contexts/{}/release".to_string(),
			SmfPDUSessionOperation::SendMoData => "sm-contexts/{}/send-mo-data".to_string(),
			SmfPDUSessionOperation::PostPduSessions => "pdu-sessions".to_string(),
			SmfPDUSessionOperation::UpdatePduSession => "pdu-sessions/{}/modify".to_string(),
			SmfPDUSessionOperation::ReleasePduSession => "pdu-sessions/{}/release".to_string(),
			SmfPDUSessionOperation::RetrievePduSession => "pdu-sessions/{}/retrieve".to_string(),
			SmfPDUSessionOperation::TransferMoData => {
				"pdu-sessions/{}/transfer-mo-data".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			SmfPDUSessionOperation::PostSmContexts => reqwest::Method::POST,
			SmfPDUSessionOperation::RetrieveSmContext => reqwest::Method::POST,
			SmfPDUSessionOperation::UpdateSmContext => reqwest::Method::POST,
			SmfPDUSessionOperation::ReleaseSmContext => reqwest::Method::POST,
			SmfPDUSessionOperation::SendMoData => reqwest::Method::POST,
			SmfPDUSessionOperation::PostPduSessions => reqwest::Method::POST,
			SmfPDUSessionOperation::UpdatePduSession => reqwest::Method::POST,
			SmfPDUSessionOperation::ReleasePduSession => reqwest::Method::POST,
			SmfPDUSessionOperation::RetrievePduSession => reqwest::Method::POST,
			SmfPDUSessionOperation::TransferMoData => reqwest::Method::POST,
		}
	}
}

pub enum SmfEventExposureOperation {
	CreateIndividualSubcription,
	GetIndividualSubcription,
	ReplaceIndividualSubcription,
	DeleteIndividualSubcription,
}

impl super::ServiceProperties for SmfEventExposureOperation {
	fn get_path(&self) -> String {
		match self {
			SmfEventExposureOperation::CreateIndividualSubcription => "subscriptions".to_string(),
			SmfEventExposureOperation::GetIndividualSubcription => "subscriptions/{}".to_string(),
			SmfEventExposureOperation::ReplaceIndividualSubcription => {
				"subscriptions/{}".to_string()
			}
			SmfEventExposureOperation::DeleteIndividualSubcription => {
				"subscriptions/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			SmfEventExposureOperation::CreateIndividualSubcription => reqwest::Method::POST,
			SmfEventExposureOperation::GetIndividualSubcription => reqwest::Method::GET,
			SmfEventExposureOperation::ReplaceIndividualSubcription => reqwest::Method::PUT,
			SmfEventExposureOperation::DeleteIndividualSubcription => reqwest::Method::DELETE,
		}
	}
}

pub enum SmfNIDDOperation {
	Deliver,
}

impl super::ServiceProperties for SmfNIDDOperation {
	fn get_path(&self) -> String {
		match self {
			SmfNIDDOperation::Deliver => "pdu-sessions/{}/deliver".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			SmfNIDDOperation::Deliver => reqwest::Method::POST,
		}
	}
}
