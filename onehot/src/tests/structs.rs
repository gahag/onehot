use super::{
	*,
	enums::{SimpleEnum, SimpleGenericEnum, ComplexGenericEnum},
};
use crate::OneHot;


#[derive(OneHot)]
struct SimpleStruct {
	f1: SimpleEnum,
	f2: SimpleEnum,
}


#[derive(OneHot)]
pub struct SimpleGenericStruct<T: OneHot>(SimpleEnum, T);


#[derive(OneHot)]
pub(super) struct ComplexGenericStruct<'a, 'b: 'a, T, U>
where
	T: 'a + Copy + OneHot,
	U: 'b + Into<T> + Sync,
{
	#[onehot(ignore)]
	_f1: SimpleEnum,
	f2: bool,
	f3: ComplexGenericEnum::<'a, 'b, T, U>,
	f4: SimpleGenericEnum::<T>,
	f5: &'a T,
}


#[test]
fn test_simple_struct() {
	type T = SimpleStruct;

	test_size::<T>(4);

	test_labels::<T>(&[
		"SimpleEnum::First",
		"SimpleEnum::Second",
		"SimpleEnum::First",
		"SimpleEnum::Second"
	]);

	test_variants(
		&[
			(T { f1: SimpleEnum::First,  f2: SimpleEnum::Second }, &[true, false, false, true]),
			(T { f1: SimpleEnum::Second, f2: SimpleEnum::Third },  &[false, true, false, false]),
			(T { f1: SimpleEnum::Third,  f2: SimpleEnum::First },  &[false, false, true, false]),
		]
	);
}


#[test]
fn test_simple_generic_struct() {
	type T = SimpleGenericStruct::<bool>;

	test_size::<T>(3);

	test_labels::<T>(&["SimpleEnum::First", "SimpleEnum::Second", "bool"]);

	test_variants(
		&[
			(SimpleGenericStruct(SimpleEnum::First, false), &[true, false, false]),
			(SimpleGenericStruct(SimpleEnum::Second, true), &[false, true, true]),
			(SimpleGenericStruct(SimpleEnum::Third, true), &[false, false, true]),
		]
	);
}


#[test]
fn test_complex_generic_struct() {
	type T<'a, 'b> = ComplexGenericStruct::<'a, 'b, bool, bool>;

	test_size::<T>(5);

	test_labels::<T>(&[
		"bool",
		"ComplexGenericEnum::Third",
		"SimpleGenericEnum::Second",
		"SimpleGenericEnum::Third",
		"bool",
	]);

	test_variants(
		&[
			(
				T {
					_f1: SimpleEnum::First,
					f2: false,
					f3: ComplexGenericEnum::Third,
					f4: SimpleGenericEnum::Second,
					f5: &false
				},
				&[false, true, true, false, false]
			),
			(
				T {
					_f1: SimpleEnum::First,
					f2: true,
					f3: ComplexGenericEnum::First(&false),
					f4: SimpleGenericEnum::Third,
					f5: &true,
				},
				&[true, false, false, true, true]
			),
			(
				T {
					_f1: SimpleEnum::First,
					f2: false,
					f3: ComplexGenericEnum::Second { _field: &false },
					f4: SimpleGenericEnum::First(false),
					f5: &false,
				},
				&[false, false, false, false, false]
			),
		]
	);
}
