/*!
This crate provides a custom auto derive macro for [onehot](https://docs.rs/onehot/).
*/

mod attr;
mod metadata;
mod enums;
mod structs;

use proc_macro::TokenStream;
use syn;


/// The auto derive macro for OneHot.
///
/// Enums and structs:
/// ```
/// # use onehot::OneHot;
/// #[derive(OneHot)]
/// pub enum SimpleEnum {
/// 	First,
/// 	Second,
/// 	#[onehot(ignore)] // Fields with the `ignore` attribute are disconsidered.
/// 	Third,
/// }
///
/// #[derive(OneHot)]
/// struct SimpleStruct {
/// 	f1: bool,
/// 	f2: SimpleEnum,
/// }
/// ```
///
/// Structs can also be tuple and/or generic:
/// ```
/// # use onehot::OneHot;
/// #[derive(OneHot)]
/// pub struct SimpleGenericStruct<'a, T: OneHot + 'a>(
/// 	bool,
/// 	 &'a T,
/// 	#[onehot(ignore)] u32
/// );
/// ```
#[proc_macro_derive(OneHot, attributes(onehot))]
pub fn onehot_derive(input: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = syn::parse(input).unwrap();

	let name = &ast.ident;

	match ast.data {
		syn::Data::Enum(ref _enum) => enums::impl_onehot(
			name,
			&_enum.variants,
			ast.vis,
			ast.generics,
		),
		syn::Data::Struct(ref _struct) => structs::impl_onehot(
			name,
			&_struct.fields,
			ast.generics,
		),
		_ => panic!("OneHot can only be derived for structs and enums."),
	}
}
