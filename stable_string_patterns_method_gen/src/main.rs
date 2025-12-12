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
        write!(f, "1.{}",self.minor)
    }
}

#[derive(Debug)]
enum ImplTarget {
    Type(&'static str),
    Generic {
        name: &'static str,
        bounds: &'static str,
    },
}

#[derive(Debug)]
struct TraitFunction {
    name: &'static str,
    ret_type: &'static str,
    since: Version,
}

static TARGETS: &[ImplTarget] = &[
    ImplTarget::Type("&str"),
    ImplTarget::Type("char"),
    ImplTarget::Generic {
        name: "F",
        bounds: "F: FnMut(char) -> bool",
    },
];

static FNS: &[TraitFunction] = &[
    TraitFunction {
        name: "find",
        ret_type: "Option<usize>",
        since: Version { minor: 0 },
    },
    TraitFunction {
        name: "trim_start_matches",
        ret_type: "&str",
        since: Version { minor: 30 },
    },
    TraitFunction {
        name: "strip_prefix",
        ret_type: "Option<&str>",
        since: Version { minor: 45 },
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
                .truncate(true)
                .open(&path)
                .with_context(|| format!("Error opening file: {}", path.display()))?,
        ))
    };

    // Render lib.rs template
    LibTemplate {
        targets: TARGETS,
        fns: FNS,
    }
    .write_into(&mut open("src/lib.rs")?)
    .context("Error rendering lib.rs template")?;

    CargoToml { latest_minor: 91 }
        .write_into(&mut open("Cargo.toml")?)
        .context("Error rendering Cargo.toml template")?;

    println!("Successfully generated crate at: {}", output_dir.display());
    Ok(())
}
