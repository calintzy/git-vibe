use chrono::{DateTime, Utc, Duration};
use serde::Serialize;
use std::path::Path;
use std::collections::HashMap;
use crate::error::VibeError;

#[derive(Debug, Clone, Serialize)]
pub struct FileChange {
    pub path: String,
    pub additions: u32,
    pub deletions: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommitData {
    pub id: String,
    pub author: String,
    pub email: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub files_changed: Vec<FileChange>,
}

/// 기간 문자열을 파싱하여 시작 날짜를 반환
pub fn parse_period(period: &str) -> Result<DateTime<Utc>, VibeError> {
    let now = Utc::now();
    let len = period.len();
    if len < 2 {
        return Err(VibeError::InvalidPeriod(period.to_string()));
    }

    let (num_str, unit) = period.split_at(len - 1);
    let num: i64 = num_str
        .parse()
        .map_err(|_| VibeError::InvalidPeriod(period.to_string()))?;

    let duration = match unit {
        "d" => Duration::days(num),
        "w" => Duration::weeks(num),
        "m" => Duration::days(num * 30),
        "y" => Duration::days(num * 365),
        _ => return Err(VibeError::InvalidPeriod(period.to_string())),
    };

    Ok(now - duration)
}

/// Git 저장소에서 커밋 데이터를 수집
pub fn collect_commits(repo_path: &Path, since: DateTime<Utc>) -> Result<Vec<CommitData>, VibeError> {
    let repo = gix::open(repo_path)
        .map_err(|e| VibeError::GitOpen(e.to_string()))?;

    let head_id = repo
        .head_id()
        .map_err(|e| VibeError::Git(e.to_string()))?;

    let mut commits = Vec::new();

    let walk = head_id
        .ancestors()
        .all()
        .map_err(|e| VibeError::Git(e.to_string()))?;

    for info in walk {
        let info = info.map_err(|e| VibeError::Git(e.to_string()))?;

        let commit_obj = info.id().object()
            .map_err(|e| VibeError::Git(e.to_string()))?
            .into_commit();

        let commit_ref = commit_obj.decode()
            .map_err(|e| VibeError::Git(e.to_string()))?;

        // SignatureRef::seconds() 메서드 사용
        let secs = commit_ref.author().seconds();
        let timestamp = DateTime::from_timestamp(secs, 0).unwrap_or(Utc::now());

        if timestamp < since {
            break;
        }

        let author_name = commit_ref.author().name.to_string();
        let author_email = commit_ref.author().email.to_string();
        let message = commit_ref.message().title.to_string();
        let id = info.id().to_string();

        // 부모 커밋 ID 목록
        let parent_ids: Vec<gix::ObjectId> = commit_ref.parents().collect();

        drop(commit_ref);

        let files_changed = collect_file_changes_by_id(&repo, &commit_obj, &parent_ids)
            .unwrap_or_default();

        commits.push(CommitData {
            id,
            author: author_name,
            email: author_email,
            message,
            timestamp,
            files_changed,
        });
    }

    Ok(commits)
}

/// 커밋의 파일 변경 목록 수집 (diff_trees 기반)
fn collect_file_changes_by_id(
    repo: &gix::Repository,
    commit: &gix::Commit,
    parent_ids: &[gix::ObjectId],
) -> Result<Vec<FileChange>, VibeError> {
    let current_tree = commit.tree()
        .map_err(|e| VibeError::Git(e.to_string()))?;

    if parent_ids.is_empty() {
        // 최초 커밋: 모든 파일 추가
        let mut additions = Vec::new();
        collect_all_as_additions(repo, &current_tree, "", &mut additions);
        return Ok(additions);
    }

    let parent_obj = repo.find_object(parent_ids[0])
        .map_err(|e| VibeError::Git(e.to_string()))?
        .into_commit();
    let parent_tree = parent_obj.tree()
        .map_err(|e| VibeError::Git(e.to_string()))?;

    // 트리 루트 OID가 같으면 변경 없음
    if current_tree.id == parent_tree.id {
        return Ok(vec![]);
    }

    let mut changes = Vec::new();
    diff_trees(repo, &parent_tree, &current_tree, "", &mut changes)?;
    Ok(changes)
}

/// 두 트리를 재귀적으로 비교하여 변경된 파일 목록 수집
fn diff_trees(
    repo: &gix::Repository,
    old_tree: &gix::Tree,
    new_tree: &gix::Tree,
    prefix: &str,
    changes: &mut Vec<FileChange>,
) -> Result<(), VibeError> {
    let old_entries = tree_to_map(old_tree)?;
    let new_entries = tree_to_map(new_tree)?;

    // 새 트리의 항목 순회: 추가 또는 수정 감지
    for (name, (new_mode, new_oid)) in &new_entries {
        let full_path = make_full_path(prefix, name);

        match old_entries.get(name) {
            None => {
                // 추가됨
                if is_blob_mode(*new_mode) {
                    changes.push(FileChange { path: full_path, additions: 1, deletions: 0 });
                } else if is_tree_mode(*new_mode) {
                    if let Ok(obj) = repo.find_object(*new_oid) {
                        let subtree = obj.into_tree();
                        collect_all_as_additions(repo, &subtree, &full_path, changes);
                    }
                }
            }
            Some((old_mode, old_oid)) => {
                if new_oid == old_oid {
                    continue; // OID 같으면 스킵 (핵심 최적화)
                }
                if is_blob_mode(*new_mode) && is_blob_mode(*old_mode) {
                    // 수정됨
                    changes.push(FileChange { path: full_path, additions: 1, deletions: 1 });
                } else if is_tree_mode(*new_mode) && is_tree_mode(*old_mode) {
                    // 서브트리 재귀 비교
                    if let (Ok(old_obj), Ok(new_obj)) = (
                        repo.find_object(*old_oid),
                        repo.find_object(*new_oid),
                    ) {
                        let old_sub = old_obj.into_tree();
                        let new_sub = new_obj.into_tree();
                        diff_trees(repo, &old_sub, &new_sub, &full_path, changes)?;
                    }
                }
            }
        }
    }

    // 삭제된 항목 감지
    for (name, (old_mode, _)) in &old_entries {
        if !new_entries.contains_key(name) {
            let full_path = make_full_path(prefix, name);
            if is_blob_mode(*old_mode) {
                changes.push(FileChange { path: full_path, additions: 0, deletions: 1 });
            }
        }
    }

    Ok(())
}

/// 트리의 엔트리를 HashMap<이름, (mode, OID)>로 변환
fn tree_to_map(tree: &gix::Tree) -> Result<HashMap<String, (gix::objs::tree::EntryMode, gix::ObjectId)>, VibeError> {
    let tree_ref = tree.decode()
        .map_err(|e| VibeError::Git(e.to_string()))?;
    let mut map = HashMap::new();
    for entry in tree_ref.entries.iter() {
        map.insert(
            entry.filename.to_string(),
            (entry.mode, entry.oid.into()),
        );
    }
    Ok(map)
}

fn make_full_path(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_string()
    } else {
        format!("{}/{}", prefix, name)
    }
}

fn is_blob_mode(mode: gix::objs::tree::EntryMode) -> bool {
    matches!(mode.kind(), gix::objs::tree::EntryKind::Blob | gix::objs::tree::EntryKind::BlobExecutable)
}

fn is_tree_mode(mode: gix::objs::tree::EntryMode) -> bool {
    matches!(mode.kind(), gix::objs::tree::EntryKind::Tree)
}

/// 트리의 모든 파일을 추가(additions)로 수집
fn collect_all_as_additions(
    repo: &gix::Repository,
    tree: &gix::Tree,
    prefix: &str,
    changes: &mut Vec<FileChange>,
) {
    let Ok(tree_ref) = tree.decode() else { return };
    for entry in tree_ref.entries.iter() {
        let full_path = make_full_path(prefix, &entry.filename.to_string());
        match entry.mode.kind() {
            gix::objs::tree::EntryKind::Blob | gix::objs::tree::EntryKind::BlobExecutable => {
                changes.push(FileChange { path: full_path, additions: 1, deletions: 0 });
            }
            gix::objs::tree::EntryKind::Tree => {
                if let Ok(obj) = repo.find_object(entry.oid) {
                    let subtree = obj.into_tree();
                    collect_all_as_additions(repo, &subtree, &full_path, changes);
                }
            }
            _ => {}
        }
    }
}

/// 트리에서 모든 파일 경로를 재귀적으로 수집
fn collect_tree_files(
    repo: &gix::Repository,
    tree: &gix::Tree,
    prefix: &str,
    files: &mut Vec<String>,
) -> Result<(), VibeError> {
    let tree_ref = tree.decode()
        .map_err(|e| VibeError::Git(e.to_string()))?;

    for entry in tree_ref.entries.iter() {
        let name = entry.filename.to_string();
        let full_path = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", prefix, name)
        };

        match entry.mode.kind() {
            gix::objs::tree::EntryKind::Tree => {
                if let Ok(obj) = repo.find_object(entry.oid) {
                    let subtree = obj.into_tree();
                    let _ = collect_tree_files(repo, &subtree, &full_path, files);
                }
            }
            gix::objs::tree::EntryKind::Blob | gix::objs::tree::EntryKind::BlobExecutable => {
                files.push(full_path);
            }
            _ => {}
        }
    }

    Ok(())
}

/// HEAD 트리의 모든 파일 목록 반환
pub fn list_all_files(repo_path: &Path) -> Result<Vec<String>, VibeError> {
    let repo = gix::open(repo_path)
        .map_err(|e| VibeError::GitOpen(e.to_string()))?;

    let head_commit = repo
        .head_id()
        .map_err(|e| VibeError::Git(e.to_string()))?
        .object()
        .map_err(|e| VibeError::Git(e.to_string()))?
        .into_commit();

    let tree = head_commit.tree()
        .map_err(|e| VibeError::Git(e.to_string()))?;

    let mut files = Vec::new();
    collect_tree_files(&repo, &tree, "", &mut files)?;

    Ok(files)
}
