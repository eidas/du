use std::fs;
use std::path::{Path, PathBuf};

pub struct DirEntry {
    pub path: PathBuf,
    pub size_bytes: u64,
}

/// Recursively computes sizes for all subdirectories under `root`.
/// Returns (entries, root_total) where entries are subdirs within the depth limit.
pub fn compute_sizes(root: &Path, max_depth: Option<usize>) -> (Vec<DirEntry>, u64) {
    let mut entries = Vec::new();
    let root_size = collect(root, 0, max_depth, &mut entries);
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    (entries, root_size)
}

fn collect(path: &Path, depth: usize, max_depth: Option<usize>, results: &mut Vec<DirEntry>) -> u64 {
    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return 0,
    };

    let mut total: u64 = 0;

    for entry in read_dir.flatten() {
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if meta.is_symlink() {
            // Skip symlinks to avoid cycles (especially Windows junction points)
            continue;
        }

        if meta.is_file() {
            total += meta.len();
        } else if meta.is_dir() {
            let child_path = entry.path();
            let child_size = collect(&child_path, depth + 1, max_depth, results);
            total += child_size;

            if max_depth.map_or(true, |d| depth + 1 <= d) {
                results.push(DirEntry {
                    path: child_path,
                    size_bytes: child_size,
                });
            }
        }
    }

    total
}
