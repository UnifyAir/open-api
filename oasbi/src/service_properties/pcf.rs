pub enum PcfService {
	AMPolicyControl(PcfAMPolicyControlOperation),
	SMPolicyControl(PcfSMPolicyControlOperation),
	PolicyAuthorization(PcfPolicyAuthorizationOperation),
	EventExposure(PcfEventExposureOperation),
	UEPolicyControl(PcfUEPolicyControlOperation),
	AMPolicyAuthorization(PcfAMPolicyAuthorizationOperation),
	MBSPolicyAuthorization(PcfMBSPolicyAuthorizationOperation),
	MBSPolicyControl(PcfMBSPolicyControlOperation),
	BDTPolicyControl(PcfBDTPolicyControlOperation),
}

impl super::ServiceProperties for PcfService {
	fn get_path(&self) -> String {
		match self {
			PcfService::AMPolicyControl(inner) => {
				format!("/npcf-am-policy-control/v1/{}", inner.get_path())
			}
			PcfService::SMPolicyControl(inner) => {
				format!("/npcf-smpolicycontrol/v1/{}", inner.get_path())
			}
			PcfService::PolicyAuthorization(inner) => {
				format!("/npcf-policyauthorization/v1/{}", inner.get_path())
			}
			PcfService::EventExposure(inner) => {
				format!("/npcf-eventexposure/v1/{}", inner.get_path())
			}
			PcfService::UEPolicyControl(inner) => {
				format!("/npcf-ue-policy-control/v1/{}", inner.get_path())
			}
			PcfService::AMPolicyAuthorization(inner) => {
				format!("/npcf-am-policyauthorization/v1/{}", inner.get_path())
			}
			PcfService::MBSPolicyAuthorization(inner) => {
				format!("/npcf-mbspolicyauth/v1/{}", inner.get_path())
			}
			PcfService::MBSPolicyControl(inner) => {
				format!("/npcf-mbspolicycontrol/v1/{}", inner.get_path())
			}
			PcfService::BDTPolicyControl(inner) => {
				format!("/npcf-bdtpolicycontrol/v1/{}", inner.get_path())
			}
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfService::AMPolicyControl(inner) => inner.get_http_method(),
			PcfService::SMPolicyControl(inner) => inner.get_http_method(),
			PcfService::PolicyAuthorization(inner) => inner.get_http_method(),
			PcfService::EventExposure(inner) => inner.get_http_method(),
			PcfService::UEPolicyControl(inner) => inner.get_http_method(),
			PcfService::AMPolicyAuthorization(inner) => inner.get_http_method(),
			PcfService::MBSPolicyAuthorization(inner) => inner.get_http_method(),
			PcfService::MBSPolicyControl(inner) => inner.get_http_method(),
			PcfService::BDTPolicyControl(inner) => inner.get_http_method(),
		}
	}
}

pub enum PcfAMPolicyControlOperation {
	CreateIndividualAMPolicyAssociation,
	ReadIndividualAMPolicyAssociation,
	DeleteIndividualAMPolicyAssociation,
	ReportObservedEventTriggersForIndividualAMPolicyAssociation,
}

impl super::ServiceProperties for PcfAMPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
            PcfAMPolicyControlOperation::CreateIndividualAMPolicyAssociation => "policies".to_string(),
            PcfAMPolicyControlOperation::ReadIndividualAMPolicyAssociation => "policies/{}".to_string(),
            PcfAMPolicyControlOperation::DeleteIndividualAMPolicyAssociation => "policies/{}".to_string(),
            PcfAMPolicyControlOperation::ReportObservedEventTriggersForIndividualAMPolicyAssociation => "policies/{}/update".to_string(),
        }
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
            PcfAMPolicyControlOperation::CreateIndividualAMPolicyAssociation => reqwest::Method::POST,
            PcfAMPolicyControlOperation::ReadIndividualAMPolicyAssociation => reqwest::Method::GET,
            PcfAMPolicyControlOperation::DeleteIndividualAMPolicyAssociation => reqwest::Method::DELETE,
            PcfAMPolicyControlOperation::ReportObservedEventTriggersForIndividualAMPolicyAssociation => reqwest::Method::POST,
        }
	}
}

pub enum PcfSMPolicyControlOperation {
	CreateSMPolicy,
	GetSMPolicy,
	UpdateSMPolicy,
	DeleteSMPolicy,
}

impl super::ServiceProperties for PcfSMPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
			PcfSMPolicyControlOperation::CreateSMPolicy => "sm-policies".to_string(),
			PcfSMPolicyControlOperation::GetSMPolicy => "sm-policies/{}".to_string(),
			PcfSMPolicyControlOperation::UpdateSMPolicy => "sm-policies/{}/update".to_string(),
			PcfSMPolicyControlOperation::DeleteSMPolicy => "sm-policies/{}/delete".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfSMPolicyControlOperation::CreateSMPolicy => reqwest::Method::POST,
			PcfSMPolicyControlOperation::GetSMPolicy => reqwest::Method::GET,
			PcfSMPolicyControlOperation::UpdateSMPolicy => reqwest::Method::POST,
			PcfSMPolicyControlOperation::DeleteSMPolicy => reqwest::Method::POST,
		}
	}
}

pub enum PcfPolicyAuthorizationOperation {
	PostAppSessions,
	PcscfRestoration,
	GetAppSession,
	ModAppSession,
	DeleteAppSession,
	UpdateEventsSubsc,
	DeleteEventsSubsc,
}

impl super::ServiceProperties for PcfPolicyAuthorizationOperation {
	fn get_path(&self) -> String {
		match self {
			PcfPolicyAuthorizationOperation::PostAppSessions => "app-sessions".to_string(),
			PcfPolicyAuthorizationOperation::PcscfRestoration => {
				"app-sessions/pcscf-restoration".to_string()
			}
			PcfPolicyAuthorizationOperation::GetAppSession => "app-sessions/{}".to_string(),
			PcfPolicyAuthorizationOperation::ModAppSession => "app-sessions/{}".to_string(),
			PcfPolicyAuthorizationOperation::DeleteAppSession => {
				"app-sessions/{}/delete".to_string()
			}
			PcfPolicyAuthorizationOperation::UpdateEventsSubsc => {
				"app-sessions/{}/events-subscription".to_string()
			}
			PcfPolicyAuthorizationOperation::DeleteEventsSubsc => {
				"app-sessions/{}/events-subscription".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfPolicyAuthorizationOperation::PostAppSessions => reqwest::Method::POST,
			PcfPolicyAuthorizationOperation::PcscfRestoration => reqwest::Method::POST,
			PcfPolicyAuthorizationOperation::GetAppSession => reqwest::Method::GET,
			PcfPolicyAuthorizationOperation::ModAppSession => reqwest::Method::PATCH,
			PcfPolicyAuthorizationOperation::DeleteAppSession => reqwest::Method::POST,
			PcfPolicyAuthorizationOperation::UpdateEventsSubsc => reqwest::Method::PUT,
			PcfPolicyAuthorizationOperation::DeleteEventsSubsc => reqwest::Method::DELETE,
		}
	}
}

pub enum PcfEventExposureOperation {
	PostPcEventExposureSubsc,
	GetPcEventExposureSubsc,
	PutPcEventExposureSubsc,
	DeletePcEventExposureSubsc,
}

impl super::ServiceProperties for PcfEventExposureOperation {
	fn get_path(&self) -> String {
		match self {
			PcfEventExposureOperation::PostPcEventExposureSubsc => "subscriptions".to_string(),
			PcfEventExposureOperation::GetPcEventExposureSubsc => "subscriptions/{}".to_string(),
			PcfEventExposureOperation::PutPcEventExposureSubsc => "subscriptions/{}".to_string(),
			PcfEventExposureOperation::DeletePcEventExposureSubsc => "subscriptions/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfEventExposureOperation::PostPcEventExposureSubsc => reqwest::Method::POST,
			PcfEventExposureOperation::GetPcEventExposureSubsc => reqwest::Method::GET,
			PcfEventExposureOperation::PutPcEventExposureSubsc => reqwest::Method::PUT,
			PcfEventExposureOperation::DeletePcEventExposureSubsc => reqwest::Method::DELETE,
		}
	}
}

pub enum PcfUEPolicyControlOperation {
	CreateIndividualUEPolicyAssociation,
	ReadIndividualUEPolicyAssociation,
	DeleteIndividualUEPolicyAssociation,
	ReportObservedEventTriggersForIndividualUEPolicyAssociation,
}

impl super::ServiceProperties for PcfUEPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
            PcfUEPolicyControlOperation::CreateIndividualUEPolicyAssociation => "policies".to_string(),
            PcfUEPolicyControlOperation::ReadIndividualUEPolicyAssociation => "policies/{}".to_string(),
            PcfUEPolicyControlOperation::DeleteIndividualUEPolicyAssociation => "policies/{}".to_string(),
            PcfUEPolicyControlOperation::ReportObservedEventTriggersForIndividualUEPolicyAssociation => "policies/{}/update".to_string(),
        }
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
            PcfUEPolicyControlOperation::CreateIndividualUEPolicyAssociation => reqwest::Method::POST,
            PcfUEPolicyControlOperation::ReadIndividualUEPolicyAssociation => reqwest::Method::GET,
            PcfUEPolicyControlOperation::DeleteIndividualUEPolicyAssociation => reqwest::Method::DELETE,
            PcfUEPolicyControlOperation::ReportObservedEventTriggersForIndividualUEPolicyAssociation => reqwest::Method::POST,
        }
	}
}

pub enum PcfAMPolicyAuthorizationOperation {
	PostAppAmContexts,
	GetAppAmContext,
	ModAppAmContext,
	DeleteAppAmContext,
	UpdateAmEventsSubsc,
	DeleteAmEventsSubsc,
}

impl super::ServiceProperties for PcfAMPolicyAuthorizationOperation {
	fn get_path(&self) -> String {
		match self {
			PcfAMPolicyAuthorizationOperation::PostAppAmContexts => "app-am-contexts".to_string(),
			PcfAMPolicyAuthorizationOperation::GetAppAmContext => "app-am-contexts/{}".to_string(),
			PcfAMPolicyAuthorizationOperation::ModAppAmContext => "app-am-contexts/{}".to_string(),
			PcfAMPolicyAuthorizationOperation::DeleteAppAmContext => {
				"app-am-contexts/{}".to_string()
			}
			PcfAMPolicyAuthorizationOperation::UpdateAmEventsSubsc => {
				"app-am-contexts/{}/events-subscription".to_string()
			}
			PcfAMPolicyAuthorizationOperation::DeleteAmEventsSubsc => {
				"app-am-contexts/{}/events-subscription".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfAMPolicyAuthorizationOperation::PostAppAmContexts => reqwest::Method::POST,
			PcfAMPolicyAuthorizationOperation::GetAppAmContext => reqwest::Method::GET,
			PcfAMPolicyAuthorizationOperation::ModAppAmContext => reqwest::Method::PATCH,
			PcfAMPolicyAuthorizationOperation::DeleteAppAmContext => reqwest::Method::DELETE,
			PcfAMPolicyAuthorizationOperation::UpdateAmEventsSubsc => reqwest::Method::PUT,
			PcfAMPolicyAuthorizationOperation::DeleteAmEventsSubsc => reqwest::Method::DELETE,
		}
	}
}

pub enum PcfMBSPolicyAuthorizationOperation {
	CreateMBSAppSessionCtxt,
	GetMBSAppSessionCtxt,
	ModifyMBSAppSessionCtxt,
	DeleteMBSAppSessionCtxt,
}

impl super::ServiceProperties for PcfMBSPolicyAuthorizationOperation {
	fn get_path(&self) -> String {
		match self {
			PcfMBSPolicyAuthorizationOperation::CreateMBSAppSessionCtxt => "contexts".to_string(),
			PcfMBSPolicyAuthorizationOperation::GetMBSAppSessionCtxt => "contexts/{}".to_string(),
			PcfMBSPolicyAuthorizationOperation::ModifyMBSAppSessionCtxt => {
				"contexts/{}".to_string()
			}
			PcfMBSPolicyAuthorizationOperation::DeleteMBSAppSessionCtxt => {
				"contexts/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfMBSPolicyAuthorizationOperation::CreateMBSAppSessionCtxt => reqwest::Method::POST,
			PcfMBSPolicyAuthorizationOperation::GetMBSAppSessionCtxt => reqwest::Method::GET,
			PcfMBSPolicyAuthorizationOperation::ModifyMBSAppSessionCtxt => reqwest::Method::PATCH,
			PcfMBSPolicyAuthorizationOperation::DeleteMBSAppSessionCtxt => reqwest::Method::DELETE,
		}
	}
}

pub enum PcfMBSPolicyControlOperation {
	CreateMBSPolicy,
	GetIndMBSPolicy,
	DeleteIndMBSPolicy,
	UpdateIndMBSPolicy,
}

impl super::ServiceProperties for PcfMBSPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
			PcfMBSPolicyControlOperation::CreateMBSPolicy => "mbs-policies".to_string(),
			PcfMBSPolicyControlOperation::GetIndMBSPolicy => "mbs-policies/{}".to_string(),
			PcfMBSPolicyControlOperation::DeleteIndMBSPolicy => "mbs-policies/{}".to_string(),
			PcfMBSPolicyControlOperation::UpdateIndMBSPolicy => {
				"mbs-policies/{}/update".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfMBSPolicyControlOperation::CreateMBSPolicy => reqwest::Method::POST,
			PcfMBSPolicyControlOperation::GetIndMBSPolicy => reqwest::Method::GET,
			PcfMBSPolicyControlOperation::DeleteIndMBSPolicy => reqwest::Method::DELETE,
			PcfMBSPolicyControlOperation::UpdateIndMBSPolicy => reqwest::Method::POST,
		}
	}
}

pub enum PcfBDTPolicyControlOperation {
	CreateBDTPolicy,
	GetBDTPolicy,
	UpdateBDTPolicy,
}

impl super::ServiceProperties for PcfBDTPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
			PcfBDTPolicyControlOperation::CreateBDTPolicy => "bdtpolicies".to_string(),
			PcfBDTPolicyControlOperation::GetBDTPolicy => "bdtpolicies/{}".to_string(),
			PcfBDTPolicyControlOperation::UpdateBDTPolicy => "bdtpolicies/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfBDTPolicyControlOperation::CreateBDTPolicy => reqwest::Method::POST,
			PcfBDTPolicyControlOperation::GetBDTPolicy => reqwest::Method::GET,
			PcfBDTPolicyControlOperation::UpdateBDTPolicy => reqwest::Method::PATCH,
		}
	}
}
