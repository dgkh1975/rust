use std::error::Error;
use std::path::PathBuf;

fn main() {
    if let Err(e) = doit() {
        eprintln!("error: {}", e);
        eprintln!(
            "
This error was generated by the lint-docs tool.
This tool extracts documentation for lints from the source code and places
them in the rustc book. See the declare_lint! documentation
https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint_defs/macro.declare_lint.html
for an example of the format of documentation this tool expects.

To re-run these tests, run: ./x.py test --keep-stage=0 src/tools/lint-docs
The --keep-stage flag should be used if you have already built the compiler
and are only modifying the doc comments to avoid rebuilding the compiler.
"
        );
        std::process::exit(1);
    }
}

fn doit() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let mut src_path = None;
    let mut out_path = None;
    let mut rustc_path = None;
    let mut rustc_target = None;
    let mut rustc_linker = None;
    let mut verbose = false;
    let mut validate = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--src" => {
                src_path = match args.next() {
                    Some(s) => Some(PathBuf::from(s)),
                    None => return Err("--src requires a value".into()),
                };
            }
            "--out" => {
                out_path = match args.next() {
                    Some(s) => Some(PathBuf::from(s)),
                    None => return Err("--out requires a value".into()),
                };
            }
            "--rustc" => {
                rustc_path = match args.next() {
                    Some(s) => Some(PathBuf::from(s)),
                    None => return Err("--rustc requires a value".into()),
                };
            }
            "--rustc-target" => {
                rustc_target = match args.next() {
                    Some(s) => Some(s),
                    None => return Err("--rustc-target requires a value".into()),
                };
            }
            "--rustc-linker" => {
                rustc_linker = match args.next() {
                    Some(s) => Some(s),
                    None => return Err("--rustc-linker requires a value".into()),
                };
            }
            "-v" | "--verbose" => verbose = true,
            "--validate" => validate = true,
            s => return Err(format!("unexpected argument `{}`", s).into()),
        }
    }
    if src_path.is_none() {
        return Err("--src must be specified to the directory with the compiler source".into());
    }
    if out_path.is_none() {
        return Err("--out must be specified to the directory with the lint listing docs".into());
    }
    if rustc_path.is_none() {
        return Err("--rustc must be specified to the path of rustc".into());
    }
    if rustc_target.is_none() {
        return Err("--rustc-target must be specified to the rustc target".into());
    }
    let le = lint_docs::LintExtractor {
        src_path: &src_path.unwrap(),
        out_path: &out_path.unwrap(),
        rustc_path: &rustc_path.unwrap(),
        rustc_target: &rustc_target.unwrap(),
        rustc_linker: rustc_linker.as_deref(),
        verbose,
        validate,
    };
    le.extract_lint_docs()
}
