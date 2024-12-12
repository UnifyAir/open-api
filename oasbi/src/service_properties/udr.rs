pub enum UdrService {
	GroupIDmap(UdrGroupIDmapOperation),
}

impl super::ServiceProperties for UdrService {
	fn get_path(&self) -> String {
		match self {
			UdrService::GroupIDmap(inner) => format!("/groupidmap/{}", inner.get_path()),
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdrService::GroupIDmap(inner) => inner.get_http_method(),
		}
	}
}

pub enum UdrGroupIDmapOperation {
	Getnfgroupids,
}

impl super::ServiceProperties for UdrGroupIDmapOperation {
	fn get_path(&self) -> String {
		match self {
			UdrGroupIDmapOperation::Getnfgroupids => "/nf-group-ids".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdrGroupIDmapOperation::Getnfgroupids => reqwest::Method::GET,
		}
	}
}
