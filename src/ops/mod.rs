//! Main functions doing actual work.
//!
//!
//! Use `create_hashes()` to prepare the hashes for a path.
//!
//! Then use `write_hashes()` to save it to disk, or `read_hashes()` to get the saved hashes, them with
//! `compare_hashes()` and print them with `write_hash_comparison_results()`.

mod compare;
mod write;

use self::super::util::{mul_str, relative_name};
use self::super::Error;
use self::super::{hash_file, Algorithm};
use futures::Future;
use futures_cpupool::{CpuFuture, CpuPool};
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use tabwriter::TabWriter;
use walkdir::{WalkDir, WalkDirIterator};

pub use self::compare::*;
pub use self::write::*;

/// Create subpath->hash mappings for a given path using a given algorithm up to a given depth.
pub fn create_hashes<We>(
    path: &Path,
    ignored_files: BTreeSet<String>,
    algo: Algorithm,
    depth: Option<usize>,
    follow_symlinks: bool,
    jobs: usize,
    pb_err: &mut We,
) -> BTreeMap<String, String>
where
    We: Write,
{
    let mut walkdir = WalkDir::new(path).follow_links(follow_symlinks);
    if let Some(depth) = depth {
        walkdir = walkdir.max_depth(depth + 1);
    }

    let mut hashes = BTreeMap::new();
    let mut hashes_f: BTreeMap<String, CpuFuture<String, ()>> = BTreeMap::new();

    let mut errored = false;
    let pool = CpuPool::new(jobs);

    let mut walkdir = walkdir.into_iter();
    while let Some(entry) = walkdir.next() {
        match entry {
            Ok(entry) => {
                let file_type = entry.file_type();
                let filename = relative_name(path, entry.path());
                let ignored = ignored_files.contains(&filename);

                if file_type.is_file() {
                    if ignored {
                        hashes.insert(filename, mul_str("-", algo.hexlen()));
                    } else {
                        hashes_f.insert(filename, pool.spawn_fn(move || Ok(hash_file(entry.path(), algo))));
                    }
                } else if ignored {
                    walkdir.skip_current_dir();
                }
            }
            Err(error) => {
                errored = true;
                writeln!(pb_err, "Symlink loop detected at {}", relative_name(path, &error.path().unwrap())).unwrap();
            }
        }
    }

    if errored {
        writeln!(pb_err, "").unwrap();
        panic!("Failed to create hash file");
    }

    hashes.extend(hashes_f.into_iter().map(|(k, f)| match f.wait() {
        Ok(result) => return (k, result),
        Err(error) => panic!("Failed to hash file \"{}\": {:?}", k, error),
    }));

    hashes
}

/// Serialise the specified hashes to the specified output file.
pub fn write_hashes(out: &mut Write, out_file: &str, algo: Algorithm, mut hashes: BTreeMap<String, String>) {
    let mut out = TabWriter::new(out);

    hashes.insert(out_file.to_string(), mul_str("-", algo.hexlen()));
    for (fname, hash) in hashes {
        writeln!(&mut out, "{}\t{}", fname, hash).unwrap();
    }

    out.flush().unwrap();
}

/// Read hashes saved with `write_hashes()` from the specified path or fail with line numbers not matching pattern.
pub fn read_hashes(err: &mut Write, file: &(String, PathBuf)) -> Result<BTreeMap<String, String>, Error> {
    lazy_static! {
        static ref LINE_RGX: Regex = Regex::new(r"^(.+?)\s{2,}([[:xdigit:]-]+)$").unwrap();
    }

    let mut hashes = BTreeMap::new();
    let mut failed = false;

    let in_file = BufReader::new(File::open(&file.1).unwrap());
    for (n, line) in in_file.lines().map(Result::unwrap).enumerate() {
        if !line.is_empty() {
            match LINE_RGX.captures(&line) {
                Some(captures) => {
                    hashes.insert(captures[1].to_string(), captures[2].to_string());
                }
                None => {
                    failed = true;
                    writeln!(err, "{}:{}: Line doesn't match accepted pattern", file.0, n).unwrap();
                }
            };
        }
    }

    if !failed {
        Ok(hashes)
    } else {
        Err(Error::HashesFileParsingFailure)
    }
}
