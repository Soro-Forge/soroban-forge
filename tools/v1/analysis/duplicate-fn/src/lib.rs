//! Detects functions defined more than once within a single block.
//!
//! The input is treated as text rather than parsed, because the files this
//! check exists for frequently do not compile. Definitions are attributed to
//! their enclosing block rather than to their brace depth, so the same method
//! name appearing in two different `impl` blocks is not reported.

/// A function name defined more than once within one block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Duplicate {
    /// The repeated function name.
    pub name: String,
    /// Lines on which each definition appears, counting from one.
    pub lines: Vec<usize>,
}

/// A single function definition and the block that encloses it.
struct Definition {
    scope: usize,
    name: String,
    line: usize,
}

/// Scans `source` and reports functions defined more than once in one block.
///
/// An empty vector means no block defines the same function twice.
pub fn analyze(source: &str) -> Vec<Duplicate> {
    let definitions = collect_definitions(source);

    let mut duplicates = Vec::new();
    let mut handled: Vec<(usize, String)> = Vec::new();

    for definition in &definitions {
        let key = (definition.scope, definition.name.clone());
        if handled.contains(&key) {
            continue;
        }

        let lines: Vec<usize> = definitions
            .iter()
            .filter(|other| other.scope == definition.scope && other.name == definition.name)
            .map(|other| other.line)
            .collect();

        if lines.len() > 1 {
            let name = definition.name.clone();
            duplicates.push(Duplicate { name, lines });
        }

        handled.push(key);
    }

    duplicates
}

/// Walks the source and records every function definition it finds.
fn collect_definitions(source: &str) -> Vec<Definition> {
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();

    let mut definitions = Vec::new();
    let mut scopes: Vec<usize> = vec![0];
    let mut next_scope = 1usize;
    let mut line = 1usize;
    let mut i = 0usize;

    while i < len {
        let c = chars[i];

        if c == '\n' {
            line += 1;
            i += 1;
            continue;
        }

        if c == '/' && i + 1 < len && chars[i + 1] == '/' {
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        if c == '/' && i + 1 < len && chars[i + 1] == '*' {
            i = skip_block_comment(&chars, i, &mut line);
            continue;
        }

        if c == 'r' && i + 1 < len && (chars[i + 1] == '"' || chars[i + 1] == '#') {
            if let Some(next) = skip_raw_string(&chars, i, &mut line) {
                i = next;
                continue;
            }
        }

        if c == '"' {
            i = skip_string(&chars, i, &mut line);
            continue;
        }

        if c == '\'' {
            i = skip_quote(&chars, i);
            continue;
        }

        if c == '{' {
            scopes.push(next_scope);
            next_scope += 1;
            i += 1;
            continue;
        }

        if c == '}' {
            if scopes.len() > 1 {
                scopes.pop();
            }
            i += 1;
            continue;
        }

        if is_ident_start(c) {
            let (word, after_word) = read_ident(&chars, i);
            i = after_word;

            if word != "fn" {
                continue;
            }

            let fn_line = line;
            let mut j = i;
            let mut newlines = 0usize;
            while j < len && chars[j].is_whitespace() {
                if chars[j] == '\n' {
                    newlines += 1;
                }
                j += 1;
            }
            line += newlines;
            i = j;

            if i < len && is_ident_start(chars[i]) {
                let (name, after_name) = read_ident(&chars, i);
                let scope = *scopes.last().unwrap_or(&0);
                definitions.push(Definition {
                    scope,
                    name,
                    line: fn_line,
                });
                i = after_name;
            }

            continue;
        }

        i += 1;
    }

    definitions
}

/// Returns true if `c` may begin a Rust identifier.
fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

/// Reads the identifier beginning at `start`, returning it and the index past it.
fn read_ident(chars: &[char], start: usize) -> (String, usize) {
    let len = chars.len();
    let mut i = start;
    let mut ident = String::new();

    while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
        ident.push(chars[i]);
        i += 1;
    }

    (ident, i)
}

/// Skips a possibly nested block comment, returning the index just past it.
fn skip_block_comment(chars: &[char], start: usize, line: &mut usize) -> usize {
    let len = chars.len();
    let mut nesting = 1usize;
    let mut i = start + 2;

    while i < len && nesting > 0 {
        if chars[i] == '\n' {
            *line += 1;
            i += 1;
        } else if chars[i] == '/' && i + 1 < len && chars[i + 1] == '*' {
            nesting += 1;
            i += 2;
        } else if chars[i] == '*' && i + 1 < len && chars[i + 1] == '/' {
            nesting -= 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    i
}

/// Skips a raw string beginning at `start`, returning the index just past it.
///
/// Returns `None` if `start` does not in fact begin a raw string.
fn skip_raw_string(chars: &[char], start: usize, line: &mut usize) -> Option<usize> {
    let len = chars.len();
    let mut j = start + 1;
    let mut hashes = 0usize;

    while j < len && chars[j] == '#' {
        hashes += 1;
        j += 1;
    }

    if j >= len || chars[j] != '"' {
        return None;
    }

    let mut i = j + 1;
    while i < len {
        if chars[i] == '\n' {
            *line += 1;
            i += 1;
            continue;
        }

        if chars[i] == '"' {
            let mut k = i + 1;
            let mut seen = 0usize;
            while k < len && seen < hashes && chars[k] == '#' {
                seen += 1;
                k += 1;
            }
            if seen == hashes {
                return Some(k);
            }
        }

        i += 1;
    }

    Some(len)
}

/// Skips an ordinary string beginning at `start`, returning the index just past it.
fn skip_string(chars: &[char], start: usize, line: &mut usize) -> usize {
    let len = chars.len();
    let mut i = start + 1;

    while i < len {
        match chars[i] {
            '\\' => i += 2,
            '\n' => {
                *line += 1;
                i += 1;
            }
            '"' => return i + 1,
            _ => i += 1,
        }
    }

    len
}

/// Skips a char literal beginning at `start`, returning the index just past it.
///
/// An apostrophe that introduces a lifetime rather than a char literal consumes
/// only itself, leaving the identifier to be scanned normally.
fn skip_quote(chars: &[char], start: usize) -> usize {
    let len = chars.len();

    if start + 1 < len && chars[start + 1] == '\\' {
        let mut j = start + 3;
        while j < len && chars[j] != '\'' && chars[j] != '\n' {
            j += 1;
        }
        if j < len && chars[j] == '\'' {
            return j + 1;
        }
        return start + 1;
    }

    if start + 2 < len && chars[start + 2] == '\'' {
        return start + 3;
    }

    start + 1
}
