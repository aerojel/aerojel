// use jelliicore::db::entry::{DbEntryKey, DbEntryValue};
// use jelliicore_macros::{DbEntryKey, DbEntryValue};

// macro_rules! serialize_primitive {
//     ($ty:ty, $num:expr) => {
//         {
//             let primitive: $ty = $num;

//             assert_eq!(<$ty>::deserialize(&<$ty>::serialize(&primitive)), primitive);
//         }
//     };
// }

// macro_rules! compare_primitive {
//     ($ty:ty, $num:expr) => {
//         {
//             let primitive: $ty = $num;

//             assert_eq!(
//                 <$ty>::cmp_in_place(&<$ty>::serialize(&primitive), &primitive.to_le_bytes()),
//                 std::cmp::Ordering::Equal,
//             );

//             assert_eq!(
//                 <$ty>::cmp_in_place(
//                     &<$ty>::serialize(&primitive),
//                     &(primitive + 1).to_le_bytes(),
//                 ),
//                 std::cmp::Ordering::Less,
//             );

//             assert_eq!(
//                 <$ty>::cmp_in_place(
//                     &<$ty>::serialize(&primitive),
//                     &(primitive - 1).to_le_bytes(),
//                 ),
//                 std::cmp::Ordering::Greater,
//             );
//         }
//     };
// }

// #[test]
// fn serialize_primitives() {
//     serialize_primitive!(u8, 55);
//     serialize_primitive!(u16, 5536);
//     serialize_primitive!(u32, 294967295);
//     serialize_primitive!(u64, 8446744073709551615);
//     serialize_primitive!(u128, 40282366920938463463374607431768211455);

//     serialize_primitive!(i8, -28);
//     serialize_primitive!(i16, -2768);
//     serialize_primitive!(i32, -147483648);
//     serialize_primitive!(i64, -223372036854775808);
//     serialize_primitive!(i128, -70141183460469231731687303715884105728);
// }

// #[test]
// fn compare_primitives() {
//     compare_primitive!(u8, 55);
//     compare_primitive!(u16, 5536);
//     compare_primitive!(u32, 294967295);
//     compare_primitive!(u64, 8446744073709551615);
//     compare_primitive!(u128, 40282366920938463463374607431768211455);

//     compare_primitive!(i8, -28);
//     compare_primitive!(i16, -2768);
//     compare_primitive!(i32, -147483648);
//     compare_primitive!(i64, -223372036854775808);
//     compare_primitive!(i128, -70141183460469231731687303715884105728);
// }

// #[test]
// fn test_struct() {
//     #[derive(
//         DbEntryValue,
//         DbEntryKey,
//         PartialEq, Eq,
//         Debug,
//     )]
//     struct TestStruct {
//         a: u128,
//         b: u128,
//     }

//     let test_struct = TestStruct {
//         a: 40282366920938463463374607431768211455u128,
//         b: 5536,
//     };

//     assert_eq!(TestStruct::deserialize(&TestStruct::serialize(&test_struct)), test_struct);
// }

// #[test]
// fn test_enum() {
//     #[derive(
//         DbEntryValue,
//         DbEntryKey,
//         PartialEq, Eq,
//         Debug,
//     )]
//     enum TestEnum {
//         #[db_entry(id = 0)]
//         VarA,
//         #[db_entry(id = 2)]
//         VarB,
//         #[db_entry(id = 1)]
//         VarC(u128),
//         #[db_entry(id = 34)]
//         VarD(String),
//         #[db_entry(id = 567)]
//         VarE {
//             field_a: u128,
//             field_c: String,
//         }
//     }

//     let test_enum_a = TestEnum::VarA;
//     assert_eq!(TestEnum::deserialize(&TestEnum::serialize(&test_enum_a)), test_enum_a);

//     let test_enum_b = TestEnum::VarB;
//     assert_eq!(TestEnum::deserialize(&TestEnum::serialize(&test_enum_b)), test_enum_b);

//     let test_enum_c = TestEnum::VarC(40282366920938463463374607431768211455);
//     println!("enum c: {test_enum_c:?}");
//     println!("enum c: {:?}", &TestEnum::serialize(&test_enum_c));
//     assert_eq!(TestEnum::deserialize(&TestEnum::serialize(&test_enum_c)), test_enum_c);

//     // let test_enum_d = TestEnum::VarD("test str?!".to_string());
//     // assert_eq!(TestEnum::deserialize(&TestEnum::serialize(&test_enum_d)), test_enum_d);

//     // let test_enum_e = TestEnum::VarE {
//     //     field_a: 40282366920938463463374607431768211455,
//     //     field_c: "test str?!".to_string(),
//     // };
//     // assert_eq!(TestEnum::deserialize(&TestEnum::serialize(&test_enum_e)), test_enum_e);
// }
