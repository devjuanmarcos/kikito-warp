use super::{DiffLine, DiffLineKind, parse_unified_diff};

#[test]
fn parses_replacements_insertions_and_context_into_aligned_rows() {
    let output = concat!(
        "diff --git a/file.txt b/file.txt\n",
        "index 1111111..2222222 100644\n",
        "--- a/file.txt\n",
        "+++ b/file.txt\n",
        "@@ -1,3 +1,4 @@\n",
        " unchanged\n",
        "-old line\n",
        "+new line\n",
        "+inserted line\n",
        " final line\n",
    );

    assert_eq!(
        parse_unified_diff(output),
        [
            DiffLine {
                old_line: Some(1),
                new_line: Some(1),
                old_text: Some("unchanged".into()),
                new_text: Some("unchanged".into()),
                kind: DiffLineKind::Context,
            },
            DiffLine {
                old_line: Some(2),
                new_line: Some(2),
                old_text: Some("old line".into()),
                new_text: Some("new line".into()),
                kind: DiffLineKind::Changed,
            },
            DiffLine {
                old_line: None,
                new_line: Some(3),
                old_text: None,
                new_text: Some("inserted line".into()),
                kind: DiffLineKind::Changed,
            },
            DiffLine {
                old_line: Some(3),
                new_line: Some(4),
                old_text: Some("final line".into()),
                new_text: Some("final line".into()),
                kind: DiffLineKind::Context,
            },
        ]
    );
}

#[test]
fn parses_multiple_hunks_and_empty_file_ranges() {
    let output = concat!("@@ -0,0 +1 @@\n", "+first\n", "@@ -4 +4,0 @@\n", "-last\n",);
    let parsed = parse_unified_diff(output);

    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].new_line, Some(1));
    assert_eq!(parsed[1].old_line, Some(4));
    assert_eq!(parsed[1].new_line, None);
}

#[test]
fn parses_patch_fragments_with_shared_file_headers() {
    let diff = concat!(
        "diff --git a/file.txt b/file.txt\n",
        "index 1111111..2222222 100644\n",
        "--- a/file.txt\n",
        "+++ b/file.txt\n",
        "@@ -1 +1 @@\n",
        "-before\n",
        "+first\n",
        "@@ -20 +20 @@\n",
        "-before last\n",
        "+second last\n",
    );
    let hunks = super::parse_diff_hunks(diff);

    assert_eq!(hunks.len(), 2);
    assert!(hunks[0].starts_with("diff --git a/file.txt b/file.txt\n"));
    assert!(hunks[0].contains("@@ -1 +1 @@\n"));
    assert!(!hunks[0].contains("@@ -20 +20 @@"));
    assert!(hunks[1].starts_with("diff --git a/file.txt b/file.txt\n"));
    assert!(hunks[1].contains("@@ -20 +20 @@\n"));
}
