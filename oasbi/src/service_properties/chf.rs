
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
pub enum ChfServiceName {
    #[default]
    #[serde(rename = "nchf_spendinglimitcontrol")]
    SpendingLimitControl,
    #[serde(rename = "nchf_convergedcharging")]
    ConvergedCharging,
    #[serde(rename = "nchf_offlineonlycharging")]
    OfflineOnlyCharging,
}

pub enum ChfService {
	SpendingLimitControl(ChfSpendingLimitControlOperation),
	ConvergedCharging(ChfConvergedChargingOperation),
	OfflineOnlyCharging(ChfOfflineOnlyChargingOperation),
}

impl super::ServiceProperties for ChfService {
	fn get_path(&self) -> String {
		match self {
			ChfService::SpendingLimitControl(inner) => {
				format!("/spendinglimitcontrol/{}", inner.get_path())
			}
			ChfService::ConvergedCharging(inner) => {
				format!("/convergedcharging/{}", inner.get_path())
			}
			ChfService::OfflineOnlyCharging(inner) => {
				format!("/offlineonlycharging/{}", inner.get_path())
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
	Postsubscriptions,
	Putsubscriptions,
	Deletesubscriptions,
}

impl super::ServiceProperties for ChfSpendingLimitControlOperation {
	fn get_path(&self) -> String {
		match self {
			ChfSpendingLimitControlOperation::Postsubscriptions => "/subscriptions".to_string(),
			ChfSpendingLimitControlOperation::Putsubscriptions => "/subscriptions/{}".to_string(),
			ChfSpendingLimitControlOperation::Deletesubscriptions => {
				"/subscriptions/{}".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			ChfSpendingLimitControlOperation::Postsubscriptions => reqwest::Method::POST,
			ChfSpendingLimitControlOperation::Putsubscriptions => reqwest::Method::PUT,
			ChfSpendingLimitControlOperation::Deletesubscriptions => reqwest::Method::DELETE,
		}
	}
}

pub enum ChfConvergedChargingOperation {
	Postchargingdata,
	Postchargingdataupdate,
	Postchargingdatarelease,
}

impl super::ServiceProperties for ChfConvergedChargingOperation {
	fn get_path(&self) -> String {
		match self {
			ChfConvergedChargingOperation::Postchargingdata => "/chargingdata".to_string(),
			ChfConvergedChargingOperation::Postchargingdataupdate => {
				"/chargingdata/{}/update".to_string()
			}
			ChfConvergedChargingOperation::Postchargingdatarelease => {
				"/chargingdata/{}/release".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			ChfConvergedChargingOperation::Postchargingdata => reqwest::Method::POST,
			ChfConvergedChargingOperation::Postchargingdataupdate => reqwest::Method::POST,
			ChfConvergedChargingOperation::Postchargingdatarelease => reqwest::Method::POST,
		}
	}
}

pub enum ChfOfflineOnlyChargingOperation {
	Postofflinechargingdata,
	Postofflinechargingdataupdate,
	Postofflinechargingdatarelease,
}

impl super::ServiceProperties for ChfOfflineOnlyChargingOperation {
	fn get_path(&self) -> String {
		match self {
			ChfOfflineOnlyChargingOperation::Postofflinechargingdata => {
				"/offlinechargingdata".to_string()
			}
			ChfOfflineOnlyChargingOperation::Postofflinechargingdataupdate => {
				"/offlinechargingdata/{}/update".to_string()
			}
			ChfOfflineOnlyChargingOperation::Postofflinechargingdatarelease => {
				"/offlinechargingdata/{}/release".to_string()
			}
		}
	}

	fn get_http_method(&self) -> reqwest::Method {
		match self {
			ChfOfflineOnlyChargingOperation::Postofflinechargingdata => reqwest::Method::POST,
			ChfOfflineOnlyChargingOperation::Postofflinechargingdataupdate => reqwest::Method::POST,
			ChfOfflineOnlyChargingOperation::Postofflinechargingdatarelease => {
				reqwest::Method::POST
			}
		}
	}
}
