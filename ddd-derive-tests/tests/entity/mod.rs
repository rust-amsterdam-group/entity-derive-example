use test_case::test_case;

#[test_case("tests/entity/named_struct_entity.rs"; "derive entity for a struct with named fields")]
fn entity_compilation_success(src_file: &str) {
    let t = trybuild::TestCases::new();
    t.pass(src_file);
}

#[test_case("tests/entity/enum_entity.rs"; "derive entity for an enum")]
#[test_case("tests/entity/empty_struct_entity.rs"; "derive entity for an empty struct")]
#[test_case("tests/entity/unnamed_struct_entity.rs"; "derive entity for a struct with unnamed fields")]
#[test_case("tests/entity/unit_struct_entity.rs"; "derive entity for a unit struct")]
#[test_case("tests/entity/named_struct_entity_with_generics.rs"; "derive entity for a struct with named fields and generics")]
fn entity_compilation_fail(src_file: &str) {
    let t = trybuild::TestCases::new();
    t.compile_fail(src_file);
}

// #[test]
// fn tests_without_test_case() {
//     let t = trybuild::TestCases::new();
//     t.compile_fail("tests/entity/enum_entity.rs");
//     t.compile_fail("tests/entity/empty_struct_entity.rs");
//     t.compile_fail("tests/entity/unnamed_struct_entity.rs");
//     t.compile_fail("tests/entity/unit_struct_entity.rs");
//     t.pass("tests/entity/named_struct_entity.rs");
// }
