use anyhow::Context as _;
use askama::Template;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Debug)]
struct Version {
    minor: u32,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1.{}", self.minor)
    }
}

#[derive(Debug)]
enum ImplTarget {
    Type {
        ty: &'static str,
        double_ended: bool,
    },
    Generic {
        name: &'static str,
        bounds: &'static str,
        double_ended: bool,
    },
}

impl ImplTarget {
    fn double_ended(&self) -> bool {
        match self {
            ImplTarget::Type { double_ended, .. } | ImplTarget::Generic { double_ended, .. } => {
                *double_ended
            }
        }
    }
}

#[derive(Debug)]
struct TraitFunction {
    name: &'static str,
    args_before: &'static [(&'static str, &'static str)],
    args_after: &'static [(&'static str, &'static str)],
    ret_type: &'static str,
    since: Version,
    double_ended: bool,
}

static TARGETS: &[ImplTarget] = &[
    ImplTarget::Type{ ty: "&str", double_ended: false},
    ImplTarget::Type{ ty: "char", double_ended: true},
    ImplTarget::Generic {
        name: "F",
        bounds: "F: FnMut(char) -> bool",
        double_ended: true
    },
];

static FNS: &[TraitFunction] = &[
    TraitFunction {
        name: "contains",
        args_before: &[],
        args_after: &[],
        ret_type: "bool",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "starts_with",
        args_before: &[],
        args_after: &[],
        ret_type: "bool",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "ends_with",
        args_before: &[],
        args_after: &[],
        ret_type: "bool",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "find",
        args_before: &[],
        args_after: &[],
        ret_type: "Option<usize>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "rfind",
        args_before: &[],
        args_after: &[],
        ret_type: "Option<usize>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "split",
        args_before: &[],
        args_after: &[],
        ret_type: "impl Iterator<Item = &str>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "rsplit",
        args_before: &[],
        args_after: &[],
        ret_type: "impl Iterator<Item = &str>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "split_terminator",
        args_before: &[],
        args_after: &[],
        ret_type: "impl Iterator<Item = &str>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "rsplit_terminator",
        args_before: &[],
        args_after: &[],
        ret_type: "impl Iterator<Item = &str>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "splitn",
        args_before: &[("n", "usize")],
        args_after: &[],
        ret_type: "impl Iterator<Item = &str>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "rsplitn",
        args_before: &[("n", "usize")],
        args_after: &[],
        ret_type: "impl Iterator<Item = &str>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "matches",
        args_before: &[],
        args_after: &[],
        ret_type: "impl Iterator<Item = &str>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "rmatches",
        args_before: &[],
        args_after: &[],
        ret_type: "impl Iterator<Item = &str>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "match_indices",
        args_before: &[],
        args_after: &[],
        ret_type: "impl Iterator<Item = (usize, &str)>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "rmatch_indices",
        args_before: &[],
        args_after: &[],
        ret_type: "impl Iterator<Item = (usize, &str)>",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "trim_start_matches",
        args_before: &[],
        args_after: &[],
        ret_type: "&str",
        since: Version { minor: 30 },
        double_ended: false,
    },
    TraitFunction {
        name: "strip_prefix",
        args_before: &[],
        args_after: &[],
        ret_type: "Option<&str>",
        since: Version { minor: 45 },
        double_ended: false,
    },
    TraitFunction {
        name: "strip_suffix",
        args_before: &[],
        args_after: &[],
        ret_type: "Option<&str>",
        since: Version { minor: 45 },
        double_ended: false,
    },
    TraitFunction {
        name: "replace",
        args_before: &[],
        args_after: &[("to", "&str")],
        ret_type: "String",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "replacen",
        args_before: &[],
        args_after: &[("to", "&str"), ("count", "usize")],
        ret_type: "String",
        since: Version { minor: 0 },
        double_ended: false,
    },
    TraitFunction {
        name: "split_inclusive",
        args_before: &[],
        args_after: &[],
        ret_type: "impl Iterator<Item = &str>",
        since: Version { minor: 51 },
        double_ended: false,
    },
    TraitFunction {
        name: "trim_matches",
        args_before: &[],
        args_after: &[],
        ret_type: "&str",
        since: Version { minor: 0 },
        double_ended: true,
    },
    TraitFunction {
        name: "trim_end_matches",
        args_before: &[],
        args_after: &[],
        ret_type: "&str",
        since: Version { minor: 30 },
        double_ended: true,
    },
];

#[derive(Template)]
#[template(path = "lib.txt")]
struct LibTemplate {
    targets: &'static [ImplTarget],
    fns: &'static [TraitFunction],
}

#[derive(Template)]
#[template(path = "cargo.txt")]
struct CargoToml {
    latest_minor: u32,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <output_directory>", args[0]);
        std::process::exit(1);
    }

    let output_dir = PathBuf::from(&args[1]);

    let open = |p| -> anyhow::Result<BufWriter<File>> {
        let path = output_dir.join(p);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Error creating directory: {}", parent.display()))?;
        }
        Ok(BufWriter::new(
            std::fs::File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&path)
                .with_context(|| format!("Error opening file: {}", path.display()))?,
        ))
    };

    LibTemplate {
        targets: TARGETS,
        fns: FNS,
    }
    .write_into(&mut open("src/lib.rs")?)
    .context("Error rendering lib.rs template")?;

    CargoToml { latest_minor: 91 }
        .write_into(&mut open("Cargo.toml")?)
        .context("Error rendering Cargo.toml template")?;

    println!(
        "Successfully generated crate at: {} ({} functions)",
        output_dir.display(),
        FNS.len()
    );
    Ok(())
}
