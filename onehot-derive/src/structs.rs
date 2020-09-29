use proc_macro::TokenStream;
use quote::quote;
use syn;

use crate::attr;


pub fn impl_onehot(
	name: &syn::Ident,
	fields: &syn::Fields,
	generics: syn::Generics,
) -> TokenStream {
	match fields {
		syn::Fields::Named(fields) => {
			let mut types = Vec::new();
			let mut names = Vec::new();

			for field in &fields.named {
				let field_properties = attr::OneHotFieldProperties::new(field);

				if field_properties.ignore {
					continue;
				}

				types.push(&field.ty);

				names.push(
					field.ident
						.as_ref()
						.expect("field should have a name")
				);
			}

			if let (Some(last_type), Some(last_name)) = (types.pop(), names.pop()) {
				impl_onehot_nonempty(
					name,
					&types,
					&names,
					last_type,
					last_name,
					generics
				)
			}
			else { // No values to encode.
				impl_onehot_empty(name, generics)
			}
		},

		syn::Fields::Unnamed(fields) => {
			let mut types = Vec::new();
			let mut indexes: Vec<syn::Index> = Vec::new();

			for (ix, field) in fields.unnamed.iter().enumerate() {
				let field_properties = attr::OneHotFieldProperties::new(field);

				if field_properties.ignore {
					continue;
				}

				types.push(&field.ty);

				indexes.push(
					ix.into()
				);
			}

			if let (Some(last_type), Some(last_index)) = (types.pop(), indexes.pop()) {
				impl_onehot_nonempty(
					name,
					&types,
					&indexes,
					last_type,
					last_index,
					generics
				)
			}
			else { // No values to encode.
				impl_onehot_empty(name, generics)
			}
		},

		syn::Fields::Unit => impl_onehot_empty(name, generics),
	}
}


fn impl_onehot_empty(
	name: &syn::Ident,
	generics: syn::Generics,
) -> TokenStream {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let gen = quote! {
		impl #impl_generics OneHot for #name #ty_generics #where_clause {
			const ONEHOT_LEN: usize = 0;

			type Labels = std::iter::Empty<&'static str>;

			type Encoder = std::iter::Empty<bool>;

			fn labels() -> Self::Labels {
				std::iter::empty()
			}

			fn onehot(&self) -> Self::Encoder {
				std::iter::empty()
			}
		}
	};

	gen.into()
}


// T should be syn::Ident or syn::Index
fn impl_onehot_nonempty<T: quote::ToTokens>(
	name: &syn::Ident,
	types: &[&syn::Type],
	names: &[T],
	last_type: &syn::Type,
	last_name: T,
	generics: syn::Generics,
) -> TokenStream {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let mut labels_type = quote! {
		<(#last_type) as OneHot>::Labels
	};

	let mut encoder_type = quote! {
		<(#last_type) as OneHot>::Encoder
	};

	let mut labels = quote! {
		<(#last_type) as OneHot>::labels()
	};

	let mut encode = quote! {
		self.#last_name.onehot()
	};

	for ty in types.iter().rev() {
		labels_type = quote! {
			std::iter::Chain<
				<(#ty) as OneHot>::Labels,
				#labels_type
			>
		};

		encoder_type = quote! {
			std::iter::Chain<
				<(#ty) as OneHot>::Encoder,
				#encoder_type
			>
		};

		labels = quote! {
			<(#ty) as OneHot>::labels().chain(
				#labels
			)
		};
	}

	for field_name in names.iter().rev() {
		encode = quote! {
			self.#field_name.onehot().chain(
				#encode
			)
		};
	}

	let gen = quote! {
		impl #impl_generics OneHot for #name #ty_generics #where_clause {
			const ONEHOT_LEN: usize = #(<#types as OneHot>::ONEHOT_LEN +)* <#last_type as OneHot>::ONEHOT_LEN;

			type Labels = #labels_type;

			type Encoder = #encoder_type;

			fn labels() -> Self::Labels { #labels }

			fn onehot(&self) -> Self::Encoder { #encode }
		}
	};

	gen.into()
}
