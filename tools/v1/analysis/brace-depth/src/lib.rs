//! Detects unbalanced brace depth in Rust source that does not compile.
//!
//! The input is treated as text rather than parsed, because this check exists
//! precisely for files that no parser will accept. Braces occurring inside
//! comments, string literals, raw string literals and char literals are
//! ignored, and an apostrophe introducing a lifetime is distinguished from one
//! introducing a char literal.

/// A structural problem discovered in the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// Line on which the problem originates, counting from one.
    pub line: usize,
    /// Brace depth remaining at the end of the file.
    pub depth: usize,
    /// Human readable explanation.
    pub message: String,
}

/// Scans `source` and reports any brace imbalance.
///
/// An empty vector means the source is balanced.
pub fn analyze(source: &str) -> Vec<Finding> {
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();

    let mut findings = Vec::new();
    let mut open_lines: Vec<usize> = Vec::new();
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
            let mut nesting = 1usize;
            i += 2;
            while i < len && nesting > 0 {
                if chars[i] == '\n' {
                    line += 1;
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
            open_lines.push(line);
            i += 1;
            continue;
        }

        if c == '}' {
            if open_lines.pop().is_none() {
                findings.push(Finding {
                    line,
                    depth: 0,
                    message: "closing `}` with no matching opening brace".to_string(),
                });
            }
            i += 1;
            continue;
        }

        i += 1;
    }

    if let Some(&first) = open_lines.first() {
        let depth = open_lines.len();
        findings.push(Finding {
            line: first,
            depth,
            message: format!(
                "brace depth ends at {depth}; the `{{` opened here was never closed"
            ),
        });
    }

    findings
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
