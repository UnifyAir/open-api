
#[derive(
	::serde::Deserialize,
	::serde::Serialize,
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	smart_default::SmartDefault,
)]
pub enum PcfServiceName {
    #[default]
    #[serde(rename = "npcf_ampolicycontrol")]
    AMPolicyControl,
    #[serde(rename = "npcf_smpolicycontrol")]
    SMPolicyControl,
    #[serde(rename = "npcf_policyauthorization")]
    PolicyAuthorization,
    #[serde(rename = "npcf_eventexposure")]
    EventExposure,
    #[serde(rename = "npcf_uepolicycontrol")]
    UEPolicyControl,
    #[serde(rename = "npcf_ampolicyauthorization")]
    AMPolicyAuthorization,
    #[serde(rename = "npcf_mbspolicyauthorization")]
    MBSPolicyAuthorization,
    #[serde(rename = "npcf_mbspolicycontrol")]
    MBSPolicyControl,
    #[serde(rename = "npcf_bdtpolicycontrol")]
    BDTPolicyControl,
}

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
			PcfService::AMPolicyControl(inner) => format!("/ampolicycontrol/{}", inner.get_path()),
			PcfService::SMPolicyControl(inner) => format!("/smpolicycontrol/{}", inner.get_path()),
			PcfService::PolicyAuthorization(inner) => {
				format!("/policyauthorization/{}", inner.get_path())
			}
			PcfService::EventExposure(inner) => format!("/eventexposure/{}", inner.get_path()),
			PcfService::UEPolicyControl(inner) => format!("/uepolicycontrol/{}", inner.get_path()),
			PcfService::AMPolicyAuthorization(inner) => {
				format!("/ampolicyauthorization/{}", inner.get_path())
			}
			PcfService::MBSPolicyAuthorization(inner) => {
				format!("/mbspolicyauthorization/{}", inner.get_path())
			}
			PcfService::MBSPolicyControl(inner) => {
				format!("/mbspolicycontrol/{}", inner.get_path())
			}
			PcfService::BDTPolicyControl(inner) => {
				format!("/bdtpolicycontrol/{}", inner.get_path())
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
	Createindividualampolicyassociation,
	Readindividualampolicyassociation,
	Deleteindividualampolicyassociation,
	Reportobservedeventtriggersforindividualampolicyassociation,
}

impl super::ServiceProperties for PcfAMPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
			PcfAMPolicyControlOperation::Createindividualampolicyassociation => "/policies".to_string(),
			PcfAMPolicyControlOperation::Readindividualampolicyassociation => "/policies/{}".to_string(),
			PcfAMPolicyControlOperation::Deleteindividualampolicyassociation => "/policies/{}".to_string(),
			PcfAMPolicyControlOperation::Reportobservedeventtriggersforindividualampolicyassociation => "/policies/{}/update".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfAMPolicyControlOperation::Createindividualampolicyassociation => reqwest::Method::POST,
			PcfAMPolicyControlOperation::Readindividualampolicyassociation => reqwest::Method::GET,
			PcfAMPolicyControlOperation::Deleteindividualampolicyassociation => reqwest::Method::DELETE,
			PcfAMPolicyControlOperation::Reportobservedeventtriggersforindividualampolicyassociation => reqwest::Method::POST,
		}
	}
}

pub enum PcfSMPolicyControlOperation {
	Createsmpolicy,
	Getsmpolicy,
	Updatesmpolicy,
	Deletesmpolicy,
}

impl super::ServiceProperties for PcfSMPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
			PcfSMPolicyControlOperation::Createsmpolicy => "/sm-policies".to_string(),
			PcfSMPolicyControlOperation::Getsmpolicy => "/sm-policies/{}".to_string(),
			PcfSMPolicyControlOperation::Updatesmpolicy => "/sm-policies/{}/update".to_string(),
			PcfSMPolicyControlOperation::Deletesmpolicy => "/sm-policies/{}/delete".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfSMPolicyControlOperation::Createsmpolicy => reqwest::Method::POST,
			PcfSMPolicyControlOperation::Getsmpolicy => reqwest::Method::GET,
			PcfSMPolicyControlOperation::Updatesmpolicy => reqwest::Method::POST,
			PcfSMPolicyControlOperation::Deletesmpolicy => reqwest::Method::POST,
		}
	}
}

pub enum PcfPolicyAuthorizationOperation {
	Postappsessions,
	Pcscfrestoration,
	Getappsession,
	Modappsession,
	Deleteappsession,
	Updateeventssubsc,
	Deleteeventssubsc,
}

impl super::ServiceProperties for PcfPolicyAuthorizationOperation {
	fn get_path(&self) -> String {
		match self {
			PcfPolicyAuthorizationOperation::Postappsessions => "/app-sessions".to_string(),
			PcfPolicyAuthorizationOperation::Pcscfrestoration => {
				"/app-sessions/pcscf-restoration".to_string()
			}
			PcfPolicyAuthorizationOperation::Getappsession => "/app-sessions/{}".to_string(),
			PcfPolicyAuthorizationOperation::Modappsession => "/app-sessions/{}".to_string(),
			PcfPolicyAuthorizationOperation::Deleteappsession => {
				"/app-sessions/{}/delete".to_string()
			}
			PcfPolicyAuthorizationOperation::Updateeventssubsc => {
				"/app-sessions/{}/events-subscription".to_string()
			}
			PcfPolicyAuthorizationOperation::Deleteeventssubsc => {
				"/app-sessions/{}/events-subscription".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfPolicyAuthorizationOperation::Postappsessions => reqwest::Method::POST,
			PcfPolicyAuthorizationOperation::Pcscfrestoration => reqwest::Method::POST,
			PcfPolicyAuthorizationOperation::Getappsession => reqwest::Method::GET,
			PcfPolicyAuthorizationOperation::Modappsession => reqwest::Method::PATCH,
			PcfPolicyAuthorizationOperation::Deleteappsession => reqwest::Method::POST,
			PcfPolicyAuthorizationOperation::Updateeventssubsc => reqwest::Method::PUT,
			PcfPolicyAuthorizationOperation::Deleteeventssubsc => reqwest::Method::DELETE,
		}
	}
}

pub enum PcfEventExposureOperation {
	Postpceventexposuresubsc,
	Getpceventexposuresubsc,
	Putpceventexposuresubsc,
	Deletepceventexposuresubsc,
}

impl super::ServiceProperties for PcfEventExposureOperation {
	fn get_path(&self) -> String {
		match self {
			PcfEventExposureOperation::Postpceventexposuresubsc => "/subscriptions".to_string(),
			PcfEventExposureOperation::Getpceventexposuresubsc => "/subscriptions/{}".to_string(),
			PcfEventExposureOperation::Putpceventexposuresubsc => "/subscriptions/{}".to_string(),
			PcfEventExposureOperation::Deletepceventexposuresubsc => {
				"/subscriptions/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfEventExposureOperation::Postpceventexposuresubsc => reqwest::Method::POST,
			PcfEventExposureOperation::Getpceventexposuresubsc => reqwest::Method::GET,
			PcfEventExposureOperation::Putpceventexposuresubsc => reqwest::Method::PUT,
			PcfEventExposureOperation::Deletepceventexposuresubsc => reqwest::Method::DELETE,
		}
	}
}

pub enum PcfUEPolicyControlOperation {
	Createindividualuepolicyassociation,
	Readindividualuepolicyassociation,
	Deleteindividualuepolicyassociation,
	Reportobservedeventtriggersforindividualuepolicyassociation,
}

impl super::ServiceProperties for PcfUEPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
			PcfUEPolicyControlOperation::Createindividualuepolicyassociation => "/policies".to_string(),
			PcfUEPolicyControlOperation::Readindividualuepolicyassociation => "/policies/{}".to_string(),
			PcfUEPolicyControlOperation::Deleteindividualuepolicyassociation => "/policies/{}".to_string(),
			PcfUEPolicyControlOperation::Reportobservedeventtriggersforindividualuepolicyassociation => "/policies/{}/update".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfUEPolicyControlOperation::Createindividualuepolicyassociation => reqwest::Method::POST,
			PcfUEPolicyControlOperation::Readindividualuepolicyassociation => reqwest::Method::GET,
			PcfUEPolicyControlOperation::Deleteindividualuepolicyassociation => reqwest::Method::DELETE,
			PcfUEPolicyControlOperation::Reportobservedeventtriggersforindividualuepolicyassociation => reqwest::Method::POST,
		}
	}
}

pub enum PcfAMPolicyAuthorizationOperation {
	Postappamcontexts,
	Getappamcontext,
	Modappamcontext,
	Deleteappamcontext,
	Updateameventssubsc,
	Deleteameventssubsc,
}

impl super::ServiceProperties for PcfAMPolicyAuthorizationOperation {
	fn get_path(&self) -> String {
		match self {
			PcfAMPolicyAuthorizationOperation::Postappamcontexts => "/app-am-contexts".to_string(),
			PcfAMPolicyAuthorizationOperation::Getappamcontext => "/app-am-contexts/{}".to_string(),
			PcfAMPolicyAuthorizationOperation::Modappamcontext => "/app-am-contexts/{}".to_string(),
			PcfAMPolicyAuthorizationOperation::Deleteappamcontext => {
				"/app-am-contexts/{}".to_string()
			}
			PcfAMPolicyAuthorizationOperation::Updateameventssubsc => {
				"/app-am-contexts/{}/events-subscription".to_string()
			}
			PcfAMPolicyAuthorizationOperation::Deleteameventssubsc => {
				"/app-am-contexts/{}/events-subscription".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfAMPolicyAuthorizationOperation::Postappamcontexts => reqwest::Method::POST,
			PcfAMPolicyAuthorizationOperation::Getappamcontext => reqwest::Method::GET,
			PcfAMPolicyAuthorizationOperation::Modappamcontext => reqwest::Method::PATCH,
			PcfAMPolicyAuthorizationOperation::Deleteappamcontext => reqwest::Method::DELETE,
			PcfAMPolicyAuthorizationOperation::Updateameventssubsc => reqwest::Method::PUT,
			PcfAMPolicyAuthorizationOperation::Deleteameventssubsc => reqwest::Method::DELETE,
		}
	}
}

pub enum PcfMBSPolicyAuthorizationOperation {
	Creatembsappsessionctxt,
	Getmbsappsessionctxt,
	Modifymbsappsessionctxt,
	Deletembsappsessionctxt,
}

impl super::ServiceProperties for PcfMBSPolicyAuthorizationOperation {
	fn get_path(&self) -> String {
		match self {
			PcfMBSPolicyAuthorizationOperation::Creatembsappsessionctxt => "/contexts".to_string(),
			PcfMBSPolicyAuthorizationOperation::Getmbsappsessionctxt => "/contexts/{}".to_string(),
			PcfMBSPolicyAuthorizationOperation::Modifymbsappsessionctxt => {
				"/contexts/{}".to_string()
			}
			PcfMBSPolicyAuthorizationOperation::Deletembsappsessionctxt => {
				"/contexts/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfMBSPolicyAuthorizationOperation::Creatembsappsessionctxt => reqwest::Method::POST,
			PcfMBSPolicyAuthorizationOperation::Getmbsappsessionctxt => reqwest::Method::GET,
			PcfMBSPolicyAuthorizationOperation::Modifymbsappsessionctxt => reqwest::Method::PATCH,
			PcfMBSPolicyAuthorizationOperation::Deletembsappsessionctxt => reqwest::Method::DELETE,
		}
	}
}

pub enum PcfMBSPolicyControlOperation {
	Creatembspolicy,
	Getindmbspolicy,
	Deleteindmbspolicy,
	Updateindmbspolicy,
}

impl super::ServiceProperties for PcfMBSPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
			PcfMBSPolicyControlOperation::Creatembspolicy => "/mbs-policies".to_string(),
			PcfMBSPolicyControlOperation::Getindmbspolicy => "/mbs-policies/{}".to_string(),
			PcfMBSPolicyControlOperation::Deleteindmbspolicy => "/mbs-policies/{}".to_string(),
			PcfMBSPolicyControlOperation::Updateindmbspolicy => {
				"/mbs-policies/{}/update".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfMBSPolicyControlOperation::Creatembspolicy => reqwest::Method::POST,
			PcfMBSPolicyControlOperation::Getindmbspolicy => reqwest::Method::GET,
			PcfMBSPolicyControlOperation::Deleteindmbspolicy => reqwest::Method::DELETE,
			PcfMBSPolicyControlOperation::Updateindmbspolicy => reqwest::Method::POST,
		}
	}
}

pub enum PcfBDTPolicyControlOperation {
	Createbdtpolicy,
	Getbdtpolicy,
	Updatebdtpolicy,
}

impl super::ServiceProperties for PcfBDTPolicyControlOperation {
	fn get_path(&self) -> String {
		match self {
			PcfBDTPolicyControlOperation::Createbdtpolicy => "/bdtpolicies".to_string(),
			PcfBDTPolicyControlOperation::Getbdtpolicy => "/bdtpolicies/{}".to_string(),
			PcfBDTPolicyControlOperation::Updatebdtpolicy => "/bdtpolicies/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			PcfBDTPolicyControlOperation::Createbdtpolicy => reqwest::Method::POST,
			PcfBDTPolicyControlOperation::Getbdtpolicy => reqwest::Method::GET,
			PcfBDTPolicyControlOperation::Updatebdtpolicy => reqwest::Method::PATCH,
		}
	}
}
