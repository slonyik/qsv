static USAGE: &str = r#"
Joins two sets of CSV data on the specified columns using the Pola.rs engine.

The default join operation is an 'inner' join. This corresponds to the
intersection of rows on the keys specified.

Unlike the join command, joinp can process files larger than RAM, is multi-threaded,
has join key validation, pre-join filtering, supports asof joins & its output doesn't
have duplicate columns.

However, joinp doesn't have an --ignore-case option & it doesn't support right outer joins.

Returns the shape of the join result (number of rows, number of columns) to stderr.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_joinp.rs.

Usage:
    qsv joinp [options] <columns1> <input1> <columns2> <input2>
    qsv joinp --cross <input1> <input2>
    qsv joinp --help

joinp arguments:
    Both <input1> and <input2> files need to have headers. Stdin is not supported.

    The columns arguments specify the columns to join for each input. Columns are
    referenced by name. Specify multiple columns by separating them with a comma.
    Both <columns1> and <columns2> must specify exactly the same number of columns.

    Note that <input1> is the left CSV data set and <input2> is the right CSV data set.

joinp options:
    --left                 Do a 'left outer' join. This returns all rows in
                           first CSV data set, including rows with no
                           corresponding row in the second data set. When no
                           corresponding row exists, it is padded out with
                           empty fields.
    --left-anti            This returns only the rows in the first CSV data set
                           that do not have a corresponding row in the second
                           data set. The output schema is the same as the
                           first dataset.
    --left-semi            This returns only the rows in the first CSV data set
                           that have a corresponding row in the second data set.
                           The output schema is the same as the first data set.
    --full                 Do a 'full outer' join. This returns all rows in
                           both data sets with matching records joined. If
                           there is no match, the missing side will be padded
                           out with empty fields.
    --cross                USE WITH CAUTION.
                           This returns the cartesian product of the CSV
                           data sets given. The number of rows return is
                           equal to N * M, where N and M correspond to the
                           number of rows in the given data sets, respectively.
                           The columns1 and columns2 arguments are ignored.

    --filter-left <arg>    Filter the left CSV data set by the given Polars SQL
                           expression BEFORE the join. Only rows that evaluates
                           to true are used in the join.
    --filter-right <arg>   Filter the right CSV data set by the given Polars SQL
                           expression BEFORE the join. Only rows that evaluates
                           to true are used in the join.
    --validate <arg>       Validate the join keys BEFORE performing the join.
                           Valid values are:
                             none - No validation is performed.
                             onetomany - join keys are unique in the left data set.
                             manytoone - join keys are unique in the right data set.
                             onetoone - join keys are unique in both left & right data sets.
                           [default: none]
    --nulls                When set, joins will work on empty fields.
                           Otherwise, empty fields are completely ignored.
    --streaming            When set, the join will be done in a streaming fashion.
                           Note that this will make the join slower. Only use this
                           when you get out of memory errors.

                           POLARS CSV PARSING OPTIONS:
    --try-parsedates       When set, will attempt to parse the columns as dates.
                           If the parse fails, columns remain as strings.
                           This is useful when the join keys are formatted as 
                           dates with differing date formats, as the date formats
                           will be normalized. Note that this will be automatically 
                           enabled when using asof joins.
    --infer-len <arg>      The number of rows to scan when inferring the schema of the CSV.
                           Set to 0 to do a full table scan (warning: very slow).
                           (default: 250)
    --low-memory           Use low memory mode when parsing CSVs. This will use less memory
                           but will be slower. It will also process the join in streaming mode.
                           Only use this when you get out of memory errors.
    --ignore-errors        Ignore errors when parsing CSVs. If set, rows with errors
                           will be skipped. If not set, the query will fail.
                           Only use this when debugging queries, as polars does batched
                           parsing and will skip the entire batch where the error occurred.

                           ASOF JOIN OPTIONS:
    --asof                 Do an 'asof' join. This is similar to a left inner
                           join, except we match on nearest key rather than
                           equal keys. Note that both CSV data sets will be SORTED
                           AUTOMATICALLY on the join columns.
    --left_by <arg>        Do an 'asof_by' join - a special implementation of the asof
                           join that searches for the nearest keys within a subgroup
                           set by the asof_by columns. This specifies the column/s for
                           the left CSV. Columns are referenced by name. Specify
                           multiple columns by separating them with a comma.
    --right_by <arg>       Do an 'asof_by' join. This specifies the column/s for
                           the right CSV.     
    --strategy <arg>       The strategy to use for the asof join:
                             backward - For each row in the first CSV data set,
                                        we find the last row in the second data set
                                        whose key is less than or equal to the key
                                        in the first data set.
                             forward -  For each row in the first CSV data set,
                                        we find the first row in the second data set
                                        whose key is greater than or equal to the key
                                        in the first data set.
                             nearest -  selects the last row in the second data set
                                        whose value is nearest to the value in the
                                        first data set.
                           [default: backward]
    --tolerance <arg>      The tolerance for the nearest asof join. This is only
                           used when the nearest strategy is used. The
                           tolerance is a positive integer that specifies
                           the maximum number of rows to search for a match.

                           If the join is done on a column of type Date, Time or
                           DateTime, then the tolerance is interpreted using
                           the following language:
                                1d - 1 day
                                1h - 1 hour
                                1m - 1 minute
                                1s - 1 second
                                1ms - 1 millisecond
                                1us - 1 microsecond
                                1ns - 1 nanosecond
                                1w - 1 week
                                1mo - 1 month
                                1q - 1 quarter
                                1y - 1 year
                                1i - 1 index count
                             Or combine them: “3d12h4m25s” # 3 days, 12 hours,
                             4 minutes, and 25 seconds
                             Suffix with “_saturating” to indicate that dates too
                             large for their month should saturate at the largest date
                             (e.g. 2022-02-29 -> 2022-02-28) instead of erroring.

                             OUTPUT FORMAT OPTIONS:
   --datetime-format <fmt>   The datetime format to use writing datetimes.
                             See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
                             for the list of valid format specifiers.
   --date-format <fmt>       The date format to use writing dates.
   --time-format <fmt>       The time format to use writing times.
   --float-precision <arg>   The number of digits of precision to use when writing floats.
                             (default: 6)
   --null-value <arg>        The string to use when writing null values.
                             (default: <empty string>)                             

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -Q, --quiet            Do not return join shape to stderr.
"#;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    str,
};

use polars::{
    chunked_array::object::{AsOfOptions, AsofStrategy},
    datatypes::AnyValue,
    frame::hash_join::{JoinType, JoinValidation},
    prelude::{CsvWriter, LazyCsvReader, LazyFileListReader, LazyFrame, SerWriter, SortOptions},
};
use serde::Deserialize;
use smartstring;

use crate::{config::Delimiter, util, CliError, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_columns1:         String,
    arg_input1:           String,
    arg_columns2:         String,
    arg_input2:           String,
    flag_left:            bool,
    flag_left_anti:       bool,
    flag_left_semi:       bool,
    flag_full:            bool,
    flag_cross:           bool,
    flag_filter_left:     Option<String>,
    flag_filter_right:    Option<String>,
    flag_validate:        Option<String>,
    flag_nulls:           bool,
    flag_streaming:       bool,
    flag_try_parsedates:  bool,
    flag_infer_len:       usize,
    flag_low_memory:      bool,
    flag_ignore_errors:   bool,
    flag_asof:            bool,
    flag_left_by:         Option<String>,
    flag_right_by:        Option<String>,
    flag_strategy:        Option<String>,
    flag_tolerance:       Option<String>,
    flag_datetime_format: Option<String>,
    flag_date_format:     Option<String>,
    flag_time_format:     Option<String>,
    flag_float_precision: Option<usize>,
    flag_null_value:      String,
    flag_output:          Option<String>,
    flag_delimiter:       Option<Delimiter>,
    flag_quiet:           bool,
}

impl From<polars::error::PolarsError> for CliError {
    fn from(err: polars::error::PolarsError) -> CliError {
        CliError::Other(format!("Polars error: {err:?}"))
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    // always try to parse dates when its an asof join
    // just in case the user doesn't specify it
    // and they're using date/time/datetime columns
    if args.flag_asof {
        args.flag_try_parsedates = true;
    }
    let join = args.new_join(
        args.flag_try_parsedates,
        args.flag_infer_len,
        args.flag_low_memory,
        args.flag_ignore_errors,
    )?;

    // safety: flag_validate is always is_some() as it has a default value
    args.flag_validate = Some(args.flag_validate.unwrap().to_lowercase());
    let validation = match args.flag_validate.as_deref() {
        Some("none") | None => JoinValidation::ManyToMany,
        Some("onetomany") => JoinValidation::OneToMany,
        Some("manytoone") => JoinValidation::ManyToOne,
        Some("onetoone") => JoinValidation::OneToOne,
        Some(s) => return fail_clierror!("Invalid join validation: {s}"),
    };

    let join_shape: (usize, usize) = match (
        args.flag_left,
        args.flag_left_anti,
        args.flag_left_semi,
        args.flag_full,
        args.flag_cross,
        args.flag_asof,
    ) {
        (false, false, false, false, false, false) => join.run(JoinType::Inner, validation, false),
        (true, false, false, false, false, false) => join.run(JoinType::Left, validation, false),
        (false, true, false, false, false, false) => join.run(JoinType::Anti, validation, false),
        (false, false, true, false, false, false) => join.run(JoinType::Semi, validation, false),
        (false, false, false, true, false, false) => join.run(JoinType::Outer, validation, false),
        (false, false, false, false, true, false) => join.run(JoinType::Cross, validation, false),
        (false, false, false, false, false, true) => {
            // safety: flag_strategy is always is_some() as it has a default value
            args.flag_strategy = Some(args.flag_strategy.unwrap().to_lowercase());
            let strategy = match args.flag_strategy.as_deref() {
                Some("backward") | None => AsofStrategy::Backward,
                Some("forward") => AsofStrategy::Forward,
                Some("nearest") => AsofStrategy::Nearest,
                Some(s) => return fail_clierror!("Invalid asof strategy: {}", s),
            };

            let mut asof_options = AsOfOptions {
                strategy,
                ..Default::default()
            };

            if strategy == AsofStrategy::Nearest {
                if let Some(ref tolerance) = args.flag_tolerance {
                    // If the tolerance is an integer, it is tolerance number of rows.
                    // Otherwise, it is a tolerance date language spec.
                    if let Ok(numeric_tolerance) = tolerance.parse::<i64>() {
                        asof_options.tolerance = Some(AnyValue::Int64(numeric_tolerance));
                    } else {
                        asof_options.tolerance_str = Some(tolerance.into());
                    }
                }
            }
            if args.flag_left_by.is_some() {
                asof_options.left_by = Some(
                    args.flag_left_by
                        .unwrap()
                        .split(',')
                        .map(smartstring::SmartString::from)
                        .collect(),
                );
            }
            if args.flag_right_by.is_some() {
                asof_options.right_by = Some(
                    args.flag_right_by
                        .unwrap()
                        .split(',')
                        .map(smartstring::SmartString::from)
                        .collect(),
                );
            }
            join.run(JoinType::AsOf(asof_options), validation, true)
        }
        _ => fail!("Please pick exactly one join operation."),
    }?;

    if !args.flag_quiet {
        eprintln!("{join_shape:?}");
    }

    Ok(())
}

struct JoinStruct {
    left_lf:         LazyFrame,
    left_sel:        String,
    right_lf:        LazyFrame,
    right_sel:       String,
    output:          Option<String>,
    delim:           u8,
    streaming:       bool,
    datetime_format: Option<String>,
    date_format:     Option<String>,
    time_format:     Option<String>,
    float_precision: Option<usize>,
    null_value:      String,
}

impl JoinStruct {
    fn run(
        mut self,
        jointype: JoinType,
        validation: JoinValidation,
        asof_join: bool,
    ) -> CliResult<(usize, usize)> {
        let left_selcols: Vec<_> = self
            .left_sel
            .split(',')
            .map(polars::lazy::dsl::col)
            .collect();
        let right_selcols: Vec<_> = self
            .right_sel
            .split(',')
            .map(polars::lazy::dsl::col)
            .collect();

        let left_selcols_len = left_selcols.len();
        let right_selcols_len = right_selcols.len();

        if left_selcols_len != right_selcols_len {
            return fail_clierror!(
                "Both columns1 ({left_selcols:?}) and columns2 ({right_selcols:?}) must specify \
                 the same number of columns ({left_selcols_len } != {right_selcols_len})."
            );
        }

        let optimize_all = polars::lazy::frame::OptState {
            projection_pushdown:        true,
            predicate_pushdown:         true,
            type_coercion:              true,
            simplify_expr:              true,
            file_caching:               true,
            slice_pushdown:             true,
            common_subplan_elimination: true,
            streaming:                  self.streaming,
        };

        let mut join_results = if jointype == JoinType::Cross {
            self.left_lf
                .with_optimizations(optimize_all)
                .join_builder()
                .with(self.right_lf.with_optimizations(optimize_all))
                .how(JoinType::Cross)
                .validate(validation)
                .force_parallel(true)
                .finish()
                .collect()?
        } else {
            if asof_join {
                // sort by the asof columns, as asof joins require sorted join column data
                self.left_lf = self.left_lf.sort(&self.left_sel, SortOptions::default());
                self.right_lf = self.right_lf.sort(&self.right_sel, SortOptions::default());
            }

            self.left_lf
                .with_optimizations(optimize_all)
                .join_builder()
                .with(self.right_lf.with_optimizations(optimize_all))
                .left_on(left_selcols)
                .right_on(right_selcols)
                .how(jointype)
                .validate(validation)
                .force_parallel(true)
                .finish()
                .collect()?
        };

        // no need to use buffered writer here, as CsvWriter already does that
        let mut out_writer = match self.output {
            Some(output_file) => {
                let path = Path::new(&output_file);
                Box::new(File::create(path).unwrap()) as Box<dyn Write>
            }
            None => Box::new(io::stdout()) as Box<dyn Write>,
        };

        // shape is the number of rows and columns
        let join_shape = join_results.shape();

        CsvWriter::new(&mut out_writer)
            .has_header(true)
            .with_delimiter(self.delim)
            .with_datetime_format(self.datetime_format)
            .with_date_format(self.date_format)
            .with_time_format(self.time_format)
            .with_float_precision(self.float_precision)
            .with_null_value(self.null_value)
            .finish(&mut join_results)?;

        Ok(join_shape)
    }
}

impl Args {
    fn new_join(
        &self,
        try_parsedates: bool,
        infer_len: usize,
        low_memory: bool,
        ignore_errors: bool,
    ) -> CliResult<JoinStruct> {
        let delim = if let Some(delimiter) = self.flag_delimiter {
            delimiter.as_byte()
        } else {
            b','
        };

        let num_rows = if infer_len == 0 {
            None
        } else {
            Some(infer_len)
        };

        let mut left_lf = LazyCsvReader::new(&self.arg_input1)
            .has_header(true)
            .with_missing_is_null(self.flag_nulls)
            .with_delimiter(delim)
            .with_infer_schema_length(num_rows)
            .with_try_parse_dates(try_parsedates)
            .low_memory(low_memory)
            .with_ignore_errors(ignore_errors)
            .finish()?;

        if let Some(filter_left) = &self.flag_filter_left {
            let filter_left_expr = polars::sql::sql_expr(filter_left)?;
            left_lf = left_lf.filter(filter_left_expr);
        }

        let mut right_lf = LazyCsvReader::new(&self.arg_input2)
            .has_header(true)
            .with_missing_is_null(self.flag_nulls)
            .with_delimiter(delim)
            .with_infer_schema_length(num_rows)
            .with_try_parse_dates(try_parsedates)
            .low_memory(low_memory)
            .with_ignore_errors(ignore_errors)
            .finish()?;

        if let Some(filter_right) = &self.flag_filter_right {
            let filter_right_exprt = polars::sql::sql_expr(filter_right)?;
            right_lf = right_lf.filter(filter_right_exprt);
        }

        Ok(JoinStruct {
            left_lf,
            left_sel: self.arg_columns1.clone(),
            right_lf,
            right_sel: self.arg_columns2.clone(),
            output: self.flag_output.clone(),
            delim,
            streaming: self.flag_streaming,
            datetime_format: self.flag_datetime_format.clone(),
            date_format: self.flag_date_format.clone(),
            time_format: self.flag_time_format.clone(),
            float_precision: self.flag_float_precision,
            null_value: if self.flag_null_value == "<empty string>" {
                String::new()
            } else {
                self.flag_null_value.clone()
            },
        })
    }
}
