//! Detects enum variants that resolve to the same discriminant value.
//!
//! `rustc` reports this as E0081, but only once the crate parses. A file
//! holding an unbalanced brace never reaches that stage, so every collision in
//! it stays hidden behind the parse error. This check reads the text directly
//! and reports collisions whether or not the crate compiles.

/// Two or more variants of one enum resolving to the same discriminant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collision {
    /// The enum holding the colliding variants.
    pub enum_name: String,
    /// The discriminant the variants share.
    pub value: i64,
    /// The colliding variant names, in declaration order.
    pub variants: Vec<String>,
    /// The line each colliding variant is declared on.
    pub lines: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Variant {
    name: String,
    line: usize,
    value: Option<i64>,
}

type Declaration = (String, Vec<Variant>);
type Group = (i64, Vec<Variant>);

/// Reports every discriminant collision in `source`, enum by enum.
pub fn analyze(source: &str) -> Vec<Collision> {
    let mut findings = Vec::new();

    for (name, variants) in declarations(source) {
        findings.extend(collisions(&name, &variants));
    }

    findings
}

fn collisions(enum_name: &str, variants: &[Variant]) -> Vec<Collision> {
    let mut groups: Vec<Group> = Vec::new();

    for variant in variants {
        let value = match variant.value {
            Some(value) => value,
            None => continue,
        };

        match groups.iter_mut().find(|(seen, _)| *seen == value) {
            Some((_, members)) => members.push(variant.clone()),
            None => groups.push((value, vec![variant.clone()])),
        }
    }

    groups
        .into_iter()
        .filter(|(_, members)| members.len() > 1)
        .map(|(value, members)| Collision {
            enum_name: enum_name.to_string(),
            value,
            variants: members.iter().map(|m| m.name.clone()).collect(),
            lines: members.iter().map(|m| m.line).collect(),
        })
        .collect()
}

fn declarations(source: &str) -> Vec<Declaration> {
    let chars: Vec<char> = source.chars().collect();
    let mut found = Vec::new();
    let mut index = 0;
    let mut line = 1;

    while index < chars.len() {
        let current = chars[index];

        if current == '\n' {
            line += 1;
            index += 1;
            continue;
        }

        if let Some(next) = skip_trivia(&chars, index, &mut line) {
            index = next;
            continue;
        }

        if is_ident_start(current) {
            let (word, next) = read_ident(&chars, index);

            if word == "enum" {
                if let Some((name, body)) = header(&chars, next, &mut line) {
                    let (variants, after) = read_variants(&chars, body, &mut line);
                    found.push((name, variants));
                    index = after;
                    continue;
                }
            }

            index = next;
            continue;
        }

        index += 1;
    }

    found
}

/// Reads the enum name and returns the index just past its opening brace.
fn header(chars: &[char], start: usize, line: &mut usize) -> Option<(String, usize)> {
    let mut index = start;

    while index < chars.len() {
        let current = chars[index];

        if current == '\n' {
            *line += 1;
            index += 1;
            continue;
        }

        if current.is_whitespace() {
            index += 1;
            continue;
        }

        if let Some(next) = skip_trivia(chars, index, line) {
            index = next;
            continue;
        }

        if is_ident_start(current) {
            let (name, next) = read_ident(chars, index);
            let body = opening_brace(chars, next, line)?;
            return Some((name, body));
        }

        return None;
    }

    None
}

fn opening_brace(chars: &[char], start: usize, line: &mut usize) -> Option<usize> {
    let mut index = start;

    while index < chars.len() {
        let current = chars[index];

        if current == '\n' {
            *line += 1;
            index += 1;
            continue;
        }

        if current == '{' {
            return Some(index + 1);
        }

        if current == ';' {
            return None;
        }

        if let Some(next) = skip_trivia(chars, index, line) {
            index = next;
            continue;
        }

        index += 1;
    }

    None
}

/// Walks the enum body, returning its variants and the index past the close.
fn read_variants(chars: &[char], start: usize, line: &mut usize) -> (Vec<Variant>, usize) {
    let mut variants = Vec::new();
    let mut depth = 0usize;
    let mut name: Option<(String, usize)> = None;
    let mut value_text = String::new();
    let mut seen_equals = false;
    let mut next_value = Some(0);
    let mut index = start;

    while index < chars.len() {
        let current = chars[index];

        if current == '\n' {
            *line += 1;
            index += 1;
            continue;
        }

        if let Some(next) = skip_trivia(chars, index, line) {
            index = next;
            continue;
        }

        if depth == 0 && current == '#' {
            index = skip_attribute(chars, index, line);
            continue;
        }

        if depth == 0 && is_ident_start(current) {
            let (word, next) = read_ident(chars, index);

            if seen_equals {
                value_text.push_str(&word);
            } else if name.is_none() {
                name = Some((word, *line));
            }

            index = next;
            continue;
        }

        match current {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' => depth = depth.saturating_sub(1),
            '}' if depth > 0 => depth -= 1,
            '}' => {
                let entry = Entry {
                    name: name.take(),
                    has_value: seen_equals,
                    value_text: &value_text,
                };
                push_variant(&mut variants, &mut next_value, entry);
                return (variants, index + 1);
            }
            ',' if depth == 0 => {
                let entry = Entry {
                    name: name.take(),
                    has_value: seen_equals,
                    value_text: &value_text,
                };
                push_variant(&mut variants, &mut next_value, entry);
                value_text.clear();
                seen_equals = false;
            }
            '=' if depth == 0 => seen_equals = true,
            _ if depth == 0 && seen_equals => value_text.push(current),
            _ => {}
        }

        index += 1;
    }

    (variants, index)
}

struct Entry<'a> {
    name: Option<(String, usize)>,
    has_value: bool,
    value_text: &'a str,
}

fn push_variant(variants: &mut Vec<Variant>, next_value: &mut Option<i64>, entry: Entry<'_>) {
    let (name, line) = match entry.name {
        Some(name) => name,
        None => return,
    };

    let value = if entry.has_value {
        parse_discriminant(entry.value_text.trim())
    } else {
        *next_value
    };

    *next_value = value.map(|current| current + 1);
    variants.push(Variant { name, line, value });
}

/// Reads an integer literal, or `None` when the discriminant is an expression.
fn parse_discriminant(text: &str) -> Option<i64> {
    let mut body = text.trim();
    let negative = body.starts_with('-');

    if negative {
        body = body[1..].trim();
    }

    let (radix, rest) = radix_of(body);
    let mut digits = String::new();
    let mut tail = "";

    for (offset, character) in rest.char_indices() {
        if character == '_' {
            continue;
        }

        if character.is_digit(radix) {
            digits.push(character);
            continue;
        }

        tail = &rest[offset..];
        break;
    }

    if digits.is_empty() {
        return None;
    }

    if !tail.is_empty() && !is_int_suffix(tail) {
        return None;
    }

    let parsed = i64::from_str_radix(&digits, radix).ok()?;

    Some(if negative { -parsed } else { parsed })
}

fn radix_of(body: &str) -> (u32, &str) {
    if let Some(rest) = body.strip_prefix("0x") {
        return (16, rest);
    }

    if let Some(rest) = body.strip_prefix("0b") {
        return (2, rest);
    }

    if let Some(rest) = body.strip_prefix("0o") {
        return (8, rest);
    }

    (10, body)
}

fn is_int_suffix(text: &str) -> bool {
    let signed = text.strip_prefix('i');
    let width = match signed.or_else(|| text.strip_prefix('u')) {
        Some(width) => width,
        None => return false,
    };

    matches!(width, "8" | "16" | "32" | "64" | "128" | "size")
}

fn skip_attribute(chars: &[char], start: usize, line: &mut usize) -> usize {
    let mut index = start + 1;

    if chars.get(index) == Some(&'!') {
        index += 1;
    }

    if chars.get(index) != Some(&'[') {
        return start + 1;
    }

    let mut depth = 0usize;

    while index < chars.len() {
        match chars[index] {
            '\n' => *line += 1,
            '[' => depth += 1,
            ']' => {
                depth -= 1;

                if depth == 0 {
                    return index + 1;
                }
            }
            _ => {}
        }

        index += 1;
    }

    index
}

/// Skips a comment, string or character literal, if one starts here.
fn skip_trivia(chars: &[char], index: usize, line: &mut usize) -> Option<usize> {
    match chars[index] {
        '/' if chars.get(index + 1) == Some(&'/') => Some(skip_line_comment(chars, index)),
        '/' if chars.get(index + 1) == Some(&'*') => Some(skip_block_comment(chars, index, line)),
        '"' => Some(skip_string(chars, index, line)),
        '\'' => Some(skip_quote(chars, index)),
        'r' => skip_raw_string(chars, index, line),
        _ => None,
    }
}

fn skip_line_comment(chars: &[char], start: usize) -> usize {
    let mut index = start + 2;

    while index < chars.len() && chars[index] != '\n' {
        index += 1;
    }

    index
}

fn skip_block_comment(chars: &[char], start: usize, line: &mut usize) -> usize {
    let mut index = start + 2;

    while index + 1 < chars.len() {
        if chars[index] == '\n' {
            *line += 1;
        }

        if chars[index] == '*' && chars[index + 1] == '/' {
            return index + 2;
        }

        index += 1;
    }

    chars.len()
}

fn skip_string(chars: &[char], start: usize, line: &mut usize) -> usize {
    let mut index = start + 1;

    while index < chars.len() {
        match chars[index] {
            '\\' => index += 2,
            '\n' => {
                *line += 1;
                index += 1;
            }
            '"' => return index + 1,
            _ => index += 1,
        }
    }

    chars.len()
}

fn skip_raw_string(chars: &[char], start: usize, line: &mut usize) -> Option<usize> {
    let mut hashes = 0;
    let mut index = start + 1;

    while chars.get(index) == Some(&'#') {
        hashes += 1;
        index += 1;
    }

    if chars.get(index) != Some(&'"') {
        return None;
    }

    index += 1;

    while index < chars.len() {
        if chars[index] == '\n' {
            *line += 1;
        }

        if chars[index] == '"' {
            let mut closing = 0;

            while closing < hashes && chars.get(index + 1 + closing) == Some(&'#') {
                closing += 1;
            }

            if closing == hashes {
                return Some(index + 1 + hashes);
            }
        }

        index += 1;
    }

    Some(chars.len())
}

fn skip_quote(chars: &[char], start: usize) -> usize {
    if chars.get(start + 1) == Some(&'\\') {
        let mut index = start + 2;

        while index < chars.len() && chars[index] != '\'' {
            index += 1;
        }

        return index + 1;
    }

    if chars.get(start + 2) == Some(&'\'') {
        return start + 3;
    }

    start + 1
}

fn is_ident_start(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

fn read_ident(chars: &[char], start: usize) -> (String, usize) {
    let mut index = start;
    let mut word = String::new();

    while index < chars.len() {
        let current = chars[index];

        if !current.is_alphanumeric() && current != '_' {
            break;
        }

        word.push(current);
        index += 1;
    }

    (word, index)
}
