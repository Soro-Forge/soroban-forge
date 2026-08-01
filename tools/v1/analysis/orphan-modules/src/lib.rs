//! Detects source files that no module declaration references.
//!
//! Rust only compiles a file if some module declares it. A test file added to
//! a directory but never added to `mod.rs` is never compiled and never run,
//! and the suite still passes. This check compares the declarations in one
//! file against the files present alongside it.
//!
//! The declarations are scanned as text rather than parsed, so the check works
//! on a crate that does not currently compile.

/// A file that no module declaration references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Orphan {
    /// The file name as supplied, such as `funding_events.rs`.
    pub file: String,
    /// The module name that would declare it, such as `funding_events`.
    pub module: String,
}

/// Returns the files in `file_names` that `declarations` never declares.
///
/// `mod.rs`, `lib.rs`, `main.rs` and non-Rust files are ignored. Reading the
/// directory is the caller's responsibility.
pub fn analyze(declarations: &str, file_names: &[&str]) -> Vec<Orphan> {
    let declared = declared_modules(declarations);
    let mut orphans = Vec::new();

    for file in file_names {
        let Some(module) = module_name_of(file) else {
            continue;
        };

        if declared.iter().any(|declared| declared == &module) {
            continue;
        }

        let file = (*file).to_string();
        orphans.push(Orphan { file, module });
    }

    orphans
}

/// Returns every module name declared in `source`, in the order they appear.
///
/// Both `mod name;` and `mod name { ... }` count. Declarations inside comments
/// or string literals do not.
pub fn declared_modules(source: &str) -> Vec<String> {
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();

    let mut modules = Vec::new();
    let mut i = 0usize;

    while i < len {
        let c = chars[i];

        if c == '/' && i + 1 < len && chars[i + 1] == '/' {
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        if c == '/' && i + 1 < len && chars[i + 1] == '*' {
            i = skip_block_comment(&chars, i);
            continue;
        }

        if c == 'r' && i + 1 < len && (chars[i + 1] == '"' || chars[i + 1] == '#') {
            if let Some(next) = skip_raw_string(&chars, i) {
                i = next;
                continue;
            }
        }

        if c == '"' {
            i = skip_string(&chars, i);
            continue;
        }

        if c == '\'' {
            i = skip_quote(&chars, i);
            continue;
        }

        if is_ident_start(c) {
            let (word, after_word) = read_ident(&chars, i);
            i = after_word;

            if word != "mod" {
                continue;
            }

            while i < len && chars[i].is_whitespace() {
                i += 1;
            }

            if i < len && is_ident_start(chars[i]) {
                let (name, after_name) = read_ident(&chars, i);
                modules.push(name);
                i = after_name;
            }

            continue;
        }

        i += 1;
    }

    modules
}

/// Returns the module name a file would be declared as, if it needs declaring.
fn module_name_of(file: &str) -> Option<String> {
    let stem = file.strip_suffix(".rs")?;

    if stem.is_empty() || stem == "mod" || stem == "lib" || stem == "main" {
        return None;
    }

    Some(stem.to_string())
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
fn skip_block_comment(chars: &[char], start: usize) -> usize {
    let len = chars.len();
    let mut nesting = 1usize;
    let mut i = start + 2;

    while i < len && nesting > 0 {
        if chars[i] == '/' && i + 1 < len && chars[i + 1] == '*' {
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
fn skip_raw_string(chars: &[char], start: usize) -> Option<usize> {
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
fn skip_string(chars: &[char], start: usize) -> usize {
    let len = chars.len();
    let mut i = start + 1;

    while i < len {
        match chars[i] {
            '\\' => i += 2,
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
