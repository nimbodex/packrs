use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

const PACKED_EXTENSION: &str = "vlc";

pub struct PackArgs {
    pub input: String,
    pub output: Option<String>,
    pub verbose: bool,
}

pub fn run(argv: Vec<String>) -> Result<(), String> {
    if argv.is_empty() || argv.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return Ok(());
    }

    let args = parse_args(argv)?;
    pack(args)
}

fn parse_args(argv: Vec<String>) -> Result<PackArgs, String> {
    let mut input: Option<String> = None;
    let mut output: Option<String> = None;
    let mut verbose = false;

    let mut it = argv.into_iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "-o" | "--output" => {
                let v = it.next().ok_or("Missing value for -o/--output")?;
                output = Some(v);
            }
            "-v" | "--verbose" => verbose = true,
            s if s.starts_with('-') => return Err(format!("Unknown option: {s}")),
            s => {
                if input.is_none() {
                    input = Some(s.to_string());
                } else {
                    return Err(format!("Unexpected argument: {s}"));
                }
            }
        }
    }

    let input = input.ok_or("Missing <input> argument")?;
    Ok(PackArgs {
        input,
        output,
        verbose,
    })
}

fn pack(args: PackArgs) -> Result<(), String> {
    let file_path = args.input;

    let mut file = File::open(&file_path)
        .map_err(|e| format!("failed to open file '{}': {}", file_path, e))?;

    let mut buf = Vec::new();

    file.read_to_end(&mut buf)
        .map_err(|e| format!("failed to read file '{}': {}", file_path, e))?;

    let mut stdout = io::stdout();

    println!("Read {} bytes", buf.len());
    stdout
        .write_all(&buf)
        .map_err(|e| format!("failed to write to stdout: {e}"))?;

    stdout
        .flush()
        .map_err(|e| format!("failed to flush stdout: {e}"))?;

    Ok(())
}

fn print_help() {
    eprintln!(
        "\
USAGE:
  packrs pack <input> [OPTIONS]

OPTIONS:
  -o, --output <file>   Output archive path
  -v, --verbose         Verbose output
  -h, --help            Print help
"
    );
}

fn packed_file_name(path: &str) -> Result<String, String> {
    let file_stem = Path::new(path).file_stem();
    let stem = file_stem.ok_or("missing file stem")?;
    let stem_str = stem.to_str().ok_or("failed to convert stem to string")?;
    let result = stem_str.to_owned() + "." + PACKED_EXTENSION;

    Ok(result)
}
