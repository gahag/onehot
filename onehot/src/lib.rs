/*!
This crate provides a trait for one-hot encoding of arbitrary structs and enums.
You will probably wan't to use the custom derive from [onehot-derive](https://docs.rs/onehot-derive/)
most of the time.

It also provides functions for encoding a collection of records into a [bitmatrix](https://docs.rs/bitmatrix/).

# Features
- `derive`: enables the derive macros.
- `matrix`: enables functions for encoding data into a [bitmatrix](https://docs.rs/bitmatrix/).
*/

#[cfg(feature = "bitmatrix")]
mod matrix;

#[cfg(feature = "bitmatrix")]
pub use matrix::*;

#[cfg(test)]
mod tests;

#[cfg(feature = "derive")]
pub use onehot_derive::*;


/// The trait for one-hot encoding of structs and enums.
pub trait OneHot {
	/// How many booleans in the resulting encoding.
	const ONEHOT_LEN: usize;

	/// An iterator to the labels of each encoded field.
	/// This iterator has the same size as `ONEHOT_LEN`
	type Labels: Iterator<Item = &'static str>;
	/// An iterator to the encoded values.
	/// This iterator has the same size as `ONEHOT_LEN`
	type Encoder: Iterator<Item = bool>;

	/// Get the iterator to the labels.
	fn labels() -> Self::Labels;

	/// One-hot encode.
	fn onehot(&self) -> Self::Encoder;
}


impl<T: OneHot> OneHot for &T {
	const ONEHOT_LEN: usize = T::ONEHOT_LEN;

	type Labels = T::Labels;

	type Encoder = T::Encoder;

	fn labels() -> Self::Labels {
		T::labels()
	}

	fn onehot(&self) -> Self::Encoder {
		(**self).onehot()
	}
}


impl<T: OneHot> OneHot for &mut T {
	const ONEHOT_LEN: usize = T::ONEHOT_LEN;

	type Labels = T::Labels;

	type Encoder = T::Encoder;

	fn labels() -> Self::Labels {
		T::labels()
	}

	fn onehot(&self) -> Self::Encoder {
		(**self).onehot()
	}
}


/// Booleans are trivially one-hot encoded.
impl OneHot for bool {
	const ONEHOT_LEN: usize = 1;

	type Labels = std::iter::Once<&'static str>;

	type Encoder = std::iter::Once<bool>;

	fn labels() -> Self::Labels {
		std::iter::once("bool")
	}

	fn onehot(&self) -> Self::Encoder {
		std::iter::once(*self)
	}
}
