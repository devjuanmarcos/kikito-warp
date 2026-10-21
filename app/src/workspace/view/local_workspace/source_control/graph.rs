use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bytes::Bytes;
use pathfinder_color::ColorU;
use warpui::assets::asset_cache::{AssetSource, AsyncAssetId, AsyncAssetType};
use warpui::elements::{CacheOption, ConstrainedBox, Element, Image};

use crate::local_git::LocalCommit;

pub(super) const ROW_HEIGHT: f32 = 24.;
const LANE_WIDTH: f32 = 12.;
const LANE_PADDING: f32 = 8.;

/// Lane assignment of one commit row, as in the kit's `gitLayout`: `before`/`after` hold the
/// commit each lane is waiting for above and below the row.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct GraphRow {
    pub col: usize,
    pub before: Vec<Option<String>>,
    pub after: Vec<Option<String>>,
    pub forks: Vec<usize>,
}

fn free_lane(lanes: &mut Vec<Option<String>>) -> usize {
    match lanes.iter().position(Option::is_none) {
        Some(index) => index,
        None => {
            lanes.push(None);
            lanes.len() - 1
        }
    }
}

/// Assigns lanes to newest-first commits; returns the rows and the widest lane count.
pub(super) fn layout(commits: &[LocalCommit]) -> (Vec<GraphRow>, usize) {
    let mut lanes: Vec<Option<String>> = Vec::new();
    let mut rows = Vec::with_capacity(commits.len());
    let mut widest = 1;
    for commit in commits {
        let before = lanes.clone();
        let col = match lanes
            .iter()
            .position(|lane| lane.as_deref() == Some(commit.hash.as_str()))
        {
            Some(index) => index,
            None => free_lane(&mut lanes),
        };
        for (index, lane) in lanes.iter_mut().enumerate() {
            if index != col && lane.as_deref() == Some(commit.hash.as_str()) {
                *lane = None;
            }
        }
        lanes[col] = commit.parents.first().cloned();
        let mut forks = Vec::new();
        for parent in commit.parents.iter().skip(1) {
            let index = match lanes
                .iter()
                .position(|lane| lane.as_deref() == Some(parent.as_str()))
            {
                Some(index) => index,
                None => {
                    let index = free_lane(&mut lanes);
                    lanes[index] = Some(parent.clone());
                    index
                }
            };
            forks.push(index);
        }
        while lanes.last().is_some_and(Option::is_none) {
            lanes.pop();
        }
        widest = widest.max(before.len()).max(lanes.len()).max(col + 1);
        rows.push(GraphRow {
            col,
            before,
            after: lanes.clone(),
            forks,
        });
    }
    (rows, widest)
}

pub(super) fn graph_width(lanes: usize) -> f32 {
    LANE_PADDING * 2. + lanes.saturating_sub(1) as f32 * LANE_WIDTH + 4.
}

pub(super) fn is_head(commit: &LocalCommit) -> bool {
    commit
        .refs
        .iter()
        .any(|reference| reference == "HEAD" || reference.starts_with("HEAD -> "))
}

/// Commits reachable from HEAD, for the "Só a atual" filter.
pub(super) fn reachable_from_head(commits: &[LocalCommit]) -> Vec<LocalCommit> {
    let by_hash: HashMap<&str, &LocalCommit> = commits
        .iter()
        .map(|commit| (commit.hash.as_str(), commit))
        .collect();
    let mut keep = HashSet::new();
    let mut stack: Vec<&LocalCommit> = commits
        .iter()
        .find(|commit| is_head(commit))
        .into_iter()
        .collect();
    while let Some(commit) = stack.pop() {
        if !keep.insert(commit.hash.as_str()) {
            continue;
        }
        stack.extend(
            commit
                .parents
                .iter()
                .filter_map(|parent| by_hash.get(parent.as_str()).copied()),
        );
    }
    commits
        .iter()
        .filter(|commit| keep.contains(commit.hash.as_str()))
        .cloned()
        .collect()
}

/// Colors of the graph: lane colors cycle, HEAD gets the accent ring.
pub(super) struct GraphColors {
    pub lanes: Vec<ColorU>,
    pub head: ColorU,
    pub background: ColorU,
}

impl GraphColors {
    pub(super) fn lane(&self, index: usize) -> ColorU {
        self.lanes[index % self.lanes.len()]
    }
}

fn hex(color: ColorU) -> String {
    format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
}

fn lane_x(index: usize) -> f32 {
    LANE_PADDING + index as f32 * LANE_WIDTH
}

fn curve(x1: f32, y1: f32, x2: f32, y2: f32) -> String {
    if x1 == x2 {
        format!("M{x1} {y1}L{x2} {y2}")
    } else {
        let middle = (y1 + y2) / 2.;
        format!("M{x1} {y1}C{x1} {middle} {x2} {middle} {x2} {y2}")
    }
}

/// SVG of one row's lanes and node, drawn like the kit's `GraphCell`.
pub(super) fn row_svg(
    row: &GraphRow,
    commit: &LocalCommit,
    width: f32,
    colors: &GraphColors,
) -> String {
    let height = ROW_HEIGHT;
    let middle = height / 2.;
    let col = row.col;
    let mut segments = Vec::new();
    for (index, lane) in row.before.iter().enumerate() {
        let Some(hash) = lane else {
            continue;
        };
        if hash == &commit.hash {
            let color = colors.lane(if index == col { col } else { index });
            segments.push((curve(lane_x(index), 0., lane_x(col), middle), color));
        } else {
            segments.push((
                curve(lane_x(index), 0., lane_x(index), middle),
                colors.lane(index),
            ));
        }
    }
    for (index, lane) in row.after.iter().enumerate() {
        let Some(hash) = lane else {
            continue;
        };
        if index == col || row.forks.contains(&index) {
            segments.push((
                curve(lane_x(col), middle, lane_x(index), height),
                colors.lane(index),
            ));
        }
        if index != col && row.before.get(index).and_then(Option::as_ref) == Some(hash) {
            segments.push((
                curve(lane_x(index), middle, lane_x(index), height),
                colors.lane(index),
            ));
        }
    }
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">"#
    );
    for (path, color) in segments {
        svg.push_str(&format!(
            r#"<path d="{path}" stroke="{}" stroke-width="1.5" fill="none"/>"#,
            hex(color)
        ));
    }
    let x = lane_x(col);
    let lane_color = hex(colors.lane(col));
    let background = hex(colors.background);
    if is_head(commit) {
        svg.push_str(&format!(
            r#"<circle cx="{x}" cy="{middle}" r="5" fill="{background}" stroke="{}" stroke-width="2"/>"#,
            hex(colors.head)
        ));
    } else if commit.parents.len() > 1 {
        svg.push_str(&format!(
            r#"<circle cx="{x}" cy="{middle}" r="4.5" fill="{background}" stroke="{lane_color}" stroke-width="1.5"/><circle cx="{x}" cy="{middle}" r="1.6" fill="{lane_color}"/>"#
        ));
    } else {
        svg.push_str(&format!(
            r#"<circle cx="{x}" cy="{middle}" r="3.5" fill="{lane_color}"/>"#
        ));
    }
    svg.push_str("</svg>");
    svg
}

struct GraphRowAsset;

impl AsyncAssetType for GraphRowAsset {}

/// Renders an SVG string at its natural size.
pub(super) fn svg_cell(svg: String, width: f32) -> Box<dyn Element> {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&svg, &mut hasher);
    let key = format!("git-graph-row-{}", std::hash::Hasher::finish(&hasher));
    let bytes = Bytes::from(svg.into_bytes());
    let source = AssetSource::Async {
        id: AsyncAssetId::new::<GraphRowAsset>(key),
        fetch: Arc::new(move || {
            let bytes = bytes.clone();
            Box::pin(async move { Ok(bytes) })
        }),
    };
    ConstrainedBox::new(Image::new(source, CacheOption::BySize).finish())
        .with_width(width)
        .with_height(ROW_HEIGHT)
        .finish()
}

#[cfg(test)]
#[path = "graph_tests.rs"]
mod tests;
