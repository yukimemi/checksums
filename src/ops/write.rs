use self::super::super::Error;
use self::super::{CompareError, CompareFileResult, CompareResult};
use std::io::Write;

/// Write hash comparison results to the output streams in a human-consumable format
pub fn write_hash_comparison_results<Wo: Write, We: Write>(
    output: &mut Wo,
    error: &mut We,
    results: Result<(Vec<CompareResult>, Vec<CompareFileResult>), CompareError>,
) -> Error {
    let result = match results {
        Ok((mut compare_results, mut file_compare_results)) => {
            compare_results.sort();
            file_compare_results.sort();

            for res in &compare_results {
                match *res {
                    CompareResult::FileAdded(ref file) => write_compare_result(output, "File added: ", file),
                    CompareResult::FileRemoved(ref file) => write_compare_result(output, "File removed: ", file),
                    CompareResult::FileIgnored(ref file) => write_compare_result(output, "File ignored, skipping: ", file),
                }
            }

            if file_compare_results.is_empty() && compare_results.is_empty() {
                writeln!(output, "No files left to verify").unwrap();
                Error::NoError
            } else if file_compare_results.is_empty() {
                writeln!(output, "No files to verify").unwrap();
                Error::NoError
            } else {
                if !compare_results.is_empty() {
                    writeln!(output, "").unwrap();
                }

                let mut differed_n = 0;
                for fres in &file_compare_results {
                    match *fres {
                        CompareFileResult::FileMatches { ref file, ref hash } => write_file_result_match(output, file, hash),
                        CompareFileResult::FileDiffers {
                            ref file,
                            ref was_hash,
                            ref new_hash,
                        } => {
                            write_file_result_diff(output, file, was_hash, new_hash);
                            differed_n += 1;
                        }
                    }
                }

                match differed_n {
                    0 => Error::NoError,
                    n => Error::NFilesDiffer(n),
                }
            }
        }
        Err(CompareError::HashLengthDiffers { previous_len, current_len }) => {
            writeln!(error, "Hash lengths do not match; selected: {}, loaded: {}", current_len, previous_len).unwrap();

            Error::HashLengthDiffers
        }
        Err(CompareError::DifferentNumberOfFiles { previous_len, current_len }) => {
            writeln!(error, "File count do not match; selected: {}, loaded: {}", current_len, previous_len).unwrap();

            Error::DifferentNumberOfFiles
        }
    };

    output.flush().unwrap();
    error.flush().unwrap();

    result
}

fn write_compare_result<W: Write>(out: &mut W, pre: &str, fname: &str) {
    write_result(out, pre, fname, true)
}

fn write_result<W: Write>(out: &mut W, pre: &str, fname: &str, quote: bool) {
    let quote_s = if quote { "\"" } else { "" };
    writeln!(out, "{}{2}{}{2}", pre, fname, quote_s).unwrap();
}

fn write_file_result_match<W: Write>(_out: &mut W, _fname: &str, _hash: &str) {
    // writeln!(out, "File matches. \"{}\" hash: {}", fname, hash).unwrap();
}

fn write_file_result_diff<W: Write>(out: &mut W, fname: &str, lhash: &str, chash: &str) {
    writeln!(out, "File doesn't match. \"{}\"", fname).unwrap();

    write_result(out, "  Was: ", lhash, false);
    write_result(out, "  Is : ", chash, false);
}
