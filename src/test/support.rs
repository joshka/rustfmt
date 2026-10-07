/// A poor man's `indoc` for test literals, without adding a dependency.
/// Dedent an indented test literal without trimming significant whitespace.
macro_rules! dedent {
    ($text:expr) => {
        $crate::test::support::dedent_text($text)
    };
}

pub(crate) use dedent;

/// Remove boundary lines and common space indentation from test literals.
/// Preserve relative indentation, interior blank lines, and trailing spaces.
pub(crate) fn dedent_text(text: &str) -> String {
    let text = text.strip_prefix('\n').unwrap_or(text);
    let mut lines: Vec<_> = text.split('\n').collect();
    let last_line = lines.last().copied().unwrap_or("");
    if last_line.trim_matches(' ').is_empty() {
        lines.pop();
    }
    // Extra passes and allocations are fine here: this test helper favors
    // readability over performance.
    let indent = lines
        .iter()
        .filter(|line| !line.trim_matches(' ').is_empty())
        .map(|line| line.bytes().take_while(|&byte| byte == b' ').count())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| &line[indent.min(line.len())..])
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn dedent_preserves_layout_and_trailing_spaces() {
    let input = "
        first<space>
          second

        last
    ";
    let expected = "first<space>
  second

last";
    let input = input.replace("<space>", " ");
    let expected = expected.replace("<space>", " ");
    assert_eq!(dedent!(&input), expected);
}
