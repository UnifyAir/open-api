mod amf_id;
mod fqdn;
mod generated;
mod ipv4_addr;
mod ipv4_addr_mask;
mod ipv6_addr;
mod ipv6_prefix;
mod mcc_mnc;
mod nf_type;
mod uri;

pub use amf_id::{AmfId, AmfRegionId, AmfSetId};
pub use fqdn::Fqdn;
pub use generated::types::*;
pub use ipv4_addr::Ipv4Addr;
pub use ipv4_addr_mask::Ipv4AddrMask;
pub use ipv6_addr::Ipv6Addr;
pub use ipv6_prefix::Ipv6Prefix;
pub use mcc_mnc::{Mcc, Mnc};
pub use nf_type::NfType;
pub use uri::Uri;
pub use macros::NewUnchecked;

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

macro_rules! deref_for_newtype {
	($newtype:ty, $target:ty) => {
		impl std::ops::Deref for $newtype {
			type Target = $target;
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
	};
}

pub(in crate::common) use deref_for_newtype;
pub(in crate::common) use display_for_newtype;
pub(in crate::common) use impl_try_from_strings;

// Do NOT add serde Serialize and Deserialize here.
// Create a macro here which would implment display and FromStr trait here in order to prevent 
// nested serde Serialize and Deserialize
macro_rules! impl_hex_type {
    ($name:ident, $inner_type:ty) => {
        #[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
        pub struct $name($inner_type);

        impl $name {
            pub fn to_hex(&self) -> String {
                format!("{:X}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let val = <$inner_type>::from_str_radix(s, 16)?;
                Ok(Self(val))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(f, "{}", self.to_hex())
            }
        }

		impl std::ops::Deref for $name {
			type Target = $inner_type;
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
    };
}

// Use the macro to define similar types
impl_hex_type!(HexU8, u8);
impl_hex_type!(HexU16, u16 );
impl_hex_type!(HexU32, u32 );
impl_hex_type!(HexU64, u64);

