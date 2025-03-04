static USAGE: &str = r#"
Compute summary statistics & infers data types for each column in a CSV. 

Summary statistics includes sum, min/max/range, min/max length, mean, stddev, variance,
nullcount, sparsity, quartiles, interquartile range (IQR), lower/upper fences, skewness, median, 
cardinality, mode/s & antimode/s, and median absolute deviation (MAD). Note that some
statistics requires loading the entire file into memory, so they must be enabled explicitly. 

By default, the following statistics are reported for *every* column in the CSV data:
sum, min/max/range values, min/max length, mean, stddev, variance, nullcount & sparsity.
The default set of statistics corresponds to statistics that can be computed efficiently
on a stream of data (i.e., constant memory) and works with arbitrarily large CSV files.

The following additional statistics require loading the entire file into memory:
cardinality, mode/antimode, median, MAD, quartiles and its related measures (IQR,
lower/upper fences & skewness).

Note that an Out-Of-Memory (OOM) check heuristic will prevent processing if the file
is larger than the available memory minus a headroom buffer of 20% (adjustable using the
QSV_FREEMEMORY_HEADROOM_PCT environment variable).

"Antimode" is the least frequently occurring non-zero value and is the opposite of mode.
It returns "*ALL" if all the values are unique, and only returns a preview of the first
10 antimodes.

If you need all the antimode values of a column, run the `frequency` command with --limit set
to zero. The resulting frequency table will have all the antimode values.

Summary statistics for dates are also computed when --infer-dates is enabled, with DateTime
results in rfc3339 format and Date results in "yyyy-mm-dd" format in the UTC timezone.
Date range, stddev, variance, MAD & IQR are returned in days, not timestamp milliseconds.

Each column's data type is also inferred (NULL, Integer, String, Float, Date, DateTime and
Boolean with --infer-boolean option).
Unlike the sniff command, stats' data type inferences are GUARANTEED, as the entire file
is scanned, and not just sampled.

Note that the Date and DateTime data types are only inferred with the --infer-dates option 
as its an expensive operation to match a date candidate against 19 possible date formats,
with each format, having several variants.

The date formats recognized and its sub-variants along with examples can be found at 
https://github.com/jqnatividad/belt/tree/main/dateparser#accepted-date-formats.

Computing statistics on a large file can be made MUCH faster if you create an index for it
first with 'qsv index' to enable multithreading.

This command caches the results in <FILESTEM>.stats.csv and <FILESTEM>.stats.csv.bin
(e.g., qsv stats nyc311.csv will create nyc311.stats.csv and nyc311.stats.csv.bin).
The .bin file is the binary format of the computed stats. The arguments used to generate the
cached stats are saved in <FILESTEM>.stats.csv.json.

If stats have already been computed for the input file with similar arguments and the file
hasn't changed, the stats will be loaded from the cache instead of recomputing it.

For examples, see the "boston311" test files in https://github.com/jqnatividad/qsv/tree/master/resources/test
and https://github.com/jqnatividad/qsv/blob/4529d51273218347fef6aca15ac24e22b85b2ec4/tests/test_stats.rs#L608.

Usage:
    qsv stats [options] [<input>]
    qsv stats --help

stats options:
    -s, --select <arg>        Select a subset of columns to compute stats for.
                              See 'qsv select --help' for the format details.
                              This is provided here because piping 'qsv select'
                              into 'qsv stats' will disable the use of indexing.
    --everything              Show all statistics available.
    --typesonly               Infer data types only and do not compute statistics.
                              Note that if you want to infer dates, you'll still need to use
                              the --infer-dates and --dates-whitelist options.
    --infer-boolean           Infer boolean data type. This automatically enables
                              the --cardinality option. When a column's cardinality is 2,
                              and the 2 values' first characters are 0/1, t/f & y/n
                              case-insensitive, the data type is inferred as boolean.
    --mode                    Show the mode/s & antimode/s. Multimodal-aware.
                              This requires loading all CSV data in memory.
    --cardinality             Show the cardinality.
                              This requires loading all CSV data in memory.
    --median                  Show the median.
                              This requires loading all CSV data in memory.
    --mad                     Shows the median absolute deviation (MAD).
                              This requires loading all CSV data in memory.
    --quartiles               Show the quartiles, the IQR, the lower/upper inner/outer
                              fences and skewness.
                              This requires loading all CSV data in memory.
    --round <decimal_places>  Round statistics to <decimal_places>. Rounding is done following
                              Midpoint Nearest Even (aka "Bankers Rounding") rule.
                              For dates - range, stddev & IQR are always at least 5 decimal places as
                              they are reported in days, and 5 places gives us millisecond precision.
                              [default: 4]
    --nulls                   Include NULLs in the population size for computing
                              mean and standard deviation.
    --infer-dates             Infer date/datetime datatypes. This is an expensive
                              option and should only be used when you know there
                              are date/datetime fields.
                              Also, if timezone is not specified in the data, it'll
                              be set to UTC.
    --dates-whitelist <list>  The case-insensitive patterns to look for when 
                              shortlisting fields for date inferencing.
                              i.e. if the field's name has any of these patterns,
                              it is shortlisted for date inferencing.
                              Set to "all" to inspect ALL fields for
                              date/datetime types. Ignored if --infer-dates is false.

                              Note that false positive date matches WILL most likely occur
                              when using "all" as unix epoch timestamps are just numbers.
                              Be sure to only use "all" if you know the columns you're
                              inspecting are dates, boolean or string fields.
                              
                              To avoid false positives, preprocess the file first 
                              with `apply datefmt` to convert unix epoch timestamp columns
                              to RFC3339 format.
                              [default: date,time,due,open,close,created]
    --prefer-dmy              Parse dates in dmy format. Otherwise, use mdy format.
                              Ignored if --infer-dates is false.
    --force                   Force recomputing stats even if valid precomputed stats
                              cache exists.
    -j, --jobs <arg>          The number of jobs to run in parallel.
                              This works only when the given CSV has an index.
                              Note that a file handle is opened for each job.
                              When not set, the number of jobs is set to the
                              number of CPUs detected.
    --stats-binout <file>     Write the stats to <file> in binary format.
                              This is used internally by other qsv commands (currently `schema`
                              and `tojsonl`) to load cached stats into memory without having to
                              process/parse the CSV again.
                              If <file> is "NONE", the stats cache will not be written.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. i.e., They will be included
                           in statistics.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
                           This option is ignored when computing default, streaming
                           statistics, as it is not needed.
"#;

/*
DEVELOPER NOTE: stats is heavily optimized and is a central command in qsv.

It was the primary reason I created the qsv fork as I needed to do GUARANTEED data type
inferencing & to compile smart Data Dictionaries in the most performant way possible
for Datapusher+ (https://github.com/dathere/datapusher-plus).

It underpins the `schema` and `validate` commands - enabling the automatic creation of
a JSONschema based on a CSV's summary statistics; and use the generated JSONschema to
quickly validate complex CSVs (NYC's 311 data) at almost 350,000 records/sec.

It's type inferences are also used by the `tojsonl` command to generate properly typed
JSONL files.

To safeguard against undefined behavior, `stats` is the most extensively tested command,
with >480 tests.
*/

use std::{
    borrow::ToOwned,
    default::Default,
    fmt, fs, io,
    io::{BufWriter, Write},
    iter::repeat,
    path::{Path, PathBuf},
    str,
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
};

use itertools::Itertools;
use qsv_dateparser::parse_with_preference;
use serde::{Deserialize, Serialize};
use simdutf8::basic::from_utf8;
use stats::{merge_all, Commute, MinMax, OnlineStats, Unsorted};
use tempfile::NamedTempFile;
use threadpool::ThreadPool;

use self::FieldType::{TDate, TDateTime, TFloat, TInteger, TNull, TString};
use crate::{
    config::{Config, Delimiter, DEFAULT_WTR_BUFFER_CAPACITY},
    select::{SelectColumns, Selection},
    util, CliResult,
};

#[derive(Clone, Deserialize, Debug)]
pub struct Args {
    pub arg_input:            Option<String>,
    pub flag_select:          SelectColumns,
    pub flag_everything:      bool,
    pub flag_typesonly:       bool,
    pub flag_infer_boolean:   bool,
    pub flag_mode:            bool,
    pub flag_cardinality:     bool,
    pub flag_median:          bool,
    pub flag_mad:             bool,
    pub flag_quartiles:       bool,
    pub flag_round:           u32,
    pub flag_nulls:           bool,
    pub flag_infer_dates:     bool,
    pub flag_dates_whitelist: String,
    pub flag_prefer_dmy:      bool,
    pub flag_force:           bool,
    pub flag_jobs:            Option<usize>,
    pub flag_output:          Option<String>,
    pub flag_no_headers:      bool,
    pub flag_delimiter:       Option<Delimiter>,
    pub flag_memcheck:        bool,
    pub flag_stats_binout:    Option<String>,
}

// this struct is used to serialize/deserialize the stats to
// the "".stats.csv.json" file which we check to see
// if we can skip recomputing stats.
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
struct StatsArgs {
    arg_input:            String,
    flag_select:          String,
    flag_everything:      bool,
    flag_typesonly:       bool,
    flag_infer_boolean:   bool,
    flag_mode:            bool,
    flag_cardinality:     bool,
    flag_median:          bool,
    flag_mad:             bool,
    flag_quartiles:       bool,
    flag_round:           u32,
    flag_nulls:           bool,
    flag_infer_dates:     bool,
    flag_dates_whitelist: String,
    flag_prefer_dmy:      bool,
    flag_no_headers:      bool,
    flag_delimiter:       String,
    flag_output_snappy:   bool,
    canonical_input_path: String,
    canonical_stats_path: String,
    record_count:         u64,
    date_generated:       String,
    compute_duration_ms:  u64,
    qsv_version:          String,
}

static INFER_DATE_FLAGS: OnceLock<Vec<bool>> = OnceLock::new();
static DMY_PREFERENCE: AtomicBool = AtomicBool::new(false);
static RECORD_COUNT: OnceLock<u64> = OnceLock::new();

// number of milliseconds per day
const MS_IN_DAY: f64 = 86_400_000.0;
// number of decimal places when rounding days
// 5 decimal places give us millisecond precision
const DAY_DECIMAL_PLACES: u32 = 5;

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;
    if args.flag_typesonly {
        args.flag_everything = false;
        args.flag_mode = false;
        args.flag_cardinality = false;
        args.flag_median = false;
        args.flag_quartiles = false;
        args.flag_mad = false;
    }

    // inferring boolean requires inferring cardinality
    if args.flag_infer_boolean && !args.flag_cardinality {
        args.flag_cardinality = true;
    }

    // set stdout output flag
    let stdout_output_flag = args.flag_output.is_none();

    // save the current args, we'll use it to generate
    // the stats.csv.json file
    let mut current_stats_args = StatsArgs {
        arg_input:            format!("{:?}", args.arg_input),
        flag_select:          format!("{:?}", args.flag_select),
        flag_everything:      args.flag_everything,
        flag_typesonly:       args.flag_typesonly,
        flag_infer_boolean:   args.flag_infer_boolean,
        flag_mode:            args.flag_mode,
        flag_cardinality:     args.flag_cardinality,
        flag_median:          args.flag_median,
        flag_mad:             args.flag_mad,
        flag_quartiles:       args.flag_quartiles,
        flag_round:           args.flag_round,
        flag_nulls:           args.flag_nulls,
        flag_infer_dates:     args.flag_infer_dates,
        flag_dates_whitelist: args.flag_dates_whitelist.clone(),
        flag_prefer_dmy:      args.flag_prefer_dmy,
        flag_no_headers:      args.flag_no_headers,
        flag_delimiter:       format!("{:?}", args.flag_delimiter.clone()),
        // when we write to stdout, we don't use snappy compression
        // when we write to a file with the --output option, we use
        // snappy compression if the file ends with ".sz"
        flag_output_snappy:   if stdout_output_flag {
            false
        } else {
            let p = args.flag_output.clone().unwrap();
            p.to_lowercase().ends_with(".sz")
        },
        canonical_input_path: String::new(),
        canonical_stats_path: String::new(),
        record_count:         0,
        date_generated:       String::new(),
        compute_duration_ms:  0,
        qsv_version:          env!("CARGO_PKG_VERSION").to_string(),
    };

    // create a temporary file to store the <FILESTEM>.stats.csv file
    let stats_csv_tempfile = if current_stats_args.flag_output_snappy {
        tempfile::Builder::new().suffix(".sz").tempfile()?
    } else {
        NamedTempFile::new()?
    };
    let stats_csv_tempfile_fname = stats_csv_tempfile.path().to_str().unwrap().to_owned();

    // we will write the stats to a temp file
    let mut wtr = Config::new(&Some(stats_csv_tempfile_fname.clone())).writer()?;
    let mut fconfig = args.rconfig();
    let mut stdin_tempfile_path = None;

    if fconfig.is_stdin() {
        // read from stdin and write to a temp file
        log::info!("Reading from stdin");
        let mut stdin_file = NamedTempFile::new()?;
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        std::io::copy(&mut stdin_handle, &mut stdin_file)?;
        drop(stdin_handle);
        let (_file, tempfile_path) = stdin_file
            .keep()
            .or(Err("Cannot keep temporary file".to_string()))?;
        stdin_tempfile_path = Some(tempfile_path.clone());
        args.arg_input = Some(tempfile_path.to_string_lossy().to_string());
        fconfig.path = Some(tempfile_path);
    } else {
        // check if the input file exists
        if let Some(path) = fconfig.path.clone() {
            if !path.exists() {
                return fail_clierror!("File {:?} does not exist", path.display());
            }
        }
    }

    // create stats_for_encoding to store the stats in binary format
    let mut stats_for_encoding: Vec<Stats> = Vec::new();

    let mut compute_stats = true;

    let mut stats_binout = String::new();
    let write_stats_cache = if let Some(s) = args.flag_stats_binout.clone() {
        stats_binout = s;
        stats_binout.to_lowercase() != "none"
    } else {
        true
    };

    if let Some(path) = fconfig.path.clone() {
        let path_file_stem = path.file_stem().unwrap().to_str().unwrap();
        let stats_file = stats_path(&path, false);
        // check if <FILESTEM>.stats.csv file already exists.
        // If it does, check if it was compiled using the same args.
        // However, if the --force flag is set,
        // regenerate the stats even if the args are the same.
        if stats_file.exists() && !args.flag_force {
            let stats_args_json_file = stats_file.with_extension("csv.json");
            let existing_stats_args_json_str =
                match fs::read_to_string(stats_args_json_file.clone()) {
                    Ok(s) => s,
                    Err(e) => {
                        log::warn!(
                            "Could not read {path_file_stem}.stats.csv.json: {e:?}, \
                             regenerating..."
                        );
                        fs::remove_file(&stats_file)?;
                        fs::remove_file(&stats_args_json_file)?;
                        String::new()
                    }
                };

            // deserialize the existing stats args json
            let existing_stats_args_json: StatsArgs =
                match serde_json::from_str::<StatsArgs>(&existing_stats_args_json_str) {
                    Ok(mut stat_args) => {
                        // we init these fields to empty values because we don't want to compare
                        // them when checking if the args are the same
                        stat_args.canonical_input_path = String::new();
                        stat_args.canonical_stats_path = String::new();
                        stat_args.record_count = 0;
                        stat_args.date_generated = String::new();
                        stat_args.compute_duration_ms = 0;
                        stat_args
                    }
                    Err(e) => {
                        log::warn!(
                            "Could not serialize {path_file_stem}.stats.csv.json: {e:?}, \
                             regenerating..."
                        );
                        fs::remove_file(&stats_file)?;
                        fs::remove_file(&stats_args_json_file)?;
                        StatsArgs::default()
                    }
                };

            // check if the cached stats are current (ie the stats file is newer than the input
            // file), use the same args or if the --everything flag was set, and
            // all the other non-stats args are equal. If so, we don't need to recompute the stats
            let input_file_modified = fs::metadata(&path)?.modified()?;
            let stats_file_modified = fs::metadata(&stats_file)?.modified()?;
            #[allow(clippy::nonminimal_bool)]
            if stats_file_modified > input_file_modified
                && (existing_stats_args_json == current_stats_args
                    || existing_stats_args_json.flag_everything
                        && existing_stats_args_json.flag_infer_dates
                            == current_stats_args.flag_infer_dates
                        && existing_stats_args_json.flag_dates_whitelist
                            == current_stats_args.flag_dates_whitelist
                        && existing_stats_args_json.flag_prefer_dmy
                            == current_stats_args.flag_prefer_dmy
                        && existing_stats_args_json.flag_no_headers
                            == current_stats_args.flag_no_headers
                        && existing_stats_args_json.flag_dates_whitelist
                            == current_stats_args.flag_dates_whitelist
                        && existing_stats_args_json.flag_delimiter
                            == current_stats_args.flag_delimiter
                        && existing_stats_args_json.flag_nulls == current_stats_args.flag_nulls
                        && existing_stats_args_json.qsv_version == current_stats_args.qsv_version)
            {
                log::info!("{path_file_stem}.stats.csv already exists and is current, skipping...",);
                compute_stats = false;
            } else {
                log::info!(
                    "{path_file_stem}.stats.csv already exists, but is older than the input file \
                     or the args have changed, regenerating...",
                );
                fs::remove_file(&stats_file)?;
            }
        }
        if compute_stats {
            let start_time = std::time::Instant::now();

            // we're loading the entire file into memory, we need to check avail mem
            if args.flag_everything
                || args.flag_mode
                || args.flag_cardinality
                || args.flag_median
                || args.flag_quartiles
                || args.flag_mad
            {
                util::mem_file_check(&path, false, args.flag_memcheck)?;
            }

            // we need to count the number of records in the file to calculate sparsity
            let record_count = RECORD_COUNT.get_or_init(|| util::count_rows(&fconfig).unwrap());

            log::info!("scanning {record_count} records...");
            let (headers, stats) = match fconfig.indexed()? {
                None => args.sequential_stats(&args.flag_dates_whitelist),
                Some(idx) => {
                    let idx_count = idx.count();
                    if let Some(num_jobs) = args.flag_jobs {
                        if num_jobs == 1 {
                            args.sequential_stats(&args.flag_dates_whitelist)
                        } else {
                            args.parallel_stats(&args.flag_dates_whitelist, idx_count)
                        }
                    } else {
                        args.parallel_stats(&args.flag_dates_whitelist, idx_count)
                    }
                }
            }?;

            // clone a copy of stats so we can binary encode it to disk later
            if write_stats_cache {
                stats_for_encoding = stats.clone();
            }

            let stats_sr_vec = args.stats_to_records(stats);

            wtr.write_record(&args.stat_headers())?;
            let fields = headers.iter().zip(stats_sr_vec);
            for (i, (header, stat)) in fields.enumerate() {
                let header = if args.flag_no_headers {
                    i.to_string().into_bytes()
                } else {
                    header.to_vec()
                };
                let stat = stat.iter().map(str::as_bytes);
                wtr.write_record(vec![&*header].into_iter().chain(stat))?;
            }

            // update the stats args json file
            if write_stats_cache {
                current_stats_args.canonical_input_path =
                    path.canonicalize()?.to_str().unwrap().to_string();
                current_stats_args.record_count = *record_count;
                current_stats_args.compute_duration_ms = start_time.elapsed().as_millis() as u64;
                current_stats_args.date_generated = chrono::Utc::now().to_rfc3339();
            }
        }
    }

    wtr.flush()?;

    if let Some(pb) = stdin_tempfile_path {
        // remove the temp file we created to store stdin
        log::info!("deleting stdin temp file");
        std::fs::remove_file(pb)?;
    }

    let currstats_filename = if compute_stats {
        // we computed the stats, use the stats temp file
        stats_csv_tempfile_fname
    } else {
        // we didn't compute the stats, re-use the existing stats file
        stats_path(fconfig.path.as_ref().unwrap(), false)
            .to_str()
            .unwrap()
            .to_owned()
    };

    if fconfig.is_stdin() {
        // if we read from stdin, copy the temp stats file to "stdin.stats.csv"
        let mut stats_pathbuf = stats_path(fconfig.path.as_ref().unwrap(), true);
        fs::copy(currstats_filename.clone(), stats_pathbuf.clone())?;

        // save the stats args to "stdin.stats.csv.json"
        stats_pathbuf.set_extension("csv.json");
        std::fs::write(
            stats_pathbuf,
            serde_json::to_string_pretty(&current_stats_args).unwrap(),
        )?;
    } else if let Some(path) = fconfig.path {
        // if we read from a file, copy the temp stats file to "<FILESTEM>.stats.csv"
        let mut stats_pathbuf = path.clone();
        stats_pathbuf.set_extension("stats.csv");
        if currstats_filename != stats_pathbuf.to_str().unwrap() {
            // if the stats file is not the same as the input file, copy it
            fs::copy(currstats_filename.clone(), stats_pathbuf.clone())?;
        }

        // only create the stats args json and binary encoded files if we computed the stats
        if compute_stats && write_stats_cache {
            // save the stats args to "<FILESTEM>.stats.csv.json"
            stats_pathbuf.set_extension("csv.json");
            // write empty file first so we can canonicalize it
            std::fs::File::create(stats_pathbuf.clone())?;
            current_stats_args.canonical_stats_path = stats_pathbuf
                .clone()
                .canonicalize()?
                .to_str()
                .unwrap()
                .to_string();
            std::fs::write(
                stats_pathbuf.clone(),
                // safety: we know that current_stats_args is JSON serializable
                serde_json::to_string_pretty(&current_stats_args).unwrap(),
            )?;

            // binary encode the stats to "<FILESTEM>.stats.csv.bin"
            let mut stats_pathbuf = path.clone();
            stats_pathbuf.set_extension("stats.csv.bin");
            // we do the binary encoding inside a block so that the encoded_file
            // gets dropped/flushed before we copy it to the output file
            {
                let encoded_file = BufWriter::with_capacity(
                    DEFAULT_WTR_BUFFER_CAPACITY * 2,
                    fs::File::create(stats_pathbuf.clone())?,
                );
                if let Err(e) = bincode::serialize_into(encoded_file, &stats_for_encoding) {
                    return fail_clierror!(
                        "Failed to write binary encoded stats {}: {e:?}",
                        stats_pathbuf.display()
                    );
                }
            }
        }

        // if the user specified --stats-binout, copy the binary stats to that file as well
        if write_stats_cache && !stats_binout.is_empty() {
            let mut stats_pathbuf = path;
            stats_pathbuf.set_extension("stats.csv.bin");
            if let Err(e) = fs::copy(stats_pathbuf, stats_binout.clone()) {
                return fail_clierror!(
                    "Failed to copy binary encoded stats to {stats_binout}: {e:?}"
                );
            }
        }
    }

    if stdout_output_flag {
        // if we're outputting to stdout, copy the stats file to stdout
        let currstats = fs::read_to_string(currstats_filename)?;
        io::stdout().write_all(currstats.as_bytes())?;
        io::stdout().flush()?;
    } else if let Some(output) = args.flag_output {
        // if we're outputting to a file, copy the stats file to the output file
        if currstats_filename != output {
            // if the stats file is not the same as the output file, copy it
            fs::copy(currstats_filename, output)?;
        }
    }

    Ok(())
}

impl Args {
    fn sequential_stats(&self, whitelist: &str) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(
            self.flag_infer_dates,
            self.flag_prefer_dmy,
            &headers,
            whitelist,
        )?;

        let stats = self.compute(&sel, rdr.byte_records());
        Ok((headers, stats))
    }

    fn parallel_stats(
        &self,
        whitelist: &str,
        idx_count: u64,
    ) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        // N.B. This method doesn't handle the case when the number of records
        // is zero correctly. So we use `sequential_stats` instead.
        if idx_count == 0 {
            return self.sequential_stats(whitelist);
        }

        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(
            self.flag_infer_dates,
            self.flag_prefer_dmy,
            &headers,
            whitelist,
        )?;

        let chunk_size = util::chunk_size(idx_count as usize, util::njobs(self.flag_jobs));
        let nchunks = util::num_of_chunks(idx_count as usize, chunk_size);

        let pool = ThreadPool::new(util::njobs(self.flag_jobs));
        let (send, recv) = channel::bounded(0);
        for i in 0..nchunks {
            // safety: the index file call is always safe seeking may fail and you have a bigger
            // problem as the index file was modified WHILE stats is running and you
            // NEED to abort if that happens, however unlikely
            let (send, args, sel) = (send.clone(), self.clone(), sel.clone());
            pool.execute(move || {
                let mut idx = args.rconfig().indexed().unwrap().unwrap();
                idx.seek((i * chunk_size) as u64)
                    .expect("File seek failed.");
                let it = idx.byte_records().take(chunk_size);
                // safety: this will only return an Error if the channel has been disconnected
                send.send(args.compute(&sel, it)).unwrap();
            });
        }
        drop(send);
        Ok((headers, merge_all(recv.iter()).unwrap_or_default()))
    }

    fn stats_to_records(&self, stats: Vec<Stats>) -> Vec<csv::StringRecord> {
        let round_places = self.flag_round;
        let infer_boolean = self.flag_infer_boolean;
        let mut records = Vec::with_capacity(stats.len());
        records.extend(repeat(csv::StringRecord::new()).take(stats.len()));
        let pool = ThreadPool::new(util::njobs(self.flag_jobs));
        let mut results = Vec::with_capacity(stats.len());
        for mut stat in stats {
            let (send, recv) = channel::bounded(0);
            results.push(recv);
            pool.execute(move || {
                // safety: this will only return an Error if the channel has been disconnected
                // which will not happen in this case
                send.send(stat.to_record(round_places, infer_boolean))
                    .unwrap();
            });
        }
        assert!(results.len() == records.len());
        for (i, recv) in results.into_iter().enumerate() {
            // safety: the assert above guarantees that records index access is safe and
            // doesn't require a bounds check.
            // The unwrap on recv.recv() is safe as the channel is bounded
            records[i] = recv.recv().unwrap();
        }
        records
    }

    #[inline]
    fn compute<I>(&self, sel: &Selection, it: I) -> Vec<Stats>
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let sel_len = sel.len();

        let mut stats = self.new_stats(sel_len);
        assert!(stats.len() == sel_len);

        let infer_date_flags = INFER_DATE_FLAGS.get().unwrap();
        assert!(infer_date_flags.len() == sel_len);

        // safety: the asserts above guarantee that the following unwraps are safe
        // and doesn't require a bounds check.
        for row in it {
            for (i, field) in sel.select(&row.unwrap()).enumerate() {
                stats.get_mut(i).unwrap().add(
                    field,
                    *infer_date_flags.get(i).unwrap(),
                    self.flag_infer_boolean,
                );
            }
        }
        stats
    }

    #[inline]
    fn sel_headers<R: io::Read>(
        &self,
        rdr: &mut csv::Reader<R>,
    ) -> CliResult<(csv::ByteRecord, Selection)> {
        let headers = rdr.byte_headers()?.clone();
        let sel = self.rconfig().selection(&headers)?;
        Ok((sel.select(&headers).collect(), sel))
    }

    #[inline]
    fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    #[inline]
    fn new_stats(&self, record_len: usize) -> Vec<Stats> {
        let mut stats: Vec<Stats> = Vec::with_capacity(record_len);
        stats.extend(
            repeat(Stats::new(WhichStats {
                include_nulls: self.flag_nulls,
                sum:           !self.flag_typesonly,
                range:         !self.flag_typesonly || self.flag_infer_boolean,
                dist:          !self.flag_typesonly,
                cardinality:   self.flag_everything || self.flag_cardinality,
                median:        !self.flag_everything && self.flag_median && !self.flag_quartiles,
                mad:           self.flag_everything || self.flag_mad,
                quartiles:     self.flag_everything || self.flag_quartiles,
                mode:          self.flag_everything || self.flag_mode,
                typesonly:     self.flag_typesonly,
            }))
            .take(record_len),
        );
        stats
    }

    pub fn stat_headers(&self) -> csv::StringRecord {
        if self.flag_typesonly {
            return csv::StringRecord::from(vec!["field", "type"]);
        }

        // with --everything, we have 30 columns at most
        let mut fields = Vec::with_capacity(30);
        fields.extend_from_slice(&[
            "field",
            "type",
            "sum",
            "min",
            "max",
            "range",
            "min_length",
            "max_length",
            "mean",
            "stddev",
            "variance",
            "nullcount",
            "sparsity",
        ]);
        let all = self.flag_everything;
        if self.flag_median && !self.flag_quartiles && !all {
            fields.push("median");
        }
        if self.flag_mad || all {
            fields.push("mad");
        }
        if self.flag_quartiles || all {
            fields.extend_from_slice(&[
                "lower_outer_fence",
                "lower_inner_fence",
                "q1",
                "q2_median",
                "q3",
                "iqr",
                "upper_inner_fence",
                "upper_outer_fence",
                "skewness",
            ]);
        }
        if self.flag_cardinality || all {
            fields.push("cardinality");
        }
        if self.flag_mode || all {
            fields.extend_from_slice(&[
                "mode",
                "mode_count",
                "mode_occurrences",
                "antimode",
                "antimode_count",
                "antimode_occurrences",
            ]);
        }
        csv::StringRecord::from(fields)
    }
}

// returns the path to the stats file
// safety: unwraps are safe because we know stats_csv_path is a valid path
fn stats_path(stats_csv_path: &Path, stdin_flag: bool) -> PathBuf {
    let mut p = stats_csv_path
        .to_path_buf()
        .into_os_string()
        .into_string()
        .unwrap();

    let fname = stats_csv_path
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    let fstem = stats_csv_path
        .file_stem()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    if let Some(nofn) = p.strip_suffix(&fname) {
        p = nofn.to_string();
    } else {
        p = String::new();
    }
    if stdin_flag {
        p.push_str("stdin.stats.csv");
    } else {
        p.push_str(&format!("{fstem}.stats.csv"));
    }

    PathBuf::from(&p)
}

#[inline]
fn init_date_inference(
    infer_dates: bool,
    prefer_dmy: bool,
    headers: &csv::ByteRecord,
    flag_whitelist: &str,
) -> Result<(), String> {
    if !infer_dates {
        // we're not inferring dates, set INFER_DATE_FLAGS to all false
        INFER_DATE_FLAGS
            .set(vec![false; headers.len()])
            .map_err(|e| format!("Cannot init empty date inference flags: {e:?}"))?;
        return Ok(());
    }

    let dmy_preferred = prefer_dmy || util::get_envvar_flag("QSV_PREFER_DMY");
    DMY_PREFERENCE.store(dmy_preferred, Ordering::Relaxed);

    let whitelist_lower = flag_whitelist.to_lowercase();
    log::info!("inferring dates with date-whitelist: {whitelist_lower}");

    let infer_date_flags = if whitelist_lower == "all" {
        log::info!("inferring dates for ALL fields with DMY preference: {dmy_preferred}");
        vec![true; headers.len()]
    } else {
        let whitelist = whitelist_lower
            .split(',')
            .map(str::trim)
            .collect::<Vec<_>>();
        headers
            .iter()
            .map(|header| {
                let header_str = from_bytes::<String>(header).unwrap().to_lowercase();
                let date_found = whitelist
                    .iter()
                    .any(|whitelist_item| header_str.contains(whitelist_item));
                if date_found {
                    log::info!(
                        "inferring dates for {header_str} with DMY preference: {dmy_preferred}"
                    );
                }
                date_found
            })
            .collect()
    };

    INFER_DATE_FLAGS
        .set(infer_date_flags)
        .map_err(|e| format!("Cannot init date inference flags: {:?}", e))?;
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
struct WhichStats {
    include_nulls: bool,
    sum:           bool,
    range:         bool,
    dist:          bool,
    cardinality:   bool,
    median:        bool,
    mad:           bool,
    quartiles:     bool,
    mode:          bool,
    typesonly:     bool,
}

impl Commute for WhichStats {
    #[inline]
    fn merge(&mut self, other: WhichStats) {
        assert_eq!(*self, other);
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Stats {
    typ:       FieldType,
    sum:       Option<TypedSum>,
    minmax:    Option<TypedMinMax>,
    online:    Option<OnlineStats>,
    nullcount: u64,
    modes:     Option<Unsorted<Vec<u8>>>,
    median:    Option<Unsorted<f64>>,
    mad:       Option<Unsorted<f64>>,
    quartiles: Option<Unsorted<f64>>,
    which:     WhichStats,
}

#[inline]
fn timestamp_ms_to_rfc3339(timestamp: i64, typ: FieldType) -> String {
    use chrono::{DateTime, NaiveDateTime, Utc};

    let date_val = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_millis(timestamp).unwrap_or_default(),
        Utc,
    )
    .to_rfc3339();

    // if type = Date, only return the date component
    // do not return the time component
    if typ == TDate {
        return date_val[..10].to_string();
    }
    date_val
}

impl Stats {
    fn new(which: WhichStats) -> Stats {
        let (mut sum, mut minmax, mut online, mut modes, mut median, mut quartiles, mut mad) =
            (None, None, None, None, None, None, None);
        if which.sum {
            sum = Some(TypedSum::default());
        }
        if which.range {
            minmax = Some(TypedMinMax::default());
        }
        if which.dist {
            online = Some(stats::OnlineStats::default());
        }
        if which.mode || which.cardinality {
            modes = Some(stats::Unsorted::default());
        }
        if which.quartiles {
            quartiles = Some(stats::Unsorted::default());
        } else if which.median {
            median = Some(stats::Unsorted::default());
        }
        if which.mad {
            mad = Some(stats::Unsorted::default());
        }
        Stats {
            typ: FieldType::default(),
            sum,
            minmax,
            online,
            nullcount: 0,
            modes,
            median,
            mad,
            quartiles,
            which,
        }
    }

    #[inline]
    fn add(&mut self, sample: &[u8], infer_dates: bool, infer_boolean: bool) {
        let (sample_type, timestamp_val) = FieldType::from_sample(infer_dates, sample, self.typ);
        self.typ.merge(sample_type);

        // we're inferring --typesonly, so don't add samples to compute statistics
        // unless we need to --infer-boolean. In which case, we need --cardinality
        // and --range, so we need to add samples.
        if self.which.typesonly && !infer_boolean {
            return;
        }

        let t = self.typ;
        if let Some(v) = self.sum.as_mut() {
            v.add(t, sample);
        };
        if let Some(v) = self.minmax.as_mut() {
            if let Some(ts_val) = timestamp_val {
                let mut buffer = itoa::Buffer::new();
                v.add(t, buffer.format(ts_val).as_bytes());
            } else {
                v.add(t, sample);
            }
        };
        if let Some(v) = self.modes.as_mut() {
            v.add(sample.to_vec());
        };
        if sample_type == TNull {
            self.nullcount += 1;
        }
        match t {
            TNull => {
                if self.which.include_nulls {
                    if let Some(v) = self.online.as_mut() {
                        v.add_null();
                    };
                }
            }
            TFloat | TInteger => {
                if sample_type == TNull {
                    if self.which.include_nulls {
                        if let Some(v) = self.online.as_mut() {
                            v.add_null();
                        };
                    }
                } else {
                    let n = from_bytes::<f64>(sample).unwrap();
                    if let Some(v) = self.median.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.mad.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.quartiles.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.online.as_mut() {
                        v.add(&n);
                    }
                }
            }
            // do nothing for String type
            TString => {}
            TDateTime | TDate => {
                if sample_type == TNull {
                    if self.which.include_nulls {
                        if let Some(v) = self.online.as_mut() {
                            v.add_null();
                        };
                    }
                // if ts_val.is_some() then we successfully inferred a date from the sample
                // and the timestamp value is not None
                } else if let Some(ts_val) = timestamp_val {
                    // calculate date statistics by adding date samples as timestamps to
                    // millisecond precision.
                    #[allow(clippy::cast_precision_loss)]
                    let n = ts_val as f64;
                    if let Some(v) = self.median.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.mad.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.quartiles.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.online.as_mut() {
                        v.add(&n);
                    }
                }
            }
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_record(&mut self, round_places: u32, infer_boolean: bool) -> csv::StringRecord {
        // we're doing typesonly and not inferring boolean, just return the type
        if self.which.typesonly && !infer_boolean {
            return csv::StringRecord::from(vec![self.typ.to_string()]);
        }

        let typ = self.typ;
        // prealloc memory for performance
        // we have 30 columns at most with --everything
        let mut pieces = Vec::with_capacity(30);

        let empty = String::new;

        // mode/modes & cardinality
        // we do this first because we need to know the cardinality to --infer-boolean
        // should that be enabled
        let mut cardinality = 0_usize;
        let mut mc_pieces = Vec::with_capacity(7);
        match self.modes.as_mut() {
            None => {
                if self.which.cardinality {
                    mc_pieces.push(empty());
                }
                if self.which.mode {
                    mc_pieces.extend_from_slice(&[empty(), empty(), empty(), empty()]);
                }
            }
            Some(ref mut v) => {
                if self.which.cardinality {
                    cardinality = v.cardinality();
                    let mut buffer = itoa::Buffer::new();
                    mc_pieces.push(buffer.format(cardinality).to_owned());
                }
                if self.which.mode {
                    // mode/s
                    let (modes_result, modes_count, mode_occurrences) = v.modes();
                    let modes_list = modes_result
                        .iter()
                        .map(|c| String::from_utf8_lossy(c))
                        .join(",");
                    mc_pieces.extend_from_slice(&[
                        modes_list,
                        modes_count.to_string(),
                        mode_occurrences.to_string(),
                    ]);

                    // antimode/s
                    if mode_occurrences == 0 {
                        // all the values are unique
                        // so instead of returning everything, just say *ALL
                        mc_pieces.extend_from_slice(&[
                            "*ALL".to_string(),
                            "0".to_string(),
                            "1".to_string(),
                        ]);
                    } else {
                        let (antimodes_result, antimodes_count, antimode_occurrences) =
                            v.antimodes();
                        let mut antimodes_list = String::new();

                        // We only store the first 10 antimodes
                        // so if antimodes_count > 10, add the "*PREVIEW: " prefix
                        if antimodes_count > 10 {
                            antimodes_list.push_str("*PREVIEW: ");
                        }

                        let antimodes_vals = &antimodes_result
                            .iter()
                            .map(|c| String::from_utf8_lossy(c))
                            .join(",");
                        if antimodes_vals.starts_with(',') {
                            antimodes_list.push_str("NULL");
                        }
                        antimodes_list.push_str(antimodes_vals);

                        // and truncate at 100 characters with an ellipsis
                        if antimodes_list.len() > 100 {
                            util::utf8_truncate(&mut antimodes_list, 101);
                            antimodes_list.push_str("...");
                        }

                        mc_pieces.extend_from_slice(&[
                            antimodes_list,
                            antimodes_count.to_string(),
                            antimode_occurrences.to_string(),
                        ]);
                    }
                }
            }
        }

        // min/max/range
        // we also do this before --infer-boolean because we need to know the min/max values
        // to determine if the range is equal to the supported boolean ranges (0/1, f/t, n/y)
        let mut minmax_range_pieces = Vec::with_capacity(3);
        let mut minval_lower: char = '\0';
        let mut maxval_lower: char = '\0';
        if let Some(mm) = self
            .minmax
            .as_ref()
            .and_then(|mm| mm.show(typ, round_places))
        {
            // get first character of min/max values
            minval_lower = mm.0.chars().next().unwrap_or_default().to_ascii_lowercase();
            maxval_lower = mm.1.chars().next().unwrap_or_default().to_ascii_lowercase();
            minmax_range_pieces.extend_from_slice(&[mm.0, mm.1, mm.2]);
        } else {
            minmax_range_pieces.extend_from_slice(&[empty(), empty(), empty()]);
        }

        // type
        if cardinality == 2 && infer_boolean {
            // if cardinality is 2, it's a boolean if its values' first character are 0/1, f/t, n/y
            if (minval_lower == '0' && maxval_lower == '1')
                || (minval_lower == 'f' && maxval_lower == 't')
                || (minval_lower == 'n' && maxval_lower == 'y')
            {
                pieces.push("Boolean".to_string());
            } else {
                pieces.push(typ.to_string());
            }
        } else {
            pieces.push(typ.to_string());
        }

        // we're doing --typesonly with --infer-boolean, we don't need to calculate anything else
        if self.which.typesonly && infer_boolean {
            return csv::StringRecord::from(pieces);
        }

        // sum
        if let Some(sum) = self.sum.as_ref().and_then(|sum| sum.show(typ)) {
            if typ == FieldType::TFloat {
                if let Ok(f64_val) = fast_float::parse::<f64, _>(&sum) {
                    pieces.push(util::round_num(f64_val, round_places));
                } else {
                    pieces.push(format!("ERROR: Cannot convert {sum} to a float."));
                }
            } else {
                pieces.push(sum);
            }
        } else {
            pieces.push(empty());
        }

        // min/max/range
        // actually append it here - to preserve legacy ordering of columns
        pieces.extend_from_slice(&minmax_range_pieces);

        // min/max length
        if typ == FieldType::TDate || typ == FieldType::TDateTime {
            // returning min/max length for dates doesn't make sense
            // especially since we convert the date stats to rfc3339 format
            pieces.extend_from_slice(&[empty(), empty()]);
        } else if let Some(mm) = self.minmax.as_ref().and_then(TypedMinMax::len_range) {
            pieces.extend_from_slice(&[mm.0, mm.1]);
        } else {
            pieces.extend_from_slice(&[empty(), empty()]);
        }

        // mean, stddev & variance
        if typ == TString || typ == TNull {
            pieces.extend_from_slice(&[empty(), empty(), empty()]);
        } else if let Some(ref v) = self.online {
            if self.typ == TFloat || self.typ == TInteger {
                pieces.extend_from_slice(&[
                    util::round_num(v.mean(), round_places),
                    util::round_num(v.stddev(), round_places),
                    util::round_num(v.variance(), round_places),
                ]);
            } else {
                pieces.push(timestamp_ms_to_rfc3339(v.mean() as i64, typ));
                // instead of returning stdev in seconds, let's return it in
                // days as it easier to handle
                // Round to at least 5 decimal places, so we have millisecond precision
                pieces.push(util::round_num(
                    v.stddev() / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                pieces.push(util::round_num(
                    v.variance() / (MS_IN_DAY * MS_IN_DAY),
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
            }
        } else {
            pieces.extend_from_slice(&[empty(), empty(), empty()]);
        }

        // nullcount
        let mut buffer = itoa::Buffer::new();
        pieces.push(buffer.format(self.nullcount).to_owned());

        // sparsity
        // stats is also called by the `schema` and `tojsonl` commands to infer a schema,
        // sparsity is not required by those cmds and we don't necessarily have the
        // record_count when called by those cmds, so just set sparsity to nullcount
        // (i.e. divide by 1) so we don't panic.
        #[allow(clippy::cast_precision_loss)]
        let sparsity: f64 = self.nullcount as f64 / *RECORD_COUNT.get().unwrap_or(&1) as f64;
        pieces.push(util::round_num(sparsity, round_places));

        // median
        let mut existing_median = None;
        if let Some(v) = self.median.as_mut().and_then(|v| {
            if let TNull | TString = typ {
                None
            } else {
                existing_median = v.median();
                existing_median
            }
        }) {
            if typ == TDateTime || typ == TDate {
                pieces.push(timestamp_ms_to_rfc3339(v as i64, typ));
            } else {
                pieces.push(util::round_num(v, round_places));
            }
        } else if self.which.median {
            pieces.push(empty());
        }

        // median absolute deviation (MAD)
        if let Some(v) = self.mad.as_mut().and_then(|v| {
            if let TNull | TString = typ {
                None
            } else {
                v.mad(existing_median)
            }
        }) {
            if typ == TDateTime || typ == TDate {
                // like stddev, return MAD in days
                pieces.push(util::round_num(
                    v / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
            } else {
                pieces.push(util::round_num(v, round_places));
            }
        } else if self.which.mad {
            pieces.push(empty());
        }

        // quartiles
        match self.quartiles.as_mut().and_then(|v| match typ {
            TInteger | TFloat | TDate | TDateTime => v.quartiles(),
            _ => None,
        }) {
            None => {
                if self.which.quartiles {
                    pieces.extend_from_slice(&[
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                    ]);
                }
            }
            Some((q1, q2, q3)) => {
                let iqr = q3 - q1;

                // use fused multiply add (mul_add) when possible
                // fused mul_add is more accurate & may be more performant if the
                // target architecture has a dedicated `fma` CPU instruction
                // https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add

                // lower_outer_fence = "q1 - (3.0 * iqr)"
                let lof = 3.0f64.mul_add(-iqr, q1);
                // lower_inner_fence = "q1 - (1.5 * iqr)"
                let lif = 1.5f64.mul_add(-iqr, q1);

                // upper inner fence = "q3 + (1.5 * iqr)"
                let uif = 1.5_f64.mul_add(iqr, q3);
                // upper_outer_fence = "q3 + (3.0 * iqr)"
                let uof = 3.0_f64.mul_add(iqr, q3);

                // calculate skewness using Quantile-based measures
                // https://en.wikipedia.org/wiki/Skewness#Quantile-based_measures
                // https://blogs.sas.com/content/iml/2017/07/19/quantile-skewness.html
                // quantile skewness = ((q3 - q2) - (q2 - q1)) / iqr;
                // which is also (q3 - (2.0 * q2) + q1) / iqr
                // which in turn, is the basis of the fused multiply add version below
                let skewness = (2.0f64.mul_add(-q2, q3) + q1) / iqr;

                if typ == TDateTime || typ == TDate {
                    // casting from f64 to i64 is OK, per
                    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
                    // as values larger/smaller than what i64 can handle will automatically
                    // saturate to i64 max/min values.
                    pieces.extend_from_slice(&[
                        timestamp_ms_to_rfc3339(lof as i64, typ),
                        timestamp_ms_to_rfc3339(lif as i64, typ),
                        timestamp_ms_to_rfc3339(q1 as i64, typ),
                        timestamp_ms_to_rfc3339(q2 as i64, typ), // q2 = median
                        timestamp_ms_to_rfc3339(q3 as i64, typ),
                        // return iqr in days - there are 86,400,000 ms in a day
                        util::round_num(
                            (q3 - q1) / MS_IN_DAY,
                            u32::max(round_places, DAY_DECIMAL_PLACES),
                        ),
                        timestamp_ms_to_rfc3339(uif as i64, typ),
                        timestamp_ms_to_rfc3339(uof as i64, typ),
                    ]);
                } else {
                    pieces.extend_from_slice(&[
                        util::round_num(lof, round_places),
                        util::round_num(lif, round_places),
                        util::round_num(q1, round_places),
                        util::round_num(q2, round_places), // q2 = median
                        util::round_num(q3, round_places),
                        util::round_num(iqr, round_places),
                        util::round_num(uif, round_places),
                        util::round_num(uof, round_places),
                    ]);
                }
                pieces.push(util::round_num(skewness, round_places));
            }
        }

        // mode/modes/antimodes & cardinality
        // append it here to preserve legacy ordering of columns
        pieces.extend_from_slice(&mc_pieces);

        csv::StringRecord::from(pieces)
    }
}

impl Commute for Stats {
    #[inline]
    fn merge(&mut self, other: Stats) {
        self.typ.merge(other.typ);
        self.sum.merge(other.sum);
        self.minmax.merge(other.minmax);
        self.online.merge(other.online);
        self.nullcount += other.nullcount;
        self.modes.merge(other.modes);
        self.median.merge(other.median);
        self.quartiles.merge(other.quartiles);
        self.which.merge(other.which);
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
enum FieldType {
    // The default - TNull, is the most specific type.
    // Type inference proceeds by assuming the most specific type and then
    // relaxing the type as counter-examples are found.
    #[default]
    TNull,
    TString,
    TFloat,
    TInteger,
    TDate,
    TDateTime,
}

impl FieldType {
    // infer data type from a given sample & current type inference
    // infer_dates signals if date inference should be attempted
    // returns the inferred type and if infer_dates is true,
    // the date in ms since the epoch if the type is a date or datetime
    // otherwise, None
    #[inline]
    pub fn from_sample(
        infer_dates: bool,
        sample: &[u8],
        current_type: FieldType,
    ) -> (FieldType, Option<i64>) {
        #[allow(clippy::len_zero)]
        if sample.len() == 0 {
            return (TNull, None);
        }
        // no need to do type checking if current_type is already a String
        if current_type == FieldType::TString {
            return (FieldType::TString, None);
        }

        let Ok(string) = from_utf8(sample) else {
            // if the string is not valid utf8, we assume it is a binary string
            // and return a string type
            return (FieldType::TString, None);
        };

        if current_type == FieldType::TFloat
            || current_type == FieldType::TInteger
            || current_type == FieldType::TNull
        {
            if let Ok(int_val) = string.parse::<i64>() {
                // leading zero, its a string (e.g. zip codes)
                if string.starts_with('0') && int_val != 0 {
                    return (TString, None);
                }
                return (TInteger, None);
            }

            if fast_float::parse::<f64, _>(string).is_ok() {
                return (TFloat, None);
            }
        }

        if infer_dates
            && (current_type == FieldType::TDate
                || current_type == FieldType::TDateTime
                || current_type == FieldType::TNull)
        {
            if let Ok(parsed_date) =
                parse_with_preference(string, DMY_PREFERENCE.load(Ordering::Relaxed))
            {
                // get date in rfc3339 format, if it ends with "T00:00:00+00:00"
                // its a Date type, otherwise, its DateTime.
                let ts_val = parsed_date.timestamp_millis();
                if parsed_date.to_rfc3339().ends_with("T00:00:00+00:00") {
                    return (TDate, Some(ts_val));
                }
                return (TDateTime, Some(ts_val));
            }
        }
        (TString, None)
    }
}

impl Commute for FieldType {
    #[inline]
    #[allow(clippy::match_same_arms)]
    // we allow match_same_arms because we want are optimizing for
    // performance and not readability, as match arms are evaluated in order
    // so we want to put the most common cases first
    fn merge(&mut self, other: FieldType) {
        *self = match (*self, other) {
            (TString, TString) => TString,
            (TFloat, TFloat) => TFloat,
            (TInteger, TInteger) => TInteger,
            // Null does not impact the type.
            (TNull, any) | (any, TNull) => any,
            // Integers can degrade to floats.
            (TFloat, TInteger) | (TInteger, TFloat) => TFloat,
            // date data types
            (TDate, TDate) => TDate,
            (TDateTime | TDate, TDateTime) | (TDateTime, TDate) => TDateTime,
            // anything else is a String
            (_, _) => TString,
        };
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
        }
    }
}

impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
        }
    }
}

/// `TypedSum` keeps a rolling sum of the data seen.
/// It sums integers until it sees a float, at which point it sums floats.
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
struct TypedSum {
    integer: i64,
    float:   Option<f64>,
}

impl TypedSum {
    #[inline]
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        #[allow(clippy::len_zero)]
        if sample.len() == 0 {
            return;
        }
        #[allow(clippy::cast_precision_loss)]
        match typ {
            TFloat => {
                let float: f64 = from_bytes::<f64>(sample).unwrap();
                match self.float {
                    None => {
                        self.float = Some((self.integer as f64) + float);
                    }
                    Some(ref mut f) => {
                        *f += float;
                    }
                }
            }
            TInteger => {
                if let Some(ref mut float) = self.float {
                    *float += from_bytes::<f64>(sample).unwrap();
                } else {
                    // so we don't panic on overflow/underflow, use saturating_add
                    self.integer = self
                        .integer
                        .saturating_add(from_bytes::<i64>(sample).unwrap());
                }
            }
            _ => {}
        }
    }

    fn show(&self, typ: FieldType) -> Option<String> {
        match typ {
            TNull | TString | TDate | TDateTime => None,
            TInteger => {
                match self.integer {
                    // with saturating_add, if this is equal to i64::MAX or i64::MIN
                    // we overflowed/underflowed
                    i64::MAX => Some("OVERFLOW".to_string()),
                    i64::MIN => Some("UNDERFLOW".to_string()),
                    _ => {
                        let mut buffer = itoa::Buffer::new();
                        Some(buffer.format(self.integer).to_owned())
                    }
                }
            }
            TFloat => {
                let mut buffer = ryu::Buffer::new();
                Some(buffer.format(self.float.unwrap_or(0.0)).to_owned())
            }
        }
    }
}

impl Commute for TypedSum {
    #[inline]
    fn merge(&mut self, other: TypedSum) {
        #[allow(clippy::cast_precision_loss)]
        match (self.float, other.float) {
            (Some(f1), Some(f2)) => self.float = Some(f1 + f2),
            (Some(f1), None) => self.float = Some(f1 + (other.integer as f64)),
            (None, Some(f2)) => self.float = Some((self.integer as f64) + f2),
            (None, None) => self.integer = self.integer.saturating_add(other.integer),
        }
    }
}

/// `TypedMinMax` keeps track of minimum/maximum/range values for each possible type
/// where min/max/range makes sense.
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
struct TypedMinMax {
    strings:  MinMax<Vec<u8>>,
    str_len:  MinMax<usize>,
    integers: MinMax<i64>,
    floats:   MinMax<f64>,
    dates:    MinMax<i64>,
}

impl TypedMinMax {
    #[inline]
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        let sample_len = sample.len();
        self.str_len.add(sample_len);
        if sample_len == 0 {
            return;
        }
        self.strings.add(sample.to_vec());
        // safety: we can use unwrap below since we know the data type of the sample
        match typ {
            TString | TNull => {}
            TFloat => {
                let n = fast_float::parse::<f64, _>(from_utf8(sample).unwrap()).unwrap();

                self.floats.add(n);
                self.integers.add(n as i64);
            }
            TInteger => {
                let n = from_utf8(sample).unwrap().parse::<i64>().unwrap();
                self.integers.add(n);
                #[allow(clippy::cast_precision_loss)]
                self.floats.add(n as f64);
            }
            TDate | TDateTime => {
                let n = from_utf8(sample).unwrap().parse::<i64>().unwrap();
                self.dates.add(n);
            }
        }
    }

    fn len_range(&self) -> Option<(String, String)> {
        if let (Some(min), Some(max)) = (self.str_len.min(), self.str_len.max()) {
            let mut buffer = itoa::Buffer::new();
            Some((
                buffer.format(*min).to_owned(),
                buffer.format(*max).to_owned(),
            ))
        } else {
            None
        }
    }

    #[inline]
    fn show(&self, typ: FieldType, round_places: u32) -> Option<(String, String, String)> {
        match typ {
            TNull => None,
            TString => {
                if let (Some(min), Some(max)) = (self.strings.min(), self.strings.max()) {
                    let min = String::from_utf8_lossy(min).to_string();
                    let max = String::from_utf8_lossy(max).to_string();
                    Some((min, max, String::new()))
                } else {
                    None
                }
            }
            TInteger => {
                if let (Some(min), Some(max)) = (self.integers.min(), self.integers.max()) {
                    let mut buffer = itoa::Buffer::new();
                    Some((
                        buffer.format(*min).to_owned(),
                        buffer.format(*max).to_owned(),
                        buffer.format(*max - *min).to_owned(),
                    ))
                } else {
                    None
                }
            }
            TFloat => {
                if let (Some(min), Some(max)) = (self.floats.min(), self.floats.max()) {
                    let mut buffer = ryu::Buffer::new();
                    Some((
                        buffer.format(*min).to_owned(),
                        buffer.format(*max).to_owned(),
                        util::round_num(*max - *min, round_places),
                    ))
                } else {
                    None
                }
            }
            TDateTime | TDate => {
                if let (Some(min), Some(max)) = (self.dates.min(), self.dates.max()) {
                    Some((
                        timestamp_ms_to_rfc3339(*min, typ),
                        timestamp_ms_to_rfc3339(*max, typ),
                        // return in days, not timestamp in milliseconds
                        #[allow(clippy::cast_precision_loss)]
                        util::round_num(
                            (*max - *min) as f64 / MS_IN_DAY,
                            u32::max(round_places, 5),
                        ),
                    ))
                } else {
                    None
                }
            }
        }
    }
}

impl Commute for TypedMinMax {
    #[inline]
    fn merge(&mut self, other: TypedMinMax) {
        self.strings.merge(other.strings);
        self.str_len.merge(other.str_len);
        self.integers.merge(other.integers);
        self.floats.merge(other.floats);
        self.dates.merge(other.dates);
    }
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn from_bytes<T: std::str::FromStr>(bytes: &[u8]) -> Option<T> {
    if let Ok(x) = simdutf8::basic::from_utf8(bytes) {
        x.parse().ok()
    } else {
        None
    }
}
