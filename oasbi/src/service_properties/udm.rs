
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
pub enum UdmServiceName {
    #[default]
    #[serde(rename = "nudm_ee")]
    EE,
    #[serde(rename = "nudm_mt")]
    MT,
    #[serde(rename = "nudm_niddau")]
    NIDDAU,
    #[serde(rename = "nudm_pp")]
    PP,
    #[serde(rename = "nudm_rsds")]
    RSDS,
    #[serde(rename = "nudm_sdm")]
    SDM,
    #[serde(rename = "nudm_ssau")]
    SSAU,
    #[serde(rename = "nudm_ueau")]
    UEAU,
    #[serde(rename = "nudm_uecm")]
    UECM,
    #[serde(rename = "nudm_ueid")]
    UEID,
}

pub enum UdmService {
	EE(UdmEEOperation),
	MT(UdmMTOperation),
	NIDDAU(UdmNIDDAUOperation),
	PP(UdmPPOperation),
	RSDS(UdmRSDSOperation),
	SDM(UdmSDMOperation),
	SSAU(UdmSSAUOperation),
	UEAU(UdmUEAUOperation),
	UECM(UdmUECMOperation),
	UEID(UdmUEIDOperation),
}

impl super::ServiceProperties for UdmService {
	fn get_path(&self) -> String {
		match self {
			UdmService::EE(inner) => format!("/ee/{}", inner.get_path()),
			UdmService::MT(inner) => format!("/mt/{}", inner.get_path()),
			UdmService::NIDDAU(inner) => format!("/niddau/{}", inner.get_path()),
			UdmService::PP(inner) => format!("/pp/{}", inner.get_path()),
			UdmService::RSDS(inner) => format!("/rsds/{}", inner.get_path()),
			UdmService::SDM(inner) => format!("/sdm/{}", inner.get_path()),
			UdmService::SSAU(inner) => format!("/ssau/{}", inner.get_path()),
			UdmService::UEAU(inner) => format!("/ueau/{}", inner.get_path()),
			UdmService::UECM(inner) => format!("/uecm/{}", inner.get_path()),
			UdmService::UEID(inner) => format!("/ueid/{}", inner.get_path()),
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmService::EE(inner) => inner.get_http_method(),
			UdmService::MT(inner) => inner.get_http_method(),
			UdmService::NIDDAU(inner) => inner.get_http_method(),
			UdmService::PP(inner) => inner.get_http_method(),
			UdmService::RSDS(inner) => inner.get_http_method(),
			UdmService::SDM(inner) => inner.get_http_method(),
			UdmService::SSAU(inner) => inner.get_http_method(),
			UdmService::UEAU(inner) => inner.get_http_method(),
			UdmService::UECM(inner) => inner.get_http_method(),
			UdmService::UEID(inner) => inner.get_http_method(),
		}
	}
}

pub enum UdmEEOperation {
	Createeesubscription,
	Deleteeesubscription,
	Updateeesubscription,
}

impl super::ServiceProperties for UdmEEOperation {
	fn get_path(&self) -> String {
		match self {
			UdmEEOperation::Createeesubscription => "/{}/ee-subscriptions".to_string(),
			UdmEEOperation::Deleteeesubscription => "/{}/ee-subscriptions/{}".to_string(),
			UdmEEOperation::Updateeesubscription => "/{}/ee-subscriptions/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmEEOperation::Createeesubscription => reqwest::Method::POST,
			UdmEEOperation::Deleteeesubscription => reqwest::Method::DELETE,
			UdmEEOperation::Updateeesubscription => reqwest::Method::PATCH,
		}
	}
}

pub enum UdmMTOperation {
	Queryueinfo,
	Providelocationinfo,
}

impl super::ServiceProperties for UdmMTOperation {
	fn get_path(&self) -> String {
		match self {
			UdmMTOperation::Queryueinfo => "/{}".to_string(),
			UdmMTOperation::Providelocationinfo => "/{}/loc-info/provide-loc-info".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmMTOperation::Queryueinfo => reqwest::Method::GET,
			UdmMTOperation::Providelocationinfo => reqwest::Method::POST,
		}
	}
}

pub enum UdmNIDDAUOperation {
	Authorizenidddata,
}

impl super::ServiceProperties for UdmNIDDAUOperation {
	fn get_path(&self) -> String {
		match self {
			UdmNIDDAUOperation::Authorizenidddata => "/{}/authorize".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmNIDDAUOperation::Authorizenidddata => reqwest::Method::POST,
		}
	}
}

pub enum UdmPPOperation {
	Update,
	Create5gvngroup,
	Delete5gvngroup,
	Modify5gvngroup,
	Get5gvngroup,
	Createppdataentry,
	Deleteppdataentry,
	Getppdataentry,
	Create5gmbsgroup,
	Delete5gmbsgroup,
	Modify5gmbsgroup,
	Get5gmbsgroup,
}

impl super::ServiceProperties for UdmPPOperation {
	fn get_path(&self) -> String {
		match self {
			UdmPPOperation::Update => "/{}/pp-data".to_string(),
			UdmPPOperation::Create5gvngroup => "/5g-vn-groups/{}".to_string(),
			UdmPPOperation::Delete5gvngroup => "/5g-vn-groups/{}".to_string(),
			UdmPPOperation::Modify5gvngroup => "/5g-vn-groups/{}".to_string(),
			UdmPPOperation::Get5gvngroup => "/5g-vn-groups/{}".to_string(),
			UdmPPOperation::Createppdataentry => "/{}/pp-data-store/{}".to_string(),
			UdmPPOperation::Deleteppdataentry => "/{}/pp-data-store/{}".to_string(),
			UdmPPOperation::Getppdataentry => "/{}/pp-data-store/{}".to_string(),
			UdmPPOperation::Create5gmbsgroup => "/mbs-group-membership/{}".to_string(),
			UdmPPOperation::Delete5gmbsgroup => "/mbs-group-membership/{}".to_string(),
			UdmPPOperation::Modify5gmbsgroup => "/mbs-group-membership/{}".to_string(),
			UdmPPOperation::Get5gmbsgroup => "/mbs-group-membership/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmPPOperation::Update => reqwest::Method::PATCH,
			UdmPPOperation::Create5gvngroup => reqwest::Method::PUT,
			UdmPPOperation::Delete5gvngroup => reqwest::Method::DELETE,
			UdmPPOperation::Modify5gvngroup => reqwest::Method::PATCH,
			UdmPPOperation::Get5gvngroup => reqwest::Method::GET,
			UdmPPOperation::Createppdataentry => reqwest::Method::PUT,
			UdmPPOperation::Deleteppdataentry => reqwest::Method::DELETE,
			UdmPPOperation::Getppdataentry => reqwest::Method::GET,
			UdmPPOperation::Create5gmbsgroup => reqwest::Method::PUT,
			UdmPPOperation::Delete5gmbsgroup => reqwest::Method::DELETE,
			UdmPPOperation::Modify5gmbsgroup => reqwest::Method::PATCH,
			UdmPPOperation::Get5gmbsgroup => reqwest::Method::GET,
		}
	}
}

pub enum UdmRSDSOperation {
	Reportsmdeliverystatus,
}

impl super::ServiceProperties for UdmRSDSOperation {
	fn get_path(&self) -> String {
		match self {
			UdmRSDSOperation::Reportsmdeliverystatus => "/{}/sm-delivery-status".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmRSDSOperation::Reportsmdeliverystatus => reqwest::Method::POST,
		}
	}
}

pub enum UdmSDMOperation {
	Getdatasets,
	Getnssai,
	Getuectxinamfdata,
	Getamdata,
	Getecrdata,
	Getsmfseldata,
	Getuectxinsmfdata,
	Getuectxinsmsfdata,
	Gettraceconfigdata,
	Getsmdata,
	Getsmsdata,
	Getsmsmngtdata,
	Getlcsprivacydata,
	Getlcsmodata,
	Getlcsbcadata,
	Getv2xdata,
	Getprosedata,
	Getmbsdata,
	Getucdata,
	Subscribe,
	Unsubscribe,
	Modify,
	Getsupiorgpsi,
	Sorackinfo,
	Upuack,
	Snssaisack,
	Cagack,
	Updatesorinfo,
	Getshareddata,
	Subscribetoshareddata,
	Unsubscribeforshareddata,
	Modifyshareddatasubs,
	Getgroupidentifiers,
	Getindividualshareddata,
	Getmultipleidentifiers,
}

impl super::ServiceProperties for UdmSDMOperation {
	fn get_path(&self) -> String {
		match self {
			UdmSDMOperation::Getdatasets => "/{}".to_string(),
			UdmSDMOperation::Getnssai => "/{}/nssai".to_string(),
			UdmSDMOperation::Getuectxinamfdata => "/{}/ue-context-in-amf-data".to_string(),
			UdmSDMOperation::Getamdata => "/{}/am-data".to_string(),
			UdmSDMOperation::Getecrdata => "/{}/am-data/ecr-data".to_string(),
			UdmSDMOperation::Getsmfseldata => "/{}/smf-select-data".to_string(),
			UdmSDMOperation::Getuectxinsmfdata => "/{}/ue-context-in-smf-data".to_string(),
			UdmSDMOperation::Getuectxinsmsfdata => "/{}/ue-context-in-smsf-data".to_string(),
			UdmSDMOperation::Gettraceconfigdata => "/{}/trace-data".to_string(),
			UdmSDMOperation::Getsmdata => "/{}/sm-data".to_string(),
			UdmSDMOperation::Getsmsdata => "/{}/sms-data".to_string(),
			UdmSDMOperation::Getsmsmngtdata => "/{}/sms-mng-data".to_string(),
			UdmSDMOperation::Getlcsprivacydata => "/{}/lcs-privacy-data".to_string(),
			UdmSDMOperation::Getlcsmodata => "/{}/lcs-mo-data".to_string(),
			UdmSDMOperation::Getlcsbcadata => "/{}/lcs-bca-data".to_string(),
			UdmSDMOperation::Getv2xdata => "/{}/v2x-data".to_string(),
			UdmSDMOperation::Getprosedata => "/{}/prose-data".to_string(),
			UdmSDMOperation::Getmbsdata => "/{}/5mbs-data".to_string(),
			UdmSDMOperation::Getucdata => "/{}/uc-data".to_string(),
			UdmSDMOperation::Subscribe => "/{}/sdm-subscriptions".to_string(),
			UdmSDMOperation::Unsubscribe => "/{}/sdm-subscriptions/{}".to_string(),
			UdmSDMOperation::Modify => "/{}/sdm-subscriptions/{}".to_string(),
			UdmSDMOperation::Getsupiorgpsi => "/{}/id-translation-result".to_string(),
			UdmSDMOperation::Sorackinfo => "/{}/am-data/sor-ack".to_string(),
			UdmSDMOperation::Upuack => "/{}/am-data/upu-ack".to_string(),
			UdmSDMOperation::Snssaisack => "/{}/am-data/subscribed-snssais-ack".to_string(),
			UdmSDMOperation::Cagack => "/{}/am-data/cag-ack".to_string(),
			UdmSDMOperation::Updatesorinfo => "/{}/am-data/update-sor".to_string(),
			UdmSDMOperation::Getshareddata => "/shared-data".to_string(),
			UdmSDMOperation::Subscribetoshareddata => "/shared-data-subscriptions".to_string(),
			UdmSDMOperation::Unsubscribeforshareddata => {
				"/shared-data-subscriptions/{}".to_string()
			}
			UdmSDMOperation::Modifyshareddatasubs => "/shared-data-subscriptions/{}".to_string(),
			UdmSDMOperation::Getgroupidentifiers => "/group-data/group-identifiers".to_string(),
			UdmSDMOperation::Getindividualshareddata => "/shared-data/{}".to_string(),
			UdmSDMOperation::Getmultipleidentifiers => "/multiple-identifiers".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmSDMOperation::Getdatasets => reqwest::Method::GET,
			UdmSDMOperation::Getnssai => reqwest::Method::GET,
			UdmSDMOperation::Getuectxinamfdata => reqwest::Method::GET,
			UdmSDMOperation::Getamdata => reqwest::Method::GET,
			UdmSDMOperation::Getecrdata => reqwest::Method::GET,
			UdmSDMOperation::Getsmfseldata => reqwest::Method::GET,
			UdmSDMOperation::Getuectxinsmfdata => reqwest::Method::GET,
			UdmSDMOperation::Getuectxinsmsfdata => reqwest::Method::GET,
			UdmSDMOperation::Gettraceconfigdata => reqwest::Method::GET,
			UdmSDMOperation::Getsmdata => reqwest::Method::GET,
			UdmSDMOperation::Getsmsdata => reqwest::Method::GET,
			UdmSDMOperation::Getsmsmngtdata => reqwest::Method::GET,
			UdmSDMOperation::Getlcsprivacydata => reqwest::Method::GET,
			UdmSDMOperation::Getlcsmodata => reqwest::Method::GET,
			UdmSDMOperation::Getlcsbcadata => reqwest::Method::GET,
			UdmSDMOperation::Getv2xdata => reqwest::Method::GET,
			UdmSDMOperation::Getprosedata => reqwest::Method::GET,
			UdmSDMOperation::Getmbsdata => reqwest::Method::GET,
			UdmSDMOperation::Getucdata => reqwest::Method::GET,
			UdmSDMOperation::Subscribe => reqwest::Method::POST,
			UdmSDMOperation::Unsubscribe => reqwest::Method::DELETE,
			UdmSDMOperation::Modify => reqwest::Method::PATCH,
			UdmSDMOperation::Getsupiorgpsi => reqwest::Method::GET,
			UdmSDMOperation::Sorackinfo => reqwest::Method::PUT,
			UdmSDMOperation::Upuack => reqwest::Method::PUT,
			UdmSDMOperation::Snssaisack => reqwest::Method::PUT,
			UdmSDMOperation::Cagack => reqwest::Method::PUT,
			UdmSDMOperation::Updatesorinfo => reqwest::Method::POST,
			UdmSDMOperation::Getshareddata => reqwest::Method::GET,
			UdmSDMOperation::Subscribetoshareddata => reqwest::Method::POST,
			UdmSDMOperation::Unsubscribeforshareddata => reqwest::Method::DELETE,
			UdmSDMOperation::Modifyshareddatasubs => reqwest::Method::PATCH,
			UdmSDMOperation::Getgroupidentifiers => reqwest::Method::GET,
			UdmSDMOperation::Getindividualshareddata => reqwest::Method::GET,
			UdmSDMOperation::Getmultipleidentifiers => reqwest::Method::GET,
		}
	}
}

pub enum UdmSSAUOperation {
	Servicespecificauthorization,
	Servicespecificauthorizationremoval,
}

impl super::ServiceProperties for UdmSSAUOperation {
	fn get_path(&self) -> String {
		match self {
			UdmSSAUOperation::Servicespecificauthorization => "/{}/{}/authorize".to_string(),
			UdmSSAUOperation::Servicespecificauthorizationremoval => "/{}/{}/remove".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmSSAUOperation::Servicespecificauthorization => reqwest::Method::POST,
			UdmSSAUOperation::Servicespecificauthorizationremoval => reqwest::Method::POST,
		}
	}
}

pub enum UdmUEAUOperation {
	Generateauthdata,
	Getrgauthdata,
	Confirmauth,
	Generateav,
	Deleteauth,
	Generategbaav,
	Generateproseav,
}

impl super::ServiceProperties for UdmUEAUOperation {
	fn get_path(&self) -> String {
		match self {
			UdmUEAUOperation::Generateauthdata => {
				"/{}/security-information/generate-auth-data".to_string()
			}
			UdmUEAUOperation::Getrgauthdata => "/{}/security-information-rg".to_string(),
			UdmUEAUOperation::Confirmauth => "/{}/auth-events".to_string(),
			UdmUEAUOperation::Generateav => {
				"/{}/hss-security-information/{}/generate-av".to_string()
			}
			UdmUEAUOperation::Deleteauth => "/{}/auth-events/{}".to_string(),
			UdmUEAUOperation::Generategbaav => {
				"/{}/gba-security-information/generate-av".to_string()
			}
			UdmUEAUOperation::Generateproseav => {
				"/{}/prose-security-information/generate-av".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmUEAUOperation::Generateauthdata => reqwest::Method::POST,
			UdmUEAUOperation::Getrgauthdata => reqwest::Method::GET,
			UdmUEAUOperation::Confirmauth => reqwest::Method::POST,
			UdmUEAUOperation::Generateav => reqwest::Method::POST,
			UdmUEAUOperation::Deleteauth => reqwest::Method::PUT,
			UdmUEAUOperation::Generategbaav => reqwest::Method::POST,
			UdmUEAUOperation::Generateproseav => reqwest::Method::POST,
		}
	}
}

pub enum UdmUECMOperation {
	Getregistrations,
	Sendroutinginfosm,
	Op3gppregistration,
	Update3gppregistration,
	Get3gppregistration,
	Deregamf,
	Peiupdate,
	Updateroaminginformation,
	Non3gppregistration,
	Updatenon3gppregistration,
	Getnon3gppregistration,
	Getsmfregistration,
	Registration,
	Smfderegistration,
	Retrievesmfregistration,
	Updatesmfregistration,
	Op3gppsmsfregistration,
	Op3gppsmsfderegistration,
	Get3gppsmsfregistration,
	Non3gppsmsfregistration,
	Non3gppsmsfderegistration,
	Getnon3gppsmsfregistration,
	Ipsmgwregistration,
	Ipsmgwderegistration,
	Getipsmgwregistration,
	Triggerpcscfrestoration,
	Getlocationinfo,
	Getnwdafregistration,
	Nwdafregistration,
	Nwdafderegistration,
	Updatenwdafregistration,
}

impl super::ServiceProperties for UdmUECMOperation {
	fn get_path(&self) -> String {
		match self {
			UdmUECMOperation::Getregistrations => "/{}/registrations".to_string(),
			UdmUECMOperation::Sendroutinginfosm => {
				"/{}/registrations/send-routing-info-sm".to_string()
			}
			UdmUECMOperation::Op3gppregistration => "/{}/registrations/amf-3gpp-access".to_string(),
			UdmUECMOperation::Update3gppregistration => {
				"/{}/registrations/amf-3gpp-access".to_string()
			}
			UdmUECMOperation::Get3gppregistration => {
				"/{}/registrations/amf-3gpp-access".to_string()
			}
			UdmUECMOperation::Deregamf => "/{}/registrations/amf-3gpp-access/dereg-amf".to_string(),
			UdmUECMOperation::Peiupdate => {
				"/{}/registrations/amf-3gpp-access/pei-update".to_string()
			}
			UdmUECMOperation::Updateroaminginformation => {
				"/{}/registrations/amf-3gpp-access/roaming-info-update".to_string()
			}
			UdmUECMOperation::Non3gppregistration => {
				"/{}/registrations/amf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::Updatenon3gppregistration => {
				"/{}/registrations/amf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::Getnon3gppregistration => {
				"/{}/registrations/amf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::Getsmfregistration => {
				"/{}/registrations/smf-registrations".to_string()
			}
			UdmUECMOperation::Registration => "/{}/registrations/smf-registrations/{}".to_string(),
			UdmUECMOperation::Smfderegistration => {
				"/{}/registrations/smf-registrations/{}".to_string()
			}
			UdmUECMOperation::Retrievesmfregistration => {
				"/{}/registrations/smf-registrations/{}".to_string()
			}
			UdmUECMOperation::Updatesmfregistration => {
				"/{}/registrations/smf-registrations/{}".to_string()
			}
			UdmUECMOperation::Op3gppsmsfregistration => {
				"/{}/registrations/smsf-3gpp-access".to_string()
			}
			UdmUECMOperation::Op3gppsmsfderegistration => {
				"/{}/registrations/smsf-3gpp-access".to_string()
			}
			UdmUECMOperation::Get3gppsmsfregistration => {
				"/{}/registrations/smsf-3gpp-access".to_string()
			}
			UdmUECMOperation::Non3gppsmsfregistration => {
				"/{}/registrations/smsf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::Non3gppsmsfderegistration => {
				"/{}/registrations/smsf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::Getnon3gppsmsfregistration => {
				"/{}/registrations/smsf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::Ipsmgwregistration => "/{}/registrations/ip-sm-gw".to_string(),
			UdmUECMOperation::Ipsmgwderegistration => "/{}/registrations/ip-sm-gw".to_string(),
			UdmUECMOperation::Getipsmgwregistration => "/{}/registrations/ip-sm-gw".to_string(),
			UdmUECMOperation::Triggerpcscfrestoration => "/restore-pcscf".to_string(),
			UdmUECMOperation::Getlocationinfo => "/{}/registrations/location".to_string(),
			UdmUECMOperation::Getnwdafregistration => {
				"/{}/registrations/nwdaf-registrations".to_string()
			}
			UdmUECMOperation::Nwdafregistration => {
				"/{}/registrations/nwdaf-registrations/{}".to_string()
			}
			UdmUECMOperation::Nwdafderegistration => {
				"/{}/registrations/nwdaf-registrations/{}".to_string()
			}
			UdmUECMOperation::Updatenwdafregistration => {
				"/{}/registrations/nwdaf-registrations/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmUECMOperation::Getregistrations => reqwest::Method::GET,
			UdmUECMOperation::Sendroutinginfosm => reqwest::Method::POST,
			UdmUECMOperation::Op3gppregistration => reqwest::Method::PUT,
			UdmUECMOperation::Update3gppregistration => reqwest::Method::PATCH,
			UdmUECMOperation::Get3gppregistration => reqwest::Method::GET,
			UdmUECMOperation::Deregamf => reqwest::Method::POST,
			UdmUECMOperation::Peiupdate => reqwest::Method::POST,
			UdmUECMOperation::Updateroaminginformation => reqwest::Method::POST,
			UdmUECMOperation::Non3gppregistration => reqwest::Method::PUT,
			UdmUECMOperation::Updatenon3gppregistration => reqwest::Method::PATCH,
			UdmUECMOperation::Getnon3gppregistration => reqwest::Method::GET,
			UdmUECMOperation::Getsmfregistration => reqwest::Method::GET,
			UdmUECMOperation::Registration => reqwest::Method::PUT,
			UdmUECMOperation::Smfderegistration => reqwest::Method::DELETE,
			UdmUECMOperation::Retrievesmfregistration => reqwest::Method::GET,
			UdmUECMOperation::Updatesmfregistration => reqwest::Method::PATCH,
			UdmUECMOperation::Op3gppsmsfregistration => reqwest::Method::PUT,
			UdmUECMOperation::Op3gppsmsfderegistration => reqwest::Method::DELETE,
			UdmUECMOperation::Get3gppsmsfregistration => reqwest::Method::GET,
			UdmUECMOperation::Non3gppsmsfregistration => reqwest::Method::PUT,
			UdmUECMOperation::Non3gppsmsfderegistration => reqwest::Method::DELETE,
			UdmUECMOperation::Getnon3gppsmsfregistration => reqwest::Method::GET,
			UdmUECMOperation::Ipsmgwregistration => reqwest::Method::PUT,
			UdmUECMOperation::Ipsmgwderegistration => reqwest::Method::DELETE,
			UdmUECMOperation::Getipsmgwregistration => reqwest::Method::GET,
			UdmUECMOperation::Triggerpcscfrestoration => reqwest::Method::POST,
			UdmUECMOperation::Getlocationinfo => reqwest::Method::GET,
			UdmUECMOperation::Getnwdafregistration => reqwest::Method::GET,
			UdmUECMOperation::Nwdafregistration => reqwest::Method::PUT,
			UdmUECMOperation::Nwdafderegistration => reqwest::Method::DELETE,
			UdmUECMOperation::Updatenwdafregistration => reqwest::Method::PATCH,
		}
	}
}

pub enum UdmUEIDOperation {
	Deconceal,
}

impl super::ServiceProperties for UdmUEIDOperation {
	fn get_path(&self) -> String {
		match self {
			UdmUEIDOperation::Deconceal => "/deconceal".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmUEIDOperation::Deconceal => reqwest::Method::POST,
		}
	}
}
