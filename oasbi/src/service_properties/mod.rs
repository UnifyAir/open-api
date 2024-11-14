mod amf;
mod ausf;
mod chf;
mod mbsmf;
mod nrf;
mod nssf;
mod pcf;
mod smf;
mod udm;
mod udr;
mod upf;

pub use amf::*;
pub use ausf::*;
pub use chf::*;
pub use mbsmf::*;
pub use nrf::*;
pub use nssf::*;
pub use pcf::*;
pub use smf::*;
pub use udm::*;
pub use udr::*;
pub use upf::*;

pub trait ServiceProperties {
	fn get_path(&self) -> String;
	fn get_http_method(&self) -> reqwest::Method;
}
