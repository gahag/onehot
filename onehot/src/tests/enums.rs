use super::*;


#[derive(OneHot)]
pub enum SimpleEnum {
	First,
	Second,
	#[onehot(ignore)]
	Third,
}


#[derive(OneHot)]
pub(super) enum SimpleGenericEnum<T> {
	#[onehot(ignore)]
	First(T),
	Second,
	Third,
}


#[derive(OneHot)]
pub(crate) enum ComplexGenericEnum<'a, 'b, T, U>
where
	T: Copy,
	U: Sync,
{
	#[onehot(ignore)]
	First(&'a T),
	#[onehot(ignore)]
	Second { _field: &'b U },
	Third,
}


#[test]
fn test_simple_enum() {
	type T = SimpleEnum;

	test_size::<T>(2);

	test_labels::<T>(&["SimpleEnum::First", "SimpleEnum::Second"]);

	test_variants(
		&[
			(T::First, &[true, false]),
			(T::Second, &[false, true]),
			(T::Third, &[false, false]),
		]
	);
}


#[test]
fn test_simple_generic_enum() {
	type T = SimpleGenericEnum::<()>;

	test_size::<T>(2);

	test_labels::<T>(&["SimpleGenericEnum::Second", "SimpleGenericEnum::Third"]);

	test_variants(
		&[
			(T::First(()), &[false, false]),
			(T::Second, &[true, false]),
			(T::Third, &[false, true]),
		]
	);
}


#[test]
fn test_complex_generic_enum() {
	type T<'a, 'b> = ComplexGenericEnum::<'a, 'b, (), ()>;

	test_size::<T>(1);

	test_labels::<T>(&["ComplexGenericEnum::Third"]);

	test_variants(
		&[
			(T::First(&()), &[false]),
			(T::Second { _field: &() }, &[false]),
			(T::Third, &[true]),
		]
	);
}
