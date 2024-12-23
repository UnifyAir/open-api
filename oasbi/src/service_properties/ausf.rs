pub enum AusfService {
	SoRProtection(AusfSoRProtectionOperation),
	UEAuthentication(AusfUEAuthenticationOperation),
	UPUProtection(AusfUPUProtectionOperation),
}

impl super::ServiceProperties for AusfService {
	fn get_path(&self) -> String {
		match self {
			AusfService::SoRProtection(inner) => {
				format!("/nausf-sorprotection/v1/{}", inner.get_path())
			}
			AusfService::UEAuthentication(inner) => format!("/nausf-auth/v1/{}", inner.get_path()),
			AusfService::UPUProtection(inner) => {
				format!("/nausf-upuprotection/v1/{}", inner.get_path())
			}
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
	CreateSoRProtection,
}

impl super::ServiceProperties for AusfSoRProtectionOperation {
	fn get_path(&self) -> String {
		match self {
			AusfSoRProtectionOperation::CreateSoRProtection => "{}/ue-sor".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AusfSoRProtectionOperation::CreateSoRProtection => reqwest::Method::POST,
		}
	}
}

pub enum AusfUEAuthenticationOperation {
	CreateUEAuthentication,
	DeregisterUE,
	Create5gAkaAuthenticationResult,
	Delete5gAkaAuthenticationResult,
	EapAuthMethod,
	DeleteEapAuthenticationResult,
	CreateRgAuthentication,
	CreateProSeEapSession,
	ProseAuth,
	DeleteProSeAuthenticationResult,
}

impl super::ServiceProperties for AusfUEAuthenticationOperation {
	fn get_path(&self) -> String {
		match self {
			AusfUEAuthenticationOperation::CreateUEAuthentication => {
				"ue-authentications".to_string()
			}
			AusfUEAuthenticationOperation::DeregisterUE => {
				"ue-authentications/deregister".to_string()
			}
			AusfUEAuthenticationOperation::Create5gAkaAuthenticationResult => {
				"ue-authentications/{}/5g-aka-confirmation".to_string()
			}
			AusfUEAuthenticationOperation::Delete5gAkaAuthenticationResult => {
				"ue-authentications/{}/5g-aka-confirmation".to_string()
			}
			AusfUEAuthenticationOperation::EapAuthMethod => {
				"ue-authentications/{}/eap-session".to_string()
			}
			AusfUEAuthenticationOperation::DeleteEapAuthenticationResult => {
				"ue-authentications/{}/eap-session".to_string()
			}
			AusfUEAuthenticationOperation::CreateRgAuthentication => {
				"rg-authentications".to_string()
			}
			AusfUEAuthenticationOperation::CreateProSeEapSession => {
				"prose-authentications".to_string()
			}
			AusfUEAuthenticationOperation::ProseAuth => {
				"prose-authentications/{}/prose-auth".to_string()
			}
			AusfUEAuthenticationOperation::DeleteProSeAuthenticationResult => {
				"prose-authentications/{}/prose-auth".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AusfUEAuthenticationOperation::CreateUEAuthentication => reqwest::Method::POST,
			AusfUEAuthenticationOperation::DeregisterUE => reqwest::Method::POST,
			AusfUEAuthenticationOperation::Create5gAkaAuthenticationResult => reqwest::Method::PUT,
			AusfUEAuthenticationOperation::Delete5gAkaAuthenticationResult => {
				reqwest::Method::DELETE
			}
			AusfUEAuthenticationOperation::EapAuthMethod => reqwest::Method::POST,
			AusfUEAuthenticationOperation::DeleteEapAuthenticationResult => reqwest::Method::DELETE,
			AusfUEAuthenticationOperation::CreateRgAuthentication => reqwest::Method::POST,
			AusfUEAuthenticationOperation::CreateProSeEapSession => reqwest::Method::POST,
			AusfUEAuthenticationOperation::ProseAuth => reqwest::Method::POST,
			AusfUEAuthenticationOperation::DeleteProSeAuthenticationResult => {
				reqwest::Method::DELETE
			}
		}
	}
}

pub enum AusfUPUProtectionOperation {
	CreateUPUProtection,
}

impl super::ServiceProperties for AusfUPUProtectionOperation {
	fn get_path(&self) -> String {
		match self {
			AusfUPUProtectionOperation::CreateUPUProtection => "{}/ue-upu".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			AusfUPUProtectionOperation::CreateUPUProtection => reqwest::Method::POST,
		}
	}
}
