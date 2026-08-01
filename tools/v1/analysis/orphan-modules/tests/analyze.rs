use orphan_modules::{analyze, declared_modules};

const DECLARATIONS: &str = include_str!("../fixtures/tests_declarations.rs");

#[test]
fn files_missing_from_the_declaration_file_are_reported() {
    let files = [
        "admin.rs",
        "funding.rs",
        "coverage.rs",
        "integration.rs",
        "funding_events.rs",
        "pauser_version_view.rs",
        "settlement_upgrade_auth.rs",
    ];

    let orphans = analyze(DECLARATIONS, &files);
    let modules: Vec<&str> = orphans.iter().map(|o| o.module.as_str()).collect();

    let expected = vec![
        "funding_events",
        "pauser_version_view",
        "settlement_upgrade_auth",
    ];
    assert_eq!(modules, expected);
}

#[test]
fn a_fully_declared_directory_reports_nothing() {
    let files = ["admin.rs", "funding.rs", "coverage.rs", "integration.rs"];

    assert_eq!(analyze(DECLARATIONS, &files), Vec::new());
}

#[test]
fn the_orphan_carries_both_the_file_and_the_module_name() {
    let orphans = analyze("mod admin;\n", &["funding_events.rs"]);

    assert_eq!(orphans.len(), 1);
    assert_eq!(orphans[0].file, "funding_events.rs");
    assert_eq!(orphans[0].module, "funding_events");
}

#[test]
fn module_roots_and_non_rust_files_are_ignored() {
    let files = ["mod.rs", "lib.rs", "main.rs", "notes.txt", "README.md"];

    assert_eq!(analyze("", &files), Vec::new());
}

#[test]
fn a_commented_out_declaration_does_not_count() {
    let orphans = analyze("// mod admin;\n", &["admin.rs"]);

    assert_eq!(orphans.len(), 1);
    assert_eq!(orphans[0].module, "admin");
}

#[test]
fn a_declaration_in_a_block_comment_does_not_count() {
    let orphans = analyze("/* mod admin; */\n", &["admin.rs"]);

    assert_eq!(orphans.len(), 1);
}

#[test]
fn an_inline_module_counts_as_declared() {
    assert_eq!(analyze("mod admin { }\n", &["admin.rs"]), Vec::new());
}

#[test]
fn a_public_declaration_counts() {
    assert_eq!(analyze("pub mod admin;\n", &["admin.rs"]), Vec::new());
}

#[test]
fn an_attribute_before_the_declaration_does_not_hide_it() {
    let source = "#[rustfmt::skip]\nmod coverage;\n";

    assert_eq!(analyze(source, &["coverage.rs"]), Vec::new());
}

#[test]
fn a_declaration_inside_a_string_does_not_count() {
    let orphans = analyze("let s = \"mod admin;\";\n", &["admin.rs"]);

    assert_eq!(orphans.len(), 1);
}

#[test]
fn an_identifier_containing_mod_is_not_a_declaration() {
    let orphans = analyze("fn module_admin() {}\n", &["admin.rs"]);

    assert_eq!(orphans.len(), 1);
}

#[test]
fn declared_modules_returns_names_in_order() {
    let declared = declared_modules(DECLARATIONS);
    let expected = vec!["admin", "funding", "coverage", "integration"];

    assert_eq!(declared, expected);
}
