pub enum UpfService {
	EventExposure(UpfEventExposureOperation),
}

impl super::ServiceProperties for UpfService {
	fn get_path(&self) -> String {
		match self {
			UpfService::EventExposure(inner) => format!("/eventexposure/{}", inner.get_path()),
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UpfService::EventExposure(inner) => inner.get_http_method(),
		}
	}
}

pub enum UpfEventExposureOperation {
	Createindividualsubcription,
}

impl super::ServiceProperties for UpfEventExposureOperation {
	fn get_path(&self) -> String {
		match self {
			UpfEventExposureOperation::Createindividualsubcription => {
				"/ee-subscriptions".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UpfEventExposureOperation::Createindividualsubcription => reqwest::Method::POST,
		}
	}
}
