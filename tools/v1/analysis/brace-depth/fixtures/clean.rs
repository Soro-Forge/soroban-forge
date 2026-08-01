// A balanced file exercising every construct the scanner must ignore.

pub struct Config {
    pub name: &'static str,
}

impl Config {
    pub fn new() -> Self {
        let unbalanced_in_string = "}{";
        let raw = r#"a raw string containing { and " and }"#;
        // a brace in a line comment: {
        /* a brace in a block comment: {
           and a /* nested */ one too */
        let brace_char = '{';
        let emoji = '\u{1F600}';

        let _ = (unbalanced_in_string, raw, brace_char, emoji);

        Self { name: "ok" }
    }
}
