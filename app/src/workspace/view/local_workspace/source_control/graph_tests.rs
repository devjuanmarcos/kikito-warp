use super::{is_head, layout, reachable_from_head};
use crate::local_git::LocalCommit;

fn commit(hash: &str, parents: &[&str], refs: &[&str]) -> LocalCommit {
    LocalCommit {
        hash: hash.to_owned(),
        parents: parents.iter().map(|parent| (*parent).to_owned()).collect(),
        author: "Juan".to_owned(),
        authored_at: "2026-10-20T10:00:00-03:00".to_owned(),
        subject: hash.to_owned(),
        refs: refs
            .iter()
            .map(|reference| (*reference).to_owned())
            .collect(),
    }
}

#[test]
fn linear_history_stays_in_one_lane() {
    let commits = [
        commit("b", &["a"], &["HEAD -> main"]),
        commit("a", &[], &[]),
    ];
    let (rows, lanes) = layout(&commits);
    assert_eq!(lanes, 1);
    assert_eq!(rows[0].col, 0);
    assert_eq!(rows[0].after, [Some("a".to_owned())]);
    assert_eq!(rows[1].col, 0);
    assert!(rows[1].after.is_empty());
}

#[test]
fn merge_forks_a_second_lane_that_rejoins_at_the_shared_parent() {
    let commits = [
        commit("m", &["a", "f"], &["HEAD -> main"]),
        commit("f", &["a"], &[]),
        commit("a", &[], &[]),
    ];
    let (rows, lanes) = layout(&commits);
    assert_eq!(lanes, 2);
    assert_eq!(rows[0].forks, [1]);
    assert_eq!(rows[1].col, 1);
    assert_eq!(rows[2].col, 0);
    assert_eq!(rows[2].before, [Some("a".to_owned()), Some("a".to_owned())]);
}

#[test]
fn head_filter_keeps_only_commits_reachable_from_head() {
    let commits = [
        commit("x", &["a"], &["origin/other"]),
        commit("b", &["a"], &["HEAD -> main"]),
        commit("a", &[], &[]),
    ];
    assert!(is_head(&commits[1]));
    let kept: Vec<_> = reachable_from_head(&commits)
        .into_iter()
        .map(|commit| commit.hash)
        .collect();
    assert_eq!(kept, ["b", "a"]);
}
