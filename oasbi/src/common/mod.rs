mod uri;
mod nf_type;
mod generated;
mod ipv4_addr;
mod fqdn;
mod ipv6_addr;
mod ipv6_prefix;
mod ipv4_addr_mask;
mod amf_id;
mod mcc_mnc;

pub use uri::Uri;
pub use nf_type::NfType;
pub use fqdn::Fqdn;
pub use ipv6_addr::Ipv6Addr;
pub use ipv4_addr::Ipv4Addr;
pub use ipv6_prefix::Ipv6Prefix;
pub use ipv4_addr_mask::Ipv4AddrMask;
pub use amf_id::{ AmfId, AmfSetId, AmfRegionId};
pub use mcc_mnc::{Mnc, Mcc};
pub use generated::types::*;

pub mod error {
	/// Error from a TryFrom or FromStr implementation.
	pub struct ConversionError(::std::borrow::Cow<'static, str>);

	impl ::std::error::Error for ConversionError {}

	impl ::std::fmt::Display for ConversionError {
		fn fmt(
			&self,
			f: &mut ::std::fmt::Formatter<'_>,
		) -> Result<(), ::std::fmt::Error> {
			::std::fmt::Display::fmt(&self.0, f)
		}
	}

	impl ::std::fmt::Debug for ConversionError {
		fn fmt(
			&self,
			f: &mut ::std::fmt::Formatter<'_>,
		) -> Result<(), ::std::fmt::Error> {
			::std::fmt::Debug::fmt(&self.0, f)
		}
	}

	impl From<&'static str> for ConversionError {
		fn from(value: &'static str) -> Self {
			Self(value.into())
		}
	}

	impl From<String> for ConversionError {
		fn from(value: String) -> Self {
			Self(value.into())
		}
	}
}

// Implement try from string types based on FromStr implementation
macro_rules! impl_try_from_strings {
    ($newtype:ty) => {
        impl std::convert::TryFrom<&str> for $newtype {
            type Error = <$newtype as FromStr>::Err;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        impl TryFrom<String> for $newtype {
            type Error = <$newtype as FromStr>::Err;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        impl TryFrom<&String> for $newtype {
            type Error = <$newtype as FromStr>::Err;

            fn try_from(value: &String) -> Result<Self, Self::Error> {
                value.as_str().parse()
            }
        }
    };
}

macro_rules! display_for_newtype {
    ($newtype:ty) => {
		impl std::fmt::Display for $newtype {
			fn fmt(
				&self,
				f: &mut std::fmt::Formatter,
			) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}
    };
}
pub(in crate::common) use impl_try_from_strings;
pub(in crate::common) use display_for_newtype;
