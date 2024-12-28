pub enum NrfService {
	AccessToken(NrfAccessTokenOperation),
	Bootstrapping(NrfBootstrappingOperation),
	NFDiscovery(NrfNFDiscoveryOperation),
	NFManagement(NrfNFManagementOperation),
}

impl super::ServiceProperties for NrfService {
	fn get_path(&self) -> String {
		match self {
			NrfService::AccessToken(inner) => format!("/{}", inner.get_path()),
			NrfService::Bootstrapping(inner) => format!("/{}", inner.get_path()),
			NrfService::NFDiscovery(inner) => format!("/nnrf-disc/v1/{}", inner.get_path()),
			NrfService::NFManagement(inner) => format!("/nnrf-nfm/v1/{}", inner.get_path()),
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NrfService::AccessToken(inner) => inner.get_http_method(),
			NrfService::Bootstrapping(inner) => inner.get_http_method(),
			NrfService::NFDiscovery(inner) => inner.get_http_method(),
			NrfService::NFManagement(inner) => inner.get_http_method(),
		}
	}
}

pub enum NrfAccessTokenOperation {
	AccessTokenRequest,
}

impl super::ServiceProperties for NrfAccessTokenOperation {
	fn get_path(&self) -> String {
		match self {
			NrfAccessTokenOperation::AccessTokenRequest => "oauth2/token".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NrfAccessTokenOperation::AccessTokenRequest => reqwest::Method::POST,
		}
	}
}

pub enum NrfBootstrappingOperation {
	BootstrappingInfoRequest,
}

impl super::ServiceProperties for NrfBootstrappingOperation {
	fn get_path(&self) -> String {
		match self {
			NrfBootstrappingOperation::BootstrappingInfoRequest => "bootstrapping".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NrfBootstrappingOperation::BootstrappingInfoRequest => reqwest::Method::GET,
		}
	}
}

pub enum NrfNFDiscoveryOperation {
	SearchNFInstances,
	RetrieveStoredSearch,
	RetrieveCompleteSearch,
	SCPDomainRoutingInfoGet,
	ScpDomainRoutingInfoSubscribe,
	ScpDomainRoutingInfoUnsubscribe,
}

impl super::ServiceProperties for NrfNFDiscoveryOperation {
	fn get_path(&self) -> String {
		match self {
			NrfNFDiscoveryOperation::SearchNFInstances => "nf-instances".to_string(),
			NrfNFDiscoveryOperation::RetrieveStoredSearch => "searches/{}".to_string(),
			NrfNFDiscoveryOperation::RetrieveCompleteSearch => "searches/{}/complete".to_string(),
			NrfNFDiscoveryOperation::SCPDomainRoutingInfoGet => {
				"scp-domain-routing-info".to_string()
			}
			NrfNFDiscoveryOperation::ScpDomainRoutingInfoSubscribe => {
				"scp-domain-routing-info-subs".to_string()
			}
			NrfNFDiscoveryOperation::ScpDomainRoutingInfoUnsubscribe => {
				"scp-domain-routing-info-subs/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NrfNFDiscoveryOperation::SearchNFInstances => reqwest::Method::GET,
			NrfNFDiscoveryOperation::RetrieveStoredSearch => reqwest::Method::GET,
			NrfNFDiscoveryOperation::RetrieveCompleteSearch => reqwest::Method::GET,
			NrfNFDiscoveryOperation::SCPDomainRoutingInfoGet => reqwest::Method::GET,
			NrfNFDiscoveryOperation::ScpDomainRoutingInfoSubscribe => reqwest::Method::POST,
			NrfNFDiscoveryOperation::ScpDomainRoutingInfoUnsubscribe => reqwest::Method::DELETE,
		}
	}
}

pub enum NrfNFManagementOperation {
	GetNFInstances,
	OptionsNFInstances,
	GetNFInstance,
	RegisterNFInstance,
	UpdateNFInstance,
	DeregisterNFInstance,
	CreateSubscription,
	UpdateSubscription,
	RemoveSubscription,
}

impl super::ServiceProperties for NrfNFManagementOperation {
	fn get_path(&self) -> String {
		match self {
			NrfNFManagementOperation::GetNFInstances => "nf-instances".to_string(),
			NrfNFManagementOperation::OptionsNFInstances => "nf-instances".to_string(),
			NrfNFManagementOperation::GetNFInstance => "nf-instances/{}".to_string(),
			NrfNFManagementOperation::RegisterNFInstance => "nf-instances/{}".to_string(),
			NrfNFManagementOperation::UpdateNFInstance => "nf-instances/{}".to_string(),
			NrfNFManagementOperation::DeregisterNFInstance => "nf-instances/{}".to_string(),
			NrfNFManagementOperation::CreateSubscription => "subscriptions".to_string(),
			NrfNFManagementOperation::UpdateSubscription => "subscriptions/{}".to_string(),
			NrfNFManagementOperation::RemoveSubscription => "subscriptions/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			NrfNFManagementOperation::GetNFInstances => reqwest::Method::GET,
			NrfNFManagementOperation::OptionsNFInstances => reqwest::Method::OPTIONS,
			NrfNFManagementOperation::GetNFInstance => reqwest::Method::GET,
			NrfNFManagementOperation::RegisterNFInstance => reqwest::Method::PUT,
			NrfNFManagementOperation::UpdateNFInstance => reqwest::Method::PATCH,
			NrfNFManagementOperation::DeregisterNFInstance => reqwest::Method::DELETE,
			NrfNFManagementOperation::CreateSubscription => reqwest::Method::POST,
			NrfNFManagementOperation::UpdateSubscription => reqwest::Method::PATCH,
			NrfNFManagementOperation::RemoveSubscription => reqwest::Method::DELETE,
		}
	}
}
