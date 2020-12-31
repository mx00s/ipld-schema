#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{convert::TryInto, fmt, path::PathBuf, str::FromStr};

use proptest::{arbitrary::Arbitrary, strategy::Strategy};

#[cfg(feature = "build-binary")]
use structopt::StructOpt;

pub mod schema;

// TODO: clean up unwraps

// TODO: create a suitable error type

#[derive(Clone, Copy, test_strategy::Arbitrary)]
pub struct Seed {
    inner: [u8; 32],
}

impl Seed {
    #[must_use]
    pub const fn fixed() -> Self {
        Self { inner: [0_u8; 32] }
    }
}

impl Default for Seed {
    /// By default seeds are non-deterministic.
    fn default() -> Self {
        Self {
            inner: rand::random(),
        }
    }
}

impl FromStr for Seed {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = {
            let bytes = base64::decode(s).unwrap();
            if bytes.len() == 32 {
                bytes.try_into().unwrap()
            } else {
                return Err("invalid input");
            }
        };

        Ok(Seed { inner })
    }
}

impl fmt::Display for Seed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", base64::encode(self.inner))
    }
}

impl fmt::Debug for Seed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "\"{}\".parse::<Seed>().unwrap()", self)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "build-binary", derive(StructOpt))]
pub enum Command {
    /// Validates IPLD schemas and data
    Validate {
        /// Path to IPLD schema file to validate
        #[cfg_attr(feature = "build-binary", structopt(parse(from_os_str)))]
        schema_file: PathBuf,

        /// Path to IPLD data file to validate against the specified schema
        #[cfg_attr(feature = "build-binary", structopt(parse(from_os_str)))]
        data_file: Option<PathBuf>,
    },
    /// Generates IPLD schemas and data
    Generate {
        /// Explicitly seed the PRNG for deterministic output
        ///
        /// If unspecified, a random seed is used.
        #[cfg_attr(feature = "build-binary", structopt(long, parse(try_from_str)))]
        seed: Option<Seed>,

        /// Path to IPLD schema file to use when generating data
        ///
        /// If unspecified, generates a schema instead of data.
        #[cfg_attr(feature = "build-binary", structopt(parse(from_os_str)))]
        schema_file: Option<PathBuf>,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "build-binary", derive(StructOpt))]
#[cfg_attr(feature = "build-binary", structopt(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION")))]
pub struct Opt {
    /// Dumps the arguments in roughly the form expected by the library's `run` function
    #[structopt(long)]
    dump_args: bool,

    #[cfg_attr(feature = "build-binary", structopt(subcommand))]
    cmd: Command,
}

#[allow(clippy::result_unit_err)]
#[allow(clippy::missing_errors_doc)]
pub fn run<W: std::io::Write>(opt: Opt, output: &mut W) -> Result<(), ()> {
    if opt.dump_args {
        writeln!(
            output,
            "{:#?}",
            Opt {
                dump_args: false,
                ..opt
            }
        )
        .unwrap();
        return Ok(());
    }

    match &opt.cmd {
        Command::Validate {
            schema_file,
            data_file,
        } => validate(schema_file, data_file, output),
        Command::Generate { seed, schema_file } => {
            generate(&seed.unwrap_or_default(), schema_file, output)
        }
    }
}

fn validate<P: AsRef<std::path::Path> + std::fmt::Debug, W: std::io::Write>(
    schema_file: &P,
    data_file: &Option<P>,
    out: &mut W,
) -> Result<(), ()> {
    match data_file {
        None => validate_schema(schema_file, out),
        Some(data) => validate_data(schema_file, data, out),
    }
}

fn validate_schema<P: AsRef<std::path::Path> + std::fmt::Debug, W: std::io::Write>(
    schema_file: &P,
    _out: &mut W,
) -> Result<(), ()> {
    schema::schema_dsl::parse(&std::fs::read_to_string(schema_file).unwrap()).unwrap();
    // TODO: write
    Ok(())
}

fn validate_data<P: AsRef<std::path::Path> + std::fmt::Debug, W: std::io::Write>(
    schema_file: &P,
    data_file: &P,
    _out: &mut W,
) -> Result<(), ()> {
    validate_schema(schema_file, &mut std::io::sink())?;

    todo!(
        "validate data ({:?}) using schema ({:?})",
        data_file,
        schema_file
    );
}

fn generate<P, W>(seed: &Seed, schema_file: &Option<P>, out: &mut W) -> Result<(), ()>
where
    P: AsRef<std::path::Path> + std::fmt::Debug,
    W: std::io::Write,
{
    let mut out = std::io::BufWriter::new(out);

    match schema_file {
        None => generate_schema(seed, &mut out),
        Some(schema) => generate_data(seed, schema, &mut out),
    }
}

// TODO: dump args in header comments

fn generate_schema<W: std::io::Write>(seed: &Seed, out: &mut W) -> Result<(), ()> {
    let config = proptest::test_runner::Config::default();
    let rng = proptest::test_runner::TestRng::from_seed(
        proptest::test_runner::RngAlgorithm::ChaCha,
        &seed.inner,
    );
    let mut runner = proptest::test_runner::TestRunner::new_with_rng(config, rng);

    let schema = schema::Schema::arbitrary()
        .new_tree(&mut runner)
        .unwrap()
        .current();

    writeln!(out, "##").unwrap();
    writeln!(
        out,
        "## Deterministically generated with {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
    .unwrap();
    writeln!(out, "##").unwrap();
    writeln!(out, "##   - reproduction seed: '{}'", seed).unwrap();
    writeln!(out, "##").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "{}", schema).unwrap();

    Ok(())
}

fn generate_data<P: AsRef<std::path::Path> + std::fmt::Debug, W: std::io::Write>(
    seed: &Seed,
    schema_file: &P,
    out: &mut W,
) -> Result<(), ()> {
    validate_schema(schema_file, &mut std::io::sink())?;

    writeln!(out, "##").unwrap();
    writeln!(
        out,
        "## Deterministically generated with {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
    .unwrap();
    writeln!(out, "##").unwrap();
    writeln!(out, "##   - reproduction seed: '{}'", seed).unwrap();
    writeln!(out, "##   - schema file: {:?}", schema_file).unwrap(); // TODO: consider emitting a CID for the schema file's contents too
    writeln!(out, "##").unwrap();
    writeln!(out).unwrap();

    todo!(
        "generate data using seed '{}' and schema {:?}",
        seed,
        schema_file
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_strategy::proptest;

    #[cfg(feature = "fast-test")]
    const CASES: u32 = 10;
    #[cfg(not(feature = "fast-test"))]
    const CASES: u32 = 1000;

    #[cfg(feature = "fast-test")]
    const MAX_SHRINK_ITERS: u32 = 1;
    #[cfg(not(feature = "fast-test"))]
    const MAX_SHRINK_ITERS: u32 = 10000;

    use insta::assert_debug_snapshot;

    #[test]
    fn snapshot_of_fixed_seed() {
        assert_debug_snapshot!(Seed::fixed());
    }

    #[test]
    #[cfg(not(feature = "fast-test"))]
    fn snapshot_of_schema_generated_from_fixed_seed() {
        let seed = Some(Seed::fixed());

        let mut schema_buffer = std::io::Cursor::new(vec![]);
        run(
            Opt {
                dump_args: false,
                cmd: Command::Generate {
                    seed,
                    schema_file: None,
                },
            },
            &mut schema_buffer,
        )
        .unwrap();

        assert_debug_snapshot!(schema::schema_dsl::parse(&String::from_utf8_lossy(
            &schema_buffer.into_inner()
        ))
        .unwrap());
    }

    #[test]
    #[cfg(not(feature = "fast-test"))]
    #[ignore]
    fn snapshot_of_data_generated_from_fixed_seed() {
        let seed = Some(Seed::fixed());

        let mut schema_file = tempfile::NamedTempFile::new().unwrap();
        run(
            Opt {
                dump_args: false,
                cmd: Command::Generate {
                    seed,
                    schema_file: None,
                },
            },
            &mut schema_file,
        )
        .unwrap();

        let mut data_buffer = std::io::Cursor::new(vec![]);
        run(
            Opt {
                dump_args: false,
                cmd: Command::Generate {
                    seed,
                    schema_file: Some(schema_file.path().into()),
                },
            },
            &mut data_buffer,
        )
        .unwrap();

        assert_debug_snapshot!(schema::schema_dsl::parse(&String::from_utf8_lossy(
            &data_buffer.into_inner()
        ))
        .unwrap());
    }

    #[proptest(cases = CASES, max_shrink_iters = MAX_SHRINK_ITERS)]
    fn generated_schemas_are_valid(seed: Seed) {
        let mut schema_file = tempfile::NamedTempFile::new()?;
        run(
            Opt {
                dump_args: false,
                cmd: Command::Generate {
                    seed: Some(seed),
                    schema_file: None,
                },
            },
            &mut schema_file,
        )
        .unwrap();

        let mut output = std::io::Cursor::new(vec![]);
        run(
            Opt {
                dump_args: false,
                cmd: Command::Validate {
                    schema_file: schema_file.path().into(),
                    data_file: None,
                },
            },
            &mut output,
        )
        .unwrap();

        // TODO: assertions about output

        schema_file.close()?;
    }

    #[proptest(cases = CASES, max_shrink_iters = MAX_SHRINK_ITERS)]
    #[ignore]
    fn generated_data_are_valid(seed: Seed) {
        let mut schema_file = tempfile::NamedTempFile::new()?;
        run(
            Opt {
                dump_args: false,
                cmd: Command::Generate {
                    seed: Some(seed),
                    schema_file: None,
                },
            },
            &mut schema_file,
        )
        .unwrap();

        let mut data_file = tempfile::NamedTempFile::new()?;
        run(
            Opt {
                dump_args: false,
                cmd: Command::Generate {
                    seed: Some(seed),
                    schema_file: Some(schema_file.path().into()),
                },
            },
            &mut data_file,
        )
        .unwrap();

        let mut output = std::io::Cursor::new(vec![]);
        run(
            Opt {
                dump_args: false,
                cmd: Command::Validate {
                    schema_file: schema_file.path().into(),
                    data_file: Some(data_file.path().into()),
                },
            },
            &mut output,
        )
        .unwrap();

        // TODO: assertions about output

        schema_file.close()?;
        data_file.close()?;
    }
}
