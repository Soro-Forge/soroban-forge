use enum_discriminants::analyze;

const COLLIDING: &str = include_str!("../fixtures/colliding_error_codes.rs");
const DISTINCT: &str = include_str!("../fixtures/distinct_error_codes.rs");

#[test]
fn variants_sharing_a_value_are_reported() {
    let found = analyze("enum Code {\n    A = 1,\n    B = 1,\n}\n");

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].value, 1);
}

#[test]
fn distinct_values_are_not_reported() {
    let found = analyze("enum Code {\n    A = 1,\n    B = 2,\n}\n");

    assert!(found.is_empty());
}

#[test]
fn the_collision_carries_every_variant_and_line() {
    let found = analyze("enum Code {\n    A = 5,\n    B = 6,\n    C = 5,\n}\n");
    let names = vec!["A".to_string(), "C".to_string()];
    let lines = vec![2, 4];

    assert_eq!(found[0].enum_name, "Code");
    assert_eq!(found[0].variants, names);
    assert_eq!(found[0].lines, lines);
}

#[test]
fn a_value_reused_across_two_enums_is_not_a_collision() {
    let source = "enum One {\n    A = 1,\n}\n\nenum Two {\n    B = 1,\n}\n";
    let found = analyze(source);

    assert!(found.is_empty());
}

#[test]
fn an_undecorated_variant_takes_the_next_value() {
    let found = analyze("enum Code {\n    A = 3,\n    B,\n    C = 4,\n}\n");
    let names = vec!["B".to_string(), "C".to_string()];

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].value, 4);
    assert_eq!(found[0].variants, names);
}

#[test]
fn two_spellings_of_one_value_collide() {
    let found = analyze("enum Code {\n    A = 0x10,\n    B = 16,\n}\n");

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].value, 16);
}

#[test]
fn an_underscore_separated_literal_is_understood() {
    let found = analyze("enum Code {\n    A = 1_000,\n    B = 1000,\n}\n");

    assert_eq!(found[0].value, 1000);
}

#[test]
fn a_suffixed_literal_is_understood() {
    let found = analyze("enum Code {\n    A = 7i64,\n    B = 7,\n}\n");

    assert_eq!(found[0].value, 7);
}

#[test]
fn a_computed_discriminant_is_left_alone() {
    let found = analyze("enum Code {\n    A = 1 << 3,\n    B = 8,\n}\n");

    assert!(found.is_empty());
}

#[test]
fn an_attribute_does_not_hide_a_variant() {
    let source = "enum Code {\n    #[cfg(test)]\n    A = 1,\n    B = 1,\n}\n";
    let found = analyze(source);
    let names = vec!["A".to_string(), "B".to_string()];

    assert_eq!(found[0].variants, names);
}

#[test]
fn a_doc_comment_is_not_mistaken_for_a_variant() {
    let source = "enum Code {\n    /// Alpha value\n    A = 1,\n    B = 1,\n}\n";
    let found = analyze(source);
    let names = vec!["A".to_string(), "B".to_string()];

    assert_eq!(found[0].variants, names);
}

#[test]
fn payload_variants_do_not_disturb_scanning() {
    let source = "enum Code {\n    P { code: u8 },\n    Q(u8, u8),\n    R = 1,\n}\n";
    let found = analyze(source);
    let names = vec!["Q".to_string(), "R".to_string()];

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].variants, names);
}

#[test]
fn an_enum_inside_a_string_is_ignored() {
    let found = analyze("let s = \"enum Fake { A = 1, B = 1 }\";\n");

    assert!(found.is_empty());
}

#[test]
fn the_colliding_fixture_reports_its_shared_code() {
    let found = analyze(COLLIDING);
    let names = vec![
        "YieldTierTableInvalid".to_string(),
        "FeesLimitOutOfRange".to_string(),
    ];

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].value, 236);
    assert_eq!(found[0].variants, names);
}

#[test]
fn the_distinct_fixture_reports_nothing() {
    let found = analyze(DISTINCT);

    assert!(found.is_empty());
}
