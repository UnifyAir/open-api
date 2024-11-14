pub enum NrfService {
    AccessToken(NrfAccessTokenOperation),
    Bootstrapping(NrfBootstrappingOperation),
    NFDiscovery(NrfNFDiscoveryOperation),
    NFManagement(NrfNFManagementOperation),
}

impl super::ServiceProperties for NrfService {
    fn get_path(&self) -> String {
        match self {
            NrfService::AccessToken(inner) => format!("/accesstoken/{}", inner.get_path()),
            NrfService::Bootstrapping(inner) => format!("/bootstrapping/{}", inner.get_path()),
            NrfService::NFDiscovery(inner) => format!("/nfdiscovery/{}", inner.get_path()),
            NrfService::NFManagement(inner) => format!("/nfmanagement/{}", inner.get_path()),
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
    Accesstokenrequest,
}

impl super::ServiceProperties for NrfAccessTokenOperation {
    fn get_path(&self) -> String {
        match self {
            NrfAccessTokenOperation::Accesstokenrequest => "/oauth2/token".to_string(),
        }
    }

    fn get_http_method(&self) -> reqwest::Method {
        match self {
            NrfAccessTokenOperation::Accesstokenrequest => reqwest::Method::POST,
        }
    }
}


pub enum NrfBootstrappingOperation {
    Bootstrappinginforequest,
}

impl super::ServiceProperties for NrfBootstrappingOperation {
    fn get_path(&self) -> String {
        match self {
            NrfBootstrappingOperation::Bootstrappinginforequest => "/bootstrapping".to_string(),
        }
    }

    fn get_http_method(&self) -> reqwest::Method {
        match self {
            NrfBootstrappingOperation::Bootstrappinginforequest => reqwest::Method::GET,
        }
    }
}


pub enum NrfNFDiscoveryOperation {
    Searchnfinstances,
    Retrievestoredsearch,
    Retrievecompletesearch,
    Scpdomainroutinginfoget,
    Scpdomainroutinginfosubscribe,
    Scpdomainroutinginfounsubscribe,
}

impl super::ServiceProperties for NrfNFDiscoveryOperation {
    fn get_path(&self) -> String {
        match self {
            NrfNFDiscoveryOperation::Searchnfinstances => "/nf-instances".to_string(),
            NrfNFDiscoveryOperation::Retrievestoredsearch => "/searches/{}".to_string(),
            NrfNFDiscoveryOperation::Retrievecompletesearch => "/searches/{}/complete".to_string(),
            NrfNFDiscoveryOperation::Scpdomainroutinginfoget => "/scp-domain-routing-info".to_string(),
            NrfNFDiscoveryOperation::Scpdomainroutinginfosubscribe => "/scp-domain-routing-info-subs".to_string(),
            NrfNFDiscoveryOperation::Scpdomainroutinginfounsubscribe => "/scp-domain-routing-info-subs/{}".to_string(),
        }
    }

    fn get_http_method(&self) -> reqwest::Method {
        match self {
            NrfNFDiscoveryOperation::Searchnfinstances => reqwest::Method::GET,
            NrfNFDiscoveryOperation::Retrievestoredsearch => reqwest::Method::GET,
            NrfNFDiscoveryOperation::Retrievecompletesearch => reqwest::Method::GET,
            NrfNFDiscoveryOperation::Scpdomainroutinginfoget => reqwest::Method::GET,
            NrfNFDiscoveryOperation::Scpdomainroutinginfosubscribe => reqwest::Method::POST,
            NrfNFDiscoveryOperation::Scpdomainroutinginfounsubscribe => reqwest::Method::DELETE,
        }
    }
}


pub enum NrfNFManagementOperation {
    Getnfinstances,
    Optionsnfinstances,
    Getnfinstance,
    Registernfinstance,
    Updatenfinstance,
    Deregisternfinstance,
    Createsubscription,
    Updatesubscription,
    Removesubscription,
}

impl super::ServiceProperties for NrfNFManagementOperation {
    fn get_path(&self) -> String {
        match self {
            NrfNFManagementOperation::Getnfinstances => "/nf-instances".to_string(),
            NrfNFManagementOperation::Optionsnfinstances => "/nf-instances".to_string(),
            NrfNFManagementOperation::Getnfinstance => "/nf-instances/{}".to_string(),
            NrfNFManagementOperation::Registernfinstance => "/nf-instances/{}".to_string(),
            NrfNFManagementOperation::Updatenfinstance => "/nf-instances/{}".to_string(),
            NrfNFManagementOperation::Deregisternfinstance => "/nf-instances/{}".to_string(),
            NrfNFManagementOperation::Createsubscription => "/subscriptions".to_string(),
            NrfNFManagementOperation::Updatesubscription => "/subscriptions/{}".to_string(),
            NrfNFManagementOperation::Removesubscription => "/subscriptions/{}".to_string(),
        }
    }

    fn get_http_method(&self) -> reqwest::Method {
        match self {
            NrfNFManagementOperation::Getnfinstances => reqwest::Method::GET,
            NrfNFManagementOperation::Optionsnfinstances => reqwest::Method::OPTIONS,
            NrfNFManagementOperation::Getnfinstance => reqwest::Method::GET,
            NrfNFManagementOperation::Registernfinstance => reqwest::Method::PUT,
            NrfNFManagementOperation::Updatenfinstance => reqwest::Method::PATCH,
            NrfNFManagementOperation::Deregisternfinstance => reqwest::Method::DELETE,
            NrfNFManagementOperation::Createsubscription => reqwest::Method::POST,
            NrfNFManagementOperation::Updatesubscription => reqwest::Method::PATCH,
            NrfNFManagementOperation::Removesubscription => reqwest::Method::DELETE,
        }
    }
}


