pub enum SmfService {
	PDUSession(SmfPDUSessionOperation),
	EventExposure(SmfEventExposureOperation),
	NIDD(SmfNIDDOperation),
}

impl super::ServiceProperties for SmfService {
	fn get_path(&self) -> String {
		match self {
			SmfService::PDUSession(inner) => format!("/pdusession/{}", inner.get_path()),
			SmfService::EventExposure(inner) => format!("/eventexposure/{}", inner.get_path()),
			SmfService::NIDD(inner) => format!("/nidd/{}", inner.get_path()),
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
	Postsmcontexts,
	Retrievesmcontext,
	Updatesmcontext,
	Releasesmcontext,
	Sendmodata,
	Postpdusessions,
	Updatepdusession,
	Releasepdusession,
	Retrievepdusession,
	Transfermodata,
}

impl super::ServiceProperties for SmfPDUSessionOperation {
	fn get_path(&self) -> String {
		match self {
			SmfPDUSessionOperation::Postsmcontexts => "/sm-contexts".to_string(),
			SmfPDUSessionOperation::Retrievesmcontext => "/sm-contexts/{}/retrieve".to_string(),
			SmfPDUSessionOperation::Updatesmcontext => "/sm-contexts/{}/modify".to_string(),
			SmfPDUSessionOperation::Releasesmcontext => "/sm-contexts/{}/release".to_string(),
			SmfPDUSessionOperation::Sendmodata => "/sm-contexts/{}/send-mo-data".to_string(),
			SmfPDUSessionOperation::Postpdusessions => "/pdu-sessions".to_string(),
			SmfPDUSessionOperation::Updatepdusession => "/pdu-sessions/{}/modify".to_string(),
			SmfPDUSessionOperation::Releasepdusession => "/pdu-sessions/{}/release".to_string(),
			SmfPDUSessionOperation::Retrievepdusession => "/pdu-sessions/{}/retrieve".to_string(),
			SmfPDUSessionOperation::Transfermodata => {
				"/pdu-sessions/{}/transfer-mo-data".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			SmfPDUSessionOperation::Postsmcontexts => reqwest::Method::POST,
			SmfPDUSessionOperation::Retrievesmcontext => reqwest::Method::POST,
			SmfPDUSessionOperation::Updatesmcontext => reqwest::Method::POST,
			SmfPDUSessionOperation::Releasesmcontext => reqwest::Method::POST,
			SmfPDUSessionOperation::Sendmodata => reqwest::Method::POST,
			SmfPDUSessionOperation::Postpdusessions => reqwest::Method::POST,
			SmfPDUSessionOperation::Updatepdusession => reqwest::Method::POST,
			SmfPDUSessionOperation::Releasepdusession => reqwest::Method::POST,
			SmfPDUSessionOperation::Retrievepdusession => reqwest::Method::POST,
			SmfPDUSessionOperation::Transfermodata => reqwest::Method::POST,
		}
	}
}

pub enum SmfEventExposureOperation {
	Createindividualsubcription,
	Getindividualsubcription,
	Replaceindividualsubcription,
	Deleteindividualsubcription,
}

impl super::ServiceProperties for SmfEventExposureOperation {
	fn get_path(&self) -> String {
		match self {
			SmfEventExposureOperation::Createindividualsubcription => "/subscriptions".to_string(),
			SmfEventExposureOperation::Getindividualsubcription => "/subscriptions/{}".to_string(),
			SmfEventExposureOperation::Replaceindividualsubcription => {
				"/subscriptions/{}".to_string()
			}
			SmfEventExposureOperation::Deleteindividualsubcription => {
				"/subscriptions/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			SmfEventExposureOperation::Createindividualsubcription => reqwest::Method::POST,
			SmfEventExposureOperation::Getindividualsubcription => reqwest::Method::GET,
			SmfEventExposureOperation::Replaceindividualsubcription => reqwest::Method::PUT,
			SmfEventExposureOperation::Deleteindividualsubcription => reqwest::Method::DELETE,
		}
	}
}

pub enum SmfNIDDOperation {
	Deliver,
}

impl super::ServiceProperties for SmfNIDDOperation {
	fn get_path(&self) -> String {
		match self {
			SmfNIDDOperation::Deliver => "/pdu-sessions/{}/deliver".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			SmfNIDDOperation::Deliver => reqwest::Method::POST,
		}
	}
}
