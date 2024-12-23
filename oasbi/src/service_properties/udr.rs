pub enum UdrService {
	GroupIDmap(UdrGroupIDmapOperation),
}

impl super::ServiceProperties for UdrService {
	fn get_path(&self) -> String {
		match self {
			UdrService::GroupIDmap(inner) => format!("/nudr-group-id-map/v1/{}", inner.get_path()),
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdrService::GroupIDmap(inner) => inner.get_http_method(),
		}
	}
}

pub enum UdrGroupIDmapOperation {
	GetNfGroupIDs,
}

impl super::ServiceProperties for UdrGroupIDmapOperation {
	fn get_path(&self) -> String {
		match self {
			UdrGroupIDmapOperation::GetNfGroupIDs => "nf-group-ids".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdrGroupIDmapOperation::GetNfGroupIDs => reqwest::Method::GET,
		}
	}
}
