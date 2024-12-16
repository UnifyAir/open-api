use std::{
	ops::{Deref, DerefMut},
	str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Uri(http::Uri);

impl From<http::Uri> for Uri {
	fn from(uri: http::Uri) -> Self {
		Uri(uri)
	}
}

impl From<Uri> for http::Uri {
	fn from(wrapper: Uri) -> Self {
		wrapper.0
	}
}

impl Serialize for Uri {
	fn serialize<S>(
		&self,
		serializer: S,
	) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(self.0.to_string().as_str())
	}
}

impl<'de> Deserialize<'de> for Uri {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let uri_str = String::deserialize(deserializer)?;
		http::Uri::from_str(&uri_str)
			.map(Uri)
			.map_err(serde::de::Error::custom)
	}
}

impl Default for Uri {
	fn default() -> Self {
		Uri(http::Uri::builder()
			.scheme("http")
			.authority("127.0.0.1")
			.path_and_query("/")
			.build()
			.unwrap())
	}
}

impl FromStr for Uri {
	type Err = super::error::ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		match value.parse() {
			res @ Ok(_) => res,
			Err(invalid) => Err(Self::Err::from(format!("Invalid Uri: {:?}", invalid)))
		}
	}
}

impl ToString for Uri {
	fn to_string(&self) -> String {
		self.0.to_string()
	}
}

impl Deref for Uri {
	type Target = http::Uri;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Uri {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[cfg(test)]
mod tests {
	use serde_json;

	use super::Uri;

	#[test]
	fn test_serialize_deserialize() {
		let uri: http::Uri = "http://example.com/test?query=1".parse().unwrap();
		let wrapper = Uri::from(uri.clone());

		// Serialize
		let serialized = serde_json::to_string(&wrapper).unwrap();
		assert_eq!(serialized, "\"http://example.com/test?query=1\"");

		// Deserialize
		let deserialized: Uri = serde_json::from_str(&serialized).unwrap();
		assert_eq!(deserialized, Uri::from(uri));
	}
}
