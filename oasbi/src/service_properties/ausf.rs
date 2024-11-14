pub enum AusfService {
    SoRProtection(AusfSoRProtectionOperation),
    UEAuthentication(AusfUEAuthenticationOperation),
    UPUProtection(AusfUPUProtectionOperation),
}

impl super::ServiceProperties for AusfService {
    fn get_path(&self) -> String {
        match self {
            AusfService::SoRProtection(inner) => format!("/sorprotection/{}", inner.get_path()),
            AusfService::UEAuthentication(inner) => format!("/ueauthentication/{}", inner.get_path()),
            AusfService::UPUProtection(inner) => format!("/upuprotection/{}", inner.get_path()),
        }
    }
    fn get_http_method(&self) -> reqwest::Method {
        match self {
            AusfService::SoRProtection(inner) => inner.get_http_method(),
            AusfService::UEAuthentication(inner) => inner.get_http_method(),
            AusfService::UPUProtection(inner) => inner.get_http_method(),
        }
    }
}


pub enum AusfSoRProtectionOperation {
    Postuesor,
}

impl super::ServiceProperties for AusfSoRProtectionOperation {
    fn get_path(&self) -> String {
        match self {
            AusfSoRProtectionOperation::Postuesor => "/{}/ue-sor".to_string(),
        }
    }

    fn get_http_method(&self) -> reqwest::Method {
        match self {
            AusfSoRProtectionOperation::Postuesor => reqwest::Method::POST,
        }
    }
}


pub enum AusfUEAuthenticationOperation {
    Postueauthentications,
    Postueauthenticationsderegister,
    Putueauthentications5gakaconfirmation,
    Delete5gakaauthenticationresult,
    Eapauthmethod,
    Deleteeapauthenticationresult,
    Postrgauthentications,
    Postproseauthentications,
    Proseauth,
    Deleteproseauthenticationresult,
}

impl super::ServiceProperties for AusfUEAuthenticationOperation {
    fn get_path(&self) -> String {
        match self {
            AusfUEAuthenticationOperation::Postueauthentications => "/ue-authentications".to_string(),
            AusfUEAuthenticationOperation::Postueauthenticationsderegister => "/ue-authentications/deregister".to_string(),
            AusfUEAuthenticationOperation::Putueauthentications5gakaconfirmation => "/ue-authentications/{}/5g-aka-confirmation".to_string(),
            AusfUEAuthenticationOperation::Delete5gakaauthenticationresult => "/ue-authentications/{}/5g-aka-confirmation".to_string(),
            AusfUEAuthenticationOperation::Eapauthmethod => "/ue-authentications/{}/eap-session".to_string(),
            AusfUEAuthenticationOperation::Deleteeapauthenticationresult => "/ue-authentications/{}/eap-session".to_string(),
            AusfUEAuthenticationOperation::Postrgauthentications => "/rg-authentications".to_string(),
            AusfUEAuthenticationOperation::Postproseauthentications => "/prose-authentications".to_string(),
            AusfUEAuthenticationOperation::Proseauth => "/prose-authentications/{}/prose-auth".to_string(),
            AusfUEAuthenticationOperation::Deleteproseauthenticationresult => "/prose-authentications/{}/prose-auth".to_string(),
        }
    }

    fn get_http_method(&self) -> reqwest::Method {
        match self {
            AusfUEAuthenticationOperation::Postueauthentications => reqwest::Method::POST,
            AusfUEAuthenticationOperation::Postueauthenticationsderegister => reqwest::Method::POST,
            AusfUEAuthenticationOperation::Putueauthentications5gakaconfirmation => reqwest::Method::PUT,
            AusfUEAuthenticationOperation::Delete5gakaauthenticationresult => reqwest::Method::DELETE,
            AusfUEAuthenticationOperation::Eapauthmethod => reqwest::Method::POST,
            AusfUEAuthenticationOperation::Deleteeapauthenticationresult => reqwest::Method::DELETE,
            AusfUEAuthenticationOperation::Postrgauthentications => reqwest::Method::POST,
            AusfUEAuthenticationOperation::Postproseauthentications => reqwest::Method::POST,
            AusfUEAuthenticationOperation::Proseauth => reqwest::Method::POST,
            AusfUEAuthenticationOperation::Deleteproseauthenticationresult => reqwest::Method::DELETE,
        }
    }
}


pub enum AusfUPUProtectionOperation {
    Postueupu,
}

impl super::ServiceProperties for AusfUPUProtectionOperation {
    fn get_path(&self) -> String {
        match self {
            AusfUPUProtectionOperation::Postueupu => "/{}/ue-upu".to_string(),
        }
    }

    fn get_http_method(&self) -> reqwest::Method {
        match self {
            AusfUPUProtectionOperation::Postueupu => reqwest::Method::POST,
        }
    }
}


