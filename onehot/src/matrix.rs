use bitmatrix::BitMatrix;

use crate::OneHot;


/// Encode a dataset vertically into a [bitmatrix](https://docs.rs/bitmatrix/) using OneHot.
///
/// The resulting matrix is of the form:
/// ```notrust
///          record 1  . . .  record N
///         +--------------------------
/// value 1 | true     . . .   false
///    .    |   .      . . .     .
///    .    |   .      . . .     .
///    .    |   .      . . .     .
/// value M | false    . . .   false
/// ```
pub fn vertical<T: OneHot>(data: &[T]) -> BitMatrix {
	let mut matrix = BitMatrix::new(
		T::ONEHOT_LEN,
		data.len(),
	);

	for (transaction, record) in data.iter().enumerate() {
		for (item, value) in record.onehot().enumerate() {
			matrix[item].set(transaction, value);
		}
	}

	matrix
}


/// Encode a dataset horizontally into a [bitmatrix](https://docs.rs/bitmatrix/) using OneHot.
///
/// The resulting matrix is of the form:
/// ```notrust
///            value 1    . . .  value N
///          +---------------------------
/// record 1 |  true      . . .   false
///    .     |    .       . . .     .
///    .     |    .       . . .     .
///    .     |    .       . . .     .
/// record M |  false     . . .   false
/// ```
pub fn horizontal<T: OneHot>(data: &[T]) -> BitMatrix {
	let mut matrix = BitMatrix::new(
		data.len(),
		T::ONEHOT_LEN,
	);

	for (transaction, record) in data.iter().enumerate() {
		for (item, value) in record.onehot().enumerate() {
			matrix[transaction].set(item, value);
		}
	}

	matrix
}
