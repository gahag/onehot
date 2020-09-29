pub fn get<'a>(
	ident: &str,
	it: impl IntoIterator<Item = &'a syn::Attribute>,
) -> Vec<syn::Meta> {
	it.into_iter()
		.filter_map(
			|attr| attr.parse_meta().ok()
		)
		.filter_map(
			|meta| match meta {
				syn::Meta::List(syn::MetaList { path, nested, .. }) => {
					if path.is_ident(ident) {
						Some(nested)
					} else {
						None
					}
				}
				_ => None,
			}
		)
		.flat_map(
			|id| id
		)
		.map(
			|nested| match nested {
				syn::NestedMeta::Meta(meta) => meta,
				_ => panic!("unexpected literal parsing onehot attributes"),
			}
		)
		.collect()
}
