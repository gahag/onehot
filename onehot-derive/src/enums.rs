use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;

use crate::attr;


pub fn impl_onehot(
	name: &syn::Ident,
	variants: &syn::punctuated::Punctuated<
		syn::Variant,
		syn::Token![,]
	>,
	visibility: syn::Visibility,
	generics: syn::Generics,
) -> TokenStream {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let encoder_name = format_ident!("{}OneHotEncoder", name);
	let mut variant_count: usize = 0;

	let mut labels = Vec::new();
	let mut values = Vec::new();

	for variant in variants {
		let ident = &variant.ident;
		let variant_properties = attr::OneHotVariantProperties::new(variant);

		if variant_properties.ignore {
			continue;
		}

		variant_count += 1;

		labels.push(
			quote! {
				concat!(
					stringify!(#name),
					"::",
					stringify!(#ident)
				)
			}
		);

		values.push(
			quote! {
				matches!(value, #name::#ident)
			}
		);
	}

	let gen = quote! {
		#visibility struct #encoder_name {
			encoding: [
				bool;
				#variant_count
			],
			ix: usize,
		}

		impl #encoder_name {
			#visibility fn new #impl_generics(value: &#name #ty_generics) -> Self #where_clause {
				Self {
					encoding: [
						#(#values),*
					],
					ix: 0,
				}
			}
		}

		impl Iterator for #encoder_name {
			type Item = bool;

			fn next(&mut self) -> Option<Self::Item> {
				if self.ix == #variant_count {
					return None;
				}

				let item = self.encoding[self.ix];
				self.ix += 1;
				Some(item)
			}

			fn size_hint(&self) -> (usize, Option<usize>) {
				(
					#variant_count,
					Some(#variant_count)
				)
			}
		}


		impl #impl_generics OneHot for #name #ty_generics #where_clause {
			const ONEHOT_LEN: usize = #variant_count;

			type Labels = std::iter::Copied<std::slice::Iter<'static, &'static str>>;
			type Encoder = #encoder_name;

			fn labels() -> Self::Labels {
				static LABELS: &'static [&'static str] = &[
					#(#labels),*
				];

				LABELS
					.iter()
					.copied()
			}

			fn onehot(&self) -> Self::Encoder {
				#encoder_name::new(self)
			}
		}
	};

	gen.into()
}
