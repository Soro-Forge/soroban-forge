use brace_depth::analyze;

const CLEAN: &str = include_str!("../fixtures/clean.rs");
const UNCLOSED_IMPL: &str = include_str!("../fixtures/unclosed_impl.rs");
const STRAY_CLOSE: &str = include_str!("../fixtures/stray_close.rs");

#[test]
fn balanced_source_produces_no_findings() {
    assert_eq!(analyze(CLEAN), Vec::new());
}

#[test]
fn unclosed_impl_is_reported_at_its_opening_line() {
    let findings = analyze(UNCLOSED_IMPL);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].line, 4);
    assert_eq!(findings[0].depth, 1);
    assert!(findings[0].message.contains("never closed"));
}

#[test]
fn stray_closing_brace_is_reported() {
    let findings = analyze(STRAY_CLOSE);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].line, 6);
    assert!(findings[0].message.contains("no matching opening brace"));
}

#[test]
fn braces_inside_string_literals_are_ignored() {
    assert_eq!(analyze("fn a() { let s = \"{{{\"; }"), Vec::new());
}

#[test]
fn braces_inside_raw_strings_are_ignored() {
    assert_eq!(analyze("fn a() { let s = r#\"{ \" {\"#; }"), Vec::new());
}

#[test]
fn braces_inside_comments_are_ignored() {
    assert_eq!(analyze("fn a() {\n // {\n /* { /* { */ */\n}"), Vec::new());
}

#[test]
fn lifetimes_are_not_mistaken_for_char_literals() {
    assert_eq!(analyze("struct S<'a> { v: &'a str }"), Vec::new());
}

#[test]
fn unicode_escape_braces_are_ignored() {
    assert_eq!(analyze("fn a() { let c = '\\u{1F600}'; }"), Vec::new());
}

#[test]
fn escaped_quote_char_literal_is_handled() {
    assert_eq!(analyze("fn a() { let c = '\\''; }"), Vec::new());
}

#[test]
fn deeply_unbalanced_source_reports_the_outermost_brace() {
    let findings = analyze("mod outer {\n    impl Thing {\n        fn f() {\n");

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].line, 1);
    assert_eq!(findings[0].depth, 3);
}
