use std::default::Default;

use crate::metadata;


#[derive(Default)]
pub struct OneHotVariantProperties {
	pub ignore: bool,
}


impl OneHotVariantProperties {
	pub fn new(variant: &syn::Variant) -> Self {
		let mut props = Self::default();

		for meta in metadata::get("onehot", &variant.attrs) {
			match meta {
				syn::Meta::Path(p) => {
					if p.is_ident("ignore") {
						props.ignore = true;
					}
					else {
						panic!("unrecognized value in onehot(..) attribute");
					}
				}

				_ => panic!("invalid OneHot attribute"),
			}
		}

		props
	}
}


#[derive(Default)]
pub struct OneHotFieldProperties {
	pub ignore: bool,
}


impl OneHotFieldProperties {
	pub fn new(field: &syn::Field) -> Self {
		let mut props = Self::default();

		for meta in metadata::get("onehot", &field.attrs) {
			match meta {
				syn::Meta::Path(p) => {
					if p.is_ident("ignore") {
						props.ignore = true;
					}
					else {
						panic!("unrecognized value in onehot(..) attribute");
					}
				}

				_ => panic!("invalid OneHot attribute"),
			}
		}

		props
	}
}
