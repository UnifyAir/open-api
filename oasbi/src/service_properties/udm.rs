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
			UdmService::EE(inner) => format!("/nudm-ee/v1/{}", inner.get_path()),
			UdmService::MT(inner) => format!("/nudm-mt/v1/{}", inner.get_path()),
			UdmService::NIDDAU(inner) => format!("/nudm-niddau/v1/{}", inner.get_path()),
			UdmService::PP(inner) => format!("/nudm-pp/v1/{}", inner.get_path()),
			UdmService::RSDS(inner) => format!("/nudm-rsds/v1/{}", inner.get_path()),
			UdmService::SDM(inner) => format!("/nudm-sdm/v2/{}", inner.get_path()),
			UdmService::SSAU(inner) => format!("/nudm-ssau/v1/{}", inner.get_path()),
			UdmService::UEAU(inner) => format!("/nudm-ueau/v1/{}", inner.get_path()),
			UdmService::UECM(inner) => format!("/nudm-uecm/v1/{}", inner.get_path()),
			UdmService::UEID(inner) => format!("/nudm-ueid/v1/{}", inner.get_path()),
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
	CreateEeSubscription,
	DeleteEeSubscription,
	UpdateEeSubscription,
}

impl super::ServiceProperties for UdmEEOperation {
	fn get_path(&self) -> String {
		match self {
			UdmEEOperation::CreateEeSubscription => "{}/ee-subscriptions".to_string(),
			UdmEEOperation::DeleteEeSubscription => "{}/ee-subscriptions/{}".to_string(),
			UdmEEOperation::UpdateEeSubscription => "{}/ee-subscriptions/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmEEOperation::CreateEeSubscription => reqwest::Method::POST,
			UdmEEOperation::DeleteEeSubscription => reqwest::Method::DELETE,
			UdmEEOperation::UpdateEeSubscription => reqwest::Method::PATCH,
		}
	}
}

pub enum UdmMTOperation {
	QueryUeInfo,
	ProvideLocationInfo,
}

impl super::ServiceProperties for UdmMTOperation {
	fn get_path(&self) -> String {
		match self {
			UdmMTOperation::QueryUeInfo => "{}".to_string(),
			UdmMTOperation::ProvideLocationInfo => "{}/loc-info/provide-loc-info".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmMTOperation::QueryUeInfo => reqwest::Method::GET,
			UdmMTOperation::ProvideLocationInfo => reqwest::Method::POST,
		}
	}
}

pub enum UdmNIDDAUOperation {
	AuthorizeNiddData,
}

impl super::ServiceProperties for UdmNIDDAUOperation {
	fn get_path(&self) -> String {
		match self {
			UdmNIDDAUOperation::AuthorizeNiddData => "{}/authorize".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmNIDDAUOperation::AuthorizeNiddData => reqwest::Method::POST,
		}
	}
}

pub enum UdmPPOperation {
	Update,
	Create5GVNGroup,
	Delete5GVNGroup,
	Modify5GVNGroup,
	Get5GVNGroup,
	CreatePPDataEntry,
	DeletePPDataEntry,
	GetPPDataEntry,
	Create5GMBSGroup,
	Delete5GMBSGroup,
	Modify5GMBSGroup,
	Get5GMBSGroup,
}

impl super::ServiceProperties for UdmPPOperation {
	fn get_path(&self) -> String {
		match self {
			UdmPPOperation::Update => "{}/pp-data".to_string(),
			UdmPPOperation::Create5GVNGroup => "5g-vn-groups/{}".to_string(),
			UdmPPOperation::Delete5GVNGroup => "5g-vn-groups/{}".to_string(),
			UdmPPOperation::Modify5GVNGroup => "5g-vn-groups/{}".to_string(),
			UdmPPOperation::Get5GVNGroup => "5g-vn-groups/{}".to_string(),
			UdmPPOperation::CreatePPDataEntry => "{}/pp-data-store/{}".to_string(),
			UdmPPOperation::DeletePPDataEntry => "{}/pp-data-store/{}".to_string(),
			UdmPPOperation::GetPPDataEntry => "{}/pp-data-store/{}".to_string(),
			UdmPPOperation::Create5GMBSGroup => "mbs-group-membership/{}".to_string(),
			UdmPPOperation::Delete5GMBSGroup => "mbs-group-membership/{}".to_string(),
			UdmPPOperation::Modify5GMBSGroup => "mbs-group-membership/{}".to_string(),
			UdmPPOperation::Get5GMBSGroup => "mbs-group-membership/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmPPOperation::Update => reqwest::Method::PATCH,
			UdmPPOperation::Create5GVNGroup => reqwest::Method::PUT,
			UdmPPOperation::Delete5GVNGroup => reqwest::Method::DELETE,
			UdmPPOperation::Modify5GVNGroup => reqwest::Method::PATCH,
			UdmPPOperation::Get5GVNGroup => reqwest::Method::GET,
			UdmPPOperation::CreatePPDataEntry => reqwest::Method::PUT,
			UdmPPOperation::DeletePPDataEntry => reqwest::Method::DELETE,
			UdmPPOperation::GetPPDataEntry => reqwest::Method::GET,
			UdmPPOperation::Create5GMBSGroup => reqwest::Method::PUT,
			UdmPPOperation::Delete5GMBSGroup => reqwest::Method::DELETE,
			UdmPPOperation::Modify5GMBSGroup => reqwest::Method::PATCH,
			UdmPPOperation::Get5GMBSGroup => reqwest::Method::GET,
		}
	}
}

pub enum UdmRSDSOperation {
	ReportSMDeliveryStatus,
}

impl super::ServiceProperties for UdmRSDSOperation {
	fn get_path(&self) -> String {
		match self {
			UdmRSDSOperation::ReportSMDeliveryStatus => "{}/sm-delivery-status".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmRSDSOperation::ReportSMDeliveryStatus => reqwest::Method::POST,
		}
	}
}

pub enum UdmSDMOperation {
	GetDataSets,
	GetNSSAI,
	GetUeCtxInAmfData,
	GetAmData,
	GetEcrData,
	GetSmfSelData,
	GetUeCtxInSmfData,
	GetUeCtxInSmsfData,
	GetTraceConfigData,
	GetSmData,
	GetSmsData,
	GetSmsMngtData,
	GetLcsPrivacyData,
	GetLcsMoData,
	GetLcsBcaData,
	GetV2xData,
	GetProseData,
	GetMbsData,
	GetUcData,
	Subscribe,
	Unsubscribe,
	Modify,
	GetSupiOrGpsi,
	SorAckInfo,
	UpuAck,
	SNSSAIsAck,
	CAGAck,
	UpdateSORInfo,
	GetSharedData,
	SubscribeToSharedData,
	UnsubscribeForSharedData,
	ModifySharedDataSubs,
	GetGroupIdentifiers,
	GetIndividualSharedData,
	GetMultipleIdentifiers,
}

impl super::ServiceProperties for UdmSDMOperation {
	fn get_path(&self) -> String {
		match self {
			UdmSDMOperation::GetDataSets => "{}".to_string(),
			UdmSDMOperation::GetNSSAI => "{}/nssai".to_string(),
			UdmSDMOperation::GetUeCtxInAmfData => "{}/ue-context-in-amf-data".to_string(),
			UdmSDMOperation::GetAmData => "{}/am-data".to_string(),
			UdmSDMOperation::GetEcrData => "{}/am-data/ecr-data".to_string(),
			UdmSDMOperation::GetSmfSelData => "{}/smf-select-data".to_string(),
			UdmSDMOperation::GetUeCtxInSmfData => "{}/ue-context-in-smf-data".to_string(),
			UdmSDMOperation::GetUeCtxInSmsfData => "{}/ue-context-in-smsf-data".to_string(),
			UdmSDMOperation::GetTraceConfigData => "{}/trace-data".to_string(),
			UdmSDMOperation::GetSmData => "{}/sm-data".to_string(),
			UdmSDMOperation::GetSmsData => "{}/sms-data".to_string(),
			UdmSDMOperation::GetSmsMngtData => "{}/sms-mng-data".to_string(),
			UdmSDMOperation::GetLcsPrivacyData => "{}/lcs-privacy-data".to_string(),
			UdmSDMOperation::GetLcsMoData => "{}/lcs-mo-data".to_string(),
			UdmSDMOperation::GetLcsBcaData => "{}/lcs-bca-data".to_string(),
			UdmSDMOperation::GetV2xData => "{}/v2x-data".to_string(),
			UdmSDMOperation::GetProseData => "{}/prose-data".to_string(),
			UdmSDMOperation::GetMbsData => "{}/5mbs-data".to_string(),
			UdmSDMOperation::GetUcData => "{}/uc-data".to_string(),
			UdmSDMOperation::Subscribe => "{}/sdm-subscriptions".to_string(),
			UdmSDMOperation::Unsubscribe => "{}/sdm-subscriptions/{}".to_string(),
			UdmSDMOperation::Modify => "{}/sdm-subscriptions/{}".to_string(),
			UdmSDMOperation::GetSupiOrGpsi => "{}/id-translation-result".to_string(),
			UdmSDMOperation::SorAckInfo => "{}/am-data/sor-ack".to_string(),
			UdmSDMOperation::UpuAck => "{}/am-data/upu-ack".to_string(),
			UdmSDMOperation::SNSSAIsAck => "{}/am-data/subscribed-snssais-ack".to_string(),
			UdmSDMOperation::CAGAck => "{}/am-data/cag-ack".to_string(),
			UdmSDMOperation::UpdateSORInfo => "{}/am-data/update-sor".to_string(),
			UdmSDMOperation::GetSharedData => "shared-data".to_string(),
			UdmSDMOperation::SubscribeToSharedData => "shared-data-subscriptions".to_string(),
			UdmSDMOperation::UnsubscribeForSharedData => "shared-data-subscriptions/{}".to_string(),
			UdmSDMOperation::ModifySharedDataSubs => "shared-data-subscriptions/{}".to_string(),
			UdmSDMOperation::GetGroupIdentifiers => "group-data/group-identifiers".to_string(),
			UdmSDMOperation::GetIndividualSharedData => "shared-data/{}".to_string(),
			UdmSDMOperation::GetMultipleIdentifiers => "multiple-identifiers".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmSDMOperation::GetDataSets => reqwest::Method::GET,
			UdmSDMOperation::GetNSSAI => reqwest::Method::GET,
			UdmSDMOperation::GetUeCtxInAmfData => reqwest::Method::GET,
			UdmSDMOperation::GetAmData => reqwest::Method::GET,
			UdmSDMOperation::GetEcrData => reqwest::Method::GET,
			UdmSDMOperation::GetSmfSelData => reqwest::Method::GET,
			UdmSDMOperation::GetUeCtxInSmfData => reqwest::Method::GET,
			UdmSDMOperation::GetUeCtxInSmsfData => reqwest::Method::GET,
			UdmSDMOperation::GetTraceConfigData => reqwest::Method::GET,
			UdmSDMOperation::GetSmData => reqwest::Method::GET,
			UdmSDMOperation::GetSmsData => reqwest::Method::GET,
			UdmSDMOperation::GetSmsMngtData => reqwest::Method::GET,
			UdmSDMOperation::GetLcsPrivacyData => reqwest::Method::GET,
			UdmSDMOperation::GetLcsMoData => reqwest::Method::GET,
			UdmSDMOperation::GetLcsBcaData => reqwest::Method::GET,
			UdmSDMOperation::GetV2xData => reqwest::Method::GET,
			UdmSDMOperation::GetProseData => reqwest::Method::GET,
			UdmSDMOperation::GetMbsData => reqwest::Method::GET,
			UdmSDMOperation::GetUcData => reqwest::Method::GET,
			UdmSDMOperation::Subscribe => reqwest::Method::POST,
			UdmSDMOperation::Unsubscribe => reqwest::Method::DELETE,
			UdmSDMOperation::Modify => reqwest::Method::PATCH,
			UdmSDMOperation::GetSupiOrGpsi => reqwest::Method::GET,
			UdmSDMOperation::SorAckInfo => reqwest::Method::PUT,
			UdmSDMOperation::UpuAck => reqwest::Method::PUT,
			UdmSDMOperation::SNSSAIsAck => reqwest::Method::PUT,
			UdmSDMOperation::CAGAck => reqwest::Method::PUT,
			UdmSDMOperation::UpdateSORInfo => reqwest::Method::POST,
			UdmSDMOperation::GetSharedData => reqwest::Method::GET,
			UdmSDMOperation::SubscribeToSharedData => reqwest::Method::POST,
			UdmSDMOperation::UnsubscribeForSharedData => reqwest::Method::DELETE,
			UdmSDMOperation::ModifySharedDataSubs => reqwest::Method::PATCH,
			UdmSDMOperation::GetGroupIdentifiers => reqwest::Method::GET,
			UdmSDMOperation::GetIndividualSharedData => reqwest::Method::GET,
			UdmSDMOperation::GetMultipleIdentifiers => reqwest::Method::GET,
		}
	}
}

pub enum UdmSSAUOperation {
	ServiceSpecificAuthorization,
	ServiceSpecificAuthorizationRemoval,
}

impl super::ServiceProperties for UdmSSAUOperation {
	fn get_path(&self) -> String {
		match self {
			UdmSSAUOperation::ServiceSpecificAuthorization => "{}/{}/authorize".to_string(),
			UdmSSAUOperation::ServiceSpecificAuthorizationRemoval => "{}/{}/remove".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmSSAUOperation::ServiceSpecificAuthorization => reqwest::Method::POST,
			UdmSSAUOperation::ServiceSpecificAuthorizationRemoval => reqwest::Method::POST,
		}
	}
}

pub enum UdmUEAUOperation {
	GenerateAuthData,
	GetRgAuthData,
	ConfirmAuth,
	GenerateAv,
	DeleteAuth,
	GenerateGbaAv,
	GenerateProseAV,
}

impl super::ServiceProperties for UdmUEAUOperation {
	fn get_path(&self) -> String {
		match self {
			UdmUEAUOperation::GenerateAuthData => {
				"{}/security-information/generate-auth-data".to_string()
			}
			UdmUEAUOperation::GetRgAuthData => "{}/security-information-rg".to_string(),
			UdmUEAUOperation::ConfirmAuth => "{}/auth-events".to_string(),
			UdmUEAUOperation::GenerateAv => {
				"{}/hss-security-information/{}/generate-av".to_string()
			}
			UdmUEAUOperation::DeleteAuth => "{}/auth-events/{}".to_string(),
			UdmUEAUOperation::GenerateGbaAv => {
				"{}/gba-security-information/generate-av".to_string()
			}
			UdmUEAUOperation::GenerateProseAV => {
				"{}/prose-security-information/generate-av".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmUEAUOperation::GenerateAuthData => reqwest::Method::POST,
			UdmUEAUOperation::GetRgAuthData => reqwest::Method::GET,
			UdmUEAUOperation::ConfirmAuth => reqwest::Method::POST,
			UdmUEAUOperation::GenerateAv => reqwest::Method::POST,
			UdmUEAUOperation::DeleteAuth => reqwest::Method::PUT,
			UdmUEAUOperation::GenerateGbaAv => reqwest::Method::POST,
			UdmUEAUOperation::GenerateProseAV => reqwest::Method::POST,
		}
	}
}

pub enum UdmUECMOperation {
	GetRegistrations,
	SendRoutingInfoSm,
	Op3GppRegistration,
	Update3GppRegistration,
	Get3GppRegistration,
	DeregAMF,
	PeiUpdate,
	UpdateRoamingInformation,
	Non3GppRegistration,
	UpdateNon3GppRegistration,
	GetNon3GppRegistration,
	GetSmfRegistration,
	Registration,
	SmfDeregistration,
	RetrieveSmfRegistration,
	UpdateSmfRegistration,
	Op3GppSmsfRegistration,
	Op3GppSmsfDeregistration,
	Get3GppSmsfRegistration,
	Non3GppSmsfRegistration,
	Non3GppSmsfDeregistration,
	GetNon3GppSmsfRegistration,
	IpSmGwRegistration,
	IpSmGwDeregistration,
	GetIpSmGwRegistration,
	TriggerPCSCFRestoration,
	GetLocationInfo,
	GetNwdafRegistration,
	NwdafRegistration,
	NwdafDeregistration,
	UpdateNwdafRegistration,
}

impl super::ServiceProperties for UdmUECMOperation {
	fn get_path(&self) -> String {
		match self {
			UdmUECMOperation::GetRegistrations => "{}/registrations".to_string(),
			UdmUECMOperation::SendRoutingInfoSm => {
				"{}/registrations/send-routing-info-sm".to_string()
			}
			UdmUECMOperation::Op3GppRegistration => "{}/registrations/amf-3gpp-access".to_string(),
			UdmUECMOperation::Update3GppRegistration => {
				"{}/registrations/amf-3gpp-access".to_string()
			}
			UdmUECMOperation::Get3GppRegistration => "{}/registrations/amf-3gpp-access".to_string(),
			UdmUECMOperation::DeregAMF => "{}/registrations/amf-3gpp-access/dereg-amf".to_string(),
			UdmUECMOperation::PeiUpdate => {
				"{}/registrations/amf-3gpp-access/pei-update".to_string()
			}
			UdmUECMOperation::UpdateRoamingInformation => {
				"{}/registrations/amf-3gpp-access/roaming-info-update".to_string()
			}
			UdmUECMOperation::Non3GppRegistration => {
				"{}/registrations/amf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::UpdateNon3GppRegistration => {
				"{}/registrations/amf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::GetNon3GppRegistration => {
				"{}/registrations/amf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::GetSmfRegistration => {
				"{}/registrations/smf-registrations".to_string()
			}
			UdmUECMOperation::Registration => "{}/registrations/smf-registrations/{}".to_string(),
			UdmUECMOperation::SmfDeregistration => {
				"{}/registrations/smf-registrations/{}".to_string()
			}
			UdmUECMOperation::RetrieveSmfRegistration => {
				"{}/registrations/smf-registrations/{}".to_string()
			}
			UdmUECMOperation::UpdateSmfRegistration => {
				"{}/registrations/smf-registrations/{}".to_string()
			}
			UdmUECMOperation::Op3GppSmsfRegistration => {
				"{}/registrations/smsf-3gpp-access".to_string()
			}
			UdmUECMOperation::Op3GppSmsfDeregistration => {
				"{}/registrations/smsf-3gpp-access".to_string()
			}
			UdmUECMOperation::Get3GppSmsfRegistration => {
				"{}/registrations/smsf-3gpp-access".to_string()
			}
			UdmUECMOperation::Non3GppSmsfRegistration => {
				"{}/registrations/smsf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::Non3GppSmsfDeregistration => {
				"{}/registrations/smsf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::GetNon3GppSmsfRegistration => {
				"{}/registrations/smsf-non-3gpp-access".to_string()
			}
			UdmUECMOperation::IpSmGwRegistration => "{}/registrations/ip-sm-gw".to_string(),
			UdmUECMOperation::IpSmGwDeregistration => "{}/registrations/ip-sm-gw".to_string(),
			UdmUECMOperation::GetIpSmGwRegistration => "{}/registrations/ip-sm-gw".to_string(),
			UdmUECMOperation::TriggerPCSCFRestoration => "restore-pcscf".to_string(),
			UdmUECMOperation::GetLocationInfo => "{}/registrations/location".to_string(),
			UdmUECMOperation::GetNwdafRegistration => {
				"{}/registrations/nwdaf-registrations".to_string()
			}
			UdmUECMOperation::NwdafRegistration => {
				"{}/registrations/nwdaf-registrations/{}".to_string()
			}
			UdmUECMOperation::NwdafDeregistration => {
				"{}/registrations/nwdaf-registrations/{}".to_string()
			}
			UdmUECMOperation::UpdateNwdafRegistration => {
				"{}/registrations/nwdaf-registrations/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmUECMOperation::GetRegistrations => reqwest::Method::GET,
			UdmUECMOperation::SendRoutingInfoSm => reqwest::Method::POST,
			UdmUECMOperation::Op3GppRegistration => reqwest::Method::PUT,
			UdmUECMOperation::Update3GppRegistration => reqwest::Method::PATCH,
			UdmUECMOperation::Get3GppRegistration => reqwest::Method::GET,
			UdmUECMOperation::DeregAMF => reqwest::Method::POST,
			UdmUECMOperation::PeiUpdate => reqwest::Method::POST,
			UdmUECMOperation::UpdateRoamingInformation => reqwest::Method::POST,
			UdmUECMOperation::Non3GppRegistration => reqwest::Method::PUT,
			UdmUECMOperation::UpdateNon3GppRegistration => reqwest::Method::PATCH,
			UdmUECMOperation::GetNon3GppRegistration => reqwest::Method::GET,
			UdmUECMOperation::GetSmfRegistration => reqwest::Method::GET,
			UdmUECMOperation::Registration => reqwest::Method::PUT,
			UdmUECMOperation::SmfDeregistration => reqwest::Method::DELETE,
			UdmUECMOperation::RetrieveSmfRegistration => reqwest::Method::GET,
			UdmUECMOperation::UpdateSmfRegistration => reqwest::Method::PATCH,
			UdmUECMOperation::Op3GppSmsfRegistration => reqwest::Method::PUT,
			UdmUECMOperation::Op3GppSmsfDeregistration => reqwest::Method::DELETE,
			UdmUECMOperation::Get3GppSmsfRegistration => reqwest::Method::GET,
			UdmUECMOperation::Non3GppSmsfRegistration => reqwest::Method::PUT,
			UdmUECMOperation::Non3GppSmsfDeregistration => reqwest::Method::DELETE,
			UdmUECMOperation::GetNon3GppSmsfRegistration => reqwest::Method::GET,
			UdmUECMOperation::IpSmGwRegistration => reqwest::Method::PUT,
			UdmUECMOperation::IpSmGwDeregistration => reqwest::Method::DELETE,
			UdmUECMOperation::GetIpSmGwRegistration => reqwest::Method::GET,
			UdmUECMOperation::TriggerPCSCFRestoration => reqwest::Method::POST,
			UdmUECMOperation::GetLocationInfo => reqwest::Method::GET,
			UdmUECMOperation::GetNwdafRegistration => reqwest::Method::GET,
			UdmUECMOperation::NwdafRegistration => reqwest::Method::PUT,
			UdmUECMOperation::NwdafDeregistration => reqwest::Method::DELETE,
			UdmUECMOperation::UpdateNwdafRegistration => reqwest::Method::PATCH,
		}
	}
}

pub enum UdmUEIDOperation {
	Deconceal,
}

impl super::ServiceProperties for UdmUEIDOperation {
	fn get_path(&self) -> String {
		match self {
			UdmUEIDOperation::Deconceal => "deconceal".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			UdmUEIDOperation::Deconceal => reqwest::Method::POST,
		}
	}
}
