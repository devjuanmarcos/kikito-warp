#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum DiffLineKind {
    Context,
    Changed,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct DiffLine {
    pub old_line: Option<usize>,
    pub new_line: Option<usize>,
    pub old_text: Option<String>,
    pub new_text: Option<String>,
    pub kind: DiffLineKind,
}

pub(crate) fn parse_unified_diff(diff: &str) -> Vec<DiffLine> {
    let mut result = Vec::new();
    let mut old_line = 0usize;
    let mut new_line = 0usize;
    let mut hunk_open = false;
    let mut removed = Vec::new();
    let mut added = Vec::new();

    let flush_changes = |result: &mut Vec<DiffLine>,
                         removed: &mut Vec<(usize, String)>,
                         added: &mut Vec<(usize, String)>| {
        for index in 0..removed.len().max(added.len()) {
            let old = removed.get(index);
            let new = added.get(index);
            result.push(DiffLine {
                old_line: old.map(|(line, _)| *line),
                new_line: new.map(|(line, _)| *line),
                old_text: old.map(|(_, text)| text.clone()),
                new_text: new.map(|(_, text)| text.clone()),
                kind: DiffLineKind::Changed,
            });
        }
        removed.clear();
        added.clear();
    };

    for line in diff.lines() {
        if line.starts_with("@@ ") {
            flush_changes(&mut result, &mut removed, &mut added);
            let Some((old, new)) = parse_hunk_header(line) else {
                return Vec::new();
            };
            old_line = old;
            new_line = new;
            hunk_open = true;
            continue;
        }
        if !hunk_open || line.starts_with("\\ ") {
            continue;
        }
        match line.as_bytes().first().copied() {
            Some(b' ') => {
                flush_changes(&mut result, &mut removed, &mut added);
                result.push(DiffLine {
                    old_line: Some(old_line),
                    new_line: Some(new_line),
                    old_text: Some(line[1..].to_owned()),
                    new_text: Some(line[1..].to_owned()),
                    kind: DiffLineKind::Context,
                });
                old_line += 1;
                new_line += 1;
            }
            Some(b'-') => {
                removed.push((old_line, line[1..].to_owned()));
                old_line += 1;
            }
            Some(b'+') => {
                added.push((new_line, line[1..].to_owned()));
                new_line += 1;
            }
            _ => {
                flush_changes(&mut result, &mut removed, &mut added);
                hunk_open = false;
            }
        }
    }
    flush_changes(&mut result, &mut removed, &mut added);
    result
}

pub(crate) fn parse_diff_hunks(diff: &str) -> Vec<String> {
    let mut preamble = Vec::new();
    let mut current = None::<Vec<String>>;
    let mut hunks = Vec::new();

    for line in diff.split_inclusive('\n') {
        if line.starts_with("diff --git ") {
            if let Some(patch) = current.take() {
                hunks.push(patch.concat());
            }
            preamble.clear();
            preamble.push(line.to_owned());
        } else if line.starts_with("@@ ") {
            if let Some(patch) = current.take() {
                hunks.push(patch.concat());
            }
            let mut patch = preamble.clone();
            patch.push(line.to_owned());
            current = Some(patch);
        } else if let Some(patch) = current.as_mut() {
            patch.push(line.to_owned());
        } else {
            preamble.push(line.to_owned());
        }
    }
    if let Some(patch) = current {
        hunks.push(patch.concat());
    }
    hunks
}

fn parse_hunk_header(header: &str) -> Option<(usize, usize)> {
    let mut ranges = header.split_whitespace().skip(1);
    let old = parse_range_start(ranges.next()?, '-')?;
    let new = parse_range_start(ranges.next()?, '+')?;
    Some((old, new))
}

fn parse_range_start(range: &str, prefix: char) -> Option<usize> {
    range.strip_prefix(prefix)?.split_once(',').map_or_else(
        || range.strip_prefix(prefix)?.parse().ok(),
        |(start, _)| start.parse().ok(),
    )
}

#[cfg(test)]
#[path = "diff_tests.rs"]
mod tests;
