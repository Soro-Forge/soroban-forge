use duplicate_fn::analyze;

const DUPLICATED_IN_IMPL: &str = include_str!("../fixtures/duplicated_in_impl.rs");
const SAME_NAME_DISTINCT_IMPLS: &str = include_str!("../fixtures/same_name_distinct_impls.rs");

#[test]
fn duplicate_definitions_in_one_impl_are_reported() {
    let duplicates = analyze(DUPLICATED_IN_IMPL);

    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].name, "get_escrow");
    assert_eq!(duplicates[0].lines, vec![5, 13]);
}

#[test]
fn same_name_in_distinct_impls_is_not_reported() {
    assert_eq!(analyze(SAME_NAME_DISTINCT_IMPLS), Vec::new());
}

#[test]
fn duplicate_free_functions_are_reported() {
    let duplicates = analyze("fn a() {}\nfn a() {}\n");

    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].lines, vec![1, 2]);
}

#[test]
fn distinct_function_names_are_not_reported() {
    assert_eq!(analyze("fn a() {}\nfn b() {}\n"), Vec::new());
}

#[test]
fn a_function_keyword_inside_a_string_is_ignored() {
    assert_eq!(analyze("fn a() {\n let s = \"fn a\";\n}\n"), Vec::new());
}

#[test]
fn a_function_keyword_inside_a_comment_is_ignored() {
    assert_eq!(analyze("fn a() {}\n// fn a() {}\n"), Vec::new());
}

#[test]
fn an_identifier_ending_in_fn_is_not_a_definition() {
    assert_eq!(analyze("fn a() {}\nfn my_fn() {}\n"), Vec::new());
}

#[test]
fn a_function_pointer_type_is_not_a_definition() {
    assert_eq!(analyze("fn a(f: fn(u32) -> u32) {}\n"), Vec::new());
}

#[test]
fn nested_functions_belong_to_their_parent_body() {
    let duplicates = analyze("fn outer() {\n    fn inner() {}\n    fn inner() {}\n}\n");

    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].name, "inner");
    assert_eq!(duplicates[0].lines, vec![2, 3]);
}

#[test]
fn lifetimes_do_not_disturb_scanning() {
    assert_eq!(analyze("impl<'a> T<'a> {\n    fn v(&'a self) {}\n}\n"), Vec::new());
}
