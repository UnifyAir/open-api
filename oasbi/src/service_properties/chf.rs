pub enum ChfService {
	SpendingLimitControl(ChfSpendingLimitControlOperation),
	ConvergedCharging(ChfConvergedChargingOperation),
	OfflineOnlyCharging(ChfOfflineOnlyChargingOperation),
}

impl super::ServiceProperties for ChfService {
	fn get_path(&self) -> String {
		match self {
			ChfService::SpendingLimitControl(inner) => {
				format!("/nchf-spendinglimitcontrol/v1/{}", inner.get_path())
			}
			ChfService::ConvergedCharging(inner) => {
				format!("/nchf-convergedcharging/v3/{}", inner.get_path())
			}
			ChfService::OfflineOnlyCharging(inner) => {
				format!("/nchf-offlineonlycharging/v1/{}", inner.get_path())
			}
		}
	}
	fn get_http_method(&self) -> reqwest::Method {
		match self {
			ChfService::SpendingLimitControl(inner) => inner.get_http_method(),
			ChfService::ConvergedCharging(inner) => inner.get_http_method(),
			ChfService::OfflineOnlyCharging(inner) => inner.get_http_method(),
		}
	}
}

pub enum ChfSpendingLimitControlOperation {
	PostSUBSCRIPTIONS,
	PutSUBSCRIPTIONS,
	DeleteSUBSCRIPTIONS,
}

impl super::ServiceProperties for ChfSpendingLimitControlOperation {
	fn get_path(&self) -> String {
		match self {
			ChfSpendingLimitControlOperation::PostSUBSCRIPTIONS => "subscriptions".to_string(),
			ChfSpendingLimitControlOperation::PutSUBSCRIPTIONS => "subscriptions/{}".to_string(),
			ChfSpendingLimitControlOperation::DeleteSUBSCRIPTIONS => "subscriptions/{}".to_string(),
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			ChfSpendingLimitControlOperation::PostSUBSCRIPTIONS => reqwest::Method::POST,
			ChfSpendingLimitControlOperation::PutSUBSCRIPTIONS => reqwest::Method::PUT,
			ChfSpendingLimitControlOperation::DeleteSUBSCRIPTIONS => reqwest::Method::DELETE,
		}
	}
}

pub enum ChfConvergedChargingOperation {
	PostCHARGINGDATA,
	PostCHARGINGDATAUPDATE,
	PostCHARGINGDATARELEASE,
}

impl super::ServiceProperties for ChfConvergedChargingOperation {
	fn get_path(&self) -> String {
		match self {
			ChfConvergedChargingOperation::PostCHARGINGDATA => "chargingdata".to_string(),
			ChfConvergedChargingOperation::PostCHARGINGDATAUPDATE => {
				"chargingdata/{}/update".to_string()
			}
			ChfConvergedChargingOperation::PostCHARGINGDATARELEASE => {
				"chargingdata/{}/release".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			ChfConvergedChargingOperation::PostCHARGINGDATA => reqwest::Method::POST,
			ChfConvergedChargingOperation::PostCHARGINGDATAUPDATE => reqwest::Method::POST,
			ChfConvergedChargingOperation::PostCHARGINGDATARELEASE => reqwest::Method::POST,
		}
	}
}

pub enum ChfOfflineOnlyChargingOperation {
	PostOFFLINECHARGINGDATA,
	PostOFFLINECHARGINGDATAUPDATE,
	PostOFFLINECHARGINGDATARELEASE,
}

impl super::ServiceProperties for ChfOfflineOnlyChargingOperation {
	fn get_path(&self) -> String {
		match self {
			ChfOfflineOnlyChargingOperation::PostOFFLINECHARGINGDATA => {
				"offlinechargingdata".to_string()
			}
			ChfOfflineOnlyChargingOperation::PostOFFLINECHARGINGDATAUPDATE => {
				"offlinechargingdata/{}/update".to_string()
			}
			ChfOfflineOnlyChargingOperation::PostOFFLINECHARGINGDATARELEASE => {
				"offlinechargingdata/{}/release".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			ChfOfflineOnlyChargingOperation::PostOFFLINECHARGINGDATA => reqwest::Method::POST,
			ChfOfflineOnlyChargingOperation::PostOFFLINECHARGINGDATAUPDATE => reqwest::Method::POST,
			ChfOfflineOnlyChargingOperation::PostOFFLINECHARGINGDATARELEASE => {
				reqwest::Method::POST
			}
		}
	}
}
