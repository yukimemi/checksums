extern crate checksums;

use std::fs::File;
use std::io::{stderr, stdout};
use std::process::exit;

fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    let opts = checksums::Options::parse();

    let mut hashes = checksums::ops::create_hashes(
        &opts.dir,
        opts.ignored_files,
        opts.algorithm,
        opts.depth,
        opts.follow_symlinks,
        opts.jobs,
        &mut stderr(),
    );
    if opts.verify {
        // Progress bar separator
        println!("");

        match checksums::ops::read_hashes(&mut stderr(), &opts.file) {
            Ok(mut loaded_hashes) => {
                let compare_result = checksums::ops::compare_hashes(&opts.file.0, &mut hashes, &mut loaded_hashes, opts.check_count);
                let err = checksums::ops::write_hash_comparison_results(&mut stdout(), &mut stderr(), compare_result);
                if err == checksums::Error::NoError {
                    checksums::ops::write_hashes(&mut stdout(), &opts.file.0, opts.algorithm, hashes);
                }
                err
            }
            Err(rval) => rval,
        }
        .exit_value()
    } else {
        let mut f = File::create(&opts.file.1).unwrap();
        checksums::ops::write_hashes(&mut f, &opts.file.0, opts.algorithm, hashes);
        0
    }
}
