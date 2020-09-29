mod structs;
mod enums;

use crate::OneHot;


fn test_size<T: OneHot>(size: usize) {
	assert_eq!(T::ONEHOT_LEN, size);
}


fn test_labels<T: OneHot>(labels: &[&str]) {
	let labels_iter = labels.iter().copied();

	assert!(
		T
			::labels()
			.eq(labels_iter)
	);
}


fn test_variants<T: OneHot>(variants: &[(T, &[bool])]) {
	for (variant, encoded) in variants.iter() {
		let encoded_iter = encoded.iter().copied();

		assert!(
			variant
				.onehot()
				.eq(encoded_iter)
		);
	}
}
