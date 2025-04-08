mod new_unchecked;

use new_unchecked::NewUnchecked;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

/// Derives a `new_unchecked` constructor for a newtype struct.
/// 
/// This macro generates a constructor function that bypasses validation checks,
/// allowing for direct instantiation of newtype structs without the usual safety checks.
/// This is useful in performance-critical scenarios where validation is already
/// guaranteed by other means.
/// 
/// # Example
/// ```
/// use macros::NewUnchecked;
/// 
/// #[derive(NewUnchecked)]
/// struct PersonId(String);
/// 
/// // Now you can use PersonId::new_unchecked() to create instances
/// ```
/// 
/// # Safety
/// This constructor bypasses validation, so it's the caller's responsibility
/// to ensure the data is valid.
/// 
/// # Limitations
/// Currently, this macro only supports newtype structs (structs with a single field).
/// Support for more complex structs may be added in the future.
#[proc_macro_derive(NewUnchecked)]
pub fn new_unchecked_derive(input: TokenStream) -> TokenStream {
	// TODO: Add support for unsafe in derive_new macro
	let parsed_input: DeriveInput = syn::parse_macro_input!(input);
	match NewUnchecked::from_derive_input(&parsed_input) {
		Ok(ok) => ok.into_token_stream().into(),
		Err(err) => err.into_compile_error().into(),
	}
}
