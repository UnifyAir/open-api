use std::marker::ConstParamTy;

/// NF types known to NRF
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "NF types known to NRF",
///  "type": "string",
///  "enum": [
///    "NRF",
///    "UDM",
///    "AMF",
///    "SMF",
///    "AUSF",
///    "NEF",
///    "PCF",
///    "SMSF",
///    "NSSF",
///    "UDR",
///    "LMF",
///    "GMLC",
///    "5G_EIR",
///    "SEPP",
///    "UPF",
///    "N3IWF",
///    "AF",
///    "UDSF",
///    "BSF",
///    "CHF",
///    "NWDAF",
///    "PCSCF",
///    "CBCF",
///    "HSS",
///    "UCMF",
///    "SOR_AF",
///    "SPAF",
///    "MME",
///    "SCSAS",
///    "SCEF",
///    "SCP",
///    "NSSAAF",
///    "ICSCF",
///    "SCSCF",
///    "DRA",
///    "IMS_AS",
///    "AANF",
///    "5G_DDNMF",
///    "NSACF",
///    "MFAF",
///    "EASDF",
///    "DCCF",
///    "MB_SMF",
///    "TSCTSF",
///    "ADRF",
///    "GBA_BSF",
///    "CEF",
///    "MB_UPF",
///    "NSWOF",
///    "PKMF",
///    "MNPF",
///    "SMS_GMSC",
///    "SMS_IWMSC",
///    "MBSF",
///    "MBSTF",
///    "PANF",
///    "IP_SM_GW",
///    "SMS_ROUTER"
///  ],
///  "x-allow-unknown": true
/// }
/// ```
/// </details>
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
	ConstParamTy,
)]
pub enum NfType {
	#[default]
	#[serde(rename = "NRF")]
	Nrf,
	#[serde(rename = "UDM")]
	Udm,
	#[serde(rename = "AMF")]
	Amf,
	#[serde(rename = "SMF")]
	Smf,
	#[serde(rename = "AUSF")]
	Ausf,
	#[serde(rename = "NEF")]
	Nef,
	#[serde(rename = "PCF")]
	Pcf,
	#[serde(rename = "SMSF")]
	Smsf,
	#[serde(rename = "NSSF")]
	Nssf,
	#[serde(rename = "UDR")]
	Udr,
	#[serde(rename = "LMF")]
	Lmf,
	#[serde(rename = "GMLC")]
	Gmlc,
	#[serde(rename = "5G_EIR")]
	FiveGEir,
	#[serde(rename = "SEPP")]
	Sepp,
	#[serde(rename = "UPF")]
	Upf,
	#[serde(rename = "N3IWF")]
	N3iwf,
	#[serde(rename = "AF")]
	Af,
	#[serde(rename = "UDSF")]
	Udsf,
	#[serde(rename = "BSF")]
	Bsf,
	#[serde(rename = "CHF")]
	Chf,
	#[serde(rename = "NWDAF")]
	Nwdaf,
	#[serde(rename = "PCSCF")]
	Pcscf,
	#[serde(rename = "CBCF")]
	Cbcf,
	#[serde(rename = "HSS")]
	Hss,
	#[serde(rename = "UCMF")]
	Ucmf,
	#[serde(rename = "SOR_AF")]
	SorAf,
	#[serde(rename = "SPAF")]
	Spaf,
	#[serde(rename = "MME")]
	Mme,
	#[serde(rename = "SCSAS")]
	Scsas,
	#[serde(rename = "SCEF")]
	Scef,
	#[serde(rename = "SCP")]
	Scp,
	#[serde(rename = "NSSAAF")]
	Nssaaf,
	#[serde(rename = "ICSCF")]
	Icscf,
	#[serde(rename = "SCSCF")]
	Scscf,
	#[serde(rename = "DRA")]
	Dra,
	#[serde(rename = "IMS_AS")]
	ImsAs,
	#[serde(rename = "AANF")]
	Aanf,
	#[serde(rename = "5G_DDNMF")]
	FiveGDdnmf,
	#[serde(rename = "NSACF")]
	Nsacf,
	#[serde(rename = "MFAF")]
	Mfaf,
	#[serde(rename = "EASDF")]
	Easdf,
	#[serde(rename = "DCCF")]
	Dccf,
	#[serde(rename = "MB_SMF")]
	MbSmf,
	#[serde(rename = "TSCTSF")]
	Tsctsf,
	#[serde(rename = "ADRF")]
	Adrf,
	#[serde(rename = "GBA_BSF")]
	GbaBsf,
	#[serde(rename = "CEF")]
	Cef,
	#[serde(rename = "MB_UPF")]
	MbUpf,
	#[serde(rename = "NSWOF")]
	Nswof,
	#[serde(rename = "PKMF")]
	Pkmf,
	#[serde(rename = "MNPF")]
	Mnpf,
	#[serde(rename = "SMS_GMSC")]
	SmsGmsc,
	#[serde(rename = "SMS_IWMSC")]
	SmsIwmsc,
	#[serde(rename = "MBSF")]
	Mbsf,
	#[serde(rename = "MBSTF")]
	Mbstf,
	#[serde(rename = "PANF")]
	Panf,
	#[serde(rename = "IP_SM_GW")]
	IpSmGw,
	#[serde(rename = "SMS_ROUTER")]
	SmsRouter,
	#[serde(untagged)]
	Unknown,
}

impl From<&NfType> for NfType {
	fn from(value: &NfType) -> Self {
		value.clone()
	}
}

impl ToString for NfType {
	fn to_string(&self) -> String {
		match *self {
			Self::Nrf => "NRF".to_string(),
			Self::Udm => "UDM".to_string(),
			Self::Amf => "AMF".to_string(),
			Self::Smf => "SMF".to_string(),
			Self::Ausf => "AUSF".to_string(),
			Self::Nef => "NEF".to_string(),
			Self::Pcf => "PCF".to_string(),
			Self::Smsf => "SMSF".to_string(),
			Self::Nssf => "NSSF".to_string(),
			Self::Udr => "UDR".to_string(),
			Self::Lmf => "LMF".to_string(),
			Self::Gmlc => "GMLC".to_string(),
			Self::FiveGEir => "5G_EIR".to_string(),
			Self::Sepp => "SEPP".to_string(),
			Self::Upf => "UPF".to_string(),
			Self::N3iwf => "N3IWF".to_string(),
			Self::Af => "AF".to_string(),
			Self::Udsf => "UDSF".to_string(),
			Self::Bsf => "BSF".to_string(),
			Self::Chf => "CHF".to_string(),
			Self::Nwdaf => "NWDAF".to_string(),
			Self::Pcscf => "PCSCF".to_string(),
			Self::Cbcf => "CBCF".to_string(),
			Self::Hss => "HSS".to_string(),
			Self::Ucmf => "UCMF".to_string(),
			Self::SorAf => "SOR_AF".to_string(),
			Self::Spaf => "SPAF".to_string(),
			Self::Mme => "MME".to_string(),
			Self::Scsas => "SCSAS".to_string(),
			Self::Scef => "SCEF".to_string(),
			Self::Scp => "SCP".to_string(),
			Self::Nssaaf => "NSSAAF".to_string(),
			Self::Icscf => "ICSCF".to_string(),
			Self::Scscf => "SCSCF".to_string(),
			Self::Dra => "DRA".to_string(),
			Self::ImsAs => "IMS_AS".to_string(),
			Self::Aanf => "AANF".to_string(),
			Self::FiveGDdnmf => "5G_DDNMF".to_string(),
			Self::Nsacf => "NSACF".to_string(),
			Self::Mfaf => "MFAF".to_string(),
			Self::Easdf => "EASDF".to_string(),
			Self::Dccf => "DCCF".to_string(),
			Self::MbSmf => "MB_SMF".to_string(),
			Self::Tsctsf => "TSCTSF".to_string(),
			Self::Adrf => "ADRF".to_string(),
			Self::GbaBsf => "GBA_BSF".to_string(),
			Self::Cef => "CEF".to_string(),
			Self::MbUpf => "MB_UPF".to_string(),
			Self::Nswof => "NSWOF".to_string(),
			Self::Pkmf => "PKMF".to_string(),
			Self::Mnpf => "MNPF".to_string(),
			Self::SmsGmsc => "SMS_GMSC".to_string(),
			Self::SmsIwmsc => "SMS_IWMSC".to_string(),
			Self::Mbsf => "MBSF".to_string(),
			Self::Mbstf => "MBSTF".to_string(),
			Self::Panf => "PANF".to_string(),
			Self::IpSmGw => "IP_SM_GW".to_string(),
			Self::SmsRouter => "SMS_ROUTER".to_string(),
			Self::Unknown => unreachable!(),
		}
	}
}

impl std::str::FromStr for NfType {
	type Err = super::error::ConversionError;
	fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
		match value {
			"NRF" => Ok(Self::Nrf),
			"UDM" => Ok(Self::Udm),
			"AMF" => Ok(Self::Amf),
			"SMF" => Ok(Self::Smf),
			"AUSF" => Ok(Self::Ausf),
			"NEF" => Ok(Self::Nef),
			"PCF" => Ok(Self::Pcf),
			"SMSF" => Ok(Self::Smsf),
			"NSSF" => Ok(Self::Nssf),
			"UDR" => Ok(Self::Udr),
			"LMF" => Ok(Self::Lmf),
			"GMLC" => Ok(Self::Gmlc),
			"5G_EIR" => Ok(Self::FiveGEir),
			"SEPP" => Ok(Self::Sepp),
			"UPF" => Ok(Self::Upf),
			"N3IWF" => Ok(Self::N3iwf),
			"AF" => Ok(Self::Af),
			"UDSF" => Ok(Self::Udsf),
			"BSF" => Ok(Self::Bsf),
			"CHF" => Ok(Self::Chf),
			"NWDAF" => Ok(Self::Nwdaf),
			"PCSCF" => Ok(Self::Pcscf),
			"CBCF" => Ok(Self::Cbcf),
			"HSS" => Ok(Self::Hss),
			"UCMF" => Ok(Self::Ucmf),
			"SOR_AF" => Ok(Self::SorAf),
			"SPAF" => Ok(Self::Spaf),
			"MME" => Ok(Self::Mme),
			"SCSAS" => Ok(Self::Scsas),
			"SCEF" => Ok(Self::Scef),
			"SCP" => Ok(Self::Scp),
			"NSSAAF" => Ok(Self::Nssaaf),
			"ICSCF" => Ok(Self::Icscf),
			"SCSCF" => Ok(Self::Scscf),
			"DRA" => Ok(Self::Dra),
			"IMS_AS" => Ok(Self::ImsAs),
			"AANF" => Ok(Self::Aanf),
			"5G_DDNMF" => Ok(Self::FiveGDdnmf),
			"NSACF" => Ok(Self::Nsacf),
			"MFAF" => Ok(Self::Mfaf),
			"EASDF" => Ok(Self::Easdf),
			"DCCF" => Ok(Self::Dccf),
			"MB_SMF" => Ok(Self::MbSmf),
			"TSCTSF" => Ok(Self::Tsctsf),
			"ADRF" => Ok(Self::Adrf),
			"GBA_BSF" => Ok(Self::GbaBsf),
			"CEF" => Ok(Self::Cef),
			"MB_UPF" => Ok(Self::MbUpf),
			"NSWOF" => Ok(Self::Nswof),
			"PKMF" => Ok(Self::Pkmf),
			"MNPF" => Ok(Self::Mnpf),
			"SMS_GMSC" => Ok(Self::SmsGmsc),
			"SMS_IWMSC" => Ok(Self::SmsIwmsc),
			"MBSF" => Ok(Self::Mbsf),
			"MBSTF" => Ok(Self::Mbstf),
			"PANF" => Ok(Self::Panf),
			"IP_SM_GW" => Ok(Self::IpSmGw),
			"SMS_ROUTER" => Ok(Self::SmsRouter),
			_ => Ok(Self::Unknown),
		}
	}
}

impl std::convert::TryFrom<&str> for NfType {
	type Error = super::error::ConversionError;
	fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
		value.parse()
	}
}

impl std::convert::TryFrom<&String> for NfType {
	type Error = super::error::ConversionError;
	fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
		value.parse()
	}
}

impl std::convert::TryFrom<String> for NfType {
	type Error = super::error::ConversionError;
	fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
		value.parse()
	}
}
